/**
 * Window size for Binance rolling window ticker streams
 *
 * Represents the supported rolling window durations for ticker statistics.
 * Used in rolling window ticker streams to specify the time period for calculations.
 */
#[derive(Debug)]
pub enum WindowSize {
    OneHour,
    FourHours,
    OneDay,
}

impl WindowSize {
    /**
     * Returns the string representation of the window size
     *
     * # Returns
     * - String slice representing the window size for stream names
     */
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneHour => "1h",
            Self::FourHours => "4h",
            Self::OneDay => "1d",
        }
    }
}

impl std::fmt::Display for WindowSize {
    /**
     * Formats the window size for display
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
