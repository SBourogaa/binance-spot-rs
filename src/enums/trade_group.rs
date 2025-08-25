use serde::{Deserialize, Serialize};

/**
 * Wrapper for dynamic trading-group IDs (`TRD_GRP_###`).
 *
 * # Fields
 * - `0`: The trading group ID number (must be ≥ 2).
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
#[serde(deny_unknown_fields)]
pub struct TradeGroup(pub u8);

impl TryFrom<u8> for TradeGroup {
    type Error = &'static str;

    /**
     * Creates a TradeGroup from a u8 value.
     *
     * # Arguments
     * - `value`: The trading group ID number.
     *
     * # Returns
     * - `Result<Self, Self::Error>`: TradeGroup if valid, error if invalid.
     */
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2..=255 => Ok(TradeGroup(value)),
            _ => Err("trade-group id must be ≥ 2 and ≤ 255."),
        }
    }
}

impl std::fmt::Display for TradeGroup {
    /**
     * Formats the TradeGroup as "TRD_GRP_###".
     *
     * # Arguments
     * - `f`: The formatter.
     *
     * # Returns
     * - `std::fmt::Result`: Formatting result.
     */
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "TRD_GRP_{:03}", self.0)
    }
}
