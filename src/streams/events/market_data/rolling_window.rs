use serde::{Deserialize, Serialize};
use crate::types::responses::TickerStatisticsFull;

/**
 * Individual Rolling Window Ticker Stream Event
 * 
 * Rolling window ticker statistics for a single symbol, computed over a specific window.
 * 
 * Fields:
 * - `event_type`: Event type identifier (e.g., "1hTicker", "4hTicker", "1dTicker")
 * - `event_time`: Event timestamp in milliseconds
 * - `window`: Window size for internal tracking (not serialized)
 * - `ticker`: Flattened ticker statistics containing symbol, price data, volumes, and trade counts
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollingWindowTickerStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(skip)]
    pub window: String,
    #[serde(flatten)]
    pub ticker: TickerStatisticsFull,
}

/**
 * All Market Rolling Window Tickers Stream Event
 * 
 * Contains an array of rolling window ticker data for all symbols that changed.
 * Uses transparent serialization to directly serialize/deserialize as an array.
 * 
 * Fields:
 * - `tickers`: Array of rolling window ticker data for symbols that changed
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AllRollingWindowTickersStreamEvent {
    pub tickers: Vec<RollingWindowTickerStreamEvent>,
}