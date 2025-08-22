use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Ticker price data from Binance API.
 * 
 * Represents the latest market price for a trading symbol.
 * 
 * # Fields
 * - `symbol`: Trading symbol name.
 * - `price`: Latest market price.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TickerPrice {
    pub symbol: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
}