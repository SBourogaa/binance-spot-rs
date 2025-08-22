use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Average price data from Binance API.
 * 
 * Represents current average price for a symbol over a time window.
 * 
 * # Fields
 * - `mins`: Average price interval in minutes.
 * - `price`: Current average price for the symbol.
 * - `close_time`: Timestamp of the last trade used in calculation.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AveragePrice {
    #[serde(rename = "mins", alias = "i")]
    pub minutes: u32,
    #[serde(with = "rust_decimal::serde::str")]
    #[serde(alias = "w")]
    pub price: Decimal,
    #[serde(alias = "T")]
    pub close_time: u64,
}