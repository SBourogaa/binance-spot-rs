use serde::{Deserialize, Serialize};

/**
 * This filter defines the maximum parts an iceberg order can have.
 *
 * The number of ICEBERG_PARTS is defined as CEIL(qty / icebergQty).
 *
 * # Fields
 * - `limit`: The maximum number of iceberg parts allowed.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IcebergPartsFilter {
    pub limit: u16,
}
