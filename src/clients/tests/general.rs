#[cfg(test)]
mod tests {
    use crate::{
        clients::{
            r#trait::GeneralClient,
            tests::helpers::*,
        },
        types::{
            requests::ExchangeInfoSpec,
            responses::{ServerTime, ExchangeInfo},
        },
        errors::{BinanceError, ErrorCategory, RequestError},
    };

    /**
     * Validates server time response is reasonable.
     */
    fn assert_valid_server_time(time: &ServerTime) {
        let now = chrono::Utc::now();
        let diff = (now - time.server_time).abs();
        
        assert!(
            diff < chrono::Duration::hours(1),
            "Server time should be close to current time"
        );
    }

    /**
     * Validates exchange info response structure.
     */
    fn assert_valid_exchange_info(info: &ExchangeInfo, expected_symbol: Option<&str>) {
        assert!(!info.symbols.is_empty(), "Should have at least one symbol");
        assert!(!info.rate_limits.is_empty(), "Should have rate limits");
        
        if let Some(symbol) = expected_symbol {
            assert!(
                info.symbols.iter().any(|s| s.symbol == symbol),
                "Should contain expected symbol: {}", symbol
            );
        }
        
        let first_symbol = &info.symbols[0];
        assert!(!first_symbol.symbol.is_empty(), "Symbol name should not be empty");
        assert!(!first_symbol.base_asset.is_empty(), "Base asset should not be empty");
        assert!(!first_symbol.quote_asset.is_empty(), "Quote asset should not be empty");
    }

    /**
     * Tests ping connectivity for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_ping() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        
        // Act
        let rest_result = with_timeout(rest_client.ping()).await;
        let ws_result = with_timeout(ws_client.ping()).await;
        
        // Assert
        assert!(rest_result.is_ok(), "REST ping should succeed");
        assert!(ws_result.is_ok(), "WebSocket ping should succeed");
    }

    /**
     * Tests server time retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_server_time() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        
        // Act
        let rest_time = with_timeout(rest_client.server_time()).await.expect("REST server time");
        let ws_time = with_timeout(ws_client.server_time()).await.expect("WebSocket server time");
        
        // Assert
        assert_valid_server_time(&rest_time);
        assert_valid_server_time(&ws_time);
        
        let diff = (rest_time.server_time - ws_time.server_time).abs();
        assert!(
            diff < chrono::Duration::seconds(10),
            "REST and WebSocket server times should be close"
        );
    }

    /**
     * Tests exchange info retrieval without filters for both clients.
     */
    #[tokio::test]
    async fn test_exchange_info_all_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        
        // Act
        let rest_spec = ExchangeInfoSpec::new().build().expect("Spec validation");
        let rest_info = with_timeout(rest_client.exchange_info(rest_spec)).await.expect("REST exchange info");
        
        let ws_spec = ExchangeInfoSpec::new().build().expect("Spec validation");
        let ws_info = with_timeout(ws_client.exchange_info(ws_spec)).await.expect("WebSocket exchange info");
        
        // Assert
        assert_valid_exchange_info(&rest_info, None);
        assert_valid_exchange_info(&ws_info, None);
        
        assert_eq!(rest_info.symbols.len(), ws_info.symbols.len(), "Symbol count should match");
    }

    /**
     * Tests exchange info retrieval with specific symbol filter.
     */
    #[tokio::test]
    async fn test_exchange_info_specific_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = ExchangeInfoSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let rest_info = with_timeout(rest_client.exchange_info(rest_spec)).await.expect("REST exchange info");
        
        let ws_spec = ExchangeInfoSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let ws_info = with_timeout(ws_client.exchange_info(ws_spec)).await.expect("WebSocket exchange info");
        
        // Assert
        assert_valid_exchange_info(&rest_info, Some(test_symbol));
        assert_valid_exchange_info(&ws_info, Some(test_symbol));
        
        assert_eq!(rest_info.symbols.len(), 1, "Should return exactly one symbol");
        assert_eq!(ws_info.symbols.len(), 1, "Should return exactly one symbol");
        assert_eq!(rest_info.symbols[0].symbol, test_symbol);
        assert_eq!(ws_info.symbols[0].symbol, test_symbol);
    }

    /**
     * Tests exchange info retrieval with multiple symbols filter.
     */
    #[tokio::test]
    async fn test_exchange_info_multiple_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbols = vec!["BTCUSDT", "ETHUSDT"];
        
        // Act
        let rest_spec = ExchangeInfoSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let rest_info = with_timeout(rest_client.exchange_info(rest_spec)).await.expect("REST exchange info");
        
        let ws_spec = ExchangeInfoSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let ws_info = with_timeout(ws_client.exchange_info(ws_spec)).await.expect("WebSocket exchange info");
        
        // Assert
        assert_eq!(rest_info.symbols.len(), 2, "Should return exactly two symbols");
        assert_eq!(ws_info.symbols.len(), 2, "Should return exactly two symbols");
        assert!(rest_info.symbols.iter().any(|s| s.symbol == "BTCUSDT"));
        assert!(rest_info.symbols.iter().any(|s| s.symbol == "ETHUSDT"));
        assert!(ws_info.symbols.iter().any(|s| s.symbol == "BTCUSDT"));
        assert!(ws_info.symbols.iter().any(|s| s.symbol == "ETHUSDT"));
    }

    /**
     * Tests exchange info error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_exchange_info_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = ExchangeInfoSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.exchange_info(rest_spec)).await;

        let ws_spec = ExchangeInfoSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.exchange_info(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(info) => panic!("Expected {} error, got successful response: {:?}", client_name, info),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -1121
                                && api_err.category == ErrorCategory::RequestIssues
                                && api_err.request_error == Some(RequestError::BadSymbol)
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }
    }

}