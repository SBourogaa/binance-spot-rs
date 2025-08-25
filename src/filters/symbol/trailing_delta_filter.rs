use serde::{Deserialize, Serialize};

/**
 * This filter defines the minimum and maximum value for the parameter trailingDelta.
 *
 * In order for a trailing stop order to pass this filter, the following must be true:
 *
 * For STOP_LOSS BUY, STOP_LOSS_LIMIT_BUY, TAKE_PROFIT SELL and TAKE_PROFIT_LIMIT SELL orders:
 * - trailingDelta >= minTrailingAboveDelta
 * - trailingDelta <= maxTrailingAboveDelta
 *
 * For STOP_LOSS SELL, STOP_LOSS_LIMIT SELL, TAKE_PROFIT BUY, and TAKE_PROFIT_LIMIT BUY orders:
 * - trailingDelta >= minTrailingBelowDelta
 * - trailingDelta <= maxTrailingBelowDelta
 *
 * # Fields
 * - `min_trailing_above_delta`: Minimum trailing delta above price.
 * - `max_trailing_above_delta`: Maximum trailing delta above price.
 * - `min_trailing_below_delta`: Minimum trailing delta below price.
 * - `max_trailing_below_delta`: Maximum trailing delta below price.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrailingDeltaFilter {
    #[serde(rename = "minTrailingAboveDelta")]
    pub min_trailing_above_delta: u32,
    #[serde(rename = "maxTrailingAboveDelta")]
    pub max_trailing_above_delta: u32,
    #[serde(rename = "minTrailingBelowDelta")]
    pub min_trailing_below_delta: u32,
    #[serde(rename = "maxTrailingBelowDelta")]
    pub max_trailing_below_delta: u32,
}
