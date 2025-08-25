use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of times an order can
 * be amended on the given symbol.
 *
 * If there are too many order amendments made on a single order,
 * you will receive the -2038 error code.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxNumOrderAmendsFilter {
    #[serde(rename = "maxNumOrderAmends")]
    pub max_num_order_amends: u32,
}
