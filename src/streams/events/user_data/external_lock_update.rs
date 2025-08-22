use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * External Lock Update Event
 * 
 * Sent when part of your spot wallet balance is locked/unlocked by an external system
 * (e.g., when used as margin collateral).
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalLockUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d")]
    #[serde(with = "rust_decimal::serde::str")]
    pub delta: Decimal,
    #[serde(rename = "T")]
    pub transaction_time: u64,
}
