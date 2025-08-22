use serde::{Deserialize, Serialize};

/**
 * Working floor types for order execution.
 *
 * # Variants
 * - `Exchange`: Order executed on the exchange.
 * - `SOR`: Order executed via Smart Order Routing.
 * - `Unknown`: Any working floor not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum WorkingFloor {
    #[serde(rename = "EXCHANGE")]
    Exchange,
    #[serde(rename = "SOR")]
    SOR,
    #[serde(other, skip_serializing)]
    Unknown,
}