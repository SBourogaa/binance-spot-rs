use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Average Price Stream Event
 * 
 * Contains average price changes over a fixed time interval.
 * 
 * # Fields:
 * - `event_type`: Event type identifier (always "avgPrice")
 * - `event_time`: Event timestamp in milliseconds
 * - `symbol`: Trading pair symbol
 * - `interval`: Average price interval (e.g., "5m")
 * - `price`: Average price over the interval
 * - `last_trade_time`: Last trade timestamp in milliseconds
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AveragePriceStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "w", with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "T")]
    pub last_trade_time: u64,
}