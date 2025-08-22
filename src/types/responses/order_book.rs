use serde::{Deserialize, Serialize};

use crate::types::responses::OrderBookLevel;

/**
 * Order book response from Binance API.
 * 
 * Contains current bid and ask levels for a trading symbol.
 * 
 * # Fields
 * - `last_update_id`: Order book update ID for synchronization.
 * - `bids`: Bid levels sorted from highest to lowest price.
 * - `asks`: Ask levels sorted from lowest to highest price.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
}

impl OrderBook {
    /**
     * Gets the best bid (highest bid price).
     * 
     * # Returns
     * - `Option<&OrderBookLevel>`: Best bid level, if any.
     */
    pub fn best_bid(&self) -> Option<&OrderBookLevel> {
        self.bids.first()
    }

    /**
     * Gets the best ask (lowest ask price).
     * 
     * # Returns
     * - `Option<&OrderBookLevel>`: Best ask level, if any.
     */
    pub fn best_ask(&self) -> Option<&OrderBookLevel> {
        self.asks.first()
    }

    /**
     * Checks if the order book is valid (bids and asks properly sorted).
     * 
     * # Returns
     * - `bool`: True if order book is properly sorted.
     */
    pub fn is_valid(&self) -> bool {
        // Check bids are sorted highest to lowest
        let bids_sorted: bool = self.bids.windows(2).all(|pair| pair[0].price >= pair[1].price);
        
        // Check asks are sorted lowest to highest  
        let asks_sorted: bool = self.asks.windows(2).all(|pair| pair[0].price <= pair[1].price);
        
        bids_sorted && asks_sorted
    }
}