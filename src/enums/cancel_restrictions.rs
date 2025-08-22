use serde::{Deserialize, Serialize};

/**
 * Cancel restrictions for order cancellation.
 * 
 * These restrictions control under what conditions an order can be canceled,
 * providing additional safety for order management operations.
 * 
 * # Variants
 * - `OnlyNew`: Cancel will succeed if the order status is NEW.
 * - `OnlyPartiallyFilled`: Cancel will succeed if order status is PARTIALLY_FILLED.
 * - `Unknown`: Any restrictions not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum CancelRestrictions {
    OnlyNew,
    OnlyPartiallyFilled,
    #[serde(other, skip_serializing)]
    Unknown,
}