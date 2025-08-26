use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Allocation query specification with builder pattern.
 *
 * This struct handles the complex parameter combinations and mutual exclusions
 * for the myAllocations endpoint using a clean builder pattern.
 *
 * # Fields
 * - `symbol`: Required trading symbol to query allocations for.
 * - `start_time`: Optional start time in milliseconds.
 * - `end_time`: Optional end time in milliseconds.
 * - `from_allocation_id`: Optional pagination starting point.
 * - `limit`: Optional limit for results (default: 500, max: 1000).
 * - `order_id`: Optional order ID to filter allocations.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationSpec<S = Unvalidated> {
    // Required field
    pub symbol: String,

    // Optional fields
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_allocation_id: Option<u32>,
    pub limit: Option<u32>,
    pub order_id: Option<u64>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl AllocationSpec<Unvalidated> {
    /**
     * Creates a new allocation specification with required parameters.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query allocations for.
     *
     * # Returns
     * - `Self`: New allocation specification.
     */
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            start_time: None,
            end_time: None,
            from_allocation_id: None,
            limit: None,
            order_id: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the start time filter for allocations.
     *
     * # Arguments
     * - `start_time`: Start time in milliseconds (inclusive).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /**
     * Sets the end time filter for allocations.
     *
     * # Arguments
     * - `end_time`: End time in milliseconds (inclusive).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /**
     * Sets the pagination starting point for allocations.
     *
     * # Arguments
     * - `from_allocation_id`: Allocation ID to start from (inclusive).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_from_allocation_id(mut self, from_allocation_id: u32) -> Self {
        self.from_allocation_id = Some(from_allocation_id);
        self
    }

    /**
     * Sets the result limit.
     *
     * # Arguments
     * - `limit`: Maximum number of results to return (default: 500, max: 1000).
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Sets the order ID to filter allocations.
     *
     * # Arguments
     * - `order_id`: Order ID to filter allocations.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /**
     * Builds the allocation specification.
     *
     * # Returns
     * - `AllocationSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<AllocationSpec<Validated>> {
        self.validate()
            .context("Failed to validate AllocationSpecification")?;
        Ok(AllocationSpec {
            symbol: self.symbol,
            start_time: self.start_time,
            end_time: self.end_time,
            from_allocation_id: self.from_allocation_id,
            limit: self.limit,
            order_id: self.order_id,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the allocation specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if let Some(limit) = self.limit
            && limit > 1000
        {
            return Err(InvalidParameter::range("limit", 1, 1000).into());
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
                    "start_time/end_time",
                    "end_time - start_time can not be longer than 24 hours",
                )
                .into());
            }
        }

        Ok(())
    }
}
