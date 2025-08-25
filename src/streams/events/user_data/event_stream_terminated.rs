use serde::{Deserialize, Serialize};

/**
 * Event Stream Terminated Event
 *
 * Appears only when subscribed on the WebSocket API. Sent when the user data
 * stream is stopped (e.g., after unsubscribe or session logout).
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventStreamTerminatedEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
}
