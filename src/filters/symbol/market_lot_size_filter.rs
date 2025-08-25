use serde::{Deserialize, Serialize};

/**
 * This filter defines the quantity (aka "lots" in auction terms) rules for MARKET orders on a symbol.
 *
 * In order to pass the market lot size, the following must be true for quantity:
 * - quantity >= minQty
 * - quantity <= maxQty
 * - quantity % stepSize == 0
 *
 * # Fields
 * - `min_qty`: Defines the minimum quantity allowed for MARKET orders.
 * - `max_qty`: Defines the maximum quantity allowed for MARKET orders.
 * - `step_size`: Defines the intervals that a quantity can be increased/decreased by for MARKET orders.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketLotSizeFilter {
    #[serde(rename = "minQty")]
    pub min_qty: String,
    #[serde(rename = "maxQty")]
    pub max_qty: String,
    #[serde(rename = "stepSize")]
    pub step_size: String,
}
