use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Ticker statistics response variants from Binance API.
 * 
 * Represents either FULL or MINI format ticker statistics for trading symbols.
 * Used by ticker_24hr, ticker_trading_day, and ticker_rolling_window endpoints.
 *
 * # Variants
 * - `Full`: Complete ticker statistics with all available fields.
 * - `Mini`: Essential ticker statistics with basic OHLCV data only.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum TickerStatistics {
    /// Full ticker statistics with all available fields
    Full(TickerStatisticsFull),
    /// Mini ticker statistics with essential fields only
    Mini(TickerStatisticsMini),
}

/**
 * Full ticker statistics from Binance API.
 * 
 * Contains comprehensive 24-hour, trading day, or rolling window statistics
 * with all available price, volume, and trading metrics.
 * 
 * Enhanced with WebSocket Stream aliases for `<symbol>@ticker` compatibility.
 *
 * # Fields
 * - `symbol`: Trading symbol name.
 * - `price_change`: Absolute price change.
 * - `price_change_percent`: Relative price change in percent.
 * - `weighted_avg_price`: Volume weighted average price.
 * - `prev_close_price`: Previous close price (optional).
 * - `last_quantity`: Quantity of the last trade (optional).
 * - `bid_price`: Best bid price (optional).
 * - `bid_quantity`: Best bid quantity (optional).
 * - `ask_price`: Best ask price (optional).
 * - `ask_quantity`: Best ask quantity (optional).
 * - `open_price`: Opening price for the period.
 * - `high_price`: Highest price during the period.
 * - `low_price`: Lowest price during the period.
 * - `last_price`: Last trade price.
 * - `volume`: Base asset volume.
 * - `quote_volume`: Quote asset volume.
 * - `open_time`: Period start time.
 * - `close_time`: Period end time.
 * - `first_id`: First trade ID in the period.
 * - `last_id`: Last trade ID in the period.
 * - `count`: Number of trades in the period.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TickerStatisticsFull {
    #[serde(alias = "s")]
    pub symbol: String,
    #[serde(alias = "p", with = "rust_decimal::serde::str")]
    pub price_change: Decimal,
    #[serde(alias = "P", with = "rust_decimal::serde::str")]
    pub price_change_percent: Decimal,
    #[serde(alias = "w", with = "rust_decimal::serde::str")]
    pub weighted_avg_price: Decimal,
    
    // Optional fields - use str_option for Decimal strings
    #[serde(default, alias = "x", with = "rust_decimal::serde::str_option", skip_serializing_if = "Option::is_none")]
    pub prev_close_price: Option<Decimal>,
    #[serde(rename = "lastQty", default, alias = "Q", with = "rust_decimal::serde::str_option", skip_serializing_if = "Option::is_none")]
    pub last_quantity: Option<Decimal>,
    #[serde(default, alias = "b", with = "rust_decimal::serde::str_option", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<Decimal>,
    #[serde(rename = "bidQty", default, alias = "B", with = "rust_decimal::serde::str_option", skip_serializing_if = "Option::is_none")]
    pub bid_quantity: Option<Decimal>,
    #[serde(default, alias = "a", with = "rust_decimal::serde::str_option", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<Decimal>,
    #[serde(rename = "askQty", default, alias = "A", with = "rust_decimal::serde::str_option", skip_serializing_if = "Option::is_none")]
    pub ask_quantity: Option<Decimal>,
    
    // Required fields
    #[serde(alias = "o", with = "rust_decimal::serde::str")]
    pub open_price: Decimal,
    #[serde(alias = "h", with = "rust_decimal::serde::str")]
    pub high_price: Decimal,
    #[serde(alias = "l", with = "rust_decimal::serde::str")]
    pub low_price: Decimal,
    #[serde(alias = "c", with = "rust_decimal::serde::str")]
    pub last_price: Decimal,
    #[serde(alias = "v", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(alias = "q", with = "rust_decimal::serde::str")]
    pub quote_volume: Decimal,
    #[serde(alias = "O")]
    pub open_time: u64,
    #[serde(alias = "C")]
    pub close_time: u64,
    #[serde(alias = "F")]
    pub first_id: i64,
    #[serde(alias = "L")]
    pub last_id: i64,
    #[serde(alias = "n")]
    pub count: u64,
}

/**
 * Mini ticker statistics from Binance API.
 * 
 * Contains essential OHLCV data and trade metrics without
 * detailed price change statistics or bid/ask information.
 *
 * # Fields
 * - `symbol`: Trading symbol name.
 * - `open_price`: Opening price for the period.
 * - `high_price`: Highest price during the period.
 * - `low_price`: Lowest price during the period.
 * - `last_price`: Last trade price.
 * - `volume`: Base asset volume.
 * - `quote_volume`: Quote asset volume.
 * - `open_time`: Period start time.
 * - `close_time`: Period end time.
 * - `first_id`: First trade ID in the period.
 * - `last_id`: Last trade ID in the period.
 * - `count`: Number of trades in the period.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TickerStatisticsMini {
    pub symbol: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub open_price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub high_price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub low_price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub last_price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub quote_volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64,
    pub last_id: i64,
    pub count: u64,
}