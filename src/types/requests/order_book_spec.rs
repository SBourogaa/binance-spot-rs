use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Order book query specification.
 *
 * This specification handles parameters for querying order book depth
 * with optional limit parameter for controlling the number of levels returned.
 *
 * # Fields
 * - `symbol`: Trading symbol to query order book for.
 * - `limit`: Optional number of entries to return (default: 100, max: 5000).
 */
#[derive(Debug, Clone, Serialize)]
pub struct OrderBookSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u16>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OrderBookSpec<Unvalidated> {
    /**
     * Creates a new order book specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: New order book specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the limit for number of order book levels to return.
     *
     * # Arguments
     * - `limit`: Number of levels to return (max: 5000).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the order book specification.
     *
     * # Returns
     * - `OrderBookSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OrderBookSpec<Validated>> {
        self.validate()
            .context("Failed to validate OrderBookSpecification")?;

        Ok(OrderBookSpec {
            symbol: self.symbol,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the order book specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if let Some(limit) = self.limit
            && limit > 5000
        {
            return Err(InvalidParameter::range("limit", 1, 5000).into());
        }

        Ok(())
    }
}
