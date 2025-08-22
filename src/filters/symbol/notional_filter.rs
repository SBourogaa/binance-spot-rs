use serde::{Deserialize, Serialize};

/**
 * The NOTIONAL filter defines the acceptable notional range allowed for an order on a symbol.
 *
 * In order to pass this filter, the notional (price * quantity) has to pass the following conditions:
 * - price * quantity <= maxNotional
 * - price * quantity >= minNotional
 *
 * For MARKET orders, the average price used over the last avgPriceMins minutes will be used for calculation.
 * If the avgPriceMins is 0, then the last price will be used.
 *
 * # Fields
 * - `min_notional`: The minimum notional value allowed for an order.
 * - `apply_min_to_market`: Determines whether the minNotional will be applied to MARKET orders.
 * - `max_notional`: The maximum notional value allowed for an order.
 * - `apply_max_to_market`: Determines whether the maxNotional will be applied to MARKET orders.
 * - `avg_price_mins`: The number of minutes the average price is calculated over. 0 means the last price is used.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotionalFilter {
    #[serde(rename = "minNotional")]
    pub min_notional: String,
    #[serde(rename = "applyMinToMarket")]
    pub apply_min_to_market: bool,
    #[serde(rename = "maxNotional")]
    pub max_notional: String,
    #[serde(rename = "applyMaxToMarket")]
    pub apply_max_to_market: bool,
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: u16,
}