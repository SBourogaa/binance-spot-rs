use serde::{Deserialize, Serialize};

/**
 * This filter defines the quantity (aka "lots" in auction terms) rules for a symbol.
 *
 * In order to pass the lot size, the following must be true for quantity/icebergQty:
 * - quantity >= minQty
 * - quantity <= maxQty
 * - quantity % stepSize == 0
 *
 * # Fields
 * - `min_qty`: Defines the minimum quantity/icebergQty allowed.
 * - `max_qty`: Defines the maximum quantity/icebergQty allowed.
 * - `step_size`: Defines the intervals that a quantity/icebergQty can be increased/decreased by.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LotSizeFilter {
    #[serde(rename = "minQty")]
    pub min_qty: String,
    #[serde(rename = "maxQty")]
    pub max_qty: String,
    #[serde(rename = "stepSize")]
    pub step_size: String,
}
