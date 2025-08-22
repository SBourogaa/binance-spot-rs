use super::super::r#trait::StreamSpec;
use crate::StreamConfig;

/**
 * Specification for Binance Average Price Stream
 * 
 * Average price streams push changes in the average price over a fixed time interval.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 */
#[derive(Debug)]
pub struct AveragePriceStreamSpec {
    symbol: String,
}

#[allow(dead_code)]
impl AveragePriceStreamSpec {
    /**
     * Creates a new average price stream specification
     * 
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * 
     * # Returns
     * - New AveragePriceStreamSpec instance
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

impl StreamSpec for AveragePriceStreamSpec {
    type Event = crate::streams::events::AveragePriceStreamEvent;
    
    /**
     * Generates the WebSocket stream name
     * 
     * # Returns
     * - Stream name in format: <symbol>@avgPrice (lowercase)
     */
    fn stream_name(&self) -> String {
        format!("{}@avgPrice", self.symbol.to_lowercase())
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
     * - Buffer size for average price events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.average_price_buffer_size
    }
}