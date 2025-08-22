use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Commission rates query specification.
 * 
 * This specification handles parameters for querying commission rates
 * for a specific trading symbol.
 * 
 * # Fields
 * - `symbol`: Trading symbol to get commission rates for.
 */
#[derive(Debug, Clone, Serialize)]
pub struct CommissionRatesSpec<S=Unvalidated> {
    pub symbol: String,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl CommissionRatesSpec<Unvalidated> {
    /**
     * Creates a new commission rates specification.
     * 
     * # Arguments
     * - `symbol`: Trading symbol to query.
     * 
     * # Returns
     * - `Self`: New commission rates specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            _state: PhantomData,
        }
    }

    /**
     * Builds the commission rates specification.
     * 
     * # Returns
     * - `CommissionRatesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<CommissionRatesSpec<Validated>> {
        self.validate().context("Failed to validate CommissionRatesSpecification")?;

        Ok(CommissionRatesSpec {
            symbol: self.symbol,
            _state: PhantomData::<Validated>,
        })
    }
    
    /**
     * Validates the commission rates specification parameters.
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