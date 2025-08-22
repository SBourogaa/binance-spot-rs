use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Prevented matches query specification with builder pattern.
 * 
 * This struct handles the complex parameter combinations and mutual exclusions
 * for the myPreventedMatches endpoint using a clean builder pattern.
 * 
 * # Fields
 * - `symbol`: Trading symbol to query prevented matches for (required).
 * - `prevented_match_id`: Specific prevented match ID to query.
 * - `order_id`: Order ID to filter prevented matches.
 * - `from_prevented_match_id`: Pagination starting point for prevented matches.
 * - `limit`: Maximum number of results to return (default: 500, max: 1000).
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreventedMatchesSpec<S=Unvalidated> {
    pub symbol: String,
    pub prevented_match_id: Option<u64>,
    pub order_id: Option<u64>,
    pub from_prevented_match_id: Option<u64>,
    pub limit: Option<u32>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl PreventedMatchesSpec<Unvalidated> {
    /**
     * Creates a new prevented matches specification with required parameters.
     * 
     * # Arguments
     * - `symbol`: Trading symbol to query prevented matches for.
     * 
     * # Returns
     * - `Self`: New prevented matches specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the specific prevented match ID to query.
     * 
     * # Arguments
     * - `prevented_match_id`: ID of the prevented match to query.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_prevented_match_id(mut self, prevented_match_id: u64) -> Self {
        self.prevented_match_id = Some(prevented_match_id);
        self
    }

    /**
     * Sets the order ID to filter prevented matches.
     * 
     * # Arguments
     * - `order_id`: Order ID to filter matches.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /**
     * Sets the pagination starting point for prevented matches.
     * 
     * # Arguments
     * - `from_prevented_match_id`: ID to start pagination from.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_from_prevented_match_id(mut self, from_prevented_match_id: u64) -> Self {
        self.from_prevented_match_id = Some(from_prevented_match_id);
        self
    }

    /**
     * Sets the result limit.
     * 
     * # Arguments
     * - `limit`: Maximum number of results to return (1-1000).
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the prevented matches specification.
     * 
     * Validates the parameters and returns a fully constructed specification.
     * 
     * # Returns
     * - `PreventedMatchesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<PreventedMatchesSpec<Validated>> {
        self.validate().context("Failed to validate PreventedMatchesSpecification")?;

        Ok(PreventedMatchesSpec {
            symbol: self.symbol,
            prevented_match_id: self.prevented_match_id,
            order_id: self.order_id,
            from_prevented_match_id: self.from_prevented_match_id,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }
    
    /**
     * Validates the prevented matches specification parameters.
     * 
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.prevented_match_id.is_some() && self.order_id.is_some() {
            return Err(InvalidParameter::mutually_exclusive(
                "prevented_match_id",
                "order_id"
            ).into());
        }
        
        if self.prevented_match_id.is_none() && self.order_id.is_none() {
            return Err(InvalidParameter::new(
                "prevented_match_id or order_id",
                "must be specified"
            ).into());
        }
        
        if self.from_prevented_match_id.is_some() && self.order_id.is_none() {
            return Err(InvalidParameter::new(
                "from_prevented_match_id",
                "requires order_id to be specified"
            ).into());
        }
        
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err(InvalidParameter::range("limit", 1, 1000).into());
            }
        }
        
        Ok(())
    }
}