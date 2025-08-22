use serde::{Deserialize, Serialize};
use crate::types::responses::{Order, OrderList};

/**
 * Response structure for order amendment operations.
 *
 * Represents the outcome of an order amendment that reduces the quantity of an existing
 * open order while maintaining its priority in the order book.
 *
 * # Fields
 * - `transaction_time`: Timestamp when the amendment was processed.
 * - `execution_id`: Unique execution identifier for the amendment.
 * - `amended_order`: Complete order information after amendment.
 * - `list_status`: Order list status (present if order is part of an order list).
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct AmendedOrder {
    #[serde(rename = "transactTime")]
    pub transaction_time: u64,
    pub execution_id: u64,
    pub amended_order: Order,
    pub list_status: Option<OrderList>,
}