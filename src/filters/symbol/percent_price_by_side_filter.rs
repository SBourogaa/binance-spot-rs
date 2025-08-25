use serde::{Deserialize, Serialize};

/**
 * This filter defines the valid range for the price based on the average of the previous trades.
 * There is a different range depending on whether the order is placed on the BUY side or the SELL side.
 *
 * Buy orders will succeed on this filter if:
 * - Order price <= weightedAveragePrice * bidMultiplierUp
 * - Order price >= weightedAveragePrice * bidMultiplierDown
 *
 * Sell orders will succeed on this filter if:
 * - Order Price <= weightedAveragePrice * askMultiplierUp
 * - Order Price >= weightedAveragePrice * askMultiplierDown
 *
 * # Fields
 * - `bid_multiplier_up`: Upper bound multiplier for BUY side orders.
 * - `bid_multiplier_down`: Lower bound multiplier for BUY side orders.
 * - `ask_multiplier_up`: Upper bound multiplier for SELL side orders.
 * - `ask_multiplier_down`: Lower bound multiplier for SELL side orders.
 * - `avg_price_mins`: The number of minutes the average price is calculated over. 0 means the last price is used.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PercentPriceBySideFilter {
    #[serde(rename = "bidMultiplierUp")]
    pub bid_multiplier_up: String,
    #[serde(rename = "bidMultiplierDown")]
    pub bid_multiplier_down: String,
    #[serde(rename = "askMultiplierUp")]
    pub ask_multiplier_up: String,
    #[serde(rename = "askMultiplierDown")]
    pub ask_multiplier_down: String,
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: u16,
}
