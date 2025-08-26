use anyhow::Context;
use reqwest;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::{debug, info, instrument};

use crate::Result;
use crate::{BinanceConfig, RestConfig, clients::common::generate_signature, errors::BinanceError};

/**
 * REST API client implementation with configurable HTTP settings.
 *
 * # Fields
 * - `config`: Binance configuration containing API credentials and REST-specific settings.
 * - `client`: HTTP client for making requests with optimized connection management.
 */
pub struct BinanceSpotRestClient {
    pub(crate) config: BinanceConfig<RestConfig>,
    pub(crate) client: reqwest::Client,
}

impl BinanceSpotRestClient {
    /**
     * Creates a new REST client instance with configuration-based HTTP settings.
     *
     * # Arguments
     * - `config`: Binance configuration with API credentials and REST settings.
     *
     * # Returns
     * - `Self`: New REST client instance.
     */
    pub fn new(config: BinanceConfig<RestConfig>) -> Result<Self> {
        let rest_config = config.rest_config();

        let client_builder = reqwest::Client::builder()
            .timeout(rest_config.request_timeout)
            .connect_timeout(rest_config.connection_timeout)
            .pool_max_idle_per_host(rest_config.pool_max_idle_per_host)
            .pool_idle_timeout(rest_config.pool_idle_timeout)
            .user_agent(&rest_config.user_agent);

        let client = client_builder
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { config, client })
    }

    /**
     * Helper method to handle Binance API error responses.
     *
     * # Arguments
     * - `response`: The HTTP response to check for errors.
     *
     * # Returns
     * - `Value`: Parsed JSON response.
     */
    pub(crate) async fn handle_response(&self, response: reqwest::Response) -> Result<Value> {
        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(error_json) = serde_json::from_str::<Value>(&text)
                && let (Some(code), Some(msg)) = (error_json.get("code"), error_json.get("msg"))
                && let (Some(code_num), Some(msg_str)) = (code.as_i64(), msg.as_str())
            {
                debug!(
                    error_type = "api_error",
                    error_code = code_num,
                    error_msg = msg_str,
                    http_status = %status,
                    "Binance API error"
                );
                return Err(BinanceError::Api(crate::errors::ApiError::new(
                    code_num as i32,
                    msg_str.to_string(),
                ))
                .into());
            }
            debug!(
                error_type = "http_error",
                http_status = %status,
                response_text = %text,
                "HTTP error response"
            );
            return Err(anyhow::anyhow!("HTTP {}: {}", status, text));
        }

        serde_json::from_str(&text).map_err(|e| {
            debug!(
                error_type = "parse_error",
                response_text = %text,
                parse_error = %e,
                "Failed to parse JSON response"
            );
            anyhow::Error::from(e).context("Failed to parse JSON response")
        })
    }

    /**
     * Sends a public (unsigned) request to the API.
     *
     * # Arguments
     * - `method`: HTTP method for the request.
     * - `endpoint`: API endpoint path.
     * - `params`: Serializable parameters for the request.
     *
     * # Returns
     * - `Value`: JSON response.
     */
    #[instrument(skip(self, params), fields(method = %method, endpoint = endpoint))]
    pub(crate) async fn send_request<T: Serialize>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: T,
    ) -> Result<Value> {
        let start = std::time::Instant::now();
        let prep_start = std::time::Instant::now();

        let params_query =
            serde_urlencoded::to_string(&params).context("Failed to serialize parameters")?;

        let url = if params_query.is_empty() {
            format!("{}{}", self.config.url(), endpoint)
        } else {
            format!("{}{}?{}", self.config.url(), endpoint, params_query)
        };

        let prep_duration = prep_start.elapsed();
        debug!(
            prep_duration_us = prep_duration.as_micros(),
            "Request preparation completed"
        );

        let network_start = std::time::Instant::now();
        let response = self
            .client
            .request(method, &url)
            .send()
            .await
            .context("Failed to send request")?;
        let network_duration = network_start.elapsed();

        let parse_start = std::time::Instant::now();
        let result = self.handle_response(response).await;
        let parse_duration = parse_start.elapsed();

        debug!(
            total_duration_us = start.elapsed().as_micros(),
            prep_duration_us = prep_duration.as_micros(),
            network_duration_us = network_duration.as_micros(),
            parse_duration_us = parse_duration.as_micros(),
            success = result.is_ok(),
            "REST request completed"
        );

        result
    }

    /**
     * Sends an authenticated request with Ed25519 signature.
     *
     * # Arguments
     * - `method`: HTTP method for the request.
     * - `endpoint`: API endpoint path.
     * - `params`: Serializable parameters for the request.
     *
     * # Returns
     * - `Value`: JSON response.
     */
    #[instrument(skip(self, params), fields(method = %method, endpoint = endpoint))]
    pub(crate) async fn send_signed_request<T: Serialize>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: T,
    ) -> Result<Value> {
        let start = std::time::Instant::now();
        let prep_start = std::time::Instant::now();

        let signer = self
            .config
            .signer()
            .ok_or_else(|| anyhow::anyhow!("No authentication configured"))?;

        let (signature, query_string) =
            generate_signature(&params, signer.as_ref(), self.config.recv_window(), false).await?;

        let final_query = format!("{}&signature={}", query_string, signature);
        let url = format!("{}{}?{}", self.config.url(), endpoint, final_query);

        let prep_duration = prep_start.elapsed();
        debug!(
            prep_duration_us = prep_duration.as_micros(),
            "Signed request preparation completed"
        );

        let network_start = std::time::Instant::now();
        let response = self
            .client
            .request(method, &url)
            .header("X-MBX-APIKEY", signer.get_api_key())
            .send()
            .await?;
        let network_duration = network_start.elapsed();

        let parse_start = std::time::Instant::now();
        let result = self.handle_response(response).await;
        let parse_duration = parse_start.elapsed();

        info!(
            total_duration_us = start.elapsed().as_micros(),
            prep_duration_us = prep_duration.as_micros(),
            network_duration_us = network_duration.as_micros(),
            parse_duration_us = parse_duration.as_micros(),
            success = result.is_ok(),
            "Signed REST request completed"
        );

        result
    }

    /**
     * Helper for public endpoint calls with validation and JSON parsing.
     *
     * # Arguments
     * - `method`: HTTP method for the request.
     * - `endpoint`: API endpoint path.
     * - `spec`: Request specification with validation.
     *
     * # Returns
     * - `R`: Parsed response object.
     */
    pub(crate) async fn request<S, R>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        spec: S,
    ) -> Result<R>
    where
        S: Serialize,
        R: DeserializeOwned,
    {
        let mut response = self.send_request(method, endpoint, spec).await?;

        if response.is_object() && std::any::type_name::<R>().starts_with("alloc::vec::Vec<") {
            response = serde_json::Value::Array(vec![response]);
        }

        serde_json::from_value(response).context("Failed to parse response")
    }

    /**
     * Helper for authenticated endpoint calls with validation and JSON parsing.
     *
     * # Arguments
     * - `method`: HTTP method for the request.
     * - `endpoint`: API endpoint path.
     * - `spec`: Request specification with validation.
     *
     * # Returns
     * - `R`: Parsed response object.
     */
    pub(crate) async fn signed_request<S, R>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        spec: S,
    ) -> Result<R>
    where
        S: Serialize,
        R: DeserializeOwned,
    {
        let response = self.send_signed_request(method, endpoint, spec).await?;
        serde_json::from_value(response).context("Failed to parse response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinanceConfig, RestConfig, errors::BinanceError};
    use serde_json::json;
    use std::time::Duration;

    /**
     * Tests client creation with default config.
     */
    #[test]
    fn test_new_default_config() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");

        // Act
        let result = BinanceSpotRestClient::new(config);

        // Assert
        assert!(result.is_ok());
    }

    /**
     * Tests client creation with custom timeouts.
     */
    #[test]
    fn test_new_custom_config() {
        // Arrange
        let rest_config = RestConfig::builder()
            .with_connection_timeout(Duration::from_secs(5))
            .with_request_timeout(Duration::from_secs(15))
            .build();

        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .with_rest_config(rest_config)
            .build()
            .expect("Config creation");

        // Act
        let result = BinanceSpotRestClient::new(config);

        // Assert
        assert!(result.is_ok());
    }

    /**
     * Tests handle_response with successful JSON.
     */
    #[tokio::test]
    async fn test_handle_response_success() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        let json_body = json!({"serverTime": 1234567890});
        let response = tokio_tungstenite::tungstenite::http::Response::builder()
            .status(200)
            .body(json_body.to_string())
            .unwrap();
        let reqwest_response = reqwest::Response::from(response);

        // Act
        let result = client.handle_response(reqwest_response).await;

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["serverTime"], 1234567890);
    }

    /**
     * Tests handle_response with Binance API error.
     */
    #[tokio::test]
    async fn test_handle_response_api_error() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        let error_body = json!({"code": -1121, "msg": "Invalid symbol."});
        let response = tokio_tungstenite::tungstenite::http::Response::builder()
            .status(400)
            .body(error_body.to_string())
            .unwrap();
        let reqwest_response = reqwest::Response::from(response);

        // Act
        let result = client.handle_response(reqwest_response).await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        let binance_error = error.downcast_ref::<BinanceError>();
        assert!(matches!(binance_error, Some(BinanceError::Api(_))));
    }

    /**
     * Tests handle_response with non-success status.
     */
    #[tokio::test]
    async fn test_handle_response_http_error() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        let response = tokio_tungstenite::tungstenite::http::Response::builder()
            .status(500)
            .body("Internal Server Error")
            .unwrap();
        let reqwest_response = reqwest::Response::from(response);

        // Act
        let result = client.handle_response(reqwest_response).await;

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("HTTP 500"));
    }

    /**
     * Tests handle_response with invalid JSON.
     */
    #[tokio::test]
    async fn test_handle_response_invalid_json() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        let response = tokio_tungstenite::tungstenite::http::Response::builder()
            .status(200)
            .body("{invalid json")
            .unwrap();
        let reqwest_response = reqwest::Response::from(response);

        // Act
        let result = client.handle_response(reqwest_response).await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse JSON")
        );
    }

    /**
     * Tests send_signed_request without signer configured.
     */
    #[tokio::test]
    async fn test_send_signed_request_no_signer() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        // Act
        let result = client
            .send_signed_request(reqwest::Method::GET, "/test", ())
            .await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No authentication configured")
        );
    }

    /**
     * Tests send_request parameter serialization.
     */
    #[tokio::test]
    async fn test_send_request_serialization() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        #[derive(serde::Serialize)]
        struct TestParams {
            symbol: String,
            limit: u32,
        }

        let params = TestParams {
            symbol: "BTCUSDT".to_string(),
            limit: 100,
        };

        // Act
        let result = client
            .send_request(reqwest::Method::GET, "/api/v3/depth", params)
            .await;

        // Assert
        match result {
            Ok(_) => {}
            Err(e) => {
                let error_string = e.to_string();
                assert!(
                    error_string.contains("Failed to send request")
                        || error_string.contains("dns error")
                        || error_string.contains("connection"),
                    "Should be a network error, not serialization error: {}",
                    error_string
                );
            }
        }
    }

    /**
     * Tests request helper method type conversion.
     */
    #[tokio::test]
    async fn test_request_helper_type_conversion() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        // Act
        let result: Result<serde_json::Value> = client
            .request(reqwest::Method::GET, "/api/v3/ping", ())
            .await;

        // Assert
        match result {
            Ok(_) => {}
            Err(e) => {
                let error_string = e.to_string();
                assert!(
                    error_string.contains("Failed to send request")
                        || error_string.contains("dns error")
                        || error_string.contains("connection")
                        || error_string.contains("Failed to parse response"),
                    "Should be a network or parse error: {}",
                    error_string
                );
            }
        }
    }

    /**
     * Tests signed_request helper without authentication.
     */
    #[tokio::test]
    async fn test_signed_request_helper_no_auth() {
        // Arrange
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotRestClient::new(config).unwrap();

        // Act
        let result: Result<serde_json::Value> = client
            .signed_request(reqwest::Method::GET, "/api/v3/account", ())
            .await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No authentication configured")
        );
    }
}
