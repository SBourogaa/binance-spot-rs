use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Kline (candlestick) data from Binance API.
 *
 * Represents OHLCV data for a specific time interval with volume and trade metrics.
 * Enhanced with WebSocket Stream aliases for `<symbol>@kline_<interval>` compatibility.
 *
 * # Fields
 * - `open_time`: Kline open time in milliseconds.
 * - `open_price`: Opening price for the interval.
 * - `high_price`: Highest price during the interval.
 * - `low_price`: Lowest price during the interval.
 * - `close_price`: Closing price for the interval.
 * - `volume`: Base asset volume traded during the interval.
 * - `close_time`: Kline close time in milliseconds.
 * - `quote_asset_volume`: Quote asset volume traded during the interval.
 * - `number_of_trades`: Number of trades executed during the interval.
 * - `taker_buy_base_asset_volume`: Taker buy base asset volume.
 * - `taker_buy_quote_asset_volume`: Taker buy quote asset volume.
 * - `_unused`: Unused field present in the response. Kept for compatibility
 *   rather than writing a custom deserializer.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Kline {
    #[serde(alias = "t")]
    pub open_time: u64,
    #[serde(alias = "o", with = "rust_decimal::serde::str")]
    pub open_price: Decimal,
    #[serde(alias = "h", with = "rust_decimal::serde::str")]
    pub high_price: Decimal,
    #[serde(alias = "l", with = "rust_decimal::serde::str")]
    pub low_price: Decimal,
    #[serde(alias = "c", with = "rust_decimal::serde::str")]
    pub close_price: Decimal,
    #[serde(alias = "v", with = "rust_decimal::serde::str")]
    pub volume: Decimal,
    #[serde(alias = "T")]
    pub close_time: u64,
    #[serde(alias = "q", with = "rust_decimal::serde::str")]
    pub quote_asset_volume: Decimal,
    #[serde(alias = "n")]
    pub number_of_trades: u64,
    #[serde(alias = "V", with = "rust_decimal::serde::str")]
    pub taker_buy_base_asset_volume: Decimal,
    #[serde(alias = "Q", with = "rust_decimal::serde::str")]
    pub taker_buy_quote_asset_volume: Decimal,
    #[serde(alias = "B")]
    pub _unused: String,
}
