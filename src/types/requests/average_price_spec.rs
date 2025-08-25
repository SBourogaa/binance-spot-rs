use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Average price query specification.
 *
 * This specification handles parameters for querying current average price
 * for a specific trading symbol over a time window.
 *
 * # Fields
 * - `symbol`: Trading symbol to query average price for.
 */
#[derive(Debug, Clone, Serialize)]
pub struct AveragePriceSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl AveragePriceSpec<Unvalidated> {
    /**
     * Creates a new average price specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: New average price specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            _state: PhantomData,
        }
    }

    /**
     * Builds the average price specification.
     *
     * # Returns
     * - `AveragePriceSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<AveragePriceSpec<Validated>> {
        self.validate()
            .context("Failed to validate AveragePriceSpecification")?;

        Ok(AveragePriceSpec {
            symbol: self.symbol,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the average price specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        Ok(())
    }
}
