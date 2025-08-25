use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * My trades query specification.
 *
 * This specification handles parameters for querying trade history
 * for a specific account and symbol with various filtering options.
 *
 * # Fields
 * - `symbol`: Symbol to retrieve trades for (required).
 * - `order_id`: Optional order ID to filter trades.
 * - `start_time`: Optional start time filter in milliseconds.
 * - `end_time`: Optional end time filter in milliseconds.
 * - `from_id`: Optional trade ID to start from.
 * - `limit`: Optional limit (default: 500, max: 1000).
 */
#[derive(Debug, Clone, Serialize)]
pub struct MyTradesSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderId")]
    pub order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fromId")]
    pub from_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl MyTradesSpec<Unvalidated> {
    /**
     * Creates a new my trades specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query trades for.
     *
     * # Returns
     * - `Self`: New my trades specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order ID filter.
     *
     * # Arguments
     * - `order_id`: Order ID to filter trades.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /**
     * Sets the start time filter.
     *
     * # Arguments
     * - `start_time`: Start time in milliseconds.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /**
     * Sets the end time filter.
     *
     * # Arguments
     * - `end_time`: End time in milliseconds.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /**
     * Sets the starting trade ID.
     *
     * # Arguments
     * - `from_id`: Trade ID to start from.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    /**
     * Sets the limit for number of trades to return.
     *
     * # Arguments
     * - `limit`: Number of trades to return (max: 1000).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the my trades specification.
     *
     * # Returns
     * - `MyTradesSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<MyTradesSpec<Validated>> {
        self.validate()
            .context("Failed to validate MyTradesSpecification")?;

        Ok(MyTradesSpec {
            symbol: self.symbol,
            order_id: self.order_id,
            start_time: self.start_time,
            end_time: self.end_time,
            from_id: self.from_id,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the my trades specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if let Some(limit) = self.limit {
            if limit > 1000 {
                return Err(InvalidParameter::range("limit", 1, 1000).into());
            }
        }

        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            if end <= start {
                return Err(
                    InvalidParameter::new("end_time", "must be greater than start_time").into(),
                );
            }
            let duration = end - start;
            if duration > 24 * 60 * 60 * 1000 {
                return Err(
                    InvalidParameter::new("time_range", "cannot be longer than 24 hours").into(),
                );
            }
        }

        if let Some(_from_id) = self.from_id {
            if self.start_time.is_some() || self.end_time.is_some() {
                return Err(
                    InvalidParameter::mutually_exclusive("fromId", "startTime/endTime").into(),
                );
            }
        }

        if let Some(_order_id) = self.order_id {
            if self.start_time.is_some() || self.end_time.is_some() {
                return Err(
                    InvalidParameter::mutually_exclusive("orderId", "startTime/endTime").into(),
                );
            }
        }

        Ok(())
    }
}
