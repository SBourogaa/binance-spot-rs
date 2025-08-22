use std::collections::HashMap;

use serde_json::Value;
use tokio::sync::oneshot;
use tracing::debug;

use crate::Result;
use super::types::{
    ValueSender, 
    HandlerMode
};

/**
 * Message router for WebSocket stream data.
 *
 * Manages the routing of incoming WebSocket messages to the appropriate broadcast
 * channels based on the connection mode.
 *
 * # Fields
 * - `dynamic_channels`: Map of stream names to broadcast senders for dynamic mode.
 * - `pending_requests`: Map of request IDs to response senders for subscription management.
 */
pub(super) struct MessageRouter {
    dynamic_channels: HashMap<String, ValueSender>,
    pending_requests: HashMap<String, oneshot::Sender<Result<()>>>,
}

impl MessageRouter {
    /**
     * Creates a new message router.
     *
     * # Returns
     * - New MessageRouter instance with empty channel maps.
     */
    pub fn new() -> Self {
        Self {
            dynamic_channels: HashMap::new(),
            pending_requests: HashMap::new(),
        }
    }

    /**
     * Adds a dynamic subscription channel.
     *
     * # Arguments
     * - `stream_name`: Name of the stream to register.
     * - `sender`: Broadcast sender for the stream.
     */
    pub fn add_subscription(&mut self, stream_name: String, sender: ValueSender) {
        self.dynamic_channels.insert(stream_name, sender);
    }


    /**
     * Removes a dynamic subscription channel.
     *
     * # Arguments
     * - `stream_name`: Name of the stream to remove.
     */
    pub fn remove_subscription(&mut self, stream_name: &str) {
        self.dynamic_channels.remove(stream_name);
    }

    /**
     * Adds a pending subscription request.
     *
     * # Arguments
     * - `request_id`: Unique ID of the subscription request.
     * - `response_sender`: Channel to send the subscription result.
     */
    pub fn add_pending_request(&mut self, request_id: String, response_sender: oneshot::Sender<Result<()>>) {
        self.pending_requests.insert(request_id, response_sender);
    }

    /**
     * Routes incoming WebSocket messages to appropriate channels.
     *
     * # Arguments
     * - `value`: The incoming JSON message.
     * - `mode`: The current connection mode.
     *
     * # Returns
     * - `true` if the message was successfully routed.
     */
    pub fn route_message(&mut self, value: &Value, mode: &HandlerMode) -> bool {
        if self.handle_subscription_response(value) {
            return true;
        }

        let routed = match mode {
            HandlerMode::Dynamic => self.route_dynamic_data(value),
            HandlerMode::Static { senders } => self.route_static_data(value, senders),
        };

        if !routed {
            debug!("Unrouted message: {}", serde_json::to_string(value).unwrap_or_else(|_| "invalid JSON".to_string()));
        }

        routed
    }

    /**
     * Handles subscription/unsubscription response messages
     * 
     * Processes responses from the WebSocket API for subscription management
     * operations and sends the results back to the requesting clients.
     * 
     * # Arguments
     * - `value`: The JSON response message
     * 
     * # Returns
     * - `true` if this was a subscription response, `false` otherwise
     */
    fn handle_subscription_response(&mut self, value: &Value) -> bool {
        if let Some(id) = value.get("id").and_then(|id| id.as_str()) {
            if let Some(sender) = self.pending_requests.remove(id) {
                let result = if value.get("error").is_some() {
                    Err(anyhow::anyhow!("WebSocket error: {:?}", value.get("error")))
                } else {
                    Ok(())
                };
                let _ = sender.send(result);
                return true;
            } else {
                // TODO: Clean this up. 
                // If this is an API status response (like account.status) and we don't have a pending request,
                // it's not an error - just continue routing
                if value.get("result").and_then(|r| r.get("apiKey")).is_some() {
                    return false;
                }
                
                return true;
            }
        }
        false
    }

    /**
     * Routes data messages in dynamic mode.
     *
     * Attempts multiple routing strategies in sequence to handle different
     * message formats from the dynamic WebSocket endpoint.
     *
     * # Arguments
     * - `value`: The JSON message to route.
     *
     * # Returns
     * - `true` if the message was successfully routed.
     */
    fn route_dynamic_data(&self, value: &Value) -> bool {
        self.route_combined_format(value) 
            || self.route_user_data_event(value)
            || self.route_nested_user_data_event(value)
            || self.route_api_response(value)
    }

    /**
     * Routes combined format messages from /stream endpoint.
     *
     * Handles messages with "stream" and "data" fields from the combined
     * stream format used by the /stream endpoint.
     *
     * # Arguments
     * - `value`: The JSON message to route.
     *
     * # Returns
     * - `true` if this was a combined format message.
     */
    fn route_combined_format(&self, value: &Value) -> bool {
        if let (Some(stream_name), Some(data)) = (value.get("stream"), value.get("data")) {
            if let Some(stream_name_str) = stream_name.as_str() {
                if let Some(sender) = self.dynamic_channels.get(stream_name_str) {
                    let _ = sender.send(data.clone());
                    debug!("Routed combined format message to stream: {}", stream_name_str);
                } else {
                    debug!("No subscription found for stream: {}", stream_name_str);
                }
            }
            return true;
        }
        false
    }

    /**
     * Routes direct user data events.
     *
     * Handles user data events that have an 'e' field containing the event type.
     * Routes these events to the "userData" subscription channel.
     *
     * # Arguments
     * - `value`: The JSON message to route.
     *
     * # Returns
     * - `true` if this was a user data event.
     */
    fn route_user_data_event(&self, value: &Value) -> bool {
        if let Some(event_type) = value.get("e").and_then(|e| e.as_str()) {
            if let Some(sender) = self.dynamic_channels.get("userData") {
                let _ = sender.send(value.clone());
                debug!("Routed user data event to userData subscription: {}", event_type);
                return true;
            } else {
                debug!("No userData subscription found for event: {}", event_type);
            }
        }
        false
    }

    /**
     * Routes user data events nested in 'event' field.
     *
     * Handles user data events that are wrapped in an "event" field,
     * extracting the nested event data for routing.
     *
     * # Arguments
     * - `value`: The JSON message to route.
     *
     * # Returns
     * - `true` if this was a nested user data event.
     */
    fn route_nested_user_data_event(&self, value: &Value) -> bool {
        if let Some(event_data) = value.get("event") {
            if let Some(event_type) = event_data.get("e").and_then(|e| e.as_str()) {
                if let Some(sender) = self.dynamic_channels.get("userData") {
                    let _ = sender.send(event_data.clone());
                    debug!("Routed nested user data event to userData subscription: {}", event_type);
                    return true;
                } else {
                    debug!("No userData subscription found for nested event: {}", event_type);
                }
            }
        }
        false
    }

    /**
     * Routes API responses that don't need specific subscription routing.
     *
     * Handles API status responses and other administrative messages
     * that don't require routing to subscription channels.
     *
     * # Arguments
     * - `value`: The JSON message to route.
     *
     * # Returns
     * - `true` if this was an API response message.
     */
    fn route_api_response(&self, value: &Value) -> bool {
        if let Some(result) = value.get("result") {
            if result.get("apiKey").is_some() || result.get("connectedSince").is_some() {
                debug!("Handled API response message");
                return true;
            }
        }
        false
    }

    /**
     * Routes data messages in static mode
     * 
     * Handles both combined format and direct event format. Uses stream 
     * name resolution to match events with the correct broadcast channels.
     * 
     * # Arguments
     * - `value`: The JSON message containing stream data
     * - `senders`: Map of stream names to broadcast senders
     * 
     * # Returns
     * - `true` indicating the message was processed (may or may not have been routed)
     */
    fn route_static_data(&self, value: &Value, senders: &HashMap<String, ValueSender>) -> bool {
        if let (Some(stream_name), Some(data)) = (value.get("stream"), value.get("data")) {
            if let Some(stream_name_str) = stream_name.as_str() {
                if let Some(sender) = senders.get(stream_name_str) {
                    let _ = sender.send(data.clone());
                }
            }
            return true;
        }
        
        if value.is_array() {
            for (stream_name, sender) in senders {
                if stream_name.starts_with('!') {
                    let _ = sender.send(value.clone());
                }
            }
            return true;
        }
        
        if senders.len() == 1 {
            if let Some((_, sender)) = senders.iter().next() {
                let _ = sender.send(value.clone());
            }
        }
        
        true
    }

    /**
     * Shuts down all pending requests
     * 
     * Sends failure responses to all pending subscription requests when
     * the connection is shutting down. This ensures no requests are left
     * hanging indefinitely.
     */
    pub fn shutdown_all_pending(&mut self) {
        for (_, sender) in self.pending_requests.drain() {
            let _ = sender.send(Err(anyhow::anyhow!("Connection shutting down")));
        }
    }
}