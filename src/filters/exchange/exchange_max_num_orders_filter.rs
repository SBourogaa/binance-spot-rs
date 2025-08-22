use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of orders an account is allowed to have open on the exchange.
 * Note that both "algo" orders and normal orders are counted for this filter.
 *
 * # Fields
 * - `max_num_orders`: Maximum number of open orders allowed on the exchange.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeMaxNumOrdersFilter {
    #[serde(rename = "maxNumOrders")]
    pub max_num_orders: u32,
}