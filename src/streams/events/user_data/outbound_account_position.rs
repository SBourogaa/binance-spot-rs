use serde::{Deserialize, Serialize};
use crate::types::responses::Balance;

/**
 * Outbound Account Position Event.
 * 
 * Sent whenever there's a change in account balances.
 * Contains all asset balances that have changed.
 * 
 * # Fields
 * - `event_type`: Always "outboundAccountPosition".
 * - `event_time`: The time the event was generated, in milliseconds since epoch.
 * - `last_update_time`: The last time the account was updated, in milliseconds since epoch.
 * - `balances`: A list of balances for each asset, including the asset name and the current balance.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundAccountPositionEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "u")]
    pub last_update_time: u64,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
}