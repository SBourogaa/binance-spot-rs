use serde::{Deserialize, Serialize};

use crate::enums::{
    ContingencyType,
    OrderListStatus,
    OrderListOrderStatus,
};

/**
 * List Status Event
 * 
 * Sent when there's an update to an order list (OCO orders, etc.)
 * 
 * # Fields
 * - `event_type`: Always "listStatus".
 * - `event_time`: The time the event was generated, in milliseconds since epoch.
 * - `symbol`: The trading pair symbol (e.g., "BTCUSDT").
 * - `order_list_id`: The unique identifier for the order list.
 * - `contingency_type`: The type of contingency for the order list (e.g., OCO, OTO).
 * - `list_status_type`: The status type of the order list (e.g., EXEC_STARTED, EXECUTING, ALL_DONE).
 * - `list_order_status`: The status of the orders in the list (e.g., EXECUTING, ALL_DONE).
 * - `list_reject_reason`: The reason for rejection of the order list (if applicable).
 * - `list_client_order_id`: The client-specified order ID for the list.
 * - `transaction_time`: The time of the transaction in milliseconds since epoch.
 * - `orders`: A list of orders in the order list, each containing.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListStatusEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "g")]
    pub order_list_id: u64,
    #[serde(rename = "c")]
    pub contingency_type: ContingencyType,
    #[serde(rename = "l")]
    pub list_status_type: OrderListStatus,
    #[serde(rename = "L")]
    pub list_order_status: OrderListOrderStatus,
    #[serde(rename = "r")]
    pub list_reject_reason: Option<String>,
    #[serde(rename = "C")]
    pub list_client_order_id: String,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "O")]
    pub orders: Vec<ListOrder>,
}

/**
 * Order information within a list status event
 * 
 * # Fields
 * - `symbol`: The trading pair symbol.
 * - `order_id`: The unique identifier for the order.
 * - `client_order_id`: The client-specified order ID.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListOrder {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "c")]
    pub client_order_id: String,
}