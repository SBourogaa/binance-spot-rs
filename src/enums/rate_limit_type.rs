use serde::{Deserialize, Serialize};

/**
 * Rate limiter types for API request limits.
 *
 * # Variants
 * - `RequestWeight`: Weight-based rate limiting.
 * - `Orders`: Order count rate limiting.
 * - `RawRequests`: Raw request count rate limiting.
 * - `Connections`: Connection count rate limiting.
 * - `Unknown`: Any type not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
    Connections,
    #[serde(other, skip_serializing)]
    Unknown,
}