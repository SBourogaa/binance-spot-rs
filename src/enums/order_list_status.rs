use serde::{Deserialize, Serialize};

/**
 * Order-list status type for OCO and other order list operations.
 *
 * # Variants
 * - `Response`: Used when the ListStatus is responding to a failed action.
 * - `ExecStarted`: The order list has been placed or there is an update to the order list status.
 * - `Updated`: The clientOrderId of an order in the order list has been changed.
 * - `AllDone`: The order list has finished executing and thus is no longer active.
 * - `Unknown`: Any status not recognized. 
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum OrderListStatus {
    Response,
    ExecStarted,
    Updated,
    AllDone,
    #[serde(other, skip_serializing)]
    Unknown,
}