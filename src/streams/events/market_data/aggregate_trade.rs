use crate::types::responses::AggregateTrade;
use serde::{Deserialize, Serialize};

/**
 * Aggregate Trade Stream Event
 *
 * Real-time event containing aggregated trade information for a single taker order.
 *
 * # Fields:
 * - `event_type`: Event type identifier (always "aggTrade")
 * - `event_time`: Event timestamp in milliseconds
 * - `symbol`: Trading pair symbol
 * - `aggregate_trade`: Flattened aggregate trade data containing trade ID, price, quantity,
 *   first/last trade IDs, trade time, and market maker information
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AggregateTradeStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(flatten)]
    pub aggregate_trade: AggregateTrade,
}
