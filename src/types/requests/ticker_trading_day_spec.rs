use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Ticker trading day statistics query specification.
 *
 * This specification handles parameters for querying trading day ticker
 * statistics for one or more trading symbols.
 *
 * # Fields
 * - `symbol`: Optional single symbol to query.
 * - `symbols`: Optional array of symbols to query.
 * - `time_zone`: Optional timezone offset (default: "0" UTC).
 * - `ticker_type`: Optional ticker type ("FULL" or "MINI").
 */
#[derive(Debug, Clone, Serialize)]
pub struct TickerTradingDaySpec<S = Unvalidated> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub ticker_type: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl TickerTradingDaySpec<Unvalidated> {
    /**
     * Creates a new ticker trading day specification.
     *
     * # Returns
     * - `Self`: New ticker trading day specification.
     */
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            time_zone: None,
            ticker_type: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets a single symbol to query.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /**
     * Sets multiple symbols to query.
     *
     * # Arguments
     * - `symbols`: Array of trading symbols to query.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_symbols(mut self, symbols: Vec<&str>) -> Self {
        self.symbols = Some(serde_json::to_string(&symbols).unwrap());
        self
    }

    /**
     * Sets the timezone for trading day calculation.
     *
     * # Arguments
     * - `time_zone`: Timezone offset (default: "0" UTC).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /**
     * Sets the ticker type filter.
     *
     * # Arguments
     * - `ticker_type`: Ticker type ("FULL" or "MINI").
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_ticker_type(mut self, ticker_type: impl Into<String>) -> Self {
        self.ticker_type = Some(ticker_type.into());
        self
    }

    /**
     * Builds the ticker trading day specification.
     *
     * # Returns
     * - `TickerTradingDaySpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<TickerTradingDaySpec<Validated>> {
        self.validate()
            .context("Failed to validate TickerTradingDaySpecification")?;

        Ok(TickerTradingDaySpec {
            symbol: self.symbol,
            symbols: self.symbols,
            time_zone: self.time_zone,
            ticker_type: self.ticker_type,
            _state: PhantomData,
        })
    }

    /**
     * Validates the ticker trading day specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.is_none() && self.symbols.is_none() {
            return Err(InvalidParameter::new("symbol or symbols", "must be specified").into());
        }

        if let Some(ref symbol) = self.symbol {
            if symbol.trim().is_empty() {
                return Err(InvalidParameter::empty("symbol").into());
            }
        }

        if let Some(ref symbols_json) = self.symbols {
            if symbols_json.trim().is_empty() {
                return Err(InvalidParameter::empty("symbols").into());
            }
            match serde_json::from_str::<Vec<String>>(symbols_json) {
                Ok(symbols_array) => {
                    if symbols_array.is_empty() {
                        return Err(InvalidParameter::empty("symbols").into());
                    }
                    if symbols_array.len() > 100 {
                        return Err(InvalidParameter::new(
                            "symbols",
                            "maximum 100 symbols allowed",
                        )
                        .into());
                    }
                    for symbol in symbols_array {
                        if symbol.trim().is_empty() {
                            return Err(InvalidParameter::empty("symbols element").into());
                        }
                    }
                }
                Err(_) => {
                    return Err(InvalidParameter::new("symbols", "must be valid JSON array").into());
                }
            }
        }

        if let Some(ref tz) = self.time_zone {
            self.validate_timezone(tz)?;
        }

        if let Some(ref ticker_type) = self.ticker_type {
            match ticker_type.as_str() {
                "FULL" | "MINI" => {}
                _ => {
                    return Err(InvalidParameter::new(
                        "ticker_type",
                        "must be either FULL or MINI",
                    )
                    .into());
                }
            }
        }

        if self.symbol.is_some() && self.symbols.is_some() {
            return Err(InvalidParameter::mutually_exclusive("symbol", "symbols").into());
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
            if hours < -12 || hours > 14 {
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

            if hours < -12 || hours > 14 {
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
