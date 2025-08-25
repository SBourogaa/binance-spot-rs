use std::time::Duration;

use crate::Result;
use crate::{
    BinanceConfig, BinanceSpotClient, RestConfig, WebSocketConfig,
    clients::{rest, websocket},
};

/**
 * Creates an unauthenticated REST client for testnet.
 */
#[allow(dead_code)]
pub fn create_rest_client() -> Result<impl BinanceSpotClient> {
    let config = BinanceConfig::<RestConfig>::builder()
        .with_testnet()
        .build()?;
    rest::client(config)
}

/**
 * Creates an unauthenticated WebSocket client for testnet.
 */
#[allow(dead_code)]
pub fn create_websocket_client() -> Result<impl BinanceSpotClient> {
    let config = BinanceConfig::<WebSocketConfig>::builder()
        .with_testnet()
        .build()?;
    websocket::client(config)
}

/**
 * Creates an authenticated REST client for testnet.
 */
#[allow(dead_code)]
pub fn create_authenticated_rest_client() -> Result<impl BinanceSpotClient> {
    let config = BinanceConfig::<RestConfig>::builder()
        .with_testnet()
        .with_credentials_from_file(
            std::env::var("BINANCE_TESTNET_API_KEY").expect("BINANCE_TESTNET_API_KEY required"),
            std::env::var("BINANCE_TESTNET_PEM_FILE").expect("BINANCE_TESTNET_PEM_FILE required"),
        )?
        .build()?;
    rest::client(config)
}

/**
 * Creates an authenticated WebSocket client for testnet.
 */
#[allow(dead_code)]
pub fn create_authenticated_websocket_client() -> Result<impl BinanceSpotClient> {
    let config = BinanceConfig::<WebSocketConfig>::builder()
        .with_testnet()
        .with_credentials_from_file(
            std::env::var("BINANCE_TESTNET_API_KEY").expect("BINANCE_TESTNET_API_KEY required"),
            std::env::var("BINANCE_TESTNET_PEM_FILE").expect("BINANCE_TESTNET_PEM_FILE required"),
        )?
        .build()?;
    websocket::client(config)
}

/**
 * Wraps API calls with timeout to prevent hanging tests.
 */
#[allow(dead_code)]
pub async fn with_timeout<T>(future: impl std::future::Future<Output = Result<T>>) -> Result<T> {
    tokio::time::timeout(Duration::from_secs(30), future)
        .await
        .map_err(|_| anyhow::anyhow!("Request timed out"))?
}

/**
 * Asserts that a result contains an API error with the expected code.
 */
#[allow(dead_code)]
pub fn expect_api_error<T>(result: &Result<T>, expected_code: i32) {
    match result {
        Err(e) => {
            if let Some(api_error) = e.downcast_ref::<crate::BinanceError>() {
                if let Some(code) = api_error.api_code() {
                    assert_eq!(
                        code, expected_code,
                        "Expected error code {}, got {}",
                        expected_code, code
                    );
                } else {
                    panic!(
                        "Expected API error with code {}, got non-API error: {}",
                        expected_code, e
                    );
                }
            } else {
                panic!("Expected Binance API error, got: {}", e);
            }
        }
        Ok(_) => panic!(
            "Expected error with code {}, but operation succeeded",
            expected_code
        ),
    }
}
