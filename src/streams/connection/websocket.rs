use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use tracing::{info, instrument};

use super::types::{WsRead, WsSink, WsStream};
use crate::Result;

/**
 * WebSocket connection wrapper
 *
 * Provides a unified interface for WebSocket read and write operations.
 *
 * # Fields
 * - `write`: WebSocket write half for sending messages
 * - `read`: WebSocket read half for receiving messages
 */
pub(super) struct WebSocketConnection {
    write: WsSink,
    read: WsRead,
}

impl WebSocketConnection {
    /**
     * Creates a new WebSocket connection wrapper
     *
     * Takes ownership of a WebSocket stream and splits it into read and write halves
     * for concurrent operation.
     *
     * # Arguments
     * - `stream`: The WebSocket stream to wrap
     *
     * # Returns
     * - New WebSocketConnection instance
     */
    pub fn new(stream: WsStream) -> Self {
        let (write, read) = stream.split();
        Self { write, read }
    }

    /**
     * Sends a message over the WebSocket connection
     *
     * # Arguments
     * - `message`: The WebSocket message to send
     *
     * # Returns
     * - Result indicating success or failure of the send operation
     */
    #[instrument(skip(self, message))]
    pub async fn send_message(&mut self, message: Message) -> Result<()> {
        let start = std::time::Instant::now();
        let result = self
            .write
            .send(message)
            .await
            .context("Failed to send WebSocket message");

        info!(
            duration_us = start.elapsed().as_micros(),
            success = result.is_ok(),
            "WebSocket message send completed"
        );

        result
    }

    /**
     * Receives the next message from the WebSocket connection
     *
     * # Returns
     * - Option containing the message result, or None if stream is closed
     */
    pub async fn next_message(
        &mut self,
    ) -> Option<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>> {
        self.read.next().await
    }

    /**
     * Gracefully closes the WebSocket connection
     *
     * Sends a close frame to the remote endpoint and closes the write half
     * of the connection. The read half should continue to be monitored for
     * the close acknowledgment.
     *
     * # Returns
     * - Result indicating success or failure of the close operation
     */
    #[instrument(skip(self))]
    pub async fn close(&mut self) -> Result<()> {
        self.write
            .send(Message::Close(None))
            .await
            .context("Failed to send close frame")
    }
}
