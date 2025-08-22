use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of orders an account is allowed to have open on a symbol.
 * Note that both "algo" orders and normal orders are counted for this filter.
 *
 * # Fields
 * - `max_num_orders`: Maximum number of open orders allowed on a symbol.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxNumOrdersFilter {
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: u32,
}