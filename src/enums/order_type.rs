use serde::{Deserialize, Serialize};

/**
 * Order types available for trading.
 *
 * # Variants
 * - `Limit`: Limit order type.
 * - `Market`: Market order type.
 * - `StopLoss`: Stop loss order type.
 * - `StopLossLimit`: Stop loss limit order type.
 * - `TakeProfit`: Take profit order type.
 * - `TakeProfitLimit`: Take profit limit order type.
 * - `LimitMaker`: Limit maker order type.
 * - `Unknown`: Any type not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
    #[serde(other, skip_serializing)]
    Unknown,
}