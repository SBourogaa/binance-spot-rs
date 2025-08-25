use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::errors::InvalidParameter;
use crate::types::requests::{Unvalidated, Validated};

/**
 * All orders query specification.
 *
 * This specification handles parameters for querying all account orders
 * (active, canceled, or filled) for a symbol with various filtering options.
 *
 * # Fields
 * - `symbol`: Symbol to retrieve orders for (required).
 * - `order_id`: Optional order ID to start from (gets orders >= this ID).
 * - `start_time`: Optional start time filter in milliseconds.
 * - `end_time`: Optional end time filter in milliseconds.
 * - `limit`: Optional limit (default: 500, max: 1000).
 */
#[derive(Debug, Clone, Serialize)]
pub struct AllOrdersSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderId")]
    pub order_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl AllOrdersSpec<Unvalidated> {
    /**
     * Creates a new all orders specification.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query orders for.
     *
     * # Returns
     * - `Self`: New all orders specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order ID to start from.
     *
     * # Arguments
     * - `order_id`: Order ID to start query from.
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
     * Sets the limit for number of orders to return.
     *
     * # Arguments
     * - `limit`: Number of orders to return (max: 1000).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the all orders specification.
     *
     * # Returns
     * - `AllOrdersSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<AllOrdersSpec<Validated>> {
        self.validate()
            .context("Invalid parameters in AllOrdersSpecification")?;

        Ok(AllOrdersSpec {
            symbol: self.symbol,
            order_id: self.order_id,
            start_time: self.start_time,
            end_time: self.end_time,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the all orders specification parameters.
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
                return Err(InvalidParameter::new(
                    "start_time/end_time",
                    "end_time must be greater than start_time",
                )
                .into());
            }
            let duration = end - start;
            if duration > 24 * 60 * 60 * 1000 {
                return Err(InvalidParameter::new(
                    "start/end",
                    "time range can not be longer than 24 hours",
                )
                .into());
            }
        }

        Ok(())
    }
}
