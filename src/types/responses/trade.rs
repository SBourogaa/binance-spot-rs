use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Individual trade from Binance API and WebSocket Streams.
 *
 * Compatible with:
 * - REST API /api/v3/trades
 * - WebSocket API trade queries  
 * - WebSocket Stream <symbol>@trade events
 *
 * # Fields
 * - `id`: Unique trade ID.
 * - `price`: Trade execution price.
 * - `quantity`: Base asset quantity traded.
 * - `quote_quantity`: Quote asset quantity traded (not present in streams).
 * - `time`: Trade execution timestamp in milliseconds.
 * - `is_buyer_maker`: True if the buyer was the maker (passive) side.
 * - `is_best_match`: True if trade was at the best price available.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    #[serde(alias = "t")] // WebSocket Stream alias
    pub id: u64,
    #[serde(with = "rust_decimal::serde::str")]
    #[serde(alias = "p")] // WebSocket Stream alias
    pub price: Decimal,
    #[serde(rename = "qty")]
    #[serde(with = "rust_decimal::serde::str")]
    #[serde(alias = "q")] // WebSocket Stream alias
    pub quantity: Decimal,
    #[serde(rename = "quoteQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_quantity: Option<Decimal>,
    #[serde(alias = "T")] // WebSocket Stream alias
    pub time: u64,
    #[serde(alias = "m")] // WebSocket Stream alias
    pub is_buyer_maker: bool,
    #[serde(alias = "M")] // WebSocket Stream alias
    pub is_best_match: bool,
}
