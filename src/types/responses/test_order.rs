use serde::{Deserialize, Serialize};

use crate::types::responses::{
    DiscountInfo,
    CommissionRates 
};

/**
 * Response from test order endpoints.
 *
 * Test order endpoints validate order parameters but don't place actual orders.
 * By default returns empty response, but can include commission rates if requested.
 *
 * # Fields
 * - `standard_commission_for_order`: Standard commission rates (if computeCommissionRates=true).
 * - `tax_commission_for_order`: Tax commission rates (if computeCommissionRates=true).
 * - `discount`: Discount information (if computeCommissionRates=true).
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TestOrder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_commission_for_order: Option<CommissionRates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_commission_for_order: Option<CommissionRates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<DiscountInfo>,
}