use super::{
    interval::Interval, 
    timezone_offset::TimezoneOffset
};
use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance Kline/Candlestick Stream with Timezone Offset
 *
 * The Kline/Candlestick Stream with timezone offset pushes updates to the current klines/candlestick 
 * every second (for 1s interval) or every 2 seconds (for other intervals) in the specified timezone.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 * - `interval`: Kline interval (e.g., OneMinute, OneHour, etc.)
 * - `timezone_offset`: Timezone offset for kline boundaries (currently only UTC+8 supported)
 */
#[derive(Debug)]
pub struct KlineWithTimezoneStreamSpec {
    symbol: String,
    interval: Interval,
    timezone_offset: TimezoneOffset,
}

#[allow(dead_code)]
impl KlineWithTimezoneStreamSpec {
    /**
     * Creates a new kline stream specification with timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * - `interval` - Kline interval for the stream
     * - `timezone_offset` - Timezone offset for kline boundaries
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>, interval: Interval, timezone_offset: TimezoneOffset) -> Self {
        Self {
            symbol: symbol.into(),
            interval,
            timezone_offset,
        }
    }

    /**
     * Creates a kline stream specification with UTC+8 timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * - `interval` - Kline interval for the stream
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance with UTC+8 timezone offset
     */
    pub fn utc_plus_8(symbol: impl Into<String>, interval: Interval) -> Self {
        Self::new(symbol, interval, TimezoneOffset::UtcPlus8)
    }

    /**
     * Creates a 1-minute kline stream specification with UTC+8 timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance with 1-minute interval and UTC+8 offset
     */
    pub fn one_minute_utc_plus_8(symbol: impl Into<String>) -> Self {
        Self::utc_plus_8(symbol, Interval::OneMinute)
    }

    /**
     * Creates a 5-minute kline stream specification with UTC+8 timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance with 5-minute interval and UTC+8 offset
     */
    pub fn five_minutes_utc_plus_8(symbol: impl Into<String>) -> Self {
        Self::utc_plus_8(symbol, Interval::FiveMinutes)
    }

    /**
     * Creates a 15-minute kline stream specification with UTC+8 timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance with 15-minute interval and UTC+8 offset
     */
    pub fn fifteen_minutes_utc_plus_8(symbol: impl Into<String>) -> Self {
        Self::utc_plus_8(symbol, Interval::FifteenMinutes)
    }

    /**
     * Creates a 1-hour kline stream specification with UTC+8 timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance with 1-hour interval and UTC+8 offset
     */
    pub fn hourly_utc_plus_8(symbol: impl Into<String>) -> Self {
        Self::utc_plus_8(symbol, Interval::OneHour)
    }

    /**
     * Creates a 1-day kline stream specification with UTC+8 timezone offset
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineWithTimezoneStreamSpec instance with 1-day interval and UTC+8 offset
     */
    pub fn daily_utc_plus_8(symbol: impl Into<String>) -> Self {
        Self::utc_plus_8(symbol, Interval::OneDay)
    }
}

impl StreamSpec for KlineWithTimezoneStreamSpec {
    type Event = crate::streams::events::KlineStreamEvent;

    /**
     * Generates the WebSocket stream name with timezone offset
     *
     * # Returns
     * - Stream name in format: <symbol>@kline_<interval>@<timezone_offset> (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@kline_{}@{}",
            self.symbol.to_lowercase(),
            self.interval,
            self.timezone_offset)
    }

    /**
     * Validates the stream specification parameters
     *
     * # Returns
     * - Result indicating if the specification is valid, or an error if validation fails.
     */
    fn validate(&self) -> crate::Result<()> {
        if self.symbol.is_empty() {
            return Err(anyhow::anyhow!("Symbol cannot be empty"));
        }
        Ok(())
    }

    /**
     * Gets the buffer size for this stream type
     *
     * # Arguments
     * - `config` - Stream configuration containing buffer size settings
     *
     * # Returns
     * - Buffer size for kline events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.kline_buffer_size
    }
}

use serde::{Deserialize, Serialize};
use crate::types::responses::Kline;

/**
 * Kline Stream Data
 *
 * Contains kline-specific metadata and flattened kline OHLCV data.
 * This structure represents the "k" field within a KlineStreamEvent.
 *
 * # Fields:
 * - `symbol`: Trading pair symbol (serde field "s")
 * - `interval`: Kline interval string (e.g., "1m", "1h")
 * - `first_trade_id`: First trade ID included in this kline
 * - `last_trade_id`: Last trade ID included in this kline
 * - `is_kline_closed`: Whether this kline period is complete/closed
 * - `kline`: Flattened kline data containing timestamps, OHLCV prices, volumes, and trade counts
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
 * Real-time kline/candlestick updates pushed every second (for 1s interval) or every 2 seconds (for other intervals).
 * This event is received when subscribed to a kline stream with timezone offset.
 *
 * # Update Speed
 * - 1000ms for 1s interval
 * - 2000ms for all other intervals
 *
 * # Fields:
 * - `event_type`: Event type identifier, always "kline" (serde field "e")
 * - `event_time`: Event timestamp in milliseconds since Unix epoch (serde field "E")
 * - `symbol`: Trading pair symbol (serde field "s") 
 * - `kline`: Detailed kline data containing interval, trade IDs, prices, volumes, and metadata (serde field "k")
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