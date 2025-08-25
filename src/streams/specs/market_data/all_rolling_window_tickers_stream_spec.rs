use super::super::r#trait::StreamSpec;
use super::window_size::WindowSize;
use crate::StreamConfig;

/**
 * Specification for Binance All Market Rolling Window Statistics Stream
 *
 * Rolling window ticker statistics for all market symbols, computed over multiple windows.
 * Note that only tickers that have changed will be present in the array.
 *
 * # Fields
 * - `window`: Window size for the rolling statistics (e.g., 1 hour, 4 hours, 1 day)
 */
#[derive(Debug)]
pub struct AllRollingWindowTickersStreamSpec {
    window: WindowSize,
}

#[allow(dead_code)]
impl AllRollingWindowTickersStreamSpec {
    /**
     * Creates a new all rolling window tickers stream specification
     *
     * # Arguments
     * - `window` - Window size for the rolling statistics
     *
     * # Returns
     * - New AllRollingWindowTickersStreamSpec instance
     */
    pub fn new(window: WindowSize) -> Self {
        Self { window }
    }

    /**
     * Creates a 1-hour rolling window specification
     *
     * # Returns
     * - Specification configured for 1-hour window
     */
    pub fn hourly() -> Self {
        Self::new(WindowSize::OneHour)
    }

    /**
     * Creates a 4-hour rolling window specification
     *
     * # Returns
     * - Specification configured for 4-hour window
     */
    pub fn four_hourly() -> Self {
        Self::new(WindowSize::FourHours)
    }

    /**
     * Creates a 1-day rolling window specification
     *
     * # Returns
     * - Specification configured for 1-day window
     */
    pub fn daily() -> Self {
        Self::new(WindowSize::OneDay)
    }
}

impl StreamSpec for AllRollingWindowTickersStreamSpec {
    type Event = crate::streams::events::AllRollingWindowTickersStreamEvent;

    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: !ticker_<window>@arr
     */
    fn stream_name(&self) -> String {
        format!("!ticker_{}@arr", self.window)
    }

    /**
     * Validates the stream specification parameters
     *
     * # Returns
     * - Always Ok(()) as window size is validated by enum
     */
    fn validate(&self) -> crate::Result<()> {
        Ok(())
    }

    /**
     * Gets the buffer size for this stream type
     *
     * # Arguments
     * - `config` - Stream configuration containing buffer size settings
     *
     * # Returns
     * - Buffer size for all rolling window tickers events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.all_rolling_window_tickers_buffer_size
    }
}
