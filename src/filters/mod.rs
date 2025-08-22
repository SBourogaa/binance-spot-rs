/**
 * Binance Spot-API Filter Definitions
 *
 * This module defines strongly-typed symbol-level and exchange-level filters
 * exactly as they are returned by the `/exchangeInfo` endpoint.
 * These definitions are **in sync with the Binance API released on
 * `2025-06-11`**.
 *
 * If the API changes, bump `BINANCE_FILTER_VERSION` and update the filter
 * structs accordingly.
 */
pub const BINANCE_FILTER_VERSION: &str = "2025-06-11";

pub mod symbol;
pub mod exchange;
mod symbol_filter;
mod exchange_filter;
mod parse_filter_failure;
mod parse_trading_rejection;

pub use symbol::*;
pub use exchange::*;
pub use symbol_filter::SymbolFilter;
pub use exchange_filter::ExchangeFilter;
pub use parse_filter_failure::{FilterFailure, parse_filter_failure};
pub use parse_trading_rejection::{TradingRejectionMessage, parse_trading_rejection};
