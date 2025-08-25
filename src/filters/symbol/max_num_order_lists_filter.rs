use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum number of open order lists an account can have on a symbol.
 *
 * Note that OTOCOs count as one order list.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaxNumOrderListsFilter {
    #[serde(rename = "maxNumOrderLists")]
    pub max_num_order_lists: u32,
}
