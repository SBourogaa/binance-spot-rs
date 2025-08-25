use serde::de::DeserializeOwned;

use crate::StreamConfig;

/**
 * Stream Specification Trait
 *
 * Defines the interface for all Binance WebSocket stream specifications.
 * Implementations must specify how to generate stream names, validate parameters,
 * determine buffer sizes, and indicate authentication requirements.
 */
pub trait StreamSpec {
    /**
     * Associated event type for this stream
     *
     * Must be deserializable, sendable between threads, cloneable, and have static lifetime.
     */
    type Event: DeserializeOwned + Send + Clone + 'static;

    /**
     * Generates the WebSocket stream name for subscription
     *
     * # Returns
     * - Stream name string used in WebSocket subscription
     */
    fn stream_name(&self) -> String;

    /**
     * Validates the stream specification parameters
     *
     * Default implementation returns Ok().
     *
     * # Returns
     * - Result indicating if the specification is valid, or an error if validation fails
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
     * - Buffer size for events of this stream type
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize;

    /**
     * Indicates whether this stream requires authentication
     *
     * Default implementation returns false for market data streams.
     * User data streams should override this to return true.
     *
     * # Returns
     * - `true` if authentication is required, `false` otherwise
     */
    fn requires_authentication(&self) -> bool {
        false
    }
}
