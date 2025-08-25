use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of iceberg orders an account is allowed to have open on the exchange.
 *
 * # Fields
 * - `max_num_iceberg_orders`: Maximum number of iceberg orders allowed on the exchange.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeMaxNumIcebergOrdersFilter {
    #[serde(rename = "maxNumIcebergOrders")]
    pub max_num_iceberg_orders: u32,
}
