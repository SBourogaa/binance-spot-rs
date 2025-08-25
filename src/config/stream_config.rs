use std::time::Duration;

const DEFAULT_MARKET_DATA_URL: &str = "wss://stream.binance.com:9443";
const DEFAULT_USER_DATA_URL: &str = "wss://ws-api.binance.com:443/ws-api/v3";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamType {
    MarketData,
    UserData,
}

impl Default for StreamType {
    fn default() -> Self {
        StreamType::MarketData
    }
}

#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub name: String,
    pub buffer_size: usize,
}

#[derive(Debug, Clone)]
pub enum StreamMode {
    Raw(StreamInfo),
    Combined(Vec<StreamInfo>),
    Dynamic,
}

impl Default for StreamMode {
    fn default() -> Self {
        StreamMode::Dynamic
    }
}

#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub stream_type: StreamType,
    pub market_data_url: String,
    pub user_data_url: String,
    pub trade_buffer_size: usize,
    pub aggregate_trade_buffer_size: usize,
    pub average_price_buffer_size: usize,
    pub book_ticker_buffer_size: usize,
    pub mini_ticker_buffer_size: usize,
    pub all_mini_tickers_buffer_size: usize,
    pub ticker_buffer_size: usize,
    pub all_tickers_buffer_size: usize,
    pub kline_buffer_size: usize,
    pub rolling_window_ticker_buffer_size: usize,
    pub all_rolling_window_tickers_buffer_size: usize,
    pub partial_book_depth_buffer_size: usize,
    pub diff_depth_buffer_size: usize,
    pub user_data_buffer_size: usize,
    pub auto_reconnect: bool,
    pub max_reconnect_attempts: u32,
    pub initial_retry_delay: Duration,
    pub max_retry_delay: Duration,
    pub connection_timeout: Duration,
    pub stream_mode: StreamMode,
}

#[derive(Debug)]
pub struct StreamConfigBuilder {
    stream_type: StreamType,
    market_data_url: String,
    user_data_url: String,
    trade_buffer_size: usize,
    aggregate_trade_buffer_size: usize,
    average_price_buffer_size: usize,
    book_ticker_buffer_size: usize,
    mini_ticker_buffer_size: usize,
    all_mini_tickers_buffer_size: usize,
    ticker_buffer_size: usize,
    all_tickers_buffer_size: usize,
    kline_buffer_size: usize,
    rolling_window_ticker_buffer_size: usize,
    all_rolling_window_tickers_buffer_size: usize,
    partial_book_depth_buffer_size: usize,
    diff_depth_buffer_size: usize,
    user_data_buffer_size: usize,
    auto_reconnect: bool,
    max_reconnect_attempts: u32,
    initial_retry_delay: Duration,
    max_retry_delay: Duration,
    connection_timeout: Duration,
    stream_mode: StreamMode,
}

impl StreamConfig {
    pub fn builder() -> StreamConfigBuilder {
        StreamConfigBuilder::new()
    }

    pub fn stream_mode(&self) -> &StreamMode {
        &self.stream_mode
    }
}

impl StreamConfigBuilder {
    fn new() -> Self {
        Self {
            stream_type: StreamType::MarketData,
            market_data_url: DEFAULT_MARKET_DATA_URL.to_string(),
            user_data_url: DEFAULT_USER_DATA_URL.to_string(),
            trade_buffer_size: 1000,
            aggregate_trade_buffer_size: 1000,
            average_price_buffer_size: 100,
            book_ticker_buffer_size: 1000,
            mini_ticker_buffer_size: 200,
            all_mini_tickers_buffer_size: 100,
            ticker_buffer_size: 100,
            all_tickers_buffer_size: 50,
            kline_buffer_size: 500,
            rolling_window_ticker_buffer_size: 100,
            all_rolling_window_tickers_buffer_size: 50,
            partial_book_depth_buffer_size: 2000,
            diff_depth_buffer_size: 5000,
            user_data_buffer_size: 500,
            auto_reconnect: true,
            max_reconnect_attempts: 10,
            initial_retry_delay: Duration::from_secs(1),
            max_retry_delay: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(10),
            stream_mode: StreamMode::default(),
        }
    }

    pub fn with_market_data_url(mut self, url: impl Into<String>) -> Self {
        self.market_data_url = url.into();
        self
    }

    pub fn with_user_data_url(mut self, url: impl Into<String>) -> Self {
        self.user_data_url = url.into();
        self
    }

    pub fn with_market_data(mut self) -> Self {
        self.stream_type = StreamType::MarketData;
        self
    }

    pub fn with_user_data(mut self) -> Self {
        self.stream_type = StreamType::UserData;
        self
    }

    pub fn with_trade_buffer_size(mut self, size: usize) -> Self {
        self.trade_buffer_size = size;
        self
    }

    pub fn with_aggregate_trade_buffer_size(mut self, size: usize) -> Self {
        self.aggregate_trade_buffer_size = size;
        self
    }

    pub fn with_average_price_buffer_size(mut self, size: usize) -> Self {
        self.average_price_buffer_size = size;
        self
    }

    pub fn with_book_ticker_buffer_size(mut self, size: usize) -> Self {
        self.book_ticker_buffer_size = size;
        self
    }

    pub fn with_mini_ticker_buffer_size(mut self, size: usize) -> Self {
        self.mini_ticker_buffer_size = size;
        self
    }

    pub fn with_all_mini_tickers_buffer_size(mut self, size: usize) -> Self {
        self.all_mini_tickers_buffer_size = size;
        self
    }

    pub fn with_ticker_buffer_size(mut self, size: usize) -> Self {
        self.ticker_buffer_size = size;
        self
    }

    pub fn with_all_tickers_buffer_size(mut self, size: usize) -> Self {
        self.all_tickers_buffer_size = size;
        self
    }

    pub fn with_kline_buffer_size(mut self, size: usize) -> Self {
        self.kline_buffer_size = size;
        self
    }

    pub fn with_rolling_window_ticker_buffer_size(mut self, size: usize) -> Self {
        self.rolling_window_ticker_buffer_size = size;
        self
    }

    pub fn with_all_rolling_window_tickers_buffer_size(mut self, size: usize) -> Self {
        self.all_rolling_window_tickers_buffer_size = size;
        self
    }

    pub fn with_partial_book_depth_buffer_size(mut self, size: usize) -> Self {
        self.partial_book_depth_buffer_size = size;
        self
    }

    pub fn with_diff_depth_buffer_size(mut self, size: usize) -> Self {
        self.diff_depth_buffer_size = size;
        self
    }

    pub fn with_user_data_buffer_size(mut self, size: usize) -> Self {
        self.user_data_buffer_size = size;
        self
    }

    pub fn with_auto_reconnect(mut self, enabled: bool) -> Self {
        self.auto_reconnect = enabled;
        self
    }

    pub fn with_max_reconnects(mut self, max: u32) -> Self {
        self.max_reconnect_attempts = max;
        self
    }

    pub fn with_initial_retry_delay(mut self, delay: Duration) -> Self {
        self.initial_retry_delay = delay;
        self
    }

    pub fn with_max_retry_delay(mut self, delay: Duration) -> Self {
        self.max_retry_delay = delay;
        self
    }

    pub fn with_connection_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    pub fn build(self) -> StreamConfig {
        StreamConfig {
            stream_type: self.stream_type,
            market_data_url: self.market_data_url,
            user_data_url: self.user_data_url,
            trade_buffer_size: self.trade_buffer_size,
            aggregate_trade_buffer_size: self.aggregate_trade_buffer_size,
            average_price_buffer_size: self.average_price_buffer_size,
            book_ticker_buffer_size: self.book_ticker_buffer_size,
            mini_ticker_buffer_size: self.mini_ticker_buffer_size,
            all_mini_tickers_buffer_size: self.all_mini_tickers_buffer_size,
            ticker_buffer_size: self.ticker_buffer_size,
            all_tickers_buffer_size: self.all_tickers_buffer_size,
            kline_buffer_size: self.kline_buffer_size,
            rolling_window_ticker_buffer_size: self.rolling_window_ticker_buffer_size,
            all_rolling_window_tickers_buffer_size: self.all_rolling_window_tickers_buffer_size,
            partial_book_depth_buffer_size: self.partial_book_depth_buffer_size,
            diff_depth_buffer_size: self.diff_depth_buffer_size,
            user_data_buffer_size: self.user_data_buffer_size,
            auto_reconnect: self.auto_reconnect,
            max_reconnect_attempts: self.max_reconnect_attempts,
            initial_retry_delay: self.initial_retry_delay,
            max_retry_delay: self.max_retry_delay,
            connection_timeout: self.connection_timeout,
            stream_mode: self.stream_mode,
        }
    }
}

impl Default for StreamConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
