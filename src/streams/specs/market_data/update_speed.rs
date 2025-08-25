/**
 * Update speed for Binance WebSocket streams
 *
 * Represents the available update frequencies for streams that support multiple speeds.
 */
#[derive(Debug)]
pub enum UpdateSpeed {
    Standard,
    Fast100ms,
}

impl UpdateSpeed {
    /**
     * Returns the string representation of the update speed
     *
     * # Returns
     * - String slice representing the update speed duration
     */
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "1000ms",
            Self::Fast100ms => "100ms",
        }
    }
}
