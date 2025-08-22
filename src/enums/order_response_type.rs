use serde::{Deserialize, Serialize};

/**
 * Order response types for different levels of response detail.
 *
 * # Variants
 * - `ACK`: Acknowledgment response only.
 * - `Result`: Result response with basic order info.
 * - `Full`: Full response with complete order details.
 * - `Unknown`: Any type not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum OrderResponseType {
    #[serde(rename = "ACK")]
    ACK,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "FULL")]
    Full,
    #[serde(other, skip_serializing)]
    Unknown,
}