use std::collections::HashMap;

use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tokio::sync::{mpsc, oneshot, watch};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

use crate::Result;
use crate::{
    BinanceConfig, WebSocketConfig, clients::common::generate_signature, errors::BinanceError,
};

/**
 * WebSocket connection status for monitoring connection health.
 *
 * # Variants
 * - `Connecting`: Initial connection attempt in progress.
 * - `Connected`: Successfully connected and operational.
 * - `Reconnecting`: Connection lost, attempting to reconnect.
 * - `Disconnected`: Connection closed gracefully.
 * - `Failed`: Connection failed permanently after max retry attempts.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Reconnecting { attempt: u32 },
    Disconnected,
    Failed,
}

/**
 * WebSocket API client implementation with configurable connection management.
 *
 * # Fields
 * - `config`: Binance configuration containing API credentials and WebSocket-specific settings.
 * - `request_sender`: Channel to send requests to the background task.
 * - `connection_handle`: Handle to the background connection task.
 * - `status_receiver`: Channel to receive connection status updates.
 */
#[allow(dead_code)]
pub struct BinanceSpotWebSocketClient {
    pub(crate) config: BinanceConfig<WebSocketConfig>,
    pub(crate) request_sender: Option<mpsc::UnboundedSender<TaskMessage>>,
    pub(crate) connection_handle: Option<tokio::task::JoinHandle<()>>,
    pub(crate) status_receiver: watch::Receiver<ConnectionStatus>,
}

/**
 * Internal message type for communicating with the background WebSocket task.
 *
 * # Fields
 * - `id`: Unique request ID.
 * - `method`: WebSocket method name.
 * - `params`: Optional parameters for the request.
 * - `response_sender`: Channel to send the response back to the caller.
 */
#[derive(Debug)]
pub(crate) struct RequestMessage {
    pub(crate) id: String,
    pub(crate) method: String,
    pub(crate) params: Option<Value>,
    pub(crate) response_sender: oneshot::Sender<Result<Value>>,
}

/**
 * Message types for communicating with the background WebSocket task.
 *
 * # Variants
 * - `Request`: Normal API request message.
 * - `Shutdown`: Graceful shutdown command.
 */
#[derive(Debug)]
pub(crate) enum TaskMessage {
    Request(RequestMessage),
    Shutdown(oneshot::Sender<Result<()>>),
}

impl BinanceSpotWebSocketClient {
    /**
     * Creates a new WebSocket client instance with configurable connection management.
     *
     * # Arguments
     * - `config`: Binance configuration with API credentials and WebSocket settings.
     *
     * # Returns
     * - `Self`: New WebSocket client instance.
     */
    pub fn new(config: BinanceConfig<WebSocketConfig>) -> Result<Self> {
        let (request_sender, request_receiver) = mpsc::unbounded_channel::<TaskMessage>();
        let (status_sender, status_receiver) = watch::channel(ConnectionStatus::Connecting);
        let ws_url = format!("{}/ws-api/v3", config.url());
        let ws_config = config.websocket_config().clone();

        let connection_handle = tokio::spawn(Self::connection_task(
            ws_url,
            request_receiver,
            ws_config,
            status_sender,
        ));

        Ok(Self {
            config,
            request_sender: Some(request_sender),
            connection_handle: Some(connection_handle),
            status_receiver,
        })
    }

    /**
     * Returns the current connection status.
     *
     * # Returns
     * - `ConnectionStatus`: Current connection state.
     */
    pub fn connection_status(&self) -> ConnectionStatus {
        self.status_receiver.borrow().clone()
    }

    /**
     * Waits for the connection to be established.
     *
     * # Returns
     * - `()`: When connection is established.
     */
    #[instrument(skip(self))]
    pub async fn wait_for_connection(&mut self) -> Result<()> {
        let start = std::time::Instant::now();
        let mut status_receiver = self.status_receiver.clone();
        loop {
            let current_status = status_receiver.borrow().clone();
            match current_status {
                ConnectionStatus::Connected => {
                    info!(
                        wait_duration_us = start.elapsed().as_micros(),
                        "WebSocket connection ready"
                    );
                    return Ok(());
                }
                ConnectionStatus::Failed => {
                    return Err(anyhow::anyhow!("Connection failed permanently"));
                }
                ConnectionStatus::Disconnected => {
                    return Err(anyhow::anyhow!("Connection disconnected"));
                }
                _ => {
                    status_receiver
                        .changed()
                        .await
                        .context("Status channel closed")?;
                }
            }
        }
    }

    /**
     * Gracefully closes the WebSocket connection.
     *
     * Ensures all pending requests are sent before closing the connection.
     *
     * # Returns
     * - `()`: When connection is gracefully closed.
     */
    pub async fn close(&mut self) -> Result<()> {
        if let Some(sender) = self.request_sender.take() {
            let (response_sender, response_receiver) = oneshot::channel();

            sender
                .send(TaskMessage::Shutdown(response_sender))
                .context("Failed to send shutdown command")?;

            let result =
                tokio::time::timeout(std::time::Duration::from_secs(10), response_receiver)
                    .await
                    .context("Shutdown timeout")?
                    .context("Failed to receive shutdown response")?;

            if let Some(handle) = self.connection_handle.take() {
                handle.abort();
            }

            result
        } else {
            Ok(())
        }
    }

    /**
     * Calculates exponential backoff delay for connection retry attempts.
     *
     * # Arguments
     * - `attempts`: Current number of connection attempts.
     * - `ws_config`: WebSocket configuration containing delay settings.
     *
     * # Returns
     * - `Duration`: Calculated delay duration with exponential backoff and max cap.
     */
    fn calculate_retry_delay(attempts: u32, ws_config: &WebSocketConfig) -> std::time::Duration {
        let base_delay = ws_config.initial_retry_delay * 2_u32.pow(attempts - 1);
        let capped_delay = std::cmp::min(base_delay, ws_config.max_retry_delay);

        debug!(
            attempt = attempts,
            base_delay_us = base_delay.as_micros(),
            capped_delay_us = capped_delay.as_micros(),
            initial_delay_us = ws_config.initial_retry_delay.as_micros(),
            max_delay_us = ws_config.max_retry_delay.as_micros(),
            "Calculated exponential backoff delay"
        );

        capped_delay
    }

    /**
     * Establishes a WebSocket connection with retry logic and status updates.
     *
     * # Arguments
     * - `url`: WebSocket URL to connect to.
     * - `ws_config`: WebSocket configuration for timeouts and retry settings.
     * - `reconnect_attempts`: Mutable reference to track current attempt count.
     * - `status_sender`: Channel to send connection status updates.
     *
     * # Returns
     * - `Option<WebSocketStream>`: Established WebSocket connection on success, None if max retries exceeded.
     */
    async fn establish_connection(
        url: &str,
        ws_config: &WebSocketConfig,
        reconnect_attempts: &mut u32,
        status_sender: &watch::Sender<ConnectionStatus>,
    ) -> Option<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    > {
        let lifecycle_start = std::time::Instant::now();

        let status = if *reconnect_attempts == 0 {
            ConnectionStatus::Connecting
        } else {
            ConnectionStatus::Reconnecting {
                attempt: *reconnect_attempts,
            }
        };
        let _ = status_sender.send(status);

        info!(
            attempt = *reconnect_attempts,
            timeout_ms = ws_config.connection_timeout.as_millis(),
            "Starting connection attempt"
        );

        let connection_start = std::time::Instant::now();
        let connection_result =
            tokio::time::timeout(ws_config.connection_timeout, connect_async(url)).await;
        let connection_duration = connection_start.elapsed();

        match connection_result {
            Ok(Ok((stream, response))) => {
                *reconnect_attempts = 0;
                let _ = status_sender.send(ConnectionStatus::Connected);

                info!(
                    total_duration_us = lifecycle_start.elapsed().as_micros(),
                    connection_duration_us = connection_duration.as_micros(),
                    response_status = response.status().as_u16(),
                    "WebSocket connection established successfully"
                );

                Some(stream)
            }
            Ok(Err(e)) => {
                if *reconnect_attempts >= ws_config.max_reconnect_attempts {
                    error!(
                        max_attempts = ws_config.max_reconnect_attempts,
                        total_duration_us = lifecycle_start.elapsed().as_micros(),
                        connection_duration_us = connection_duration.as_micros(),
                        error = ?e,
                        "Max reconnection attempts reached, giving up"
                    );
                    let _ = status_sender.send(ConnectionStatus::Failed);
                    return None;
                }

                *reconnect_attempts += 1;
                let delay = Self::calculate_retry_delay(*reconnect_attempts, ws_config);

                warn!(
                    attempt = *reconnect_attempts,
                    connection_duration_us = connection_duration.as_micros(),
                    error = ?e,
                    retry_delay_us = delay.as_micros(),
                    "Connection failed, retrying"
                );
                tokio::time::sleep(delay).await;
                None
            }
            Err(_) => {
                if *reconnect_attempts >= ws_config.max_reconnect_attempts {
                    error!(
                        max_attempts = ws_config.max_reconnect_attempts,
                        total_duration_us = lifecycle_start.elapsed().as_micros(),
                        timeout_us = ws_config.connection_timeout.as_micros(),
                        "Max reconnection attempts reached after timeout, giving up"
                    );
                    let _ = status_sender.send(ConnectionStatus::Failed);
                    return None;
                }

                *reconnect_attempts += 1;
                let delay = Self::calculate_retry_delay(*reconnect_attempts, ws_config);

                warn!(
                    attempt = *reconnect_attempts,
                    timeout_us = ws_config.connection_timeout.as_micros(),
                    retry_delay_us = delay.as_micros(),
                    "Connection timeout, retrying"
                );
                tokio::time::sleep(delay).await;
                None
            }
        }
    }

    /**
     * Handles incoming client requests by serializing and sending over WebSocket.
     *
     * # Arguments
     * - `req`: Client request message containing method and parameters.
     * - `write`: WebSocket write half for sending messages.
     * - `pending_requests`: Map to track pending requests awaiting responses.
     *
     * # Returns
     * - `bool`: True to continue processing, false if connection should restart.
     */
    async fn handle_client_request(
        req: RequestMessage,
        write: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            Message,
        >,
        pending_requests: &mut HashMap<String, oneshot::Sender<Result<Value>>>,
    ) -> bool {
        let task_start = std::time::Instant::now();
        let serialize_start = std::time::Instant::now();

        let mut ws_request = json!({
            "id": req.id.clone(),
            "method": req.method
        });

        if let Some(params) = req.params {
            ws_request["params"] = params;
        }

        let request_text = match serde_json::to_string(&ws_request) {
            Ok(text) => text,
            Err(e) => {
                let _ = req.response_sender.send(Err(
                    anyhow::Error::from(e).context("Failed to serialize request")
                ));
                return true;
            }
        };

        let serialize_duration = serialize_start.elapsed();

        let send_start = std::time::Instant::now();
        if let Err(e) = write.send(Message::Text(request_text)).await {
            let _ = req.response_sender.send(Err(
                anyhow::Error::from(e).context("Failed to send WebSocket message")
            ));
            return false;
        }
        let send_duration = send_start.elapsed();

        pending_requests.insert(req.id.clone(), req.response_sender);

        info!(
            request_id = req.id,
            method = req.method,
            task_duration_us = task_start.elapsed().as_micros(),
            serialize_duration_us = serialize_duration.as_micros(),
            send_duration_us = send_duration.as_micros(),
            pending_count = pending_requests.len(),
            "Background task processed client request"
        );

        true
    }

    /**
     * Handles incoming WebSocket messages including responses, pings, and connection events.
     *
     * # Arguments
     * - `message`: WebSocket message result from the server.
     * - `write`: WebSocket write half for sending responses (pongs).
     * - `pending_requests`: Map to match responses with pending requests.
     *
     * # Returns
     * - `bool`: True to continue processing, false if connection should restart.
     */
    async fn handle_websocket_message(
        message: Option<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
        write: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            Message,
        >,
        pending_requests: &mut HashMap<String, oneshot::Sender<Result<Value>>>,
    ) -> bool {
        let message_start = std::time::Instant::now();

        match message {
            Some(Ok(Message::Text(text))) => {
                let parse_start = std::time::Instant::now();
                let response: Value = match serde_json::from_str(&text) {
                    Ok(json) => json,
                    Err(e) => {
                        error!(error = %e, "Failed to parse WebSocket response");
                        return true;
                    }
                };
                let parse_duration = parse_start.elapsed();

                let request_id = match response.get("id").and_then(|id| id.as_str()) {
                    Some(id) => id.to_string(),
                    None => {
                        warn!(message = %text, "Received WebSocket message without ID");
                        return true;
                    }
                };

                let process_start = std::time::Instant::now();
                if let Some(response_sender) = pending_requests.remove(&request_id) {
                    let result = Self::parse_websocket_response(response);
                    let _ = response_sender.send(result);
                }
                let process_duration = process_start.elapsed();

                info!(
                    request_id = request_id,
                    message_duration_us = message_start.elapsed().as_micros(),
                    parse_duration_us = parse_duration.as_micros(),
                    process_duration_us = process_duration.as_micros(),
                    pending_count = pending_requests.len(),
                    "Background task processed WebSocket response"
                );

                true
            }
            Some(Ok(Message::Ping(data))) => {
                let pong_start = std::time::Instant::now();
                if let Err(e) = write.send(Message::Pong(data)).await {
                    error!(error = %e, "Failed to send pong");
                    return false;
                }
                let pong_duration = pong_start.elapsed();
                debug!(
                    pong_duration_us = pong_duration.as_micros(),
                    "Background task handled ping/pong"
                );
                true
            }
            Some(Ok(Message::Pong(_))) => {
                debug!("Background task received pong");
                true
            }
            Some(Ok(Message::Close(_))) => {
                debug!("Background task received close message");
                false
            }
            Some(Err(e)) => {
                error!(error = %e, "WebSocket error");
                false
            }
            None => {
                debug!("Background task: WebSocket stream ended");
                false
            }
            _ => true,
        }
    }

    /**
     * Handles graceful shutdown of WebSocket connection with proper close handshake.
     *
     * # Arguments
     * - `response_sender`: Channel to send shutdown completion result.
     * - `write`: WebSocket write half for sending close frame.
     * - `read`: WebSocket read half for receiving close acknowledgment.
     * - `pending_requests`: Map of pending requests to fail during shutdown.
     * - `status_sender`: Channel to update connection status.
     */
    async fn handle_shutdown(
        response_sender: oneshot::Sender<Result<()>>,
        write: &mut futures_util::stream::SplitSink<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            Message,
        >,
        read: &mut futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
        >,
        pending_requests: &mut HashMap<String, oneshot::Sender<Result<Value>>>,
        status_sender: &watch::Sender<ConnectionStatus>,
    ) {
        let _ = status_sender.send(ConnectionStatus::Disconnected);

        if let Err(e) = write.send(Message::Close(None)).await {
            let _ = response_sender.send(Err(
                anyhow::Error::from(e).context("Failed to send close frame")
            ));
            return;
        }

        let close_timeout = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Close(_)) => break,
                    Ok(Message::Text(text)) => {
                        if let Ok(response) = serde_json::from_str::<Value>(&text) {
                            if let Some(id) = response.get("id").and_then(|id| id.as_str()) {
                                if let Some(sender) = pending_requests.remove(id) {
                                    let result = Self::parse_websocket_response(response);
                                    let _ = sender.send(result);
                                }
                            }
                        }
                    }
                    Err(_) => break,
                    _ => {}
                }
            }
        })
        .await;

        for (_, sender) in pending_requests.drain() {
            let _ = sender.send(Err(anyhow::anyhow!("Connection closed during shutdown")));
        }

        match close_timeout {
            Ok(_) => {
                let _ = response_sender.send(Ok(()));
            }
            Err(_) => {
                let _ = response_sender.send(Err(anyhow::anyhow!("Close acknowledgment timeout")));
            }
        }
    }

    /**
     * Background task that maintains the persistent WebSocket connection with configurable behavior.
     *
     * # Arguments
     * - `url`: WebSocket URL to connect to.
     * - `request_receiver`: Channel to receive requests from the client.
     * - `ws_config`: WebSocket configuration for connection management.
     * - `status_sender`: Channel to send connection status updates.
     */
    async fn connection_task(
        url: String,
        mut request_receiver: mpsc::UnboundedReceiver<TaskMessage>,
        ws_config: WebSocketConfig,
        status_sender: watch::Sender<ConnectionStatus>,
    ) {
        let mut reconnect_attempts = 0;

        loop {
            let ws_stream = match Self::establish_connection(
                &url,
                &ws_config,
                &mut reconnect_attempts,
                &status_sender,
            )
            .await
            {
                Some(stream) => stream,
                None => break,
            };

            let (mut write, mut read) = ws_stream.split();
            let mut pending_requests: HashMap<String, oneshot::Sender<Result<Value>>> =
                HashMap::new();

            // Track message processing stats
            let mut message_count = 0u64;
            let mut last_stats_time = std::time::Instant::now();
            const STATS_INTERVAL: std::time::Duration = std::time::Duration::from_secs(10);

            loop {
                tokio::select! {
                    request = request_receiver.recv() => {
                        match request {
                            Some(TaskMessage::Request(req)) => {
                                // Monitor channel pressure
                                info!(
                                    pending_requests = pending_requests.len(),
                                    "Channel pressure check"
                                );

                                if pending_requests.len() > 100 {
                                    warn!(
                                        pending_requests = pending_requests.len(),
                                        "High channel pressure detected"
                                    );
                                }

                                if !Self::handle_client_request(req, &mut write, &mut pending_requests).await {
                                    break;
                                }
                            }
                            Some(TaskMessage::Shutdown(response_sender)) => {
                                Self::handle_shutdown(response_sender, &mut write, &mut read, &mut pending_requests, &status_sender).await;
                                return;
                            }
                            None => {
                                let _ = status_sender.send(ConnectionStatus::Disconnected);
                                return;
                            }
                        }
                    }
                    message = read.next() => {
                        message_count += 1;

                        // Log message processing rates periodically
                        if last_stats_time.elapsed() >= STATS_INTERVAL {
                            let rate = message_count as f64 / last_stats_time.elapsed().as_secs_f64();
                            info!(
                                messages_processed = message_count,
                                messages_per_second = rate,
                                pending_requests = pending_requests.len(),
                                "WebSocket message processing stats"
                            );
                            message_count = 0;
                            last_stats_time = std::time::Instant::now();
                        }

                        if !Self::handle_websocket_message(message, &mut write, &mut pending_requests).await {
                            break;
                        }
                    }
                }
            }

            for (_, sender) in pending_requests.drain() {
                let _ = sender.send(Err(anyhow::anyhow!("WebSocket connection lost")));
            }

            tokio::time::sleep(ws_config.initial_retry_delay).await;
        }
    }

    /**
     * Parses a WebSocket response and extracts the result.
     *
     * # Arguments
     * - `response`: The parsed JSON response from WebSocket.
     *
     * # Returns
     * - `Value`: The result.
     */
    fn parse_websocket_response(response: Value) -> Result<Value> {
        if let Some(error) = response.get("error") {
            if let (Some(code), Some(msg)) = (error.get("code"), error.get("msg")) {
                if let (Some(code_num), Some(msg_str)) = (code.as_i64(), msg.as_str()) {
                    return Err(BinanceError::Api(crate::errors::ApiError::new(
                        code_num as i32,
                        msg_str.to_string(),
                    ))
                    .into());
                }
            }
            return Err(anyhow::anyhow!("WebSocket error: {}", error));
        }

        if let Some(status) = response.get("status") {
            if status.as_u64() != Some(200) {
                return Err(anyhow::anyhow!("WebSocket status error: {}", status));
            }
        }

        if let Some(result) = response.get("result") {
            Ok(result.clone())
        } else {
            Err(anyhow::anyhow!("Missing result field in response"))
        }
    }

    /**
     * Sends a public (unsigned) request over the persistent WebSocket connection.
     *
     * # Arguments
     * - `method`: The API method name.
     * - `params`: Serializable parameters for the request.
     *
     * # Returns
     * - `Value`: The response result.
     */
    #[instrument(skip(self, params), fields(method = method))]
    pub(crate) async fn send_request<T: Serialize>(
        &self,
        method: &str,
        params: T,
    ) -> Result<Value> {
        let start = std::time::Instant::now();
        let prep_start = std::time::Instant::now();

        let request_sender = self
            .request_sender
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("WebSocket client not initialized"))?;

        let request_id = Uuid::new_v4().to_string();
        let (response_sender, response_receiver) = oneshot::channel();

        let json_params =
            serde_json::to_value(&params).context("Failed to serialize parameters")?;

        let params_option = if json_params.is_null()
            || (json_params.is_object() && json_params.as_object().unwrap().is_empty())
        {
            None
        } else {
            Some(json_params)
        };

        let request_message = RequestMessage {
            id: request_id.clone(),
            method: method.to_string(),
            params: params_option,
            response_sender,
        };

        let prep_duration = prep_start.elapsed();
        info!(
            method = method,
            request_id = request_id,
            prep_duration_us = prep_duration.as_micros(),
            "WebSocket request preparation completed"
        );

        let channel_start = std::time::Instant::now();
        request_sender
            .send(TaskMessage::Request(request_message))
            .context("Failed to send request to WebSocket task")?;
        let channel_send_duration = channel_start.elapsed();

        let wait_start = std::time::Instant::now();
        let result = tokio::time::timeout(std::time::Duration::from_secs(30), response_receiver)
            .await
            .context("WebSocket request timeout")?
            .context("Failed to receive WebSocket response")?;
        let wait_duration = wait_start.elapsed();

        info!(
            method = method,
            request_id = request_id,
            total_duration_us = start.elapsed().as_micros(),
            prep_duration_us = prep_duration.as_micros(),
            channel_send_duration_us = channel_send_duration.as_micros(),
            wait_duration_us = wait_duration.as_micros(),
            success = result.is_ok(),
            "WebSocket API request completed"
        );

        result
    }

    /**
     * Sends an authenticated WebSocket API request.
     *
     * # Arguments
     * - `method`: API method name.
     * - `params`: Serializable request parameters.
     *
     * # Returns
     * - `Value`: JSON response.
     */
    #[instrument(skip(self, params), fields(method = method))]
    pub(crate) async fn send_signed_request<T: Serialize>(
        &self,
        method: &str,
        params: T,
    ) -> Result<Value> {
        let start = std::time::Instant::now();
        let prep_start = std::time::Instant::now();

        let signer = self
            .config
            .signer()
            .ok_or_else(|| anyhow::anyhow!("No authentication configured"))?;

        let (signature, query_string) =
            generate_signature(&params, signer.as_ref(), self.config.recv_window(), true).await?;

        let mut final_params = serde_json::Map::new();
        for pair in query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                final_params.insert(key.to_string(), json!(value));
            }
        }
        final_params.insert("signature".to_string(), json!(signature));

        let prep_duration = prep_start.elapsed();
        info!(
            method = method,
            prep_duration_us = prep_duration.as_micros(),
            "Signed WebSocket request preparation completed"
        );

        let result = self
            .send_request(method, serde_json::Value::Object(final_params))
            .await;

        info!(
            method = method,
            total_duration_us = start.elapsed().as_micros(),
            signing_prep_duration_us = prep_duration.as_micros(),
            success = result.is_ok(),
            "Signed WebSocket API request completed"
        );

        result
    }

    /**
     * Helper for public WebSocket calls with validation and JSON parsing.
     *
     * # Arguments
     * - `method_name`: WebSocket method name.
     * - `spec`: Request specification with validation.
     *
     * # Returns
     * - `R`: Parsed response object.
     */
    pub(crate) async fn request<S, R>(&self, method_name: &str, spec: S) -> Result<R>
    where
        S: Serialize,
        R: DeserializeOwned,
    {
        let mut response = self.send_request(method_name, spec).await?;

        if response.is_object() && std::any::type_name::<R>().starts_with("alloc::vec::Vec<") {
            response = serde_json::Value::Array(vec![response]);
        }

        serde_json::from_value(response).context("Failed to parse response")
    }

    /**
     * Helper for authenticated WebSocket calls with validation and JSON parsing.
     *
     * # Arguments
     * - `method_name`: WebSocket method name.
     * - `spec`: Request specification with validation.
     *
     * # Returns
     * - `R`: Parsed response object.
     */
    pub(crate) async fn signed_request<S, R>(&self, method_name: &str, spec: S) -> Result<R>
    where
        S: Serialize,
        R: DeserializeOwned,
    {
        let response = self.send_signed_request(method_name, spec).await?;
        serde_json::from_value(response).context("Failed to parse response")
    }
}

impl Drop for BinanceSpotWebSocketClient {
    /**
     * Cleanup when the client is dropped.
     * Signals shutdown by dropping the request sender and aborts the background task.
     *
     * Note: Graceful shutdown must be done explicitly by calling `close()` before drop.
     * This implementation only does immediate cleanup to avoid runtime conflicts.
     */
    fn drop(&mut self) {
        self.request_sender.take();

        if let Some(handle) = self.connection_handle.take() {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinanceConfig, WebSocketConfig, errors::BinanceError};
    use serde_json::json;
    use std::time::Duration;

    /**
     * Tests client creation with default config.
     */
    #[tokio::test]
    async fn test_new_default_config() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");

        // Act
        let result = BinanceSpotWebSocketClient::new(config);

        // Assert
        assert!(result.is_ok());
    }

    /**
     * Tests client creation with custom WebSocket timeouts.
     */
    #[tokio::test]
    async fn test_new_custom_config() {
        // Arrange
        let ws_config = WebSocketConfig::builder()
            .with_connection_timeout(Duration::from_secs(5))
            .with_max_reconnects(3)
            .build();

        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .with_websocket_config(ws_config)
            .build()
            .expect("Config creation");

        // Act
        let result = BinanceSpotWebSocketClient::new(config);

        // Assert
        assert!(result.is_ok());
    }

    /**
     * Tests parse_websocket_response with successful response.
     */
    #[tokio::test]
    async fn test_parse_websocket_response_success() {
        // Arrange
        let response = json!({
            "id": "test-123",
            "status": 200,
            "result": {"serverTime": 1234567890}
        });

        // Act
        let result = BinanceSpotWebSocketClient::parse_websocket_response(response);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["serverTime"], 1234567890);
    }

    /**
     * Tests parse_websocket_response with API error.
     */
    #[tokio::test]
    async fn test_parse_websocket_response_api_error() {
        // Arrange
        let response = json!({
            "id": "test-123",
            "error": {
                "code": -1121,
                "msg": "Invalid symbol."
            }
        });

        // Act
        let result = BinanceSpotWebSocketClient::parse_websocket_response(response);

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        let binance_error = error.downcast_ref::<BinanceError>();
        assert!(matches!(binance_error, Some(BinanceError::Api(_))));
    }

    /**
     * Tests parse_websocket_response with WebSocket status error.
     */
    #[tokio::test]
    async fn test_parse_websocket_response_status_error() {
        // Arrange
        let response = json!({
            "id": "test-123",
            "status": 400,
            "result": {}
        });

        // Act
        let result = BinanceSpotWebSocketClient::parse_websocket_response(response);

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("WebSocket status error")
        );
    }

    /**
     * Tests parse_websocket_response with malformed response.
     */
    #[tokio::test]
    async fn test_parse_websocket_response_missing_result() {
        // Arrange
        let response = json!({
            "id": "test-123",
            "status": 200
        });

        // Act
        let result = BinanceSpotWebSocketClient::parse_websocket_response(response);

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Missing result field")
        );
    }

    /**
     * Tests send_signed_request without authentication configured.
     */
    #[tokio::test]
    async fn test_send_signed_request_no_auth() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotWebSocketClient::new(config).unwrap();

        // Act
        let result = client.send_signed_request("account.status", ()).await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No authentication configured")
        );
    }

    /**
     * Tests send_request with uninitialized client.
     */
    #[tokio::test]
    async fn test_send_request_not_initialized() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let mut client = BinanceSpotWebSocketClient::new(config).unwrap();

        client.request_sender = None;

        // Act
        let result = client.send_request("ping", ()).await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("WebSocket client not initialized")
        );
    }

    /**
     * Tests initial connection status.
     */
    #[tokio::test]
    async fn test_connection_status_initial() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotWebSocketClient::new(config).unwrap();

        // Act
        let status = client.connection_status();

        // Assert
        assert_eq!(status, ConnectionStatus::Connecting);
    }

    /**
     * Tests send_request parameter serialization.
     */
    #[tokio::test]
    async fn test_send_request_serialization() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotWebSocketClient::new(config).unwrap();

        #[derive(serde::Serialize)]
        struct TestParams {
            symbol: String,
            limit: u32,
        }

        let params = TestParams {
            symbol: "BTCUSDT".to_string(),
            limit: 100,
        };

        // Act
        let result = client.send_request("depth", params).await;

        // Assert
        match result {
            Ok(_) => {}
            Err(e) => {
                let error_string = e.to_string();
                assert!(
                    error_string.contains("WebSocket request timeout")
                        || error_string.contains("Failed to send request")
                        || error_string.contains("WebSocket client not initialized"),
                    "Should be timeout or connection error, not serialization error: {}",
                    error_string
                );
            }
        }
    }

    /**
     * Tests request helper method type conversion.
     */
    #[tokio::test]
    async fn test_request_helper_type_conversion() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotWebSocketClient::new(config).unwrap();

        // Act
        let result: Result<serde_json::Value> = client.request("ping", ()).await;

        // Assert
        match result {
            Ok(_) => {}
            Err(e) => {
                let error_string = e.to_string();
                assert!(
                    error_string.contains("WebSocket request timeout")
                        || error_string.contains("Failed to receive WebSocket response")
                        || error_string.contains("Failed to parse response"),
                    "Should be timeout or parse error: {}",
                    error_string
                );
            }
        }
    }

    /**
     * Tests signed_request helper without authentication.
     */
    #[tokio::test]
    async fn test_signed_request_helper_no_auth() {
        // Arrange
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .build()
            .expect("Config creation");
        let client = BinanceSpotWebSocketClient::new(config).unwrap();

        // Act
        let result: Result<serde_json::Value> = client.signed_request("account.status", ()).await;

        // Assert
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No authentication configured")
        );
    }
}
