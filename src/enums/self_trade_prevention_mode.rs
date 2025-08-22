use serde::{Deserialize, Serialize};

/**
 * Self Trade Prevention (STP) modes to prevent self-trading.
 *
 * A self-trade can occur in either scenario:
 * - The order traded against the same account.
 * - The order traded against an account with the same tradeGroupId.
 *
 * # Variants
 * - `NoProtection`: This mode exempts the order from self-trade prevention (sent/recv as `"NONE"`).
 * - `ExpireMaker`: This mode prevents a trade by immediately expiring the potential maker order's remaining quantity.
 * - `ExpireTaker`: This mode prevents a trade by immediately expiring the taker order's remaining quantity.
 * - `ExpireBoth`: This mode prevents a trade by immediately expiring both the taker and the potential maker orders' remaining quantities.
 * - `Decrement`: This mode increases the prevented quantity of both orders by the amount of the prevented match.
 * - `Unknown`: Any mode not recognized.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum SelfTradePreventionMode {
    #[serde(rename = "NONE")]
    NoProtection,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
    Decrement,
    #[serde(other, skip_serializing)]
    Unknown,
}