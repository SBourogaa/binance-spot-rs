use serde::{Deserialize, Serialize};

/**
 * Rate limit interval types.
 *
 * # Variants
 * - `Second`: Per second interval.
 * - `Minute`: Per minute interval.
 * - `Hour`: Per hour interval.
 * - `Day`: Per day interval.
 * - `Unknown`: Any interval not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum RateLimitInterval {
    Second,
    Minute,
    Hour,
    Day,
    #[serde(other, skip_serializing)]
    Unknown,
}
