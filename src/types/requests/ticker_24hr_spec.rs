use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};
use anyhow::Context;
use serde::Serialize;
use std::marker::PhantomData;

/**
 * 24hr ticker statistics query specification.
 *
 * This specification handles parameters for querying 24-hour ticker statistics
 * with optional filters for specific symbols and ticker types.
 *
 * # Fields
 * - `symbol`: Optional single symbol to query.
 * - `symbols`: Optional array of symbols to query.
 * - `ticker_type`: Optional ticker type ("FULL" or "MINI").
 */
#[derive(Debug, Clone, Serialize)]
pub struct Ticker24HrSpec<S = Unvalidated> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    // TODO: Create TickerType enum.
    pub ticker_type: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl Ticker24HrSpec<Unvalidated> {
    /**
     * Creates a new 24hr ticker specification.
     *
     * # Returns
     * - `Self`: New ticker specification.
     */
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            ticker_type: None,
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
     * Sets the ticker type filter.
     *
     * # Arguments
     * - `ticker_type`: Ticker type ("FULL" or "MINI").
     *
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_ticker_type(mut self, ticker_type: impl Into<String>) -> Self {
        self.ticker_type = Some(ticker_type.into());
        self
    }

    /**
     * Builds the ticker specification.
     *
     * # Returns
     * - `Ticker24HrSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<Ticker24HrSpec<Validated>> {
        self.validate()
            .context("Failed to validate Ticker24HrSpecification")?;
        Ok(Ticker24HrSpec {
            symbol: self.symbol,
            symbols: self.symbols,
            ticker_type: self.ticker_type,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the ticker specification parameters.
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

        if let Some(ref ticker_type) = self.ticker_type {
            match ticker_type.as_str() {
                "FULL" | "MINI" => {}
                _ => {
                    return Err(InvalidParameter::new(
                        "ticker_type",
                        "must be either FULL or MINI",
                    )
                    .into());
                }
            }
        }

        if self.symbol.is_some() && self.symbols.is_some() {
            return Err(InvalidParameter::mutually_exclusive("symbol", "symbols").into());
        }

        Ok(())
    }
}
