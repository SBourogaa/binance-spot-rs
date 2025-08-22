/**
 * Kline/Candlestick Chart Interval
 * 
 * Represents the time intervals supported for kline/candlestick streams.
 * Intervals: s=seconds, m=minutes, h=hours, d=days, w=weeks, M=months
 * 
 * # Variants:
 * - `OneSecond`: 1 second interval ("1s")
 * - `OneMinute`: 1 minute interval ("1m")
 * - `ThreeMinutes`: 3 minutes interval ("3m")
 * - `FiveMinutes`: 5 minutes interval ("5m")
 * - `FifteenMinutes`: 15 minutes interval ("15m")
 * - `ThirtyMinutes`: 30 minutes interval ("30m")
 * - `OneHour`: 1 hour interval ("1h")
 * - `TwoHours`: 2 hours interval ("2h")
 * - `FourHours`: 4 hours interval ("4h")
 * - `SixHours`: 6 hours interval ("6h")
 * - `EightHours`: 8 hours interval ("8h")
 * - `TwelveHours`: 12 hours interval ("12h")
 * - `OneDay`: 1 day interval ("1d")
 * - `ThreeDays`: 3 days interval ("3d")
 * - `OneWeek`: 1 week interval ("1w")
 * - `OneMonth`: 1 month interval ("1M")
 */
#[derive(Debug)]
#[allow(dead_code)]
pub enum Interval {
    OneSecond,
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
}

impl Interval {
    /**
     * Gets the string representation of the interval
     * 
     * # Returns
     * - String representation used in stream names and API calls
     */
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneSecond => "1s",
            Self::OneMinute => "1m",
            Self::ThreeMinutes => "3m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::ThirtyMinutes => "30m",
            Self::OneHour => "1h",
            Self::TwoHours => "2h",
            Self::FourHours => "4h",
            Self::SixHours => "6h",
            Self::EightHours => "8h",
            Self::TwelveHours => "12h",
            Self::OneDay => "1d",
            Self::ThreeDays => "3d",
            Self::OneWeek => "1w",
            Self::OneMonth => "1M",
        }
    }
}

impl std::fmt::Display for Interval {
    /**
     * Formats the interval for display
     * 
     * # Arguments
     * - `f` - Formatter
     * 
     * # Returns
     * - Formatted result using the string representation
     */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}