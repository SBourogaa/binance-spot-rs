use super::super::r#trait::StreamSpec;
use crate::StreamConfig;
use crate::streams::events::UserDataEvent;

/**
 * Specification for Binance User Data Stream
 *
 * Configures subscription to user data events including order updates,
 * balance changes, and account position updates. This stream requires
 * authentication via API key and produces multiple event types.
 */
#[derive(Debug, Clone)]
pub struct UserDataStreamSpec;

impl UserDataStreamSpec {
    /**
     * Creates a new user data stream specification
     *
     * # Returns
     * - New UserDataStreamSpec instance
     */
    pub fn new() -> Self {
        Self
    }
}

impl Default for UserDataStreamSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamSpec for UserDataStreamSpec {
    type Event = UserDataEvent;

    /**
     * Generates the WebSocket stream name
     *
     * User data streams use a special "userData" stream name that requires
     * authentication during connection establishment.
     *
     * # Returns
     * - Stream name: "userData"
     */
    fn stream_name(&self) -> String {
        "userData".to_string()
    }

    /**
     * Validates the stream specification parameters
     *
     * User data streams require authentication, but validation is handled
     * at the connection level when authentication is checked.
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
     * - Buffer size for user data events
     */
    fn buffer_size(&self, config: &StreamConfig) -> usize {
        config.user_data_buffer_size
    }

    /**
     * Indicates that this stream requires authentication
     *
     * User data streams always require API key authentication to access
     * account-specific information.
     *
     * # Returns
     * - Always `true` for user data streams
     */
    fn requires_authentication(&self) -> bool {
        true
    }
}
