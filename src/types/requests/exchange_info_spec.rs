use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Exchange information query specification.
 *
 * This specification handles parameters for querying exchange trading rules,
 * rate limits, and symbol information with various filtering options.
 *
 * # Fields
 * - `symbol`: Optional single symbol to query.
 * - `symbols`: Optional JSON array of symbols to query (sent as JSON string).
 * - `permissions`: Optional permissions filter (e.g., ["SPOT", "MARGIN"]).
 * - `show_permission_sets`: Optional boolean controlling whether permissionSets field is populated.
 * - `symbol_status`: Optional trading status filter ("TRADING", "HALT", "BREAK").
 */
#[derive(Debug, Clone, Serialize)]
pub struct ExchangeInfoSpec<S = Unvalidated> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showPermissionSets")]
    pub show_permission_sets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolStatus")]
    pub symbol_status: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl ExchangeInfoSpec<Unvalidated> {
    /**
     * Creates a new exchange info specification.
     *
     * # Returns
     * - `Self`: New exchange info specification with no filters.
     */
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            permissions: None,
            show_permission_sets: None,
            symbol_status: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets a single symbol to query.
     *
     * # Arguments
     * - `symbol`: Trading symbol to query.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /**
     * Sets multiple symbols to query.
     *
     * # Arguments
     * - `symbols`: Array of trading symbols to query.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_symbols(mut self, symbols: Vec<impl Into<String>>) -> Self {
        let symbol_strings: Vec<String> = symbols.into_iter().map(|s| s.into()).collect();
        self.symbols = Some(serde_json::to_string(&symbol_strings).unwrap());
        self
    }

    /**
     * Sets the permissions filter.
     *
     * # Arguments
     * - `permissions`: Array of permissions to filter by.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_permissions(mut self, permissions: Vec<impl Into<String>>) -> Self {
        let permission_strings: Vec<String> = permissions.into_iter().map(|p| p.into()).collect();
        self.permissions = Some(permission_strings);
        self
    }

    /**
     * Sets whether to show permission sets in response.
     *
     * # Arguments
     * - `show`: Whether to populate permissionSets field.
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_show_permission_sets(mut self, show: bool) -> Self {
        self.show_permission_sets = Some(show);
        self
    }

    /**
     * Sets the symbol status filter.
     *
     * # Arguments
     * - `status`: Trading status to filter by ("TRADING", "HALT", or "BREAK").
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_symbol_status(mut self, status: impl Into<String>) -> Self {
        self.symbol_status = Some(status.into());
        self
    }

    /**
     * Builds the exchange info specification.
     *
     * # Returns
     * - `ExchangeInfoSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<ExchangeInfoSpec<Validated>> {
        self.validate()
            .context("Failed to validate ExchangeInfoSpecification")?;

        Ok(ExchangeInfoSpec {
            symbol: self.symbol,
            symbols: self.symbols,
            permissions: self.permissions,
            show_permission_sets: self.show_permission_sets,
            symbol_status: self.symbol_status,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the exchange info specification parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if let Some(ref symbol) = self.symbol {
            if symbol.trim().is_empty() {
                return Err(InvalidParameter::empty("symbol").into());
            }
        }

        if let Some(ref symbols_json) = self.symbols {
            if symbols_json.trim().is_empty() {
                return Err(InvalidParameter::empty("symbols").into());
            }

            match serde_json::from_str::<Vec<String>>(symbols_json) {
                Ok(symbols_array) => {
                    if symbols_array.is_empty() {
                        return Err(InvalidParameter::empty("symbols").into());
                    }
                    for symbol in symbols_array {
                        if symbol.trim().is_empty() {
                            return Err(InvalidParameter::empty("symbols element").into());
                        }
                    }
                }
                Err(_) => {
                    return Err(InvalidParameter::new("symbols", "must be valid JSON array").into());
                }
            }
        }

        if let Some(ref status) = self.symbol_status {
            match status.as_str() {
                "TRADING" | "HALT" | "BREAK" => {}
                _ => {
                    return Err(InvalidParameter::new(
                        "symbol_status",
                        "must be one of: TRADING, HALT, BREAK",
                    )
                    .into());
                }
            }
        }

        if self.symbol.is_some() && self.symbols.is_some() {
            return Err(InvalidParameter::mutually_exclusive("symbol", "symbols").into());
        }

        if self.symbol_status.is_some() && (self.symbol.is_some() || self.symbols.is_some()) {
            return Err(InvalidParameter::new(
                "symbolStatus",
                "cannot be used with 'symbol' or 'symbols' parameters",
            )
            .into());
        }

        if self.permissions.is_some() && (self.symbol.is_some() || self.symbols.is_some()) {
            return Err(
                InvalidParameter::mutually_exclusive("permissions", "symbol/symbols").into(),
            );
        }

        Ok(())
    }
}
