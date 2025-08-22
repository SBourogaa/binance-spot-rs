use std::sync::Arc;
use std::collections::HashMap;

use uuid::Uuid;
use anyhow::Context;
use serde_json::json;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::protocol::Message;
use chrono::Utc;
use tracing::{error, warn};

use crate::Result;
use crate::auth::SignatureProvider;
use super::{
    types::{
        HandlerMode, 
        StreamMessage, 
        ValueSender
    },
    websocket::WebSocketConnection,
    router::MessageRouter,
    state::ConnectionState,
};

/**
 * Unified connection handler for both dynamic and static modes.
 *
 * Manages the WebSocket connection lifecycle, handles incoming messages,
 * and coordinates between message routing and state management components.
 *
 * # Fields
 * - `mode`: Connection operation mode (dynamic or static).
 * - `message_receiver`: Channel for receiving commands from clients.
 * - `message_router`: Routes incoming messages to appropriate channels.
 * - `state`: Tracks active subscriptions for reconnection.
 * - `signer`: Optional signature provider for user data stream authentication.
 * - `pending_user_data_subscriptions`: Map of logon request IDs to pending user data subscriptions.
 */
pub(super) struct UnifiedConnectionHandler {
    mode: HandlerMode,
    message_receiver: mpsc::UnboundedReceiver<StreamMessage>,
    message_router: MessageRouter,
    state: ConnectionState,
    signer: Option<Arc<dyn SignatureProvider>>,
    pending_user_data_subscriptions: std::collections::HashMap<String, (String, ValueSender, tokio::sync::oneshot::Sender<crate::Result<()>>)>,
}

impl UnifiedConnectionHandler {
    /**
     * Creates a new handler for dynamic mode.
     *
     * # Arguments
     * - `message_receiver`: Channel for receiving client commands.
     * - `signer`: Optional signature provider for user data stream authentication.
     *
     * # Returns
     * - New UnifiedConnectionHandler configured for dynamic mode.
     */
    pub fn new_dynamic(
        message_receiver: mpsc::UnboundedReceiver<StreamMessage>, 
        signer: Option<Arc<dyn SignatureProvider>>
    ) -> Self {
        Self {
            mode: HandlerMode::Dynamic,
            message_receiver,
            message_router: MessageRouter::new(),
            state: ConnectionState::new(),
            signer,
            pending_user_data_subscriptions: std::collections::HashMap::new(),
        }
    }

    /**
     * Creates a new handler for static mode.
     *
     * # Arguments
     * - `message_receiver`: Channel for receiving client commands.
     * - `senders`: Map of stream names to broadcast senders.
     *
     * # Returns
     * - New UnifiedConnectionHandler configured for static mode.
     */
    pub fn new_static(
        message_receiver: mpsc::UnboundedReceiver<StreamMessage>,
        senders: HashMap<String, ValueSender>,
    ) -> Self {
        Self {
            mode: HandlerMode::Static { senders },
            message_receiver,
            message_router: MessageRouter::new(),
            state: ConnectionState::new(),
            signer: None, // Static mode doesn't use authentication
            pending_user_data_subscriptions: std::collections::HashMap::new(),
        }
    }

    /**
     * Handles post-connection setup and state restoration.
     *
     * # Arguments
     * - `ws_connection`: The established WebSocket connection.
     *
     * # Returns
     * - `()` on successful state restoration.
     */
    pub async fn on_connected(&mut self, ws_connection: &mut WebSocketConnection) -> Result<()> {
        if matches!(self.mode, HandlerMode::Dynamic) && self.state.has_active_subscriptions() {
            let active_subs = self.state.active_subscriptions();
            
            let user_data_streams: Vec<_> = active_subs.iter().filter(|s| *s == "userData").collect();
            let market_data_streams: Vec<_> = active_subs.iter().filter(|s| *s != "userData").collect();
            
            if !user_data_streams.is_empty() && self.signer.is_some() {
                let logon_id = uuid::Uuid::new_v4().to_string();
                let timestamp = Utc::now().timestamp_millis();
                let recv_window = 5000;
                let signer = self.signer.as_ref()
                    .expect("Signer must be available for user data stream restoration");
                let api_key = signer.get_api_key();
                
                let signature_payload = format!(
                    "apiKey={}&recvWindow={}&timestamp={}", 
                    api_key, recv_window, timestamp
                );
                
                let signature = signer.sign(&signature_payload).await
                    .context("Failed to generate signature for session authentication")?;
                
                let logon_msg = json!({
                    "method": "session.logon",
                    "params": {
                        "apiKey": api_key,
                        "timestamp": timestamp,
                        "recvWindow": recv_window,
                        "signature": signature
                    },
                    "id": logon_id
                });
                
                ws_connection
                    .send_message(Message::Text(logon_msg.to_string()))
                    .await
                    .context("Failed to authenticate session during reconnection")?;
                
                let subscribe_id = uuid::Uuid::new_v4().to_string();
                let subscribe_msg = json!({
                    "method": "userDataStream.subscribe",
                    "id": subscribe_id
                });
                
                ws_connection
                    .send_message(Message::Text(subscribe_msg.to_string()))
                    .await
                    .context("Failed to subscribe to user data stream during reconnection")?;
            }
            
            if !market_data_streams.is_empty() {
                let request_id = uuid::Uuid::new_v4().to_string();
                let subscribe_msg = json!({
                    "method": "SUBSCRIBE",
                    "params": market_data_streams,
                    "id": request_id
                });
                
                ws_connection
                    .send_message(Message::Text(subscribe_msg.to_string()))
                    .await
                    .context("Failed to restore market data subscriptions after reconnection")?;
            }
        }
        Ok(())
    }

    /**
     * Main connection handling loop
     * 
     * Manages the WebSocket connection by handling both client commands
     * and incoming WebSocket messages concurrently using tokio::select!.
     * 
     * # Arguments
     * - `ws_connection`: The WebSocket connection to handle
     * 
     * # Returns
     * - Result indicating the reason for connection termination
     */
    pub async fn handle_connection(&mut self, ws_connection: &mut WebSocketConnection) -> Result<()> {
        loop {
            tokio::select! {
                Some(cmd) = self.message_receiver.recv() => {
                    if self.handle_command(cmd, ws_connection).await? {
                        return Ok(());
                    }
                }
                Some(msg) = ws_connection.next_message() => {
                    if !self.handle_websocket_message(msg, ws_connection).await? {
                        break;
                    }
                }
                else => break,
            }
        }
        
        Err(anyhow::anyhow!("Connection lost"))
    }

    /**
     * Handles client commands (subscribe, unsubscribe, shutdown)
     * 
     * Processes commands from clients and sends appropriate WebSocket
     * messages for subscription management.
     * 
     * # Arguments
     * - `command`: The client command to process
     * - `ws_connection`: WebSocket connection for sending messages
     * 
     * # Returns
     * - Result<bool> where true indicates shutdown was requested
     */
    async fn handle_command(&mut self, command: StreamMessage, ws_connection: &mut WebSocketConnection) -> Result<bool> {
        match command {
            StreamMessage::Subscribe { stream_name, sender, response } => {
                match &self.mode {
                    HandlerMode::Dynamic => {
                        if stream_name == "userData" {
                            let logon_id = Uuid::new_v4().to_string();
                            let timestamp = Utc::now().timestamp_millis();
                            let recv_window = 5000;
                            
                            let signer = self.signer.as_ref()
                                .ok_or_else(|| anyhow::anyhow!("Signer required for user data streams"))?;
                            let api_key = signer.get_api_key();
                            
                            let signature_payload = format!(
                                "apiKey={}&recvWindow={}&timestamp={}", 
                                api_key, recv_window, timestamp
                            );
                            
                            let signature = signer.sign(&signature_payload).await
                                .context("Failed to generate signature for user data stream authentication")?;
                            
                            let logon_msg = json!({
                                "method": "session.logon",
                                "params": {
                                    "apiKey": api_key,
                                    "timestamp": timestamp,
                                    "recvWindow": recv_window,
                                    "signature": signature
                                },
                                "id": logon_id
                            });
                            
                            if let Err(e) = ws_connection.send_message(Message::Text(logon_msg.to_string())).await {
                                let _ = response.send(Err(e.context("Failed to authenticate session")));
                                return Err(anyhow::anyhow!("Failed to authenticate session"));
                            }
                            
                            self.pending_user_data_subscriptions.insert(
                                logon_id.clone(), 
                                (stream_name.clone(), sender, response)
                            );
                        } else {
                            let request_id = Uuid::new_v4().to_string();
                            let subscribe_msg = json!({
                                "method": "SUBSCRIBE",
                                "params": [&stream_name],
                                "id": request_id
                            });
                            
                            if let Err(e) = ws_connection.send_message(Message::Text(subscribe_msg.to_string())).await {
                                let _ = response.send(Err(e.context("Failed to send subscription")));
                                return Err(anyhow::anyhow!("Failed to send subscription"));
                            }
                            
                            self.message_router.add_subscription(stream_name.clone(), sender);
                            self.state.add_subscription(stream_name);
                            self.message_router.add_pending_request(request_id, response);
                        }
                    }
                    HandlerMode::Static { .. } => {
                        let _ = response.send(Err(anyhow::anyhow!("Subscribe not supported in static mode")));
                    }
                }
                Ok(false)
            }
            StreamMessage::Unsubscribe { stream_names, response } => {
                match &self.mode {
                    HandlerMode::Dynamic => {
                        let user_data_streams: Vec<_> = stream_names.iter()
                            .filter(|name| name.as_str() == "userData")
                            .collect();
                        let market_data_streams: Vec<_> = stream_names.iter()
                            .filter(|name| name.as_str() != "userData")
                            .collect();

                        if !user_data_streams.is_empty() {
                            let request_id = Uuid::new_v4().to_string();
                            let unsubscribe_msg = json!({
                                "method": "userDataStream.unsubscribe",
                                "id": request_id
                            });
                            
                            if let Err(e) = ws_connection.send_message(Message::Text(unsubscribe_msg.to_string())).await {
                                warn!("Failed to send user data stream unsubscribe message: {}", e);
                            }
                            
                            for stream_name in &user_data_streams {
                                self.message_router.remove_subscription(stream_name);
                            }
                            self.state.remove_subscriptions(&user_data_streams.iter().map(|s| s.to_string()).collect::<Vec<_>>());
                        }

                        if !market_data_streams.is_empty() {
                            let request_id = Uuid::new_v4().to_string();
                            let unsubscribe_msg = json!({
                                "method": "UNSUBSCRIBE",
                                "params": market_data_streams,
                                "id": request_id
                            });
                            
                            if let Err(e) = ws_connection.send_message(Message::Text(unsubscribe_msg.to_string())).await {
                                let _ = response.send(Err(e.context("Failed to send unsubscription")));
                                return Err(anyhow::anyhow!("Failed to send unsubscription"));
                            }
                            
                            for stream_name in &market_data_streams {
                                self.message_router.remove_subscription(stream_name);
                            }
                            self.state.remove_subscriptions(&market_data_streams.iter().map(|s| s.to_string()).collect::<Vec<_>>());
                            self.message_router.add_pending_request(request_id, response);
                        } else {
                            let _ = response.send(Ok(()));
                        }
                    }
                    HandlerMode::Static { .. } => {
                        let _ = response.send(Err(anyhow::anyhow!("Unsubscribe not supported in static mode")));
                    }
                }
                Ok(false)
            }
            StreamMessage::Shutdown(response) => {
                self.message_router.shutdown_all_pending();
                let _ = ws_connection.close().await;
                let _ = response.send(Ok(()));
                Ok(true)
            }
        }
    }

    /**
     * Handles incoming WebSocket messages
     * 
     * Processes WebSocket messages including text data, ping/pong frames,
     * and close frames. Routes data messages appropriately based on mode.
     * Also handles two-step user data stream authentication.
     * 
     * # Arguments
     * - `message`: The WebSocket message result
     * - `ws_connection`: WebSocket connection for sending responses
     * 
     * # Returns
     * - Result<bool> where false indicates connection should be restarted
     */
    async fn handle_websocket_message(
        &mut self, 
        message: std::result::Result<Message, tokio_tungstenite::tungstenite::Error>,
        ws_connection: &mut WebSocketConnection
    ) -> Result<bool> {
        match message? {
            Message::Text(text) => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(id) = value.get("id").and_then(|id| id.as_str()) {
                        if let Some((stream_name, sender, response)) = self.pending_user_data_subscriptions.remove(id) {
                            if value.get("error").is_some() {
                                error!("Session logon failed: {:?}", value.get("error"));
                                let _ = response.send(Err(anyhow::anyhow!("Session authentication failed: {:?}", value.get("error"))));
                                return Ok(true);
                            }
                            
                            let subscribe_id = Uuid::new_v4().to_string();
                            let subscribe_msg = json!({
                                "method": "userDataStream.subscribe",
                                "id": subscribe_id
                            });
                            
                            if let Err(e) = ws_connection.send_message(Message::Text(subscribe_msg.to_string())).await {
                                let _ = response.send(Err(e.context("Failed to subscribe to user data stream after authentication")));
                                return Err(anyhow::anyhow!("Failed to subscribe to user data stream"));
                            }
                            
                            self.message_router.add_subscription(stream_name.clone(), sender);
                            self.state.add_subscription(stream_name);
                            self.message_router.add_pending_request(subscribe_id, response);
                        }
                    }
                    
                    // Route the message through the normal router
                    self.message_router.route_message(&value, &self.mode);
                }
                Ok(true)
            }
            Message::Ping(data) => {
                ws_connection.send_message(Message::Pong(data)).await?;
                Ok(true)
            }
            Message::Close(_) => Ok(false),
            _ => Ok(true),
        }
    }
}