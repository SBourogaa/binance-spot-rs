use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::enums::{
    OrderStatus,
    TimeInForce,
    OrderType,
    OrderSide,
    SelfTradePreventionMode,
    WorkingFloor
};
use crate::types::responses::Fill;

/**
 * Complete order information returned by order placement and status queries.
 *
 * Contains all possible fields that may be returned depending on the order type
 * and response type requested (ACK/RESULT/FULL).
 *
 * # Fields
 * - `symbol`: Trading symbol.
 * - `order_id`: Unique order identifier.
 * - `order_list_id`: Order list ID (-1 if not part of an order list).
 * - `client_order_id`: Client-specified order identifier.
 * - `original_client_order_id`: Original client order ID (for amendments/cancellations).
 * - `transaction_time`: Order transaction timestamp (placement/update).
 * - `time`: Order creation timestamp.
 * - `update_time`: Last order update timestamp.
 * - `is_working`: Whether order is actively working in the order book.
 * - `price`: Order price (for limit orders).
 * - `original_quantity`: Original order quantity.
 * - `executed_quantity`: Quantity that has been executed.
 * - `original_quote_order_quantity`: Original quote order quantity.
 * - `cumulative_quote_quantity`: Total quote quantity executed.
 * - `status`: Current order status.
 * - `time_in_force`: Time in force policy.
 * - `order_type`: Type of order (LIMIT, MARKET, etc.).
 * - `side`: Order side (BUY/SELL).
 * - `working_time`: Timestamp when order started working.
 * - `self_trade_prevention_mode`: STP mode for this order.
 * - `fills`: List of trade fills (FULL response only).
 * - `stop_price`: Stop price (for stop orders).
 * - `iceberg_quantity`: Iceberg order quantity.
 * - `strategy_id`: Strategy identifier.
 * - `strategy_type`: Strategy type identifier.
 * - `trailing_delta`: Trailing stop delta.
 * - `trailing_time`: Trailing order activation time.
 * - `working_floor`: Execution venue (SOR vs order book).
 * - `used_sor`: Whether Smart Order Routing was used.
 * - `prevented_match_id`: ID for prevented matches (STP).
 * - `prevented_quantity`: Quantity prevented from matching (STP).
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    #[serde(rename = "origClientOrderId")]
    pub original_client_order_id: Option<String>,
    
    // Timestamp fields - different between placement and status responses
    #[serde(rename = "transactTime")]
    pub transaction_time: Option<u64>,
    pub time: Option<u64>,
    pub update_time: Option<u64>,
    pub is_working: Option<bool>,
    
    // Core order fields
    #[serde(with = "rust_decimal::serde::str_option")]
    pub price: Option<Decimal>,
    #[serde(rename = "origQty", alias = "qty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub original_quantity: Option<Decimal>,
    #[serde(rename = "executedQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub executed_quantity: Option<Decimal>,
    #[serde(rename = "origQuoteOrderQty", alias = "quoteOrderQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub original_quote_order_quantity: Option<Decimal>,
    #[serde(rename = "cummulativeQuoteQty", alias = "cumulativeQuoteQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub cumulative_quote_quantity: Option<Decimal>,
    
    pub status: Option<OrderStatus>,
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "type")]
    pub order_type: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub working_time: Option<i64>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    
    // Full response only - fills information
    pub fills: Option<Vec<Fill>>,
    
    // Conditional fields based on order type and parameters
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(default)]
    pub stop_price: Option<Decimal>,
    #[serde(rename = "icebergQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(default)]
    pub iceberg_quantity: Option<Decimal>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u32>,
    pub trailing_delta: Option<u32>,
    pub trailing_time: Option<i64>,
    pub working_floor: Option<WorkingFloor>,
    pub used_sor: Option<bool>,
    pub prevented_match_id: Option<u64>,
    #[serde(rename = "preventedQuantity", alias = "preventedQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(default)]
    pub prevented_quantity: Option<Decimal>,
}