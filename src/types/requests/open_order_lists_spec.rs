use std::marker::PhantomData;

use serde::Serialize;

use crate::Result;
use crate::types::requests::{Unvalidated, Validated};

/**
 * Open order lists query specification for getting current active order lists.
 * 
 * This struct handles open order lists query with no parameters required.
 * Returns all currently open order lists.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrderListsSpec<S=Unvalidated> {
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OpenOrderListsSpec<Unvalidated> {
    /**
     * Creates a new open order lists specification.
     * 
     * # Returns
     * - `Self`: New open order lists specification.
     */
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    /**
     * Builds the open order lists specification.
     * 
     * # Returns
     * - `OpenOrderListsSpec<Validated>`: Validated specification.
     */
    pub fn build(self) -> Result<OpenOrderListsSpec<Validated>> {
        Ok(OpenOrderListsSpec {
            _state: PhantomData::<Validated>,
        })
    }
}

impl Default for OpenOrderListsSpec<Unvalidated> {
    fn default() -> Self {
        Self::new()
    }
}