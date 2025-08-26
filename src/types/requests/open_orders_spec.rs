use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Open orders query specification.
 *
 * This specification handles parameters for querying open orders
 * for a specific symbol or all symbols.
 *
 * # Fields
 * - `symbol`: Optional symbol filter - if None, returns orders for all symbols.
 */
#[derive(Debug, Clone, Serialize)]
pub struct OpenOrdersSpec<S = Unvalidated> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OpenOrdersSpec<Unvalidated> {
    /**
     * Creates a new open orders specification.
     *
     * # Returns
     * - `Self`: New open orders specification.
     */
    pub fn new() -> Self {
        Self {
            symbol: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the symbol filter.
     *
     * # Arguments
     * - `symbol`: Trading symbol to filter orders.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /**
     * Builds the open orders specification.
     *
     * # Returns
     * - `OpenOrdersSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OpenOrdersSpec<Validated>> {
        self.validate()
            .context("Failed to validate OpenOrdersSpecification")?;

        Ok(OpenOrdersSpec {
            symbol: self.symbol,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the open orders specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if let Some(ref symbol) = self.symbol
            && symbol.trim().is_empty()
        {
            return Err(InvalidParameter::empty("symbol").into());
        }
        Ok(())
    }
}

impl Default for OpenOrdersSpec<Unvalidated> {
    fn default() -> Self {
        Self::new()
    }
}
