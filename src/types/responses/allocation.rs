use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::enums::AllocationType;

/**
 * Allocation information from SOR (Smart Order Routing) order placement.
 *
 * Contains details about allocations resulting from SOR order placement,
 * showing how orders were distributed across different venues/liquidity sources.
 *
 * # Fields
 * - `symbol`: Trading symbol for the allocation.
 * - `allocation_id`: Unique identifier for this allocation.
 * - `allocation_type`: Type of allocation (e.g., SOR for Smart Order Routing).
 * - `order_id`: Order ID that generated this allocation.
 * - `order_list_id`: Order list ID (-1 if not part of an order list).
 * - `price`: Price at which the allocation occurred.
 * - `quantity`: Quantity allocated.
 * - `quote_quantity`: Quote quantity for the allocation.
 * - `commission`: Commission charged for this allocation.
 * - `commission_asset`: Asset used to pay commission.
 * - `time`: Timestamp of the allocation in milliseconds.
 * - `is_buyer`: Whether this allocation was on the buy side.
 * - `is_maker`: Whether this allocation was as a maker.
 * - `is_allocator`: Whether this allocation was as an allocator.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Allocation {
    pub symbol: String,
    pub allocation_id: u64,
    pub allocation_type: AllocationType,
    pub order_id: u64,
    pub order_list_id: i64,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "qty")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,
    #[serde(rename = "quoteQty")]
    #[serde(with = "rust_decimal::serde::str")]
    pub quote_quantity: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: Decimal,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_allocator: bool,
}
