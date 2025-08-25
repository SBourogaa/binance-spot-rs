use serde::{Deserialize, Serialize};

/**
 * This filter defines the allowed maximum position an account can have on the base asset of a symbol.
 *
 * An account's position defined as the sum of the account's:
 * - free balance of the base asset
 * - locked balance of the base asset
 * - sum of the qty of all open BUY orders
 *
 * BUY orders will be rejected if the account's position is greater than the maximum position allowed.
 * If an order's quantity can cause the position to overflow, this will also fail the MAX_POSITION filter.
 *
 * # Fields
 * - `max_position`: Maximum base-asset position allowed.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxPositionFilter {
    #[serde(rename = "maxPosition")]
    pub max_position: String,
}
