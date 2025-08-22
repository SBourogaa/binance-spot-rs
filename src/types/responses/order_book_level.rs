use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Order book level representing a single bid or ask.
 * 
 * # Fields
 * - `price`: Price level as a precise decimal.
 * - `quantity`: Quantity available at this price level.
 */
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderBookLevel {
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: Decimal,
}