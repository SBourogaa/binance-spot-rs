use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance Book Ticker Stream
 *
 * Pushes any update to the best bid or ask's price or quantity in real-time for a specified symbol.
 * Multiple <symbol>@bookTicker streams can be subscribed to over one connection.
 *
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 */
#[derive(Debug)]
pub struct BookTickerStreamSpec {
    symbol: String,
}

#[allow(dead_code)]
impl BookTickerStreamSpec {
    /**
     * Creates a new book ticker stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New BookTickerStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

impl StreamSpec for BookTickerStreamSpec {
    type Event = crate::streams::events::BookTickerStreamEvent;

    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: <symbol>@bookTicker (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@bookTicker", self.symbol.to_lowercase())
    }

    /**
     * Validates the stream specification parameters
     *
     * # Returns
     * - Result indicating if the specification is valid, error otherwise.
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
     * - Buffer size for book ticker events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.book_ticker_buffer_size
    }
}
