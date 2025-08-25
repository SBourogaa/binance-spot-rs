use super::super::r#trait::StreamSpec;
use super::window_size::WindowSize;
use crate::StreamConfig;

/**
 * Specification for Binance Individual Symbol Rolling Window Statistics Stream
 *
 * Rolling window ticker statistics for a single symbol, computed over multiple windows.
 *
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 * - `window`: Rolling window size (1h, 4h, or 1d)
 */
#[derive(Debug)]
pub struct RollingWindowTickerStreamSpec {
    symbol: String,
    window: WindowSize,
}

#[allow(dead_code)]
impl RollingWindowTickerStreamSpec {
    /**
     * Creates a new rolling window ticker stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * - `window` - Rolling window size
     *
     * # Returns
     * - New RollingWindowTickerStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>, window: WindowSize) -> Self {
        Self {
            symbol: symbol.into(),
            window,
        }
    }

    /**
     * Creates a 1-hour rolling window ticker stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New RollingWindowTickerStreamSpec instance with 1-hour window
     */
    pub fn hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, WindowSize::OneHour)
    }

    /**
     * Creates a 4-hour rolling window ticker stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New RollingWindowTickerStreamSpec instance with 4-hour window
     */
    pub fn four_hourly(symbol: impl Into<String>) -> Self {
        Self::new(symbol, WindowSize::FourHours)
    }

    /**
     * Creates a 1-day rolling window ticker stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New RollingWindowTickerStreamSpec instance with 1-day window
     */
    pub fn daily(symbol: impl Into<String>) -> Self {
        Self::new(symbol, WindowSize::OneDay)
    }
}

impl StreamSpec for RollingWindowTickerStreamSpec {
    type Event = crate::streams::events::RollingWindowTickerStreamEvent;

    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: <symbol>@ticker_<window_size> (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@ticker_{}", self.symbol.to_lowercase(), self.window)
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
     * - Buffer size for rolling window ticker events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.rolling_window_ticker_buffer_size
    }
}
