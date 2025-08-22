use serde::{Deserialize, Serialize};

/**
 * Allocation types for order routing.
 *
 * # Variants
 * - `SOR`: Smart Order Routing allocation.
 * - `Unknown`: Any type not recognized. 
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum AllocationType {
    #[serde(rename = "SOR")]
    SOR,
    #[serde(other, skip_serializing)]
    Unknown,
}