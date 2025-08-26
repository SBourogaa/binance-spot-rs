use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Cancel order list specification for canceling OCO/OTO/OTOCO orders.
 *
 * # Fields
 * - `symbol`: Trading symbol for the order list (required).
 * - `order_list_id`: Optional order list ID to cancel (mutually exclusive with list_client_order_id).
 * - `list_client_order_id`: Optional client-specified order list identifier (mutually exclusive with order_list_id).
 * - `new_client_order_id`: Optional new client order ID used to uniquely identify this cancel operation.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderListSpec<S = Unvalidated> {
    pub symbol: String,
    pub order_list_id: Option<u64>,
    pub list_client_order_id: Option<String>,
    pub new_client_order_id: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl CancelOrderListSpec<Unvalidated> {
    /**
     * Creates a new cancel order list specification with symbol.
     *
     * # Arguments
     * - `symbol`: Trading symbol for the order list.
     *
     * # Returns
     * - `Self`: New cancel order list specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_list_id: None,
            list_client_order_id: None,
            new_client_order_id: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order list ID to cancel.
     *
     * # Arguments
     * - `order_list_id`: Order list ID to cancel.
     *
     * # Returns
     * - `Self`: Updated cancel order list specification.
     */
    pub fn with_order_list_id(mut self, order_list_id: u64) -> Self {
        self.order_list_id = Some(order_list_id);
        self
    }

    /**
     * Sets the list client order ID to cancel.
     *
     * # Arguments
     * - `list_client_order_id`: Client order list ID to cancel.
     *
     * # Returns
     * - `Self`: Updated cancel order list specification.
     */
    pub fn with_list_client_order_id(mut self, list_client_order_id: impl Into<String>) -> Self {
        self.list_client_order_id = Some(list_client_order_id.into());
        self
    }

    /**
     * Sets the new client order ID to cancel.
     *
     * # Arguments
     * - `new_client_order_id`: New client order ID used to uniquely identify this cancel operation.
     *
     * # Returns
     * - `Self`: Updated cancel order list specification.
     */
    pub fn with_new_client_order_id(mut self, new_client_order_id: impl Into<String>) -> Self {
        self.new_client_order_id = Some(new_client_order_id.into());
        self
    }

    /**
     * Builds the cancel order list specification.
     *
     * # Returns
     * - `CancelOrderListSpec<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<CancelOrderListSpec<Validated>> {
        self.validate()
            .context("Failed to validate CancelOrderListSpec")?;

        Ok(CancelOrderListSpec {
            symbol: self.symbol,
            order_list_id: self.order_list_id,
            list_client_order_id: self.list_client_order_id,
            new_client_order_id: self.new_client_order_id,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the cancel order list parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.order_list_id.is_none() && self.list_client_order_id.is_none() {
            return Err(InvalidParameter::required("order_list_id or list_client_order_id").into());
        }

        if self.order_list_id.is_some() && self.list_client_order_id.is_some() {
            return Err(InvalidParameter::mutually_exclusive(
                "order_list_id",
                "list_client_order_id",
            )
            .into());
        }

        if let Some(ref list_client_id) = self.list_client_order_id
            && list_client_id.trim().is_empty()
        {
            return Err(InvalidParameter::empty("list_client_order_id").into());
        }

        if let Some(ref new_client_id) = self.new_client_order_id
            && new_client_id.trim().is_empty()
        {
            return Err(InvalidParameter::empty("new_client_order_id").into());
        }

        Ok(())
    }
}
