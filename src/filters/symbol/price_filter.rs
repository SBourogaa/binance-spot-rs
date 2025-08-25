use serde::{Deserialize, Serialize};

/**
 * This fileter defines the price rules for a symbol.
 *
 * Any of the variables can be set to 0, which disables that rule in the price filter.
 * In order to pass the price filter, the following must be true for price/stopPrice of the enabled rules:
 * - price >= minPrice
 * - price <= maxPrice  
 * - price % tickSize == 0
 *
 * # Fields
 * - `min_price`: Defines the minimum price/stopPrice allowed; disabled on minPrice == 0.
 * - `max_price`: Defines the maximum price/stopPrice allowed; disabled on maxPrice == 0.
 * - `tick_size`: Defines the intervals that a price/stopPrice can be increased/decreased by; disabled on tickSize == 0.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PriceFilter {
    #[serde(rename = "minPrice")]
    pub min_price: String,
    #[serde(rename = "maxPrice")]
    pub max_price: String,
    #[serde(rename = "tickSize")]
    pub tick_size: String,
}
