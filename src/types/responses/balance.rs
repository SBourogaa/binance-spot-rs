use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Asset balance information.
 *
 * This struct handles both REST API and WebSocket API formats:
 * - REST API uses full field names: "asset", "free", "locked"
 * - WebSocket API uses abbreviated names: "a", "f", "l"
 * 
 * # Fields
 * - `asset`: Asset symbol (e.g., "BTC", "USDT").
 * - `free`: Available balance for trading.
 * - `locked`: Balance locked in open orders.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    #[serde(alias = "a")]
    pub asset: String,
    #[serde(alias = "f")]
    #[serde(with = "rust_decimal::serde::str")]
    pub free: Decimal,
    #[serde(alias = "l")]
    #[serde(with = "rust_decimal::serde::str")]  
    pub locked: Decimal,
}