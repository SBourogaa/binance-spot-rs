use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Individual Symbol Mini Ticker Stream Event
 *
 * 24hr rolling window mini-ticker statistics for a single symbol.
 * These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.
 *
 * # Fields:
 * - `event_type`: Event type identifier (always "24hrMiniTicker")
 * - `event_time`: Event timestamp in milliseconds
 * - `symbol`: Trading pair symbol
 * - `close_price`: Close price
 * - `open_price`: Open price  
 * - `high_price`: High price
 * - `low_price`: Low price
 * - `volume`: Total traded base asset volume
 * - `quote_volume`: Total traded quote asset volume
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiniTickerStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", with = "rust_decimal::serde::str")]
    pub close_price: Decimal,
    #[serde(rename = "o", with = "rust_decimal::serde::str")]
    pub open_price: Decimal,
    #[serde(rename = "h", with = "rust_decimal::serde::str")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "rust_decimal::serde::str")]
    pub low_price: Decimal,
    #[serde(rename = "v", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(rename = "q", with = "rust_decimal::serde::str")]
    pub quote_volume: Decimal,
}

/**
 * All Market Mini Tickers Stream Event
 *
 * Contains an array of mini ticker data for all symbols that changed.
 * Uses transparent serialization to directly serialize/deserialize as an array.
 *
 * # Fields:
 * - `tickers`: Array of mini ticker data for symbols that changed
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AllMiniTickersStreamEvent {
    pub tickers: Vec<MiniTickerData>,
}

/**
 * Mini Ticker Data
 *
 * Individual mini ticker data used within the all market mini tickers array.
 * Same structure as MiniTickerStreamEvent but used in array context.
 *
 * # Fields:
 * - `event_type`: Event type identifier (always "24hrMiniTicker")
 * - `event_time`: Event timestamp in milliseconds
 * - `symbol`: Trading pair symbol
 * - `close_price`: Close price
 * - `open_price`: Open price  
 * - `high_price`: High price
 * - `low_price`: Low price
 * - `volume`: Total traded base asset volume
 * - `quote_volume`: Total traded quote asset volume
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiniTickerData {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", with = "rust_decimal::serde::str")]
    pub close_price: Decimal,
    #[serde(rename = "o", with = "rust_decimal::serde::str")]
    pub open_price: Decimal,
    #[serde(rename = "h", with = "rust_decimal::serde::str")]
    pub high_price: Decimal,
    #[serde(rename = "l", with = "rust_decimal::serde::str")]
    pub low_price: Decimal,
    #[serde(rename = "v", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(rename = "q", with = "rust_decimal::serde::str")]
    pub quote_volume: Decimal,
}
