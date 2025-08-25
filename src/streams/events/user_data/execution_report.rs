use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::enums::{OrderSide, OrderStatus, OrderType, SelfTradePreventionMode, TimeInForce};

/**
 * Execution Report Event
 *
 * Sent whenever there's an update on an order (placement, cancellation, fill, etc.)
 * This is the primary event for tracking order lifecycle.
 *
 * Based on Binance WebSocket API documentation, includes all fields that may appear
 * in executionReport events from the WebSocket stream.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReportEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,
    #[serde(rename = "p")]
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "P")]
    #[serde(with = "rust_decimal::serde::str")]
    pub stop_price: Decimal,
    #[serde(rename = "F")]
    #[serde(with = "rust_decimal::serde::str")]
    pub iceberg_quantity: Decimal,
    #[serde(rename = "g")]
    pub order_list_id: i64,
    #[serde(rename = "C")]
    pub original_client_order_id: String,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "r")]
    pub order_reject_reason: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l")]
    #[serde(with = "rust_decimal::serde::str")]
    pub last_executed_quantity: Decimal,
    #[serde(rename = "z")]
    #[serde(with = "rust_decimal::serde::str")]
    pub cumulative_filled_quantity: Decimal,
    #[serde(rename = "L")]
    #[serde(with = "rust_decimal::serde::str")]
    pub last_executed_price: Decimal,
    #[serde(rename = "n")]
    #[serde(with = "rust_decimal::serde::str")]
    pub commission_amount: Decimal,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "t")]
    pub trade_id: Option<i64>,
    #[serde(rename = "I")]
    pub execution_id: u64,
    #[serde(rename = "w")]
    pub is_on_book: bool,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "M")]
    pub ignore_field: bool,
    #[serde(rename = "O")]
    pub order_creation_time: u64,
    #[serde(rename = "Z")]
    #[serde(with = "rust_decimal::serde::str")]
    pub cumulative_quote_quantity: Decimal,
    #[serde(rename = "Y")]
    #[serde(with = "rust_decimal::serde::str")]
    pub last_quote_quantity: Decimal,
    #[serde(rename = "Q")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quote_order_quantity: Decimal,
    #[serde(rename = "W")]
    pub working_time: u64,
    #[serde(rename = "V")]
    pub self_trade_prevention_mode: SelfTradePreventionMode,
}
