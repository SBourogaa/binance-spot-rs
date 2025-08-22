use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Cancel all orders specification.
 * 
 * This specification handles parameters for canceling all active orders
 * on a specified symbol.
 * 
 * # Fields
 * - `symbol`: Trading symbol to cancel orders for.
 */
#[derive(Debug, Clone, Serialize)]
pub struct CancelAllOrdersSpec<S=Unvalidated> {
    pub symbol: String,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl CancelAllOrdersSpec<Unvalidated> {
    /**
     * Creates a new cancel all orders specification.
     * 
     * # Arguments
     * - `symbol`: Trading symbol to cancel orders for.
     * 
     * # Returns
     * - `Self`: New cancel all orders specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            _state: PhantomData,
        }
    }

    /**
     * Builds the cancel all orders specification.
     * 
     * # Returns
     * - `CancelAllOrdersSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<CancelAllOrdersSpec<Validated>> {
        self.validate().context("Failed to validate CancelAllOrdersSpecification")?;

        Ok(CancelAllOrdersSpec {
            symbol: self.symbol,
            _state: PhantomData::<Validated>,
        })
    }
    
    /**
     * Validates the cancel all orders specification parameters.
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