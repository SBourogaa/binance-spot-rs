use serde::{Deserialize, Serialize};

/**
 * Time in force options for order duration.
 *
 * # Variants
 * - `GTC`: Good Til Canceled - An order will be on the book unless the order is canceled.
 * - `IOC`: Immediate Or Cancel - An order will try to fill as much as it can before expiring.
 * - `FOK`: Fill or Kill - An order will expire if the full order cannot be filled upon execution.
 * - `Unknown`: Any time in force not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    GTC,
    #[serde(rename = "IOC")]
    IOC,
    #[serde(rename = "FOK")]
    FOK,
    #[serde(other, skip_serializing)]
    Unknown,
}