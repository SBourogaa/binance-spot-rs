use super::update_speed::UpdateSpeed;
use super::super::r#trait::StreamSpec; 
use crate::StreamConfig;

/**
 * Specification for Binance Diff. Depth Stream
 * 
 * Order book price and quantity depth updates used to locally manage an order book.
 * 
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 * - `update_speed`: Speed of updates (standard 1000ms or fast 100ms)
 */
#[derive(Debug)]
pub struct DiffDepthStreamSpec {
    symbol: String,
    update_speed: UpdateSpeed,
}

#[allow(dead_code)]
impl DiffDepthStreamSpec {
    /**
     * Creates a new diff depth stream specification with standard update speed
     * 
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * 
     * # Returns
     * - New DiffDepthStreamSpec instance with 1000ms updates
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            update_speed: UpdateSpeed::Standard,
        }
    }
    
    /**
     * Creates a new diff depth stream specification with fast update speed
     * 
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * 
     * # Returns
     * - New DiffDepthStreamSpec instance with 100ms updates
     */
    pub fn with_fast_updates(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            update_speed: UpdateSpeed::Fast100ms,
        }
    }
    
    /**
     * Creates a standard speed diff depth stream specification
     * 
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * 
     * # Returns
     * - New DiffDepthStreamSpec instance with 1000ms updates
     */
    pub fn standard(symbol: impl Into<String>) -> Self {
        Self::new(symbol)
    }
    
    /**
     * Creates a fast speed diff depth stream specification
     * 
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * 
     * # Returns
     * - New DiffDepthStreamSpec instance with 100ms updates
     */
    pub fn fast(symbol: impl Into<String>) -> Self {
        Self::with_fast_updates(symbol)
    }
}

impl StreamSpec for DiffDepthStreamSpec {
    type Event = crate::streams::events::DiffDepthStreamEvent;
    
    /**
     * Generates the WebSocket stream name
     * 
     * # Returns
     * - Stream name: <symbol>@depth or <symbol>@depth@100ms based on update speed
     */
    fn stream_name(&self) -> String {
        match self.update_speed {
            UpdateSpeed::Standard => format!("{}@depth", self.symbol.to_lowercase()),
            UpdateSpeed::Fast100ms => format!("{}@depth@100ms", self.symbol.to_lowercase()),
        }
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
     * - Buffer size for diff depth events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.diff_depth_buffer_size
    }
}