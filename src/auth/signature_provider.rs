use async_trait::async_trait;

use crate::Result;

/**
 * Trait for providing digital signatures for authenticated Binance API requests.
 */
#[async_trait]
pub trait SignatureProvider: Send + Sync + std::fmt::Debug {
    /**
     * Returns the API key for request authentication headers.
     *
     * This key is included in the X-MBX-APIKEY header for all authenticated
     * requests to identify the account making the request.
     *
     * # Returns
     * - `&str`: The API key string for header authentication.
     */
    fn get_api_key(&self) -> &str;

    /**
     * Generates a digital signature for the given payload.
     *
     * The payload format depends on the request type:
     * - REST: Query parameters sorted alphabetically and URL-encoded
     * - WebSocket: JSON parameters sorted alphabetically and formatted as key=value pairs
     *
     * # Arguments
     * - `payload`: The request payload string to be signed.
     *
     * # Returns
     * - `String`: The generated signature as a hex or base64 string.
     */
    async fn sign(&self, payload: &str) -> Result<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::InvalidCredentials;

    /**
     * Mock implementation of SignatureProvider for testing trait behavior.
     *
     * # Fields
     * - `api_key`: Test API key for authentication.
     * - `should_fail`: Flag to simulate signature failures.
     */
    #[derive(Debug)]
    struct MockSigner {
        api_key: String,
        should_fail: bool,
    }

    impl MockSigner {
        /**
         * Creates a new mock signer for testing.
         *
         * # Arguments
         * - `api_key`: The API key to return from get_api_key().
         * - `should_fail`: Whether sign() should return an error.
         *
         * # Returns
         * - `Self`: A new MockSigner instance.
         */
        fn new(api_key: impl Into<String>, should_fail: bool) -> Self {
            Self {
                api_key: api_key.into(),
                should_fail,
            }
        }
    }

    #[async_trait]
    impl SignatureProvider for MockSigner {
        fn get_api_key(&self) -> &str {
            &self.api_key
        }

        async fn sign(&self, payload: &str) -> Result<String> {
            if self.should_fail {
                // Use the appropriate error type for signature failures
                Err(InvalidCredentials::signature_failed("Mock signature failure").into())
            } else {
                // Generate a deterministic mock signature for testing
                Ok(format!("mock_signature_{}", payload.len()))
            }
        }
    }

    /**
     * Tests that SignatureProvider trait provides correct API key access.
     * Tested using mock implementation with known API key.
     */
    #[tokio::test]
    async fn test_signature_provider_api_key() {
        // Arrange
        let test_api_key = "test_api_key_12345";
        let signer = MockSigner::new(test_api_key, false);

        // Act
        let returned_key = signer.get_api_key();

        // Assert
        assert_eq!(returned_key, test_api_key);
    }

    /**
     * Tests that SignatureProvider can generate signatures for various payload sizes.
     * Tested using mock implementation with different payload lengths.
     */
    #[tokio::test]
    async fn test_signature_provider_sign_success() {
        // Arrange
        let signer = MockSigner::new("test_key", false);
        let short_payload = "symbol=BTCUSDT";
        let long_payload = "symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1.0000&price=50000.00&timestamp=1234567890";

        // Act
        let short_result = signer.sign(short_payload).await;
        let long_result = signer.sign(long_payload).await;

        // Assert
        assert!(short_result.is_ok(), "Short payload signing should succeed");
        assert!(long_result.is_ok(), "Long payload signing should succeed");

        let short_signature = short_result.unwrap();
        let long_signature = long_result.unwrap();

        // Verify deterministic behavior
        assert_eq!(
            short_signature,
            format!("mock_signature_{}", short_payload.len())
        );
        assert_eq!(
            long_signature,
            format!("mock_signature_{}", long_payload.len())
        );
    }

    /**
     * Tests that SignatureProvider properly handles signature failures.
     * Tested using mock implementation configured to fail.
     */
    #[tokio::test]
    async fn test_signature_provider_sign_failure() {
        // Arrange
        let signer = MockSigner::new("test_key", true);
        let payload = "symbol=BTCUSDT&timestamp=1234567890";

        // Act
        let result = signer.sign(payload).await;

        // Assert
        assert!(result.is_err(), "Configured failure should return error");

        // Since you're using anyhow::Result, check the error chain
        let error = result.unwrap_err();
        let error_chain = format!("{:#}", error);
        assert!(error_chain.contains("Mock signature failure"));
    }

    /**
     * Tests that SignatureProvider handles empty and special character payloads.
     * Tested using edge case inputs for payload signing.
     */
    #[tokio::test]
    async fn test_signature_provider_edge_cases() {
        // Arrange
        let signer = MockSigner::new("test_key", false);

        // Test cases
        let test_cases = vec![
            ("", "Empty payload"),
            ("symbol=BTC%2FUSDT", "URL encoded payload"),
            (
                "symbol=BTCUSDT&price=50000.123456789",
                "High precision decimals",
            ),
            (
                "a=1&b=2&c=3&d=4&e=5&f=6&g=7&h=8&i=9&j=10",
                "Many parameters",
            ),
        ];

        for (payload, description) in test_cases {
            // Act
            let result = signer.sign(payload).await;

            // Assert
            assert!(result.is_ok(), "{} should be signable", description);
            let signature = result.unwrap();
            assert!(
                !signature.is_empty(),
                "{} signature should not be empty",
                description
            );
        }
    }

    /**
     * Tests that SignatureProvider trait objects work correctly.
     * Tested using boxed trait objects for dynamic dispatch.
     */
    #[tokio::test]
    async fn test_signature_provider_trait_object() {
        // Arrange
        let signer: Box<dyn SignatureProvider> = Box::new(MockSigner::new("boxed_key", false));
        let payload = "symbol=ETHUSDT&timestamp=9876543210";

        // Act
        let api_key = signer.get_api_key();
        let signature_result = signer.sign(payload).await;

        // Assert
        assert_eq!(api_key, "boxed_key");
        assert!(
            signature_result.is_ok(),
            "Boxed signer should work correctly"
        );
    }

    /**
     * Tests that SignatureProvider trait objects implement Debug correctly.
     * Tested using debug formatting of trait objects.
     */
    #[tokio::test]
    async fn test_signature_provider_debug_trait() {
        // Arrange
        let signer: Box<dyn SignatureProvider> = Box::new(MockSigner::new("debug_test_key", false));

        // Act
        let debug_output = format!("{:?}", signer);

        // Assert
        assert!(
            debug_output.contains("MockSigner"),
            "Debug output should contain struct name"
        );
        assert!(
            debug_output.contains("debug_test_key"),
            "Debug output should contain API key"
        );
    }
}
