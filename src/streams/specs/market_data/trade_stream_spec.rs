use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance Trade Stream
 *
 * The Trade Stream pushes raw trade information; each trade has a unique buyer and seller.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 */
#[derive(Debug)]
pub struct TradeStreamSpec {
    symbol: String,
}

#[allow(dead_code)]
impl TradeStreamSpec {
    /**
     * Creates a new trade stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New TradeStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

impl StreamSpec for TradeStreamSpec {
    type Event = crate::streams::events::TradeStreamEvent;

    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: <symbol>@trade (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@trade", self.symbol.to_lowercase())
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
     * - Buffer size for trade events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.trade_buffer_size
    }
}