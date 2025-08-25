use serde::{Deserialize, Serialize};

/**
 * This filter defines the valid range for the price based on the average of the previous trades.
 *
 * In order to pass the percent price, the following must be true for price:
 * - price <= weightedAveragePrice * multiplierUp
 * - price >= weightedAveragePrice * multiplierDown
 *
 * # Fields
 * - `multiplier_up`: Upper bound multiplier for weighted average price.
 * - `multiplier_down`: Lower bound multiplier for weighted average price.
 * - `avg_price_mins`: The number of minutes the average price is calculated over. 0 means the last price is used.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PercentPriceFilter {
    #[serde(rename = "multiplierUp")]
    pub multiplier_up: String,
    #[serde(rename = "multiplierDown")]
    pub multiplier_down: String,
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: u16,
}
