/**
 * Timezone offset for Binance kline streams
 *
 * Represents supported timezone offsets for kline stream boundaries.
 * Currently only UTC+8 is supported by Binance.
 */
#[derive(Debug, Clone)]
pub enum TimezoneOffset {
    UtcPlus8,
}

impl TimezoneOffset {
    /**
     * Returns the string representation of the timezone offset
     *
     * # Returns
     * - String slice representing the timezone offset for stream names
     */
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UtcPlus8 => "+08:00",
        }
    }
}

impl std::fmt::Display for TimezoneOffset {
    /**
     * Formats the timezone offset for display
     *
     * # Arguments
     * - `f` - Formatter
     *
     * # Returns
     * - Result of formatting operation
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}