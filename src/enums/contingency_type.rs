use serde::{Deserialize, Serialize};

/**
 * Contingency types for order relationships.
 *
 * # Variants
 * - `OCO`: One-Cancels-Other order type.
 * - `OTO`: One-Triggers-Other order type.
 * - `Unknown`: Any type not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ContingencyType {
    #[serde(rename = "OCO")]
    OCO,
    #[serde(rename = "OTO")]
    OTO,
    #[serde(other, skip_serializing)]
    Unknown,
}
