use std::time::Duration;

const DEFAULT_WEBSOCKET_URL: &str = "wss://ws-api.binance.com:443";

/**
 * Configuration for WebSocket connection management and behavior.
 *
 * # Fields
 * - `url`: WebSocket URL for streaming connections.
 * - `max_reconnect_attempts`: Maximum number of reconnection attempts before giving up.
 * - `initial_retry_delay`: Initial delay between reconnection attempts.
 * - `max_retry_delay`: Maximum delay between reconnection attempts (for exponential backoff).
 * - `connection_timeout`: Timeout for establishing WebSocket connections.
 */
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    pub url: String,
    pub max_reconnect_attempts: u32,
    pub initial_retry_delay: Duration,
    pub max_retry_delay: Duration,
    pub connection_timeout: Duration,
}

/**
 * Builder for WebSocketConfig.
 *
 * # Fields
 * - `url`: WebSocket URL for streaming connections.
 * - `max_reconnect_attempts`: Maximum number of reconnection attempts before giving up.
 * - `initial_retry_delay`: Initial delay between reconnection attempts.
 * - `max_retry_delay`: Maximum delay between reconnection attempts (for exponential backoff).
 * - `connection_timeout`: Timeout for establishing WebSocket connections.
 */
#[derive(Debug)]
pub struct WebSocketConfigBuilder {
    url: String,
    max_reconnect_attempts: u32,
    initial_retry_delay: Duration,
    max_retry_delay: Duration,
    connection_timeout: Duration,
}

impl WebSocketConfig {
    /**
     * Creates a new WebSocket configuration builder.
     *
     * # Returns
     * - `WebSocketConfigBuilder`: New builder instance.
     */
    pub fn builder() -> WebSocketConfigBuilder {
        WebSocketConfigBuilder::new()
    }
}

impl WebSocketConfigBuilder {
    /**
     * Creates a new WebSocket configuration builder with default values.
     *
     * # Returns
     * - `Self`: New builder instance.
     */
    fn new() -> Self {
        Self {
            url: DEFAULT_WEBSOCKET_URL.to_string(),
            max_reconnect_attempts: 5,
            initial_retry_delay: Duration::from_secs(1),
            max_retry_delay: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(10),
        }
    }

    /**
     * Sets the WebSocket URL.
     *
     * # Arguments
     * - `url`: WebSocket URL for connections.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /**
     * Sets the maximum number of reconnection attempts.
     *
     * # Arguments
     * - `max`: Maximum reconnection attempts.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_max_reconnects(mut self, max: u32) -> Self {
        self.max_reconnect_attempts = max;
        self
    }

    /**
     * Sets the initial retry delay.
     *
     * # Arguments
     * - `delay`: Initial delay between reconnection attempts.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_initial_retry_delay(mut self, delay: Duration) -> Self {
        self.initial_retry_delay = delay;
        self
    }

    /**
     * Sets the maximum retry delay for exponential backoff.
     *
     * # Arguments
     * - `delay`: Maximum delay between reconnection attempts.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = delay;
        self
    }

    /**
     * Sets the connection establishment timeout.
     *
     * # Arguments
     * - `timeout`: Connection timeout duration.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /**
     * Builds the WebSocket configuration.
     *
     * # Returns
     * - `WebSocketConfig`: Configured WebSocket settings.
     */
    pub fn build(self) -> WebSocketConfig {
        WebSocketConfig {
            url: self.url,
            max_reconnect_attempts: self.max_reconnect_attempts,
            initial_retry_delay: self.initial_retry_delay,
            max_retry_delay: self.max_retry_delay,
            connection_timeout: self.connection_timeout,
        }
    }
}

impl Default for WebSocketConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}