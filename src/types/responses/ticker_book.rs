use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Ticker book response with best bid/ask prices and quantities.
 *
 * Compatible with:
 * - REST API /api/v3/ticker/bookTicker
 * - WebSocket Stream <symbol>@bookTicker events
 *
 * # Fields
 * - `symbol`: Trading symbol.
 * - `bid_price`: Best bid price.
 * - `bid_quantity`: Quantity at best bid.
 * - `ask_price`: Best ask price.  
 * - `ask_quantity`: Quantity at best ask.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TickerBook {
    #[serde(alias = "s")] // WebSocket Stream alias
    pub symbol: String,
    #[serde(rename = "bidPrice", with = "rust_decimal::serde::str")]
    #[serde(alias = "b")] // WebSocket Stream alias
    pub bid_price: Decimal,
    #[serde(rename = "bidQty", with = "rust_decimal::serde::str")]
    #[serde(alias = "B")] // WebSocket Stream alias
    pub bid_quantity: Decimal,
    #[serde(rename = "askPrice", with = "rust_decimal::serde::str")]
    #[serde(alias = "a")] // WebSocket Stream alias
    pub ask_price: Decimal,
    #[serde(rename = "askQty", with = "rust_decimal::serde::str")]
    #[serde(alias = "A")] // WebSocket Stream alias
    pub ask_quantity: Decimal,
}
