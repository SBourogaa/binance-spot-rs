use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Aggregate trades query specification.
 *
 * This specification handles parameters for querying compressed/aggregate trades
 * with time range, ID range, and limit controls.
 *
 * # Fields
 * - `symbol`: Trading symbol to query aggregate trades for.
 * - `from_id`: Optional aggregate trade ID to fetch from (returns trades with ID >= from_id).
 * - `start_time`: Optional start time in milliseconds (inclusive).
 * - `end_time`: Optional end time in milliseconds (inclusive).
 * - `limit`: Optional number of trades to return (default: 500, max: 1000).
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTradesSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromId")]
    pub from_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl AggregateTradesSpec<Unvalidated> {
    /**
     * Creates a new aggregate trades specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: New aggregate trades specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the starting aggregate trade ID.
     *
     * # Arguments
     * - `from_id`: Aggregate trade ID to fetch from (inclusive).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    /**
     * Sets the start time for trades query.
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
     * Sets the end time for trades query.
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
     * Sets the limit for number of trades to return.
     *
     * # Arguments
     * - `limit`: Number of trades to return (max: 1000).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the aggregate trades specification.
     *
     * # Returns
     * - `AggregateTradesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<AggregateTradesSpec<Validated>> {
        self.validate()
            .context("Failed to validate AggregateTradesSpecification")?;

        Ok(AggregateTradesSpec {
            symbol: self.symbol,
            from_id: self.from_id,
            start_time: self.start_time,
            end_time: self.end_time,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the aggregate trades specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if let Some(limit) = self.limit
            && limit > 1000
        {
            return Err(InvalidParameter::range("limit", 1, 1000).into());
        }

        Ok(())
    }
}
