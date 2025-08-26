use std::marker::PhantomData;

use anyhow::Context;
use rust_decimal::Decimal;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Amendment order specification for reducing order quantity while keeping priority.
 *
 * This struct handles order amendment parameters with validation to ensure
 * either orderId or origClientOrderId is provided (but not both), and that
 * the new quantity is valid.
 *
 * # Fields
 * - `symbol`: Trading symbol to amend the order for.
 * - `order_id`: Optional order ID to amend (mutually exclusive with origClientOrderId).
 * - `orig_client_order_id`: Optional original client order ID to amend (mutually exclusive with orderId).
 * - `new_client_order_id`: Optional new client order ID for the amended order.
 * - `new_quantity`: New quantity to set for the order (must be greater than 0 and less than the original order quantity).
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderSpec<S = Unvalidated> {
    pub symbol: String,
    pub order_id: Option<u64>,
    #[serde(rename = "origClientOrderId")]
    pub original_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    #[serde(rename = "newQty")]
    pub new_quantity: Decimal,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl AmendOrderSpec<Unvalidated> {
    /**
     * Creates a new amend order specification with required parameters.
     *
     * # Arguments
     * - `symbol`: Trading symbol to amend the order for.
     * - `new_quantity`: New quantity to set for the order (must be greater than 0 and less than the original order quantity).
     *
     * # Returns
     * - `Self`: New amend order specification.
     */
    pub fn new(symbol: impl Into<String>, new_quantity: Decimal) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            original_client_order_id: None,
            new_client_order_id: None,
            new_quantity,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order ID to amend.
     *
     * # Arguments
     * - `order_id`: Order ID to amend.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /**
     * Sets the original client order ID to amend.
     *
     * # Arguments
     * - `original_id`: Original client order ID to amend.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_original_client_order_id(mut self, original_id: impl Into<String>) -> Self {
        self.original_client_order_id = Some(original_id.into());
        self
    }

    /**
     * Sets a new client order ID for the amended order.
     *
     * # Arguments
     * - `new_id`: New client order ID to set for the amended order
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_new_client_order_id(mut self, new_id: impl Into<String>) -> Self {
        self.new_client_order_id = Some(new_id.into());
        self
    }

    /**
     * Builds the amend order specification.
     *
     * # Returns
     * - `AmendOrderSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<AmendOrderSpec<Validated>> {
        self.validate()
            .context("Failed to validate AmendOrderSpecification")?;

        Ok(AmendOrderSpec {
            symbol: self.symbol,
            order_id: self.order_id,
            original_client_order_id: self.original_client_order_id,
            new_client_order_id: self.new_client_order_id,
            new_quantity: self.new_quantity,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the amend order specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        match (&self.order_id, &self.original_client_order_id) {
            (None, None) => {
                return Err(InvalidParameter::required("order_id or orig_client_order_id").into());
            }
            (Some(_), Some(_)) => {
                return Err(InvalidParameter::mutually_exclusive(
                    "order_id",
                    "orig_client_order_id",
                )
                .into());
            }
            _ => {}
        }

        if let Some(ref orig_client_id) = self.original_client_order_id
            && orig_client_id.trim().is_empty()
        {
            return Err(InvalidParameter::empty("orig_client_order_id").into());
        }

        if self.new_quantity <= Decimal::ZERO {
            return Err(InvalidParameter::new("new_quantity", "must be greater than 0").into());
        }

        Ok(())
    }
}
