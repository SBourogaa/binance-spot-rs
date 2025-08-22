use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::enums::SelfTradePreventionMode;

/**
 * Prevented match information for orders expired due to STP.
 * 
 * Contains details about orders that were prevented from matching
 * due to self-trade prevention (STP) rules.
 * 
 * # Fields
 * - `symbol`: Trading symbol for the prevented match.
 * - `prevented_match_id`: Unique identifier for this prevented match.
 * - `taker_order_id`: Order ID of the taker order that was prevented.
 * - `maker_symbol`: Symbol of the maker order (same as symbol).
 * - `maker_order_id`: Order ID of the maker order that was prevented.
 * - `trade_group_id`: Trade group identifier.
 * - `self_trade_prevention_mode`: STP mode that caused the prevention.
 * - `price`: Price at which the match would have occurred.
 * - `maker_prevented_quantity`: Quantity that was prevented on maker side.
 * - `transaction_time`: Timestamp when the prevention occurred in milliseconds.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PreventedMatch {
    pub symbol: String,
    pub prevented_match_id: u64,
    pub taker_order_id: u64,
    pub maker_symbol: String,
    pub maker_order_id: u64,
    pub trade_group_id: u64,
    pub self_trade_prevention_mode: SelfTradePreventionMode,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub maker_prevented_quantity: Decimal,
    #[serde(rename = "transactTime")]
    pub transaction_time: u64,
}