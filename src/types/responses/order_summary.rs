use serde::{Deserialize, Serialize};

/**
 * Order summary information for order lists.
 *
 * This structure represents the minimal order information included in order list responses.
 * Contains only the essential identifiers for orders within a list.
 *
 * # Fields
 * - `symbol`: Trading symbol for the order.
 * - `order_id`: Unique order identifier assigned by the exchange.
 * - `client_order_id`: Client-specified order identifier.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OrderSummary {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
}
