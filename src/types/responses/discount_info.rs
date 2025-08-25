use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Discount information for commission payments.
 *
 * # Fields
 * - `enabled_for_account`: Whether discount is enabled for the account.
 * - `enabled_for_symbol`: Whether discount is enabled for the symbol.
 * - `discount_asset`: Asset used for discount (e.g., "BNB").
 * - `discount`: Discount rate applied when paying in discount asset.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DiscountInfo {
    pub enabled_for_account: bool,
    pub enabled_for_symbol: bool,
    pub discount_asset: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub discount: Decimal,
}
