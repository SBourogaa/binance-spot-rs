use serde::{Deserialize, Serialize};

use super::{
    ExchangeMaxNumAlgoOrdersFilter, ExchangeMaxNumIcebergOrdersFilter, ExchangeMaxNumOrdersFilter,
};

/**
 * Exchange-level filters.
 *
 * # Variants
 * - Each variant wraps a struct containing the concrete rule fields.
 */
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "filterType")]
#[serde(deny_unknown_fields)]
pub enum ExchangeFilter {
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    ExchangeMaxNumOrders(ExchangeMaxNumOrdersFilter),
    #[serde(rename = "EXCHANGE_MAX_NUM_ALGO_ORDERS")]
    ExchangeMaxNumAlgoOrders(ExchangeMaxNumAlgoOrdersFilter),
    #[serde(rename = "EXCHANGE_MAX_NUM_ICEBERG_ORDERS")]
    ExchangeMaxNumIcebergOrders(ExchangeMaxNumIcebergOrdersFilter),
}
