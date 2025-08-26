use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * All order lists query specification for getting historical order list information.
 *
 * # Fields
 * - `from_id`: Optional order list ID to start from (if supplied, neither start_time nor end_time can be provided).
 * - `start_time`: Optional timestamp to start from.
 * - `end_time`: Optional timestamp to end at.
 * - `limit`: Optional number of results to return (default 500, max 1000).
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllOrderListsSpec<S = Unvalidated> {
    pub from_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u32>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl AllOrderListsSpec<Unvalidated> {
    /**
     * Creates a new all order lists specification.
     *
     * # Returns
     * - `Self`: New all order lists specification.
     */
    pub fn new() -> Self {
        Self {
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the starting order list ID.
     *
     * # Arguments
     * - `from_id`: Order list ID to start from.
     *
     * # Returns
     * - `Self`: Updated all order lists specification.
     */
    pub fn with_from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    /**
     * Sets the start time filter.
     *
     * # Arguments
     * - `start_time`: Timestamp to start from.
     *
     * # Returns
     * - `Self`: Updated all order lists specification.
     */
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /**
     * Sets the end time filter.
     *
     * # Arguments
     * - `end_time`: Timestamp to end at.
     *
     * # Returns
     * - `Self`: Updated all order lists specification.
     */
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /**
     * Sets the result limit.
     *
     * # Arguments
     * - `limit`: Number of results to return (max 1000).
     *
     * # Returns
     * - `Self`: Updated all order lists specification.
     */
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /**
     * Builds the all order lists specification.
     *
     * # Returns
     * - `AllOrderListsSpec<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<AllOrderListsSpec<Validated>> {
        self.validate()
            .context("Failed to validate AllOrderListsSpec")?;

        Ok(AllOrderListsSpec {
            from_id: self.from_id,
            start_time: self.start_time,
            end_time: self.end_time,
            limit: self.limit,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the all order lists parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if let Some(limit) = self.limit
            && (limit == 0 || limit > 1000)
        {
            return Err(InvalidParameter::range("limit", 1, 1000).into());
        }

        if let (Some(start_time), Some(end_time)) = (self.start_time, self.end_time) {
            if start_time >= end_time {
                return Err(
                    InvalidParameter::new("start_time", "must be less than end_time").into(),
                );
            }

            if end_time - start_time > 86400000 {
                return Err(InvalidParameter::new(
                    "start_time/end_time",
                    "time range cannot be longer than 24 hours",
                )
                .into());
            }
        }

        if self.from_id.is_some() && (self.start_time.is_some() || self.end_time.is_some()) {
            return Err(InvalidParameter::new(
                "from_id",
                "cannot be used with start_time or end_time",
            )
            .into());
        }

        Ok(())
    }
}

impl Default for AllOrderListsSpec<Unvalidated> {
    fn default() -> Self {
        Self::new()
    }
}
