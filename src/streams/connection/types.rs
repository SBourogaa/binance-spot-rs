use std::collections::HashMap;

use serde_json::Value;
use tokio::sync::{broadcast, oneshot};

use crate::Result;

/**
 * Used to send stream data to multiple subscribers in both dynamic and static modes.
 */
pub type ValueSender = broadcast::Sender<Value>;

/**
 * Used by subscribers to receive stream data from the connection manager.
 */
pub type ValueReceiver = broadcast::Receiver<Value>;

/**
 * Type aliases for WebSocket stream components.
 */
pub(super) type WsStream = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
pub(super) type WsSink = futures_util::stream::SplitSink<WsStream, tokio_tungstenite::tungstenite::protocol::Message>;
pub(super) type WsRead = futures_util::stream::SplitStream<WsStream>;

/**
 * WebSocket connection status for monitoring connection health.
 *
 * # Variants
 * - `Connecting`: Initial connection attempt in progress.
 * - `Connected`: Successfully connected and operational.
 * - `Reconnecting`: Connection lost, attempting to reconnect with attempt count.
 * - `Disconnected`: Connection closed gracefully.
 * - `Failed`: Connection failed permanently after maximum retry attempts.
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
 * Message types for communicating with the connection handler.
 *
 * # Variants
 * - `Subscribe`: Request to subscribe to a specific stream.
 * - `Unsubscribe`: Request to unsubscribe from specific streams.
 * - `Shutdown`: Request to gracefully shutdown the connection.
 */
#[derive(Debug)]
pub enum StreamMessage {
    /**
     * Subscribe to a stream.
     *
     * # Fields
     * - `stream_name`: Name of the stream to subscribe to.
     * - `sender`: Broadcast sender for routing stream data.
     * - `response`: Channel to send subscription result.
     */
    Subscribe {
        stream_name: String,
        sender: ValueSender,
        response: oneshot::Sender<Result<()>>,
    },
    
    /**
     * Unsubscribe from streams.
     *
     * # Fields
     * - `stream_names`: Names of streams to unsubscribe from.
     * - `response`: Channel to send unsubscription result.
     */
    Unsubscribe {
        stream_names: Vec<String>,
        response: oneshot::Sender<Result<()>>,
    },
    
    /**
     * Gracefully shutdown the connection.
     *
     * # Fields
     * - Channel to send shutdown completion result.
     */
    Shutdown(oneshot::Sender<Result<()>>),
}

/**
 * Connection handler operation mode.
 *
 * # Variants
 * - `Dynamic`: Supports runtime subscription management via WebSocket API.
 * - `Static`: Pre-configured streams with fixed routing to broadcast channels.
 */
#[derive(Debug)]
pub(super) enum HandlerMode {
    /**
     * Dynamic mode for runtime subscription management.
     */
    Dynamic,
    
    /**
     * Static mode for pre-configured streams.
     *
     * # Fields
     * - `senders`: Map of stream names to their broadcast senders.
     */
    Static {
        senders: HashMap<String, ValueSender>,
    },
}