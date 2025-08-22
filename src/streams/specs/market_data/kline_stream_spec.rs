use super::interval::Interval;
use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance Kline/Candlestick Stream
 *
 * The Kline/Candlestick Stream pushes updates to the current klines/candlestick every second in UTC+0 timezone.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 * - `interval`: Kline interval (e.g., OneMinute, OneHour, etc.)
 */
#[derive(Debug)]
pub struct KlineStreamSpec {
    symbol: String,
    interval: Interval,
}

#[allow(dead_code)]
impl KlineStreamSpec {
    /**
     * Creates a new kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * - `interval` - Kline interval for the stream
     *
     * # Returns
     * - New KlineStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>, interval: Interval) -> Self {
        Self {
            symbol: symbol.into(),
            interval,
        }
    }
    
    /**
     * Creates a 1-second kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 1-second interval
     */
    pub fn one_second(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::OneSecond)
    }
    
    /**
     * Creates a 1-minute kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 1-minute interval
     */
    pub fn one_minute(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::OneMinute)
    }
    
    /**
     * Creates a 3-minute kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 3-minute interval
     */
    pub fn three_minutes(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::ThreeMinutes)
    }
    
    /**
     * Creates a 5-minute kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 5-minute interval
     */
    pub fn five_minutes(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::FiveMinutes)
    }
    
    /**
     * Creates a 15-minute kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 15-minute interval
     */
    pub fn fifteen_minutes(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::FifteenMinutes)
    }
    
    /**
     * Creates a 30-minute kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 30-minute interval
     */
    pub fn thirty_minutes(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::ThirtyMinutes)
    }
    
    /**
     * Creates a 1-hour kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 1-hour interval
     */
    pub fn hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::OneHour)
    }
    
    /**
     * Creates a 2-hour kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 2-hour interval
     */
    pub fn two_hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::TwoHours)
    }
    
    /**
     * Creates a 4-hour kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 4-hour interval
     */
    pub fn four_hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::FourHours)
    }
    
    /**
     * Creates a 6-hour kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 6-hour interval
     */
    pub fn six_hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::SixHours)
    }
    
    /**
     * Creates an 8-hour kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 8-hour interval
     */
    pub fn eight_hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::EightHours)
    }
    
    /**
     * Creates a 12-hour kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 12-hour interval
     */
    pub fn twelve_hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::TwelveHours)
    }
    
    /**
     * Creates a 1-day kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 1-day interval
     */
    pub fn daily(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::OneDay)
    }
    
    /**
     * Creates a 3-day kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 3-day interval
     */
    pub fn three_days(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::ThreeDays)
    }
    
    /**
     * Creates a 1-week kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 1-week interval
     */
    pub fn weekly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::OneWeek)
    }
    
    /**
     * Creates a 1-month kline stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New KlineStreamSpec instance with 1-month interval
     */
    pub fn monthly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, Interval::OneMonth)
    }
}

impl StreamSpec for KlineStreamSpec {
    type Event = crate::streams::events::KlineStreamEvent;
    
    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: <symbol>@kline_<interval> (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@kline_{}", self.symbol.to_lowercase(), self.interval)
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