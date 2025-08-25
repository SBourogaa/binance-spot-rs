use serde::{Deserialize, Serialize};

/**
 * Cancel replace mode options for atomic cancel-replace operations.
 *
 * Controls the behavior when a cancel-replace operation encounters failures
 * during the cancellation or new order placement phases.
 *
 * # Variants
 * - `StopOnFailure`: If cancellation request fails, new order placement will not be attempted.
 * - `AllowFailure`: New order placement will be attempted even if the cancel request fails.
 * - `Unknown`: Any mode not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum CancelReplaceMode {
    StopOnFailure,
    AllowFailure,
    #[serde(other, skip_serializing)]
    Unknown,
}
