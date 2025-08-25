use serde::{Deserialize, Serialize};

/**
 * Symbol trading status.
 *
 * # Variants
 * - `Trading`: Symbol is currently available for trading.
 * - `EndOfDay`: Symbol is in end-of-day processing.
 * - `Halt`: Symbol trading is halted.
 * - `Break`: Symbol is in a scheduled trading break.
 * - `Unknown`: Any status not recognised.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(deny_unknown_fields)]
pub enum SymbolStatus {
    Trading,
    EndOfDay,
    Halt,
    Break,
    #[serde(other, skip_serializing)]
    Unknown,
}
