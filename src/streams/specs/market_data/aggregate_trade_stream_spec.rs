use super::super::r#trait::StreamSpec; 
use crate::StreamConfig;

/**
 * Specification for Binance Aggregate Trade Stream
 * 
 * Configures subscription to aggregate trade data for a specific trading pair.
 * The stream provides real-time aggregated trade information for single taker orders.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 */
#[derive(Debug)]
pub struct AggregateTradeStreamSpec {
    symbol: String,
}

#[allow(dead_code)]
impl AggregateTradeStreamSpec {
    /**
     * Creates a new aggregate trade stream specification
     * 
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * 
     * # Returns
     * - New AggregateTradeStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

impl StreamSpec for AggregateTradeStreamSpec {
    type Event = crate::streams::events::AggregateTradeStreamEvent;
    
    /**
     * Generates the WebSocket stream name
     * 
     * # Returns
     * - Stream name in format: <symbol>@aggTrade (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@aggTrade", self.symbol.to_lowercase())
    }
    
    /**
     * Validates the stream specification parameters
     * 
     * # Returns
     * - Result indicating if the specification is valid
     * 
     * # Errors
     * - Returns error if symbol is empty
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
     * - Buffer size for aggregate trade events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.aggregate_trade_buffer_size
    }
}