use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Account trade information for myTrades endpoint.
 *
 * Contains trade execution details with order and commission information
 * specific to account trading history queries.
 *
 * # Fields
 * - `symbol`: Trading symbol for this trade.
 * - `id`: Unique trade identifier.
 * - `order_id`: Order ID that this trade belongs to.
 * - `order_list_id`: Order list ID (-1 for regular orders).
 * - `price`: Price at which the trade was executed.
 * - `quantity`: Quantity traded.
 * - `quote_quantity`: Quote asset quantity (price * quantity).
 * - `commission`: Commission charged for this trade.
 * - `commission_asset`: Asset in which commission was charged.
 * - `time`: Trade execution timestamp in milliseconds.
 * - `is_buyer`: Whether the account was the buyer in this trade.
 * - `is_maker`: Whether the account was the maker in this trade.
 * - `is_best_match`: Whether this trade was the best price match.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccountTrade {
    pub symbol: String,
    pub id: u64,
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "qty")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,
    #[serde(rename = "quoteQty")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quote_quantity: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: Decimal,
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,
    pub time: u64,
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
    #[serde(rename = "isBestMatch")]
    pub is_best_match: bool,
}
