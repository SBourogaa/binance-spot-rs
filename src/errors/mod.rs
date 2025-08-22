/**
 * Binance Spot-API Error Definitions
 *
 * This module contains comprehensive error handling for the Binance API.
 * These error definitions are **in sync with the Binance API released on `2025-06-11`**.
 *
 * If the API changes, bump the version in `BINANCE_ERROR_VERSION` and update the error handling
 * according to latest specifications.
 */

/// Version tracking for error definitions - update when Binance API changes
pub const BINANCE_ERROR_VERSION: &str = "2025-06-11";

mod binance_error;
mod api_error;
mod validation_errors;
mod error_categories;
mod server_error;
mod request_error;
mod trading_error;

pub use binance_error::BinanceError;
pub use api_error::ApiError;
pub use validation_errors::{
    InvalidParameter,
    InvalidCredentials, 
    InvalidUrl,
    InvalidConfig,
};
pub use error_categories::ErrorCategory;
pub use server_error::ServerError;
pub use request_error::RequestError;
pub use trading_error::TradingError;

