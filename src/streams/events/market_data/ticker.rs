use serde::{Deserialize, Serialize};
use crate::types::responses::TickerStatisticsFull;

/**
 * Individual Ticker Stream Event
 * 
 * 24hr rolling window ticker statistics for a single symbol.
 * These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.
 * 
 * # Fields:
 * - `event_type`: Event type identifier (always "24hrTicker")
 * - `event_time`: Event timestamp in milliseconds
 * - `ticker`: Flattened ticker statistics containing comprehensive price, volume, and trade data
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TickerStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(flatten)]
    pub ticker: TickerStatisticsFull,
}

/**
 * All Market Tickers Stream Event
 * 
 * Contains an array of ticker data for all symbols that changed.
 * Uses transparent serialization to directly serialize/deserialize as an array.
 * 
 * # Fields:
 * - `tickers`: Array of ticker data for symbols that changed
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AllTickersStreamEvent {
    pub tickers: Vec<TickerStreamEvent>,
}