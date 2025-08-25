use serde::{Deserialize, Serialize};

use crate::enums::CancelReplaceStatus;
use crate::types::responses::Order;

/**
 * Response structure for cancel-replace order operations.
 *
 * Represents the outcome of an atomic cancel-replace operation.
 *
 * # Fields
 * - `cancel_status`: Result of the cancel operation (SUCCESS, FAILURE, NOT_ATTEMPTED).
 * - `new_order_status`: Result of the new order placement (SUCCESS, FAILURE, NOT_ATTEMPTED).
 * - `cancel_order`: Cancel operation response (order details on success, error on failure, null if not attempted).
 * - `new_order`: New order response (order details on success, error on failure, null if not attempted).
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CancelReplaceOrder {
    #[serde(rename = "cancelResult")]
    pub cancel_status: CancelReplaceStatus,
    #[serde(rename = "newOrderResult")]
    pub new_order_status: CancelReplaceStatus,
    #[serde(rename = "cancelResponse")]
    pub cancel_order: Option<Order>,
    #[serde(rename = "newOrderResponse")]
    pub new_order: Option<Order>,
}
