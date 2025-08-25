use serde::{Deserialize, Serialize};

/**
 * Order side indicating buy or sell direction.
 *
 * # Variants
 * - `Buy`: Buy order side.
 * - `Sell`: Sell order side.
 * - `Unknown`: Any orderside not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum OrderSide {
    Buy,
    Sell,
    #[serde(other, skip_serializing)]
    Unknown,
}
