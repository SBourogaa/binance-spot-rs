use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Balance Update Event
 *
 * Sent whenever there's a balance update for a single asset.
 *
 * # Fields
 * - `event_type`: Always "balanceUpdate".
 * - `event_time`: The time the event was generated, in milliseconds since epoch.
 * - `asset`: The asset for which the balance was updated.
 * - `balance_delta`: The change in balance for the asset.
 * - `clear_time`: The time when the balance update was cleared, in milliseconds since epoch.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceUpdateEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d")]
    #[serde(with = "rust_decimal::serde::str")]
    pub balance_delta: Decimal,
    #[serde(rename = "T")]
    pub clear_time: u64,
}
