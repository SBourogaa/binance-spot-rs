use std::collections::HashMap;

use serde_json::{Value, json};
use tokio::sync::oneshot;
use tracing::{debug, trace, instrument, error};
use uuid::Uuid;

use crate::Result;
use super::types::{
    ValueSender, 
    HandlerMode
};

/**
 * Message router for WebSocket stream data.
 *
 * Manages the routing of incoming WebSocket messages to the appropriate broadcast
 * channels based on the connection mode. Handles both market data and user data streams
 * with different routing strategies.
 *
 * # Fields
 * - `dynamic_channels`: Map of stream names to broadcast senders for dynamic subscription mode.
 * - `pending_requests`: Map of request IDs to response senders for tracking subscription/unsubscription requests.
 * - `pending_user_data_logons`: Map of session.logon request IDs to user data subscription context,
 *   used for the two-step user data authentication flow.
 */
pub(super) struct MessageRouter {
    dynamic_channels: HashMap<String, ValueSender>,
    pending_requests: HashMap<String, oneshot::Sender<Result<()>>>,
    pending_user_data_logons: HashMap<String, (String, ValueSender, oneshot::Sender<Result<()>>)>,
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
            pending_user_data_logons: HashMap::new(),
        }
    }

    pub fn add_subscription(&mut self, stream_name: String, sender: ValueSender) {
        self.dynamic_channels.insert(stream_name, sender);
    }

    pub fn remove_subscription(&mut self, stream_name: &str) {
        self.dynamic_channels.remove(stream_name);
    }

    pub fn add_pending_request(&mut self, request_id: String, response_sender: oneshot::Sender<Result<()>>) {
        self.pending_requests.insert(request_id, response_sender);
    }

    pub fn add_pending_user_data_logon(
        &mut self, 
        logon_id: String, 
        stream_name: String, 
        sender: ValueSender, 
        response_sender: oneshot::Sender<Result<()>>
    ) {
        self.pending_user_data_logons.insert(logon_id, (stream_name, sender, response_sender));
    }
    
    pub fn try_handle_user_data_logon(&mut self, value: &Value) -> Option<Value> {
        if let Some(id) = value.get("id").and_then(|id| id.as_str()) {
            if let Some((stream_name, sender, response)) = self.pending_user_data_logons.remove(id) {
                return self.handle_user_data_logon_response(value, stream_name, sender, response);
            }
        }
        None
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
    #[instrument(skip(self, value), fields(has_id = value.get("id").is_some()))]
    pub fn route_message(&mut self, value: &Value, mode: &HandlerMode) -> bool {
        let start = std::time::Instant::now();
        if self.handle_subscription_response(value) {
            return true;
        }

        let routed = match mode {
            HandlerMode::Dynamic => self.route_dynamic_data(value),
            HandlerMode::Static { senders } => self.route_static_data(value, senders),
        };

        let duration = start.elapsed();
        
        if !routed {
            debug!("Unrouted message: {}", serde_json::to_string(value).unwrap_or_else(|_| "invalid JSON".to_string()));
        }
        
        trace!(
            routing_time_us = duration.as_micros(),
            routed = routed,
            "Message routing completed"
        );
        
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
    #[instrument(skip(self, value))]
    fn handle_subscription_response(&mut self, value: &Value) -> bool {
        if let Some(id) = value.get("id").and_then(|id| id.as_str()) {
            if let Some(sender) = self.pending_requests.remove(id) {
                let result = if value.get("error").is_some() {
                    Err(anyhow::anyhow!("WebSocket error: {:?}", value.get("error")))
                } else {
                    Ok(())
                };
                let success = result.is_ok();
                let _ = sender.send(result);
                debug!(
                    request_id = id,
                    success = success,
                    "Subscription response processed"
                );
                return true;
            } else {
                if value.get("result").and_then(|r| r.get("apiKey")).is_some() {
                    debug!(request_id = id, "Received API status response, continuing to route");
                    return false;
                }
                
                debug!(request_id = id, "Received response for unknown request ID");
                return true;
            }
        }
        false
    }

    /**
     * Handles user data logon response and generates the subscribe message.
     *
     * # Arguments
     * - `value`: The logon response.
     * - `stream_name`: The stream name ("userData").
     * - `sender`: Channel sender for stream data.
     * - `response`: Response sender for subscription result.
     *
     * # Returns
     * - `Option<Value>` containing the subscribe message to send next, or None if logon failed.
     */
    fn handle_user_data_logon_response(
        &mut self,
        value: &Value,
        stream_name: String,
        sender: ValueSender,
        response: oneshot::Sender<Result<()>>
    ) -> Option<Value> {
        if let Some(error) = value.get("error") {
            error!("Session logon failed: {:?}", error);
            let _ = response.send(Err(anyhow::anyhow!("Session authentication failed: {:?}", error)));
            return None;
        }
        
        self.add_subscription(stream_name, sender);
        let subscribe_id = Uuid::new_v4().to_string();
        self.add_pending_request(subscribe_id.clone(), response);
        
        debug!(
            logon_success = true,
            subscribe_id = %subscribe_id,
            "User data logon successful, sending subscription request"
        );
        
        Some(json!({
            "method": "userDataStream.subscribe",
            "id": subscribe_id
        }))
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
     * - `true` if this was a combined format message, regardless of routing success.
     */
    fn route_combined_format(&self, value: &Value) -> bool {
        if let (Some(stream_name), Some(data)) = (value.get("stream"), value.get("data")) {
            if let Some(stream_name_str) = stream_name.as_str() {
                if let Some(sender) = self.dynamic_channels.get(stream_name_str) {
                    let _ = sender.send(data.clone());
                }
            }
            true
        } else {
            false
        }
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
     * - `true` if this was a user data event, regardless of routing success.
     */
    fn route_user_data_event(&self, value: &Value) -> bool {
        if value.get("e").is_some() {
            if let Some(sender) = self.dynamic_channels.get("userData") {
                let _ = sender.send(value.clone());
            }
            true
        } else {
            false
        }
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
     * - `true` if this was a nested user data event, regardless of routing success.
     */
    fn route_nested_user_data_event(&self, value: &Value) -> bool {
        if let Some(event_data) = value.get("event") {
            if event_data.get("e").is_some() {
                if let Some(sender) = self.dynamic_channels.get("userData") {
                    let _ = sender.send(event_data.clone());
                }
                true
            } else {
                false
            }
        } else {
            false
        }
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
                true
            } else {
                false
            }
        } else {
            false
        }
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
     * Sends failure responses to all pending subscription requests and user data logons
     * when the connection is shutting down. This ensures no requests are left hanging indefinitely.
     */
    #[instrument(skip(self))]
    pub fn shutdown_all_pending(&mut self) {
        let pending_count = self.pending_requests.len();
        let user_data_count = self.pending_user_data_logons.len();
        
        for (_, sender) in self.pending_requests.drain() {
            let _ = sender.send(Err(anyhow::anyhow!("Connection shutting down")));
        }
        
        for (_, (_, _, response)) in self.pending_user_data_logons.drain() {
            let _ = response.send(Err(anyhow::anyhow!("Connection shutting down")));
        }
        
        if pending_count > 0 || user_data_count > 0 {
            debug!(
                cancelled_requests = pending_count,
                cancelled_user_data_logons = user_data_count,
                "Cancelled pending requests during shutdown"
            );
        }
    }
}