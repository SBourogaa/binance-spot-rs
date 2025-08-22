use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance All Market Mini Tickers Stream
 * 
 * Configures subscription to mini ticker data for all trading pairs.
 * Only tickers that have changed will be present in the array.
 */
#[derive(Debug)]
pub struct AllMiniTickersStreamSpec;

#[allow(dead_code)]
impl AllMiniTickersStreamSpec {
    /**
     * Creates a new all mini tickers stream specification
     * 
     * # Returns
     * - New AllMiniTickersStreamSpec instance
     */
    pub fn new() -> Self {
        Self
    }
}

impl StreamSpec for AllMiniTickersStreamSpec {
    type Event = crate::streams::events::AllMiniTickersStreamEvent;
    
    /**
     * Gets the WebSocket stream name
     * 
     * # Returns
     * - Stream name: !miniTicker@arr
     */
    fn stream_name(&self) -> String {
        "!miniTicker@arr".to_string()
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
     * - Buffer size for all mini tickers events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.all_mini_tickers_buffer_size
    }
}