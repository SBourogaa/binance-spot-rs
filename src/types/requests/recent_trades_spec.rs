use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Recent trades query specification.
 *
 * This specification handles parameters for querying recent trades
 * with optional limit parameter for controlling the number of trades returned.
 *
 * # Fields
 * - `symbol`: Trading symbol to query trades for.
 * - `limit`: Optional number of trades to return (default: 500, max: 1000).
 */
#[derive(Debug, Clone, Serialize)]
pub struct RecentTradesSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl RecentTradesSpec<Unvalidated> {
    /**
     * Creates a new recent trades specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: New recent trades specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            limit: None,
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
     * Builds the recent trades specification.
     *
     * # Returns
     * - `RecentTradesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<RecentTradesSpec<Validated>> {
        self.validate()
            .context("Failed to validate RecentTradesSpecification")?;

        Ok(RecentTradesSpec {
            symbol: self.symbol,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the recent trades specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    pub fn validate(&self) -> Result<()> {
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
