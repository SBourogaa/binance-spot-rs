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

mod api_error;
mod binance_error;
mod error_categories;
mod request_error;
mod server_error;
mod trading_error;
mod validation_errors;

pub use api_error::ApiError;
pub use binance_error::BinanceError;
pub use error_categories::ErrorCategory;
pub use request_error::RequestError;
pub use server_error::ServerError;
pub use trading_error::TradingError;
pub use validation_errors::{InvalidConfig, InvalidCredentials, InvalidParameter, InvalidUrl};
