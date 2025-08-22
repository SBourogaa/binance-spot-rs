use tokio::sync::{mpsc, watch};

use crate::Result;
use crate::{BinanceConfig, StreamConfig};
use super::{
    types::{ConnectionStatus, StreamMessage},
    handler::UnifiedConnectionHandler,
    common::{ConnectionManager, ConnectionUtils},
};

/**
 * Connection manager for User Data WebSocket streams.
 *
 * Manages user data stream connections using the WebSocket API method.
 * Always operates in dynamic mode and requires API key authentication.
 *
 * # Fields
 * - `config`: Binance configuration with authentication credentials.
 * - `status_receiver`: Channel for monitoring connection status.
 * - `_task_handles`: Background task handles for proper cleanup.
 */
pub struct UserDataConnectionManager {
    config: BinanceConfig<StreamConfig>,
    status_receiver: watch::Receiver<ConnectionStatus>,
    _task_handles: Vec<tokio::task::JoinHandle<()>>,
}

impl UserDataConnectionManager {
    /**
     * Creates a new user data connection manager.
     *
     * User data streams require authentication and always use dynamic mode.
     *
     * # Arguments
     * - `config`: Binance configuration with authentication credentials.
     *
     * # Returns
     * - Tuple containing the connection manager and message sender channel.
     */
    pub fn new(config: BinanceConfig<StreamConfig>) -> Result<(Self, mpsc::UnboundedSender<StreamMessage>)> {
        if !config.has_authentication() {
            return Err(anyhow::anyhow!(
                "User data streams require authentication credentials (API key)"
            ));
        }

        let (status_sender, status_receiver) = watch::channel(ConnectionStatus::Connecting);
        let (message_sender, message_receiver) = mpsc::unbounded_channel();
        
        let url = config.user_data_url().to_string();
        let stream_config = config.stream_config().clone();
        let signer = config.signer().clone(); // Get the full signer

        let task_handle = tokio::spawn(
            ConnectionUtils::run_connection(
                url, 
                stream_config, 
                status_sender, 
                UnifiedConnectionHandler::new_dynamic(message_receiver, signer)
            )
        );

        let manager = Self {
            config,
            status_receiver,
            _task_handles: vec![task_handle],
        };

        Ok((manager, message_sender))
    }
}

impl ConnectionManager for UserDataConnectionManager {
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
