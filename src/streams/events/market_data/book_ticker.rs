use crate::types::responses::TickerBook;
use serde::{Deserialize, Serialize};

/**
 * Book Ticker Stream Event
 *
 * Real-time updates to the best bid or ask's price or quantity for a specified symbol.
 *
 * # Fields:
 * - `update_id`: Order book update ID
 * - `ticker`: Flattened ticker book data containing symbol, best bid price, best bid quantity,
 *    best ask price, and best ask quantity
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BookTickerStreamEvent {
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(flatten)]
    pub ticker: TickerBook,
}
