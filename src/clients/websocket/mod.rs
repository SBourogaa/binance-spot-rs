pub mod client;
pub mod general;
pub mod account;
pub mod ticker;
pub mod trading;
pub mod market_data;

use client::BinanceSpotWebSocketClient;
use crate::Result;
use crate::{BinanceConfig, WebSocketConfig};

/**
 * Creates a new WebSocket-based Binance client.
 * 
 * # Arguments
 * - `config`: Binance configuration with API credentials and WebSocket-specific settings.
 * 
 * # Returns
 * - `BinanceWebSocketClient`: New WebSocket client instance.
 */
pub fn client(config: BinanceConfig<WebSocketConfig>) -> Result<BinanceSpotWebSocketClient> {
    BinanceSpotWebSocketClient::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinanceConfig, WebSocketConfig};

    /**
     * Tests the modular constructor pattern.
     */
    #[tokio::test]
    async fn test_websocket_constructor() {
        // Arrange
        let config: BinanceConfig<WebSocketConfig> = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation should succeed");

        // Act
        let result = client(config);

        // Assert
        assert!(result.is_ok(), "Constructor should work");
    }

    /**
     * Tests direct struct instantiation.
     */
    #[tokio::test]
    async fn test_direct_websocket_creation() {
        // Arrange
        let config: BinanceConfig<WebSocketConfig> = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation should succeed");

        // Act
        let result = BinanceSpotWebSocketClient::new(config);

        // Assert
        assert!(result.is_ok(), "Direct creation should work");
    }

    /**
     * Tests WebSocket client creation with custom configuration.
     */
    #[tokio::test]
    async fn test_websocket_client_with_custom_config() {
        // Arrange
        let ws_config = crate::WebSocketConfig::builder()
            .with_max_reconnects(10)
            .build();

        let config: BinanceConfig<WebSocketConfig> = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .with_websocket_config(ws_config)
            .build()
            .expect("Config creation should succeed");

        // Act
        let result = client(config);

        // Assert
        assert!(result.is_ok(), "Custom config creation should work");
    }
}