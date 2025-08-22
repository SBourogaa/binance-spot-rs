use serde::{Deserialize, Serialize};
use crate::enums::{ContingencyType, OrderListStatus, OrderListOrderStatus};
use crate::types::responses::{OrderSummary, Order};

/**
 * Order list information for OCO and other multi-order operations.
 *
 * Contains both summary information and detailed order reports for order lists.
 *
 * # Fields
 * - `order_list_id`: Unique identifier for the order list.
 * - `contingency_type`: Type of contingency (OCO, OTO, OTOCO).
 * - `list_status_type`: Current status type of the order list.
 * - `list_order_status`: Current order status of the list.
 * - `list_client_order_id`: Client-specified order list identifier.
 * - `transaction_time`: Timestamp when the order list was processed.
 * - `symbol`: Trading symbol for all orders in the list.
 * - `orders`: Summary information for each order in the list.
 * - `order_reports`: Detailed order information for each order in the list.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OrderList {
    pub order_list_id: u64,
    pub contingency_type: ContingencyType,
    pub list_status_type: OrderListStatus,
    pub list_order_status: OrderListOrderStatus,
    pub list_client_order_id: String,
    #[serde(rename = "transactionTime")]
    pub transaction_time: u64,
    pub symbol: String,
    pub orders: Vec<OrderSummary>,
    pub order_reports: Vec<Order>,
}