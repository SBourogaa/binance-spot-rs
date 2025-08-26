use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Klines/candlestick data query specification.
 *
 * This specification handles parameters for querying kline data
 * with time range, interval, and timezone controls.
 *
 * # Fields
 * - `symbol`: Trading symbol to query klines for.
 * - `interval`: Kline interval (e.g., "1m", "5m", "1h", "1d").
 * - `start_time`: Optional start time in milliseconds.
 * - `end_time`: Optional end time in milliseconds.
 * - `time_zone`: Optional timezone offset (default: "0" UTC).
 * - `limit`: Optional number of klines to return (default: 500, max: 1000).
 */
#[derive(Debug, Clone, Serialize)]
pub struct KlinesSpec<S = Unvalidated> {
    pub symbol: String,
    // TODO: Create a KlineInterval enum.
    pub interval: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl KlinesSpec<Unvalidated> {
    /**
     * Creates a new klines specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     * - `interval`: Kline interval (e.g., "1m", "5m", "1h", "1d").
     *
     * # Returns
     * - `Self`: New klines specification.
     */
    pub fn new(symbol: impl Into<String>, interval: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            interval: interval.into(),
            start_time: None,
            end_time: None,
            time_zone: None,
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the start time for klines query.
     *
     * # Arguments
     * - `start_time`: Start time in milliseconds.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /**
     * Sets the end time for klines query.
     *
     * # Arguments
     * - `end_time`: End time in milliseconds.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /**
     * Sets the timezone for klines query.
     *
     * # Arguments
     * - `time_zone`: Timezone offset (e.g., "0", "-1:00", "05:45", range: [-12:00 to +14:00]).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /**
     * Sets the limit for number of klines to return.
     *
     * # Arguments
     * - `limit`: Number of klines to return (max: 1000).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the klines specification.
     *
     * # Returns
     * - `KlinesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<KlinesSpec<Validated>> {
        self.validate()
            .context("Failed to validate KlinesSpecification")?;

        Ok(KlinesSpec {
            symbol: self.symbol,
            interval: self.interval,
            start_time: self.start_time,
            end_time: self.end_time,
            time_zone: self.time_zone,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the klines specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.interval.trim().is_empty() {
            return Err(InvalidParameter::empty("interval").into());
        }

        match self.interval.as_str() {
            "1s" | "1m" | "3m" | "5m" | "15m" | "30m" | "1h" | "2h" | "4h" | "6h" | "8h"
            | "12h" | "1d" | "3d" | "1w" | "1M" => {}
            _ => return Err(InvalidParameter::new(
                "interval",
                "must be one of: 1s, 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d, 3d, 1w, 1M",
            )
            .into()),
        }

        if let Some(limit) = self.limit
            && limit > 1000
        {
            return Err(InvalidParameter::range("limit", 1, 1000).into());
        }

        if let (Some(start), Some(end)) = (self.start_time, self.end_time)
            && end <= start
        {
            return Err(InvalidParameter::new(
                "start_time/end_time",
                "end_time must be greater than start_time",
            )
            .into());
        }

        if let Some(ref tz) = self.time_zone {
            self.validate_timezone(tz)?;
        }

        Ok(())
    }

    /**
     * Validates timezone format and range according to API requirements.
     *
     * # Arguments
     * - `timezone`: Timezone string to validate.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid timezone.
     */
    fn validate_timezone(&self, timezone: &str) -> Result<()> {
        let tz = timezone.trim();

        if let Ok(hours) = tz.parse::<i32>() {
            if !(-12..=14).contains(&hours) {
                return Err(InvalidParameter::new(
                    "time_zone",
                    "timezone hours must be in range [-12 to +14]",
                )
                .into());
            }
            return Ok(());
        }

        if tz.contains(':') {
            let parts: Vec<&str> = tz.split(':').collect();
            if parts.len() != 2 {
                return Err(InvalidParameter::new(
                    "time_zone",
                    "timezone format must be like '0', '8', '-1:00', or '05:45'",
                )
                .into());
            }

            let hours = parts[0]
                .parse::<i32>()
                .map_err(|_| InvalidParameter::new("time_zone", "invalid hour format"))?;

            let minutes = parts[1]
                .parse::<u32>()
                .map_err(|_| InvalidParameter::new("time_zone", "invalid minute format"))?;

            if !(-12..=14).contains(&hours) {
                return Err(InvalidParameter::new(
                    "time_zone",
                    "timezone hours must be in range [-12 to +14]",
                )
                .into());
            }

            if minutes >= 60 {
                return Err(
                    InvalidParameter::new("time_zone", "timezone minutes must be 0-59").into(),
                );
            }

            return Ok(());
        }

        Err(InvalidParameter::new(
            "time_zone",
            "timezone format must be like '0', '8', '-1:00', or '05:45'",
        )
        .into())
    }
}
