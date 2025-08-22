use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance Individual Symbol Mini Ticker Stream
 *
 * 24hr rolling window mini-ticker statistics for a single symbol. These are NOT the statistics 
 * of the UTC day, but a 24hr rolling window for the previous 24hrs.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 */
#[derive(Debug)]
pub struct MiniTickerStreamSpec {
    symbol: String,
}

#[allow(dead_code)]
impl MiniTickerStreamSpec {
    /**
     * Creates a new mini ticker stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New MiniTickerStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

impl StreamSpec for MiniTickerStreamSpec {
    type Event = crate::streams::events::MiniTickerStreamEvent;

    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: <symbol>@miniTicker (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@miniTicker", self.symbol.to_lowercase())
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
     * - Buffer size for mini ticker events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.mini_ticker_buffer_size
    }
}