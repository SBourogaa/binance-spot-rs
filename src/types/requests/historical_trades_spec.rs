use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Historical trades query specification.
 *
 * This specification handles parameters for querying historical trades
 * with optional limit and starting trade ID controls.
 *
 * # Fields
 * - `symbol`: Trading symbol to query trades for.
 * - `limit`: Optional number of trades to return (default: 500, max: 1000).
 * - `from_id`: Optional trade ID to fetch from (returns trades with ID >= from_id).
 */
#[derive(Debug, Clone, Serialize)]
pub struct HistoricalTradesSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromId")]
    pub from_id: Option<u64>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl HistoricalTradesSpec<Unvalidated> {
    /**
     * Creates a new historical trades specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: New historical trades specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            limit: None,
            from_id: None,
            _state: PhantomData,
        }
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
     * Sets the starting trade ID.
     *
     * # Arguments
     * - `from_id`: Trade ID to fetch from (inclusive).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    /**
     * Builds the historical trades specification.
     *
     * # Returns
     * - `HistoricalTradesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<HistoricalTradesSpec<Validated>> {
        self.validate()
            .context("Failed to validate HistoricalTradesSpecification")?;

        Ok(HistoricalTradesSpec {
            symbol: self.symbol,
            limit: self.limit,
            from_id: self.from_id,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the historical trades specification parameters.
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
