use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Order fill information for executed trades.
 * 
 * # Fields
 * - `price`: Price at which the trade was executed.
 * - `quantity`: Quantity filled in this trade.
 * - `commission`: Commission charged for this trade.
 * - `commission_asset`: Asset in which commission was charged.
 * - `trade_id`: Unique trade identifier.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Fill {
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "qty")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: Decimal,
    pub commission_asset: String,
    pub trade_id: i64,
}