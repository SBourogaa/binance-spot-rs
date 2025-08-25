use std::time::Duration;

const DEFAULT_BINANCE_API_URL: &str = "https://api.binance.com";
const DEFAULT_USER_AGENT: &str = "binance-rust-client/1.0";

/**
 * Configuration for REST API client behavior.
 *
 * # Fields
 * - `url`: Base URL for REST API requests.
 * - `connection_timeout`: Timeout for establishing HTTP connections.
 * - `request_timeout`: Timeout for complete request-response cycle.
 * - `pool_max_idle_per_host`: Maximum idle connections to keep per host.
 * - `pool_idle_timeout`: How long to keep idle connections before closing.
 * - `user_agent`: User agent string for requests.
 */
#[derive(Debug, Clone)]
pub struct RestConfig {
    pub url: String,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub pool_max_idle_per_host: usize,
    pub pool_idle_timeout: Duration,
    pub user_agent: String,
}

/**
 * Builder for RestConfig.
 *
 * # Fields
 * - `url`: Base URL for REST API requests.
 * - `connection_timeout`: Timeout for establishing HTTP connections.
 * - `request_timeout`: Timeout for complete request-response cycle.
 * - `pool_max_idle_per_host`: Maximum idle connections to keep per host.
 * - `pool_idle_timeout`: How long to keep idle connections before closing.
 * - `user_agent`: User agent string for requests.
 */
#[derive(Debug)]
pub struct RestConfigBuilder {
    url: String,
    connection_timeout: Duration,
    request_timeout: Duration,
    pool_max_idle_per_host: usize,
    pool_idle_timeout: Duration,
    user_agent: String,
}

impl RestConfig {
    /**
     * Creates a new REST configuration builder.
     *
     * # Returns
     * - `RestConfigBuilder`: New builder instance.
     */
    pub fn builder() -> RestConfigBuilder {
        RestConfigBuilder::new()
    }
}

impl RestConfigBuilder {
    /**
     * Creates a new REST configuration builder with default values.
     *
     * # Returns
     * - `Self`: New builder instance.
     */
    fn new() -> Self {
        Self {
            url: DEFAULT_BINANCE_API_URL.to_string(),
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            pool_max_idle_per_host: 10,
            pool_idle_timeout: Duration::from_secs(30),
            user_agent: DEFAULT_USER_AGENT.to_string(),
        }
    }

    /**
     * Sets the base URL for REST API requests.
     *
     * # Arguments
     * - `url`: Base URL for the REST API.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /**
     * Sets the connection timeout.
     *
     * # Arguments
     * - `timeout`: Connection establishment timeout.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /**
     * Sets the request timeout.
     *
     * # Arguments
     * - `timeout`: Complete request-response timeout.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /**
     * Sets the maximum idle connections per host.
     *
     * # Arguments
     * - `max`: Maximum idle connections to keep.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_pool_max_idle_per_host(mut self, max: usize) -> Self {
        self.pool_max_idle_per_host = max;
        self
    }

    /**
     * Sets the idle connection timeout.
     *
     * # Arguments
     * - `timeout`: How long to keep idle connections.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_pool_idle_timeout(mut self, timeout: Duration) -> Self {
        self.pool_idle_timeout = timeout;
        self
    }

    /**
     * Sets the user agent string.
     *
     * # Arguments
     * - `user_agent`: User agent string for requests.
     *
     * # Returns
     * - `Self`: Updated builder.
     */
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /**
     * Builds the REST configuration.
     *
     * # Returns
     * - `RestConfig`: Configured REST settings.
     */
    pub fn build(self) -> RestConfig {
        RestConfig {
            url: self.url,
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
            pool_max_idle_per_host: self.pool_max_idle_per_host,
            pool_idle_timeout: self.pool_idle_timeout,
            user_agent: self.user_agent,
        }
    }
}

impl Default for RestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
