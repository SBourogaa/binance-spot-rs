use std::marker::PhantomData;
use serde::Serialize;
use anyhow::Context;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Specification for querying a specific order.
 *
 * This specification is used to query the status of a specific order by either
 * order ID or original client order ID. 
 * 
 * # Fields
 * - `symbol`: Trading symbol to query the order for (required).
 * - `order_id`: Order ID to query.
 * - `original_client_order_id`: Original client order ID to query.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderSpec<S=Unvalidated> {
    pub symbol: String,
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId")]
    pub original_client_order_id: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl QueryOrderSpec<Unvalidated> {
    /**
     * Creates a new query order specification.
     * 
     * # Arguments
     * - `symbol`: Trading symbol to query the order for.
     * 
     * # Returns
     * - `Self`: New query order specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            original_client_order_id: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order ID to query.
     * 
     * # Arguments
     * - `order_id`: ID of the order to query.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /**
     * Sets the original client order ID to query.
     * 
     * # Arguments
     * - `original_client_order_id`: Original client order ID to query.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_original_client_order_id(mut self, original_client_order_id: impl Into<String>) -> Self {
        self.original_client_order_id = Some(original_client_order_id.into());
        self
    }

    /**
     * Builds the query order specification.
     * 
     * # Returns
     * - `QueryOrderSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<QueryOrderSpec<Validated>> {
        self.validate().context("Failed to validate QueryOrderSpecification")?;
        
        Ok(QueryOrderSpec {
            symbol: self.symbol,
            order_id: self.order_id,
            original_client_order_id: self.original_client_order_id,
            _state: PhantomData,
        })
    }

    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.order_id.is_none() && self.original_client_order_id.is_none() {
            return Err(InvalidParameter::new(
                "order_id or original_client_order_id",
                "must be specified"
            ).into());
        }
        
        Ok(())
    }
}