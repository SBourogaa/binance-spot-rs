use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of "algo" orders an account is allowed to have open on a symbol.
 * "Algo" orders are STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, and TAKE_PROFIT_LIMIT orders.
 *
 * # Fields
 * - `max_num_algo_orders`: Maximum number of open algo orders allowed on a symbol.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxNumAlgoOrdersFilter {
    #[serde(rename = "maxNumAlgoOrders")]
    pub max_num_algo_orders: u32,
}