use serde::{Deserialize, Serialize};

/**
 * Order-list order status for individual orders within an order list.
 *
 * # Variants
 * - `Executing`: Either an order list has been placed or there is an update to the status of the list.
 * - `AllDone`: An order list has completed execution and thus no longer active.
 * - `Reject`: The List Status is responding to a failed action either during order placement or order canceled.
 * - `Unknown`: Any status not recognized. 
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum OrderListOrderStatus {
    Executing,
    AllDone,
    Reject,
    #[serde(other, skip_serializing)]
    Unknown,
}