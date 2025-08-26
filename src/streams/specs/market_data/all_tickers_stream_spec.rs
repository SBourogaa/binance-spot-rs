use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance All Market Tickers Stream
 *
 * 24hr rolling window ticker statistics for all symbols that changed in an array.
 * These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.
 * Note that only tickers that have changed will be present in the array.
 */
#[derive(Debug)]
pub struct AllTickersStreamSpec;

#[allow(dead_code)]
impl AllTickersStreamSpec {
    /**
     * Creates a new all tickers stream specification
     *
     * # Returns
     * - New AllTickersStreamSpec instance
     */
    pub fn new() -> Self {
        Self
    }
}

impl Default for AllTickersStreamSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamSpec for AllTickersStreamSpec {
    type Event = crate::streams::events::AllTickersStreamEvent;

    /**
     * Gets the WebSocket stream name
     *
     * # Returns
     * - Stream name: !ticker@arr
     */
    fn stream_name(&self) -> String {
        "!ticker@arr".to_string()
    }

    /**
     * Validates the stream specification parameters
     *
     * # Returns
     * - Always Ok(()) as no parameters need validation
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
     * - Buffer size for all tickers events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.all_tickers_buffer_size
    }
}
