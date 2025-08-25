use super::super::r#trait::StreamSpec;
use super::update_speed::UpdateSpeed;
use crate::StreamConfig;

/**
 * Specification for Binance Partial Book Depth Stream
 *
 * Top bids and asks for a specified number of levels, pushed every second or every 100ms.
 *
 * # Fields
 * - `symbol`: Trading pair symbol (e.g., "BTCUSDT")
 * - `levels`: Number of price levels (5, 10, or 20)
 * - `update_speed`: Update frequency (1000ms standard or 100ms fast)
 */
#[derive(Debug)]
pub struct PartialBookDepthStreamSpec {
    symbol: String,
    levels: u8,
    update_speed: UpdateSpeed,
}

#[allow(dead_code)]
impl PartialBookDepthStreamSpec {
    /**
     * Creates a new partial book depth stream specification with standard update speed
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * - `levels` - Number of price levels (5, 10, or 20)
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 1000ms updates
     */
    pub fn new(symbol: impl Into<String>, levels: u8) -> Self {
        Self {
            symbol: symbol.into(),
            levels,
            update_speed: UpdateSpeed::Standard,
        }
    }

    /**
     * Creates a new partial book depth stream specification with fast update speed
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     * - `levels` - Number of price levels (5, 10, or 20)
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 100ms updates
     */
    pub fn with_fast_updates(symbol: impl Into<String>, levels: u8) -> Self {
        Self {
            symbol: symbol.into(),
            levels,
            update_speed: UpdateSpeed::Fast100ms,
        }
    }

    /**
     * Creates a 5-level partial book depth stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 5 levels and standard updates
     */
    pub fn levels_5(symbol: impl Into<String>) -> Self {
        Self::new(symbol, 5)
    }

    /**
     * Creates a 10-level partial book depth stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 10 levels and standard updates
     */
    pub fn levels_10(symbol: impl Into<String>) -> Self {
        Self::new(symbol, 10)
    }

    /**
     * Creates a 20-level partial book depth stream specification
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 20 levels and standard updates
     */
    pub fn levels_20(symbol: impl Into<String>) -> Self {
        Self::new(symbol, 20)
    }

    /**
     * Creates a 5-level partial book depth stream specification with fast updates
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 5 levels and 100ms updates
     */
    pub fn levels_5_fast(symbol: impl Into<String>) -> Self {
        Self::with_fast_updates(symbol, 5)
    }

    /**
     * Creates a 10-level partial book depth stream specification with fast updates
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 10 levels and 100ms updates
     */
    pub fn levels_10_fast(symbol: impl Into<String>) -> Self {
        Self::with_fast_updates(symbol, 10)
    }

    /**
     * Creates a 20-level partial book depth stream specification with fast updates
     *
     * # Arguments
     * - `symbol` - Trading pair symbol (e.g., "BTCUSDT")
     *
     * # Returns
     * - New PartialBookDepthStreamSpec instance with 20 levels and 100ms updates
     */
    pub fn levels_20_fast(symbol: impl Into<String>) -> Self {
        Self::with_fast_updates(symbol, 20)
    }
}

impl StreamSpec for PartialBookDepthStreamSpec {
    type Event = crate::streams::events::PartialBookDepthStreamEvent;

    /**
     * Generates the WebSocket stream name
     *
     * # Returns
     * - Stream name in format: <symbol>@depth<levels> or <symbol>@depth<levels>@100ms (lowercase)
     */
    fn stream_name(&self) -> String {
        match self.update_speed {
            UpdateSpeed::Standard => format!("{}@depth{}", self.symbol.to_lowercase(), self.levels),
            UpdateSpeed::Fast100ms => {
                format!("{}@depth{}@100ms", self.symbol.to_lowercase(), self.levels)
            }
        }
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
        if ![5, 10, 20].contains(&self.levels) {
            return Err(anyhow::anyhow!("Levels must be 5, 10, or 20"));
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
     * - Buffer size for partial book depth events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.partial_book_depth_buffer_size
    }
}
