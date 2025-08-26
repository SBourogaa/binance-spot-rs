use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/**
 * Server time response from Binance API.
 *
 * # Fields
 * - `server_time`: Current server timestamp as UTC DateTime.
 */
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerTime {
    #[serde(rename = "serverTime", with = "timestamp_millis")]
    pub server_time: DateTime<Utc>,
}

impl ServerTime {
    /**
     * Creates a ServerTime from milliseconds since Unix epoch.
     */
    pub fn from_millis(millis: u64) -> Self {
        let secs = (millis / 1000) as i64;
        let nanos = ((millis % 1000) * 1_000_000) as u32;
        let dt = Utc
            .timestamp_opt(secs, nanos)
            .single()
            .unwrap_or_else(Utc::now);

        Self { server_time: dt }
    }

    /**
     * Creates ServerTime from current system time.
     */
    pub fn now() -> Self {
        Self {
            server_time: Utc::now(),
        }
    }
}

mod timestamp_millis {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let millis = dt.timestamp_millis() as u64;
        serializer.serialize_u64(millis)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        let secs = (millis / 1000) as i64;
        let nanos = ((millis % 1000) * 1_000_000) as u32;
        Ok(Utc
            .timestamp_opt(secs, nanos)
            .single()
            .unwrap_or_else(Utc::now))
    }
}
