use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Aggregate trade from Binance API.
 *
 * Represents one or more individual trades that filled at the same time,
 * from the same taker order, with the same price aggregated together.
 *
 * # Fields
 * - `id`: Unique aggregate trade ID.
 * - `price`: Trade execution price.
 * - `quantity`: Total aggregated quantity.
 * - `first_trade_id`: ID of the first individual trade in this aggregate.
 * - `last_trade_id`: ID of the last individual trade in this aggregate.
 * - `timestamp`: Trade execution timestamp in milliseconds.
 * - `is_buyer_maker`: True if the buyer was the maker (passive) side.
 * - `is_best_match`: True if trade was at the best price available.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateTrade {
    #[serde(rename = "a")]
    pub id: u64,
    #[serde(rename = "p")]
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "q")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub timestamp: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M")]
    pub is_best_match: bool,
}
