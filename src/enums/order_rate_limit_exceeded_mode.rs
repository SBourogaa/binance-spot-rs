use serde::{Deserialize, Serialize};

/**
 * Order rate limit exceeded mode for cancel-replace operations.
 *
 * Controls the behavior when account's unfilled order count has been exceeded
 * during cancel-replace operations.
 *
 * # Variants
 * - `DoNothing`: Will only attempt to cancel the order if account has not exceeded the unfilled order rate limit (default).
 * - `CancelOnly`: Will always cancel the order regardless of rate limits.
 * - `Unknown`: Any mode not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum OrderRateLimitExceededMode {
    DoNothing,
    CancelOnly,
    #[serde(other, skip_serializing)]
    Unknown,
}
