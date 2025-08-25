use serde::{Deserialize, Serialize};

/**
 * This filter defines the minimum notional value allowed for an order on a symbol.
 * An order's notional value is the price * quantity.
 *
 * Since MARKET orders have no price, the average price is used over the last avgPriceMins minutes.
 *
 * # Fields
 * - `min_notional`: The minimum notional value allowed for an order.
 * - `apply_to_market`: Determines whether or not the MIN_NOTIONAL filter will also be applied to MARKET orders.
 * - `avg_price_mins`: The number of minutes the average price is calculated over. 0 means the last price is used.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinNotionalFilter {
    #[serde(rename = "minNotional")]
    pub min_notional: String,
    #[serde(rename = "applyToMarket")]
    pub apply_to_market: bool,
    #[serde(rename = "avgPriceMins")]
    pub avg_price_mins: u16,
}
