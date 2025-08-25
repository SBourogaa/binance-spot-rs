use serde::{Deserialize, Serialize};

use crate::types::responses::{CommissionRates, DiscountInfo};

/**
 * Symbol-specific commission rates response.
 *
 * Contains commission rates for a specific trading symbol including standard rates,
 * tax rates, and discount information when paying with specific assets.
 *
 * # Fields
 * - `symbol`: Trading symbol these rates apply to.
 * - `standard_commission`: Standard commission rates for trades.
 * - `tax_commission`: Tax commission rates for trades.
 * - `special_commission`: Special commission rates (if applicable).
 * - `discount`: Discount information when paying in specific assets.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SymbolCommissionRates {
    pub symbol: String,
    pub standard_commission: CommissionRates,
    pub tax_commission: CommissionRates,
    pub special_commission: Option<CommissionRates>,
    pub discount: DiscountInfo,
}
