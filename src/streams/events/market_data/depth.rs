use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Depth Level
 * 
 * Represents a single price level in the order book with price and quantity.
 * Serialized as a two-element array: [price, quantity]
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthLevel(
    #[serde(with = "rust_decimal::serde::str")]
    pub Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub Decimal,
);

#[allow(dead_code)]
impl DepthLevel {
    /**
     * Gets the price level
     * 
     * # Returns
     * - Price as Decimal
     */
    pub fn price(&self) -> Decimal {
        self.0
    }
    
    /**
     * Gets the quantity at this price level
     * 
     * # Returns
     * - Quantity as Decimal
     */
    pub fn quantity(&self) -> Decimal {
        self.1
    }
}

/**
 * Partial Book Depth Stream Event
 * 
 * Top bids and asks snapshot, pushed at specified intervals.
 * Used for partial book depth streams with specific level counts.
 * 
 * # Fields:
 * - `symbol`: Trading pair symbol (for internal tracking, not serialized)
 * - `levels`: Number of levels (for internal tracking, not serialized)
 * - `update_speed`: Update speed setting (for internal tracking, not serialized)
 * - `last_update_id`: Last update ID from the order book
 * - `bids`: Array of bid price levels to be updated
 * - `asks`: Array of ask price levels to be updated
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartialBookDepthStreamEvent {
    #[serde(skip)]
    pub symbol: String,
    #[serde(skip)]
    pub levels: u8,
    #[serde(skip)]
    pub update_speed: String,
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<DepthLevel>,
    pub asks: Vec<DepthLevel>,
}

/**
 * Diff Depth Stream Event
 * 
 * Order book price and quantity depth updates used to locally manage an order book.
 * Contains incremental updates to apply to a local order book.
 * 
 * # Fields:
 * - `event_type`: Event type identifier (always "depthUpdate")
 * - `event_time`: Event timestamp in milliseconds
 * - `symbol`: Trading pair symbol
 * - `first_update_id`: First update ID in this event
 * - `final_update_id`: Final update ID in this event
 * - `bids`: Array of bid price levels to be updated
 * - `asks`: Array of ask price levels to be updated
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiffDepthStreamEvent {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<DepthLevel>,
    #[serde(rename = "a")]
    pub asks: Vec<DepthLevel>,
}