use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Order list status query specification for getting specific order list information.
 * 
 * # Fields
 * - `order_list_id`: Optional order list ID to query (mutually exclusive with original_client_order_id).
 * - `original_client_order_id`: Optional original client order ID to query (mutually exclusive with order_list_id).
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListStatusSpec<S=Unvalidated> {
    pub order_list_id: Option<u64>,
    #[serde(rename = "origClientOrderId")]
    pub original_client_order_id: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OrderListStatusSpec<Unvalidated> {
    /**
     * Creates a new order list status specification.
     * 
     * # Returns
     * - `Self`: New order list status specification.
     */
    pub fn new() -> Self {
        Self {
            order_list_id: None,
            original_client_order_id: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order list ID to query.
     * 
     * # Arguments
     * - `order_list_id`: Order list ID to query.
     * 
     * # Returns
     * - `Self`: Updated order list status specification.
     */
    pub fn with_order_list_id(mut self, order_list_id: u64) -> Self {
        self.order_list_id = Some(order_list_id);
        self
    }

    /**
     * Sets the original client order ID to query.
     * 
     * # Arguments
     * - `original_client_order_id`: Original client order ID to query.
     * 
     * # Returns
     * - `Self`: Updated order list status specification.
     */
    pub fn with_original_client_order_id(mut self, original_client_order_id: impl Into<String>) -> Self {
        self.original_client_order_id = Some(original_client_order_id.into());
        self
    }

    /**
     * Builds the order list status specification.
     * 
     * # Returns
     * - `OrderListStatusSpec<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OrderListStatusSpec<Validated>> {
        self.validate().context("Failed to validate OrderListStatusSpec")?;

        Ok(OrderListStatusSpec {
            order_list_id: self.order_list_id,
            original_client_order_id: self.original_client_order_id,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the order list status parameters.
     * 
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.order_list_id.is_none() && self.original_client_order_id.is_none() {
            return Err(InvalidParameter::required("order_list_id or original_client_order_id").into());
        }

        if self.order_list_id.is_some() && self.original_client_order_id.is_some() {
            return Err(InvalidParameter::mutually_exclusive(
                "order_list_id",
                "original_client_order_id"
            ).into());
        }

        if let Some(ref orig_client_id) = self.original_client_order_id {
            if orig_client_id.trim().is_empty() {
                return Err(InvalidParameter::empty("original_client_order_id").into());
            }
        }

        Ok(())
    }
}

impl Default for OrderListStatusSpec<Unvalidated> {
    fn default() -> Self {
        Self::new()
    }
}