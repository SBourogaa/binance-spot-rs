use serde::{Deserialize, Serialize};

use crate::types::responses::{Order, OrderList};

/**
 * Union type for cancel all orders response.
 *
 * The cancel all orders endpoint returns a heterogeneous array containing both
 * individual order cancellations and order list cancellations.
 *
 * # Variants
 * - `Individual`: Cancelled individual order information.
 * - `OrderList`: Cancelled order list (OCO, OTO, etc.) information.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum CancelledOrder {
    Individual(Order),
    OrderList(OrderList),
}
