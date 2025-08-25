use anyhow::Context;
use tokio::sync::watch;
use tokio_tungstenite::tungstenite::http::Request;
use tracing::{debug, error, info, warn};

use super::{
    handler::UnifiedConnectionHandler,
    types::{ConnectionStatus, WsStream},
    websocket::WebSocketConnection,
};
use crate::Result;
use crate::StreamConfig;

/**
 * Trait for connection managers that handle different stream types.
 */
pub trait ConnectionManager {
    /**
     * Gets the current connection status.
     *
     * # Returns
     * - Current connection status.
     */
    fn connection_status(&self) -> ConnectionStatus;

    /**
     * Checks if the connection is currently established.
     *
     * # Returns
     * - `true` if connected, `false` otherwise.
     */
    fn is_connected(&self) -> bool {
        matches!(self.connection_status(), ConnectionStatus::Connected)
    }

    /**
     * Gets the stream configuration.
     *
     * # Returns
     * - Reference to the stream configuration.
     */
    fn stream_config(&self) -> &StreamConfig;

    /**
     * Checks if authentication is configured.
     *
     * # Returns
     * - `true` if authentication credentials are available.
     */
    fn has_authentication(&self) -> bool;

    /**
     * Gets the market data WebSocket URL.
     *
     * # Returns
     * - Market data URL from configuration.
     */
    fn market_data_url(&self) -> &str;

    /**
     * Gets the user data WebSocket URL.
     *
     * # Returns
     * - User data URL from configuration.
     */
    fn user_data_url(&self) -> &str;

    /**
     * Waits for connection to be established.
     *
     * # Returns
     * - `()` on success.
     */
    fn wait_for_connection(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    /**
     * Aborts the connection and all background tasks.
     */
    fn abort_connection(&mut self);
}

/**
 * Shared connection utilities for WebSocket management.
 *
 * Contains common connection logic that is reused across different
 * connection manager implementations.
 */
pub struct ConnectionUtils;

impl ConnectionUtils {
    /**
     * Main connection runner with automatic reconnection.
     *
     * # Arguments
     * - `url`: WebSocket URL to connect to.
     * - `config`: Stream configuration with connection parameters.
     * - `status_sender`: Channel for sending connection status updates.
     * - `handler`: Connection handler for processing messages.
     */
    pub async fn run_connection(
        url: String,
        config: StreamConfig,
        status_sender: watch::Sender<ConnectionStatus>,
        mut handler: UnifiedConnectionHandler,
    ) {
        let mut reconnect_attempts = 0;
        let mut failed_due_to_max_retries = false;

        loop {
            let stream = match Self::establish_connection(
                &url,
                &config,
                &mut reconnect_attempts,
                &status_sender,
            )
            .await
            {
                Ok(stream) => stream,
                Err(_) => {
                    if reconnect_attempts > config.max_reconnect_attempts {
                        failed_due_to_max_retries = true;
                        break;
                    }
                    continue;
                }
            };

            let mut ws_connection = WebSocketConnection::new(stream);

            if let Err(e) = handler.on_connected(&mut ws_connection).await {
                error!("Failed to restore state after connection: {}", e);
                continue;
            }

            if handler.handle_connection(&mut ws_connection).await.is_ok() {
                break;
            }

            tokio::time::sleep(config.initial_retry_delay).await;
        }

        if !failed_due_to_max_retries {
            let _ = status_sender.send(ConnectionStatus::Disconnected);
        }
    }

    /**
     * Establishes WebSocket connection with retry logic.
     *
     * # Arguments
     * - `url`: WebSocket URL to connect to.
     * - `config`: Stream configuration with timeout and retry settings.
     * - `reconnect_attempts`: Mutable reference to current attempt count.
     * - `status_sender`: Channel for sending status updates.
     *
     * # Returns
     * - WebSocket stream on success.
     */
    pub async fn establish_connection(
        url: &str,
        config: &StreamConfig,
        reconnect_attempts: &mut u32,
        status_sender: &watch::Sender<ConnectionStatus>,
    ) -> Result<WsStream> {
        let status = if *reconnect_attempts == 0 {
            ConnectionStatus::Connecting
        } else {
            ConnectionStatus::Reconnecting {
                attempt: *reconnect_attempts,
            }
        };
        let _ = status_sender.send(status);

        let request = Self::build_websocket_request(url)?;
        let connection_result = tokio::time::timeout(
            config.connection_timeout,
            tokio_tungstenite::connect_async(request),
        )
        .await;

        match connection_result {
            Ok(Ok((stream, _))) => {
                *reconnect_attempts = 0;
                let _ = status_sender.send(ConnectionStatus::Connected);
                info!("WebSocket connection established to {}", url);
                Ok(stream)
            }
            Ok(Err(e)) => {
                *reconnect_attempts += 1;

                if *reconnect_attempts > config.max_reconnect_attempts {
                    error!(
                        "Max reconnection attempts ({}) reached, giving up",
                        config.max_reconnect_attempts
                    );
                    let _ = status_sender.send(ConnectionStatus::Failed);
                    return Err(anyhow::anyhow!(
                        "Connection failed after {} attempts",
                        config.max_reconnect_attempts
                    ));
                }

                let delay = Self::calculate_retry_delay(*reconnect_attempts, config);
                warn!(
                    "Failed to connect (attempt {}): {:?}, retrying in {:?}",
                    *reconnect_attempts, e, delay
                );
                tokio::time::sleep(delay).await;
                Err(anyhow::anyhow!("Connection failed: {}", e))
            }
            Err(_) => {
                *reconnect_attempts += 1;

                if *reconnect_attempts > config.max_reconnect_attempts {
                    error!(
                        "Max reconnection attempts ({}) reached, giving up",
                        config.max_reconnect_attempts
                    );
                    let _ = status_sender.send(ConnectionStatus::Failed);
                    return Err(anyhow::anyhow!(
                        "Connection timeout after {} attempts",
                        config.max_reconnect_attempts
                    ));
                }

                let delay = Self::calculate_retry_delay(*reconnect_attempts, config);
                warn!(
                    "Connection timeout (attempt {}), retrying in {:?}",
                    *reconnect_attempts, delay
                );
                tokio::time::sleep(delay).await;
                Err(anyhow::anyhow!("Connection timeout"))
            }
        }
    }

    /**
     * Builds a standard WebSocket request.
     *
     * # Arguments
     * - `url`: WebSocket URL for the request.
     *
     * # Returns
     * - HTTP request for WebSocket upgrade.
     */
    pub fn build_websocket_request(url: &str) -> Result<Request<()>> {
        use tokio_tungstenite::tungstenite::http::Uri;

        let uri: Uri = url.parse()?;
        let host = uri
            .host()
            .ok_or_else(|| anyhow::anyhow!("Invalid URL: missing host"))?;
        let host_header = if let Some(port) = uri.port() {
            format!("{}:{}", host, port)
        } else {
            host.to_string()
        };

        let request = Request::builder()
            .uri(url)
            .header("Host", host_header)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .body(())?;

        Ok(request)
    }

    /**
     * Calculates exponential backoff delay for retry attempts.
     *
     * # Arguments
     * - `attempts`: Current number of retry attempts.
     * - `config`: Stream configuration with delay settings.
     *
     * # Returns
     * - Duration to wait before next retry attempt.
     */
    pub fn calculate_retry_delay(attempts: u32, config: &StreamConfig) -> std::time::Duration {
        std::cmp::min(
            config.initial_retry_delay * 2_u32.pow(attempts - 1),
            config.max_retry_delay,
        )
    }

    /**
     * Common wait for connection implementation.
     *
     * # Arguments
     * - `status_receiver`: Channel for monitoring connection status.
     *
     * # Returns
     * - `()` on successful connection.
     */
    pub async fn wait_for_connection(
        status_receiver: &watch::Receiver<ConnectionStatus>,
    ) -> Result<()> {
        let mut status_receiver = status_receiver.clone();
        let connection_timeout = std::time::Duration::from_secs(30);
        let start_time = tokio::time::Instant::now();

        while start_time.elapsed() < connection_timeout {
            let current_status = status_receiver.borrow().clone();

            match current_status {
                ConnectionStatus::Connected => return Ok(()),
                ConnectionStatus::Failed => {
                    return Err(anyhow::anyhow!("Connection failed permanently"));
                }
                ConnectionStatus::Disconnected => {
                    return Err(anyhow::anyhow!("Connection disconnected"));
                }
                _ => {
                    tokio::select! {
                        result = status_receiver.changed() => {
                            result.context("Status channel closed")?;
                        }
                        _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
                    }
                }
            }
        }

        Err(anyhow::anyhow!(
            "Connection timeout after {:?}",
            connection_timeout
        ))
    }

    /**
     * Common abort connection implementation.
     *
     * # Arguments
     * - `task_handles`: Background task handles to abort.
     */
    pub fn abort_connection(task_handles: &[tokio::task::JoinHandle<()>]) {
        debug!("Aborting connection and background tasks");
        for handle in task_handles {
            handle.abort();
        }
    }
}
