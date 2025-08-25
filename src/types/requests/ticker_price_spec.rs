use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Ticker price query specification.
 *
 * This specification handles parameters for querying latest price
 * information for one or more trading symbols.
 *
 * # Fields
 * - `symbol`: Optional single symbol to query.
 * - `symbols`: Optional array of symbols to query.
 */
#[derive(Debug, Clone, Serialize)]
pub struct TickerPriceSpec<S = Unvalidated> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl TickerPriceSpec<Unvalidated> {
    /**
     * Creates a new ticker price specification.
     *
     * # Returns
     * - `Self`: New ticker price specification.
     */
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
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
     * Builds the ticker price specification.
     *
     * # Returns
     * - `TickerPriceSpecification<Validated>`: Validated specification or error if validation fails
     */
    pub fn build(self) -> Result<TickerPriceSpec<Validated>> {
        self.validate()
            .context("Failed to validate TickerPriceSpecification")?;

        Ok(TickerPriceSpec {
            symbol: self.symbol,
            symbols: self.symbols,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the ticker price specification parameters.
     *
     * # Returns
     * - `Result<()>`: Ok if valid, error if invalid parameters.
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

        if self.symbol.is_some() && self.symbols.is_some() {
            return Err(InvalidParameter::mutually_exclusive("symbol", "symbols").into());
        }

        Ok(())
    }
}
