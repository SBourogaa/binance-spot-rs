mod client;
mod general;
mod account;
mod ticker;
mod trading;
mod market_data;

use client::BinanceSpotRestClient;
use crate::Result;
use crate::{BinanceConfig, RestConfig};

/**
 * Creates a new REST-based Binance client.
 * 
 * # Arguments
 * - `config`: Binance configuration with API credentials and REST-specific settings.
 * 
 * # Returns
 * - `BinanceRestClient`: New REST client instance.
 */
pub fn client(config: BinanceConfig<RestConfig>) -> Result<BinanceSpotRestClient> {
    BinanceSpotRestClient::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinanceConfig, RestConfig};

    /**
     * Tests the modular constructor pattern.
     */
    #[test]
    fn test_rest_constructor() {
        // Arrange
        let config: BinanceConfig<RestConfig> = BinanceConfig::<RestConfig>::builder()
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
    #[test]
    fn test_direct_rest_creation() {
        // Arrange
        let config: BinanceConfig<RestConfig> = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation should succeed");

        // Act
        let result = BinanceSpotRestClient::new(config);

        // Assert
        assert!(result.is_ok(), "Direct creation should work");
    }

    /**
     * Tests REST client creation with custom configuration.
     */
    #[test]
    fn test_rest_client_with_custom_config() {
        // Arrange
        let rest_config = crate::RestConfig::builder()
            .with_connection_timeout(std::time::Duration::from_secs(15))
            .with_request_timeout(std::time::Duration::from_secs(45))
            .build();

        let config: BinanceConfig<RestConfig> = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .with_rest_config(rest_config)
            .build()
            .expect("Config creation should succeed");

        // Act
        let result = client(config);

        // Assert
        assert!(result.is_ok(), "Custom config creation should work");
    }
}