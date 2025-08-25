use std::collections::HashMap;

use tokio::sync::{mpsc, watch};
use tracing::{info, instrument};

use crate::Result;
use crate::{BinanceConfig, StreamConfig};
use super::{
    types::{ConnectionStatus, StreamMessage, ValueSender},
    handler::UnifiedConnectionHandler,
    endpoint::StreamEndpoint,
    common::{ConnectionManager, ConnectionUtils},
};

/**
 * Connection manager for WebSocket streams.
 *
 * Manages the complete lifecycle of WebSocket connections for market data.
 *
 * # Fields
 * - `config`: Binance configuration with connection settings.
 * - `status_receiver`: Channel for monitoring connection status.
 * - `_task_handles`: Background task handles for proper cleanup.
 */
pub struct MarketDataConnectionManager {
    config: BinanceConfig<StreamConfig>,
    status_receiver: watch::Receiver<ConnectionStatus>,
    _task_handles: Vec<tokio::task::JoinHandle<()>>,
}

impl MarketDataConnectionManager {
    /**
     * Creates a new dynamic connection manager.
     *
     * Dynamic mode supports runtime subscription management through
     * WebSocket API calls. Clients can subscribe and unsubscribe at runtime.
     *
     * # Arguments
     * - `config`: Binance configuration with stream settings.
     *
     * # Returns
     * - Tuple containing the connection manager and message sender channel.
     */
    #[instrument(skip(config))]
    pub fn new_dynamic(config: BinanceConfig<StreamConfig>) -> Result<(Self, mpsc::UnboundedSender<StreamMessage>)> {
        let start = std::time::Instant::now();
        let (status_sender, status_receiver) = watch::channel(ConnectionStatus::Connecting);
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        let url = StreamEndpoint::from_config(&config).build_url(config.market_data_url());
        let stream_config = config.stream_config().clone();

        let task_handle = tokio::spawn(
            ConnectionUtils::run_connection(url, stream_config, status_sender, UnifiedConnectionHandler::new_dynamic(message_receiver, None))
        );

        let manager = Self {
            config,
            status_receiver,
            _task_handles: vec![task_handle],
        };

        info!(
            duration_us = start.elapsed().as_micros(),
            mode = "dynamic",
            "Market data connection manager created"
        );

        Ok((manager, message_sender))
    }

    /**
     * Creates a new static connection manager.
     *
     * Static mode uses pre-configured streams with fixed routing to broadcast
     * channels. All streams are connected at startup and cannot be changed.
     *
     * # Arguments
     * - `config`: Binance configuration with pre-configured streams.
     *
     * # Returns
     * - Tuple containing manager, message sender, and stream broadcast senders.
     */
    #[instrument(skip(config))]
    pub fn new_static(config: BinanceConfig<StreamConfig>) -> Result<(Self, mpsc::UnboundedSender<StreamMessage>, HashMap<String, ValueSender>)> {
        let start = std::time::Instant::now();
        let stream_infos = match config.stream_config().stream_mode() {
            crate::config::StreamMode::Raw(info) => vec![info.clone()],
            crate::config::StreamMode::Combined(infos) => infos.clone(),
            crate::config::StreamMode::Dynamic => {
                return Err(anyhow::anyhow!("Dynamic mode not supported for static connection manager"));
            }
        };
        
        let mut senders = HashMap::new();
        for stream_info in stream_infos {
            let (sender, _) = tokio::sync::broadcast::channel(stream_info.buffer_size);
            senders.insert(stream_info.name, sender);
        }

        let (status_sender, status_receiver) = watch::channel(ConnectionStatus::Connecting);
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        let url = StreamEndpoint::from_config(&config).build_url(config.market_data_url());
        let stream_config = config.stream_config().clone();
        let senders_clone = senders.clone();

        let task_handle = tokio::spawn(
            ConnectionUtils::run_connection(url, stream_config, status_sender, UnifiedConnectionHandler::new_static(message_receiver, senders_clone))
        );

        let manager = Self {
            config,
            status_receiver,
            _task_handles: vec![task_handle],
        };

        info!(
            duration_us = start.elapsed().as_micros(),
            mode = "static",
            stream_count = senders.len(),
            "Market data connection manager created"
        );

        Ok((manager, message_sender, senders))
    }
}

impl ConnectionManager for MarketDataConnectionManager {
    fn connection_status(&self) -> ConnectionStatus {
        self.status_receiver.borrow().clone()
    }

    fn stream_config(&self) -> &StreamConfig {
        self.config.stream_config()
    }

    fn has_authentication(&self) -> bool {
        self.config.has_authentication()
    }

    fn market_data_url(&self) -> &str {
        self.config.market_data_url()
    }

    fn user_data_url(&self) -> &str {
        self.config.user_data_url()
    }

    fn wait_for_connection(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        ConnectionUtils::wait_for_connection(&self.status_receiver)
    }

    fn abort_connection(&mut self) {
        ConnectionUtils::abort_connection(&self._task_handles);
    }
}