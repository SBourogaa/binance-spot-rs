use serde::{Deserialize, Serialize};

use super::{
    BalanceUpdateEvent, EventStreamTerminatedEvent, ExecutionReportEvent, ExternalLockUpdateEvent,
    ListStatusEvent, OutboundAccountPositionEvent,
};

/**
 * Unified User Data Event Enum
 *
 * Represents all possible user data stream events from Binance.
 * Uses serde tag-based deserialization on the "e" field to automatically
 * route to the correct event variant.
 *
 * # Variants
 * - `ExecutionReport`: For order execution reports.
 * - `OutboundAccountPosition`: For account position updates.
 * - `BalanceUpdate`: For single asset balance updates.
 * - `ListStatus`: For order list status updates.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e")]
#[serde(deny_unknown_fields)]
pub enum UserDataEvent {
    #[serde(rename = "executionReport")]
    ExecutionReport(ExecutionReportEvent),
    #[serde(rename = "outboundAccountPosition")]
    OutboundAccountPosition(OutboundAccountPositionEvent),
    #[serde(rename = "balanceUpdate")]
    BalanceUpdate(BalanceUpdateEvent),
    #[serde(rename = "listStatus")]
    ListStatus(ListStatusEvent),
    #[serde(rename = "externalLockUpdate")]
    ExternalLockUpdate(ExternalLockUpdateEvent),
    #[serde(rename = "eventStreamTerminated")]
    EventStreamTerminated(EventStreamTerminatedEvent),
}
