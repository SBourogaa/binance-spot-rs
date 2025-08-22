use serde::{Deserialize, Serialize};

/**
 * Order execution status.
 *
 * # Variants
 * - `New`: The order has been accepted by the engine.
 * - `PendingNew`: The order is in a pending phase until the working order of an order list has been fully filled.
 * - `PartiallyFilled`: A part of the order has been filled.
 * - `Filled`: The order has been completed.
 * - `Canceled`: The order has been canceled by the user.
 * - `PendingCancel`: Currently unused.
 * - `Rejected`: The order was not accepted by the engine and not processed.
 * - `Expired`: The order was canceled according to the order type's rules or by the exchange.
 * - `ExpiredInMatch`: The order was expired by the exchange due to STP.
 * - `Unknown`: Any order status not recognized. 
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum OrderStatus {
    New,
    PendingNew,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    ExpiredInMatch,
    #[serde(other, skip_serializing)]
    Unknown,
}