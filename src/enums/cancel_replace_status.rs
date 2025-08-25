use serde::{Deserialize, Serialize};

/**
 * Cancel replace operation status for individual operations.
 *
 * Indicates the outcome of either the cancellation or new order placement
 * phases of a cancel-replace operation.
 *
 * # Variants
 * - `Success`: The operation completed successfully.
 * - `Failure`: The operation failed.
 * - `NotAttempted`: The operation was not attempted (e.g., due to STOP_ON_FAILURE mode).
 * - `Unknown`: Any status not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum CancelReplaceStatus {
    Success,
    Failure,
    NotAttempted,
    #[serde(other, skip_serializing)]
    Unknown,
}
