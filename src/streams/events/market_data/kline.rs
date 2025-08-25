use crate::types::responses::Kline;
use serde::{Deserialize, Serialize};

/**
 * Kline Stream Data
 *
 * Contains kline-specific metadata and flattened kline OHLCV data.
 *
 * # Fields:
 * - `symbol`: Trading pair symbol
 * - `interval`: Kline interval (e.g., "1m", "1h")
 * - `first_trade_id`: First trade ID in this kline
 * - `last_trade_id`: Last trade ID in this kline
 * - `is_kline_closed`: Whether this kline is closed (complete)
 * - `kline`: Flattened kline data containing start time, close time, OHLCV prices, volumes, and trade counts
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KlineStreamData {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "x")]
    pub is_kline_closed: bool,
    #[serde(flatten)]
    pub kline: Kline,
}

/**
 * Kline Stream Event
 *
 * Real-time kline/candlestick updates pushed every second (1s interval) or every 2 seconds (other intervals).
 *
 * # Fields:
 * - `event_type`: Event type identifier (always "kline")
 * - `event_time`: Event timestamp in milliseconds
 * - `symbol`: Trading pair symbol
 * - `kline`: Kline data containing interval, trade IDs, prices, volumes, and metadata
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KlineStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: KlineStreamData,
}
