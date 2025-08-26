use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    enums::CancelRestrictions,
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Cancel order specification for clean parameter specification.
 *
 * # Fields
 * - `symbol`: Trading symbol to cancel order for.
 * - `order_id`: Optional order ID to cancel.
 * - `original_client_order_id`: Optional original client order ID to cancel.
 * - `new_client_order_id`: Optional new client order ID to assign to the cancellation.
 * - `cancel_restrictions`: Optional restrictions on the cancellation.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderSpec<S = Unvalidated> {
    pub symbol: String,
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId")]
    pub original_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    pub cancel_restrictions: Option<CancelRestrictions>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl CancelOrderSpec<Unvalidated> {
    /**
     * Creates a new cancel order specification with required symbol.
     *
     * # Arguments
     * - `symbol`: Trading symbol to cancel order for.
     *
     * # Returns
     * - `Self`: New cancel order specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            original_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order ID to cancel.
     *
     * # Arguments
     * - `order_id`: The ID of the order to cancel.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /**
     * Sets the original client order ID to cancel.
     *
     * # Arguments
     * - `original_id`: The original client order ID to cancel.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_original_client_order_id(mut self, original_id: impl Into<String>) -> Self {
        self.original_client_order_id = Some(original_id.into());
        self
    }

    /**
     * Sets a new client order ID for the cancellation.
     *
     * # Arguments
     * - `new_id`: The new client order ID to assign to the cancellation
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_new_client_order_id(mut self, new_id: impl Into<String>) -> Self {
        self.new_client_order_id = Some(new_id.into());
        self
    }

    /**
     * Sets cancel restrictions for the order.
     *
     * # Arguments
     * - `cancel_restrictions`: Restrictions on the cancellation (e.g., no market
     *   orders, no stop orders).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_cancel_restrictions(mut self, cancel_restrictions: CancelRestrictions) -> Self {
        self.cancel_restrictions = Some(cancel_restrictions);
        self
    }

    /**
     * Builds the cancel order specification.
     *
     * # Returns
     * - `CancelOrderSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<CancelOrderSpec<Validated>> {
        self.validate()
            .context("Failed to validate CancelOrderSpecification")?;

        Ok(CancelOrderSpec {
            symbol: self.symbol,
            order_id: self.order_id,
            original_client_order_id: self.original_client_order_id,
            new_client_order_id: self.new_client_order_id,
            cancel_restrictions: self.cancel_restrictions,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Sets cancel restrictions for the order.
     *
     * # Arguments
     * - `cancel_restrictions`: Restrictions on the cancellation based on order status
     *   (ONLY_NEW: only cancel if status is NEW, ONLY_PARTIALLY_FILLED: only cancel if partially filled).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.order_id.is_none() && self.original_client_order_id.is_none() {
            return Err(InvalidParameter::required("orderId or origClientOrderId").into());
        }

        Ok(())
    }
}
