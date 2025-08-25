use crate::types::responses::Trade;
use serde::{Deserialize, Serialize};

/**
 * Trade Stream Event
 *
 * Raw trade information with unique buyer and seller for each trade.
 *
 * # Fields:
 * - `event_type`: Event type identifier, always "trade"
 * - `event_time`: Event timestamp in milliseconds since Unix epoch
 * - `symbol`: Trading pair symbol
 * - `trade`: Flattened trade data containing trade ID, price, quantity, timestamps, and market maker flag
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TradeStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(flatten)]
    pub trade: Trade,
}
