//TODO: Clean up configs. 
use std::sync::Arc;

use crate::Result;
use crate::{
    errors::InvalidUrl,
    auth::{SignatureProvider, Ed25519Signer},
    config::{
        RestConfig, 
        StreamConfig,
        WebSocketConfig,
        stream_config::{StreamMode, StreamInfo, StreamType},
    },
    streams::specs::StreamSpec,
};

const BINANCE_API_MAINNET: &str = "https://api.binance.com";
const BINANCE_API_TESTNET: &str = "https://testnet.binance.vision";
const BINANCE_WS_MAINNET: &str = "wss://ws-api.binance.com:443";
const BINANCE_WS_TESTNET: &str = "wss://ws-api.testnet.binance.vision";
const BINANCE_STREAM_MAINNET: &str = "wss://stream.binance.com:9443";
const BINANCE_STREAM_USER_MAINNET: &str = "wss://ws-api.binance.com:443/ws-api/v3";
const BINANCE_STREAM_USER_TESTNET: &str = "wss://ws-api.testnet.binance.vision/ws-api/v3";

#[derive(Debug)]
pub struct BinanceConfig<T> {
    signer: Option<Arc<dyn SignatureProvider>>,
    recv_window: u64,
    specific_config: T,
}

#[derive(Debug)]
pub struct BinanceConfigBuilder {
    testnet: bool,
    recv_window: u64,
    credentials: Option<(String, String)>,
}

#[derive(Debug)]
pub struct RestBinanceConfigBuilder {
    base: BinanceConfigBuilder,
    rest_config: RestConfig,
}

#[derive(Debug)]
pub struct WebSocketBinanceConfigBuilder {
    base: BinanceConfigBuilder,
    websocket_config: WebSocketConfig,
}

#[derive(Debug)]
pub struct StreamBinanceConfigBuilder {
    base: BinanceConfigBuilder,
    stream_config: StreamConfig,
}

#[derive(Debug)]
pub struct MarketDataStreamBuilder {
    base: BinanceConfigBuilder,
    stream_config: StreamConfig,
}

#[derive(Debug)]
pub struct UserDataStreamBuilder {
    base: BinanceConfigBuilder,
    stream_config: StreamConfig,
}

impl<T> BinanceConfig<T> {
    pub fn signer(&self) -> Option<Arc<dyn SignatureProvider>> {
        self.signer.clone()
    }

    pub fn recv_window(&self) -> u64 {
        self.recv_window
    }

    pub fn api_key(&self) -> Option<&str> {
        self.signer.as_ref().map(|s| s.get_api_key())
    }

    pub fn has_authentication(&self) -> bool {
        self.signer.is_some()
    }

    pub fn set_recv_window(&mut self, recv_window_ms: u64) {
        self.recv_window = recv_window_ms;
    }
}

impl BinanceConfig<WebSocketConfig> {
    pub fn builder() -> WebSocketBinanceConfigBuilder {
        BinanceConfigBuilder::new().for_websocket()
    }
    
    pub fn websocket_config(&self) -> &WebSocketConfig {
        &self.specific_config
    }

    pub fn url(&self) -> &str {
        &self.specific_config.url
    }
}

impl BinanceConfig<RestConfig> {
    pub fn builder() -> RestBinanceConfigBuilder {
        BinanceConfigBuilder::new().for_rest()
    }

    pub fn rest_config(&self) -> &RestConfig {
        &self.specific_config
    }

    pub fn url(&self) -> &str {
        &self.specific_config.url
    }
}

impl BinanceConfig<StreamConfig> {
    pub fn builder() -> StreamBinanceConfigBuilder {
        BinanceConfigBuilder::new().for_streams()
    }
    
    pub fn stream_config(&self) -> &StreamConfig {
        &self.specific_config
    }

    pub fn market_data_url(&self) -> &str {
        &self.specific_config.market_data_url
    }

    pub fn user_data_url(&self) -> &str {
        &self.specific_config.user_data_url
    }
}

impl BinanceConfigBuilder {
    pub fn new() -> Self {
        Self {
            testnet: false,
            recv_window: 5000,
            credentials: None,
        }
    }

    pub fn with_testnet(mut self) -> Self {
        self.testnet = true;
        self
    }

    pub fn with_mainnet(mut self) -> Self {
        self.testnet = false;
        self
    }

    pub fn with_recv_window(mut self, window: u64) -> Self {
        self.recv_window = window;
        self
    }

    pub fn with_credentials(mut self, api_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        self.credentials = Some((api_key.into(), private_key.into()));
        self
    }

    pub fn for_rest(self) -> RestBinanceConfigBuilder {
        let url = if self.testnet {
            BINANCE_API_TESTNET
        } else {
            BINANCE_API_MAINNET
        };

        RestBinanceConfigBuilder {
            base: self,
            rest_config: RestConfig::builder().with_url(url).build(),
        }
    }

    pub fn for_websocket(self) -> WebSocketBinanceConfigBuilder {
        let url = if self.testnet {
            BINANCE_WS_TESTNET
        } else {
            BINANCE_WS_MAINNET
        };

        WebSocketBinanceConfigBuilder {
            base: self,
            websocket_config: WebSocketConfig::builder().with_url(url).build(),
        }
    }

    pub fn for_streams(self) -> StreamBinanceConfigBuilder {
        let market_data_url = BINANCE_STREAM_MAINNET;
        let user_data_url = if self.testnet {
            BINANCE_STREAM_USER_TESTNET
        } else {
            BINANCE_STREAM_USER_MAINNET
        };

        StreamBinanceConfigBuilder {
            base: self,
            stream_config: StreamConfig::builder()
                .with_market_data_url(market_data_url)
                .with_user_data_url(user_data_url)
                .build(),
        }
    }
}

impl RestBinanceConfigBuilder {
    pub fn with_testnet(mut self) -> Self {
        self.base = self.base.with_testnet();
        self.rest_config.url = BINANCE_API_TESTNET.to_string();
        self
    }

    pub fn with_mainnet(mut self) -> Self {
        self.base = self.base.with_mainnet();
        self.rest_config.url = BINANCE_API_MAINNET.to_string();
        self
    }

    pub fn with_recv_window(mut self, window: u64) -> Self {
        self.base = self.base.with_recv_window(window);
        self
    }

    pub fn with_credentials(mut self, api_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        self.base = self.base.with_credentials(api_key, private_key);
        self
    }

    pub fn with_credentials_from_file(mut self, api_key: impl Into<String>, pem_file_path: impl Into<String>) -> Result<Self> {
        let private_key_pem = std::fs::read_to_string(pem_file_path.into())?;
        self.base = self.base.with_credentials(api_key, private_key_pem);
        Ok(self)
    }

    pub fn with_rest_config(mut self, config: RestConfig) -> Self {
        self.rest_config = config;
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.rest_config.url = url.into();
        self
    }

    pub fn with_connection_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.rest_config = RestConfig::builder()
            .with_url(self.rest_config.url.clone())
            .with_connection_timeout(timeout)
            .with_request_timeout(self.rest_config.request_timeout)
            .with_pool_max_idle_per_host(self.rest_config.pool_max_idle_per_host)
            .with_pool_idle_timeout(self.rest_config.pool_idle_timeout)
            .with_user_agent(self.rest_config.user_agent.clone())
            .build();
        self
    }

    pub fn build(self) -> Result<BinanceConfig<RestConfig>> {
        if !self.rest_config.url.starts_with("https://") {
            return Err(InvalidUrl::invalid_scheme(
                &self.rest_config.url,
                "https://",
            ).into());
        }

        let signer = if let Some((api_key, private_key)) = self.base.credentials {
            Some(Arc::new(Ed25519Signer::new(&api_key, &private_key)?) as Arc<dyn SignatureProvider>)
        } else {
            None
        };

        Ok(BinanceConfig {
            signer,
            recv_window: self.base.recv_window,
            specific_config: self.rest_config,
        })
    }
}

impl WebSocketBinanceConfigBuilder {
    pub fn with_testnet(mut self) -> Self {
        self.base = self.base.with_testnet();
        self.websocket_config.url = BINANCE_WS_TESTNET.to_string();
        self
    }

    pub fn with_mainnet(mut self) -> Self {
        self.base = self.base.with_mainnet();
        self.websocket_config.url = BINANCE_WS_MAINNET.to_string();
        self
    }

    pub fn with_recv_window(mut self, window: u64) -> Self {
        self.base = self.base.with_recv_window(window);
        self
    }

    pub fn with_credentials(mut self, api_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        self.base = self.base.with_credentials(api_key, private_key);
        self
    }

    pub fn with_credentials_from_file(mut self, api_key: impl Into<String>, pem_file_path: impl Into<String>) -> Result<Self> {
        let private_key_pem = std::fs::read_to_string(pem_file_path.into())?;
        self.base = self.base.with_credentials(api_key, private_key_pem);
        Ok(self)
    }

    pub fn with_websocket_config(mut self, config: WebSocketConfig) -> Self {
        self.websocket_config = config;
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.websocket_config.url = url.into();
        self
    }

    pub fn with_max_reconnects(mut self, max: u32) -> Self {
        self.websocket_config = WebSocketConfig::builder()
            .with_url(self.websocket_config.url.clone())
            .with_max_reconnects(max)
            .with_initial_retry_delay(self.websocket_config.initial_retry_delay)
            .with_max_retry_delay(self.websocket_config.max_retry_delay)
            .with_connection_timeout(self.websocket_config.connection_timeout)
            .build();
        self
    }

    pub fn build(self) -> Result<BinanceConfig<WebSocketConfig>> {
        if !self.websocket_config.url.starts_with("wss://") {
            return Err(InvalidUrl::invalid_scheme(
                &self.websocket_config.url,
                "wss://",
            ).into());
        }

        let signer = if let Some((api_key, private_key)) = self.base.credentials {
            Some(Arc::new(Ed25519Signer::new(&api_key, &private_key)?) as Arc<dyn SignatureProvider>)
        } else {
            None
        };

        Ok(BinanceConfig {
            signer,
            recv_window: self.base.recv_window,
            specific_config: self.websocket_config,
        })
    }
}

impl StreamBinanceConfigBuilder {
    pub fn with_testnet(mut self) -> Self {
        self.base = self.base.with_testnet();
        self.stream_config.user_data_url = BINANCE_STREAM_USER_TESTNET.to_string();
        self
    }

    pub fn with_mainnet(mut self) -> Self {
        self.base = self.base.with_mainnet();
        self.stream_config.user_data_url = BINANCE_STREAM_USER_MAINNET.to_string();
        self
    }

    pub fn with_recv_window(mut self, window: u64) -> Self {
        self.base = self.base.with_recv_window(window);
        self
    }

    pub fn with_credentials(mut self, api_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        self.base = self.base.with_credentials(api_key, private_key);
        self
    }

    pub fn with_credentials_from_file(mut self, api_key: impl Into<String>, pem_file_path: impl Into<String>) -> Result<Self> {
        let private_key_pem = std::fs::read_to_string(pem_file_path.into())?;
        self.base = self.base.with_credentials(api_key, private_key_pem);
        Ok(self)
    }

    pub fn with_dynamic_streams(mut self) -> Self {
        self.stream_config.stream_mode = StreamMode::Dynamic;
        self
    }

    pub fn with_stream_config(mut self, config: StreamConfig) -> Self {
        self.stream_config = config;
        self
    }

    pub fn with_market_data(mut self) -> MarketDataStreamBuilder {
        self.stream_config.stream_type = StreamType::MarketData;
        MarketDataStreamBuilder {
            base: self.base,
            stream_config: self.stream_config,
        }
    }

    pub fn with_user_data(mut self) -> UserDataStreamBuilder {
        self.stream_config.stream_type = StreamType::UserData;
        UserDataStreamBuilder {
            base: self.base,
            stream_config: self.stream_config,
        }
    }

    pub fn with_market_data_url(mut self, url: impl Into<String>) -> Self {
        self.stream_config.market_data_url = url.into();
        self
    }

    pub fn with_user_data_url(mut self, url: impl Into<String>) -> Self {
        self.stream_config.user_data_url = url.into();
        self
    }

    pub fn with_trade_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.trade_buffer_size = size;
        self
    }

    pub fn with_aggregate_trade_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.aggregate_trade_buffer_size = size;
        self
    }

    pub fn with_average_price_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.average_price_buffer_size = size;
        self
    }

    pub fn with_book_ticker_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.book_ticker_buffer_size = size;
        self
    }

    pub fn with_mini_ticker_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.mini_ticker_buffer_size = size;
        self
    }

    pub fn with_all_mini_tickers_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.all_mini_tickers_buffer_size = size;
        self
    }

    pub fn with_ticker_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.ticker_buffer_size = size;
        self
    }

    pub fn with_all_tickers_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.all_tickers_buffer_size = size;
        self
    }

    pub fn with_kline_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.kline_buffer_size = size;
        self
    }

    pub fn with_rolling_window_ticker_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.rolling_window_ticker_buffer_size = size;
        self
    }

    pub fn with_all_rolling_window_tickers_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.all_rolling_window_tickers_buffer_size = size;
        self
    }

    pub fn with_partial_book_depth_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.partial_book_depth_buffer_size = size;
        self
    }

    pub fn with_depth_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.diff_depth_buffer_size = size;
        self
    }

    pub fn with_user_data_buffer_size(mut self, size: usize) -> Self {
        self.stream_config.user_data_buffer_size = size;
        self
    }

    pub fn with_max_reconnects(mut self, max: u32) -> Self {
        self.stream_config.max_reconnect_attempts = max;
        self
    }

}

impl<T: Clone> Clone for BinanceConfig<T> {
    fn clone(&self) -> Self {
        Self {
            signer: self.signer.clone(),
            recv_window: self.recv_window,
            specific_config: self.specific_config.clone(),
        }
    }
}

impl Default for BinanceConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketDataStreamBuilder {
    pub fn with_testnet(mut self) -> Self {
        self.base = self.base.with_testnet();
        self.stream_config.user_data_url = BINANCE_STREAM_USER_TESTNET.to_string();
        self
    }

    pub fn with_mainnet(mut self) -> Self {
        self.base = self.base.with_mainnet();
        self.stream_config.user_data_url = BINANCE_STREAM_USER_MAINNET.to_string();
        self
    }

    pub fn with_recv_window(mut self, window: u64) -> Self {
        self.base = self.base.with_recv_window(window);
        self
    }

    pub fn with_credentials(mut self, api_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        self.base = self.base.with_credentials(api_key, private_key);
        self
    }

    pub fn with_credentials_from_file(mut self, api_key: impl Into<String>, pem_file_path: impl Into<String>) -> Result<Self> {
        let private_key_pem = std::fs::read_to_string(pem_file_path.into())?;
        self.base = self.base.with_credentials(api_key, private_key_pem);
        Ok(self)
    }

    pub fn with_raw_stream<S: StreamSpec>(mut self, spec: &S) -> Result<Self> {
        spec.validate()?;
        let stream_info = StreamInfo {
            name: spec.stream_name(),
            buffer_size: spec.buffer_size(&self.stream_config),
        };
        self.stream_config.stream_mode = StreamMode::Raw(stream_info);
        Ok(self)
    }

    pub fn with_combined_streams<'a, I, S>(mut self, specs: I) -> Result<Self> 
    where 
        I: IntoIterator<Item = &'a S>,
        S: StreamSpec + 'a,
    {
        let stream_infos: Result<Vec<StreamInfo>> = specs
            .into_iter()
            .map(|spec| {
                spec.validate()?;
                Ok(StreamInfo {
                    name: spec.stream_name(),
                    buffer_size: spec.buffer_size(&self.stream_config),
                })
            })
            .collect();
            
        self.stream_config.stream_mode = StreamMode::Combined(stream_infos?);
        Ok(self)
    }

    pub fn with_dynamic_streams(mut self) -> Self {
        self.stream_config.stream_mode = StreamMode::Dynamic;
        self
    }

    pub fn build(self) -> Result<BinanceConfig<StreamConfig>> {
        if !self.stream_config.market_data_url.starts_with("wss://") {
            return Err(InvalidUrl::invalid_scheme(
                &self.stream_config.market_data_url,
                "wss://",
            ).into());
        }

        if !self.stream_config.user_data_url.starts_with("wss://") {
            return Err(InvalidUrl::invalid_scheme(
                &self.stream_config.user_data_url,
                "wss://",
            ).into());
        }

        let signer = if let Some((api_key, private_key)) = self.base.credentials {
            Some(Arc::new(Ed25519Signer::new(&api_key, &private_key)?) as Arc<dyn SignatureProvider>)
        } else {
            None
        };

        Ok(BinanceConfig {
            signer,
            recv_window: self.base.recv_window,
            specific_config: self.stream_config,
        })
    }
}

impl UserDataStreamBuilder {
    pub fn with_testnet(mut self) -> Self {
        self.base = self.base.with_testnet();
        self.stream_config.user_data_url = BINANCE_STREAM_USER_TESTNET.to_string();
        self
    }

    pub fn with_mainnet(mut self) -> Self {
        self.base = self.base.with_mainnet();
        self.stream_config.user_data_url = BINANCE_STREAM_USER_MAINNET.to_string();
        self
    }

    pub fn with_recv_window(mut self, window: u64) -> Self {
        self.base = self.base.with_recv_window(window);
        self
    }

    pub fn with_credentials(mut self, api_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        self.base = self.base.with_credentials(api_key, private_key);
        self
    }

    pub fn with_credentials_from_file(mut self, api_key: impl Into<String>, pem_file_path: impl Into<String>) -> Result<Self> {
        let private_key_pem = std::fs::read_to_string(pem_file_path.into())?;
        self.base = self.base.with_credentials(api_key, private_key_pem);
        Ok(self)
    }

    pub fn build(self) -> Result<BinanceConfig<StreamConfig>> {
        if !self.stream_config.market_data_url.starts_with("wss://") {
            return Err(InvalidUrl::invalid_scheme(
                &self.stream_config.market_data_url,
                "wss://",
            ).into());
        }

        if !self.stream_config.user_data_url.starts_with("wss://") {
            return Err(InvalidUrl::invalid_scheme(
                &self.stream_config.user_data_url,
                "wss://",
            ).into());
        }

        let signer = if let Some((api_key, private_key)) = self.base.credentials {
            Some(Arc::new(Ed25519Signer::new(&api_key, &private_key)?) as Arc<dyn SignatureProvider>)
        } else {
            None
        };

        Ok(BinanceConfig {
            signer,
            recv_window: self.base.recv_window,
            specific_config: self.stream_config,
        })
    }
}