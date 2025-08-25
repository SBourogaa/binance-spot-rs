use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of ICEBERG orders an account is allowed to have open on a symbol.
 * An ICEBERG order is any order where the icebergQty is > 0.
 *
 * # Fields
 * - `max_num_iceberg_orders`: Maximum number of iceberg orders allowed on a symbol.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxNumIcebergOrdersFilter {
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: u32,
}
