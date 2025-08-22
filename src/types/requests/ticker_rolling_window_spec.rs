use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Ticker rolling window statistics query specification.
 * 
 * This specification handles parameters for querying rolling window price
 * change statistics for one or more trading symbols.
 * 
 * # Fields
 * - `symbol`: Optional single symbol to query.
 * - `symbols`: Optional array of symbols to query.
 * - `window_size`: Optional window size for statistics (e.g., "1d", "7d").
 * - `ticker_type`: Optional ticker type ("FULL" or "MINI").
 */
#[derive(Debug, Clone, Serialize)]
pub struct TickerRollingWindowSpec<S=Unvalidated> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "windowSize")]
    pub window_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    // TODO: Create TickerType enum.
    pub ticker_type: Option<String>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl TickerRollingWindowSpec<Unvalidated> {
    /**
     * Creates a new ticker rolling window specification.
     * 
     * # Returns
     * - `Self`: New ticker rolling window specification.
     */
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            window_size: None,
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
     * Sets the window size for rolling statistics.
     * 
     * # Arguments
     * - `window_size`: Window size (e.g., "1d", "7d").
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_window_size(mut self, window_size: impl Into<String>) -> Self {
        self.window_size = Some(window_size.into());
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
     * Builds the ticker rolling window specification.
     * 
     * # Returns
     * - `TickerRollingWindowSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<TickerRollingWindowSpec<Validated>> {
        self.validate().context("Failed to validate TickerRollingWindowSpecification")?;

        Ok(TickerRollingWindowSpec {
            symbol: self.symbol,
            symbols: self.symbols,
            window_size: self.window_size,
            ticker_type: self.ticker_type,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the ticker rolling window specification parameters.
     * 
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.is_none() && self.symbols.is_none() {
            return Err(InvalidParameter::new(
                "symbol or symbols", 
                "must be specified"
            ).into());
        }

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

        if let Some(ref window_size) = self.window_size {
            self.validate_window_size(window_size)?;
        }

        if let Some(ref ticker_type) = self.ticker_type {
            match ticker_type.as_str() {
                "FULL" | "MINI" => {},
                _ => return Err(InvalidParameter::new(
                    "ticker_type", 
                    "must be either FULL or MINI"
                ).into()),
            }
        }

        if self.symbol.is_some() && self.symbols.is_some() {
            return Err(InvalidParameter::mutually_exclusive(
                "symbol",
                "symbols"
            ).into());
        }

        Ok(())
    }

    /**
     * Validates window size format and range according to API requirements.
     * 
     * # Arguments
     * - `window_size`: Window size string to validate.
     * 
     * # Returns
     * - `()`: Ok if valid, error if invalid window size.
     */
    fn validate_window_size(&self, window_size: &str) -> Result<()> {
        if window_size.is_empty() {
            return Err(InvalidParameter::empty("window_size").into());
        }

        let last_char = window_size.chars().last().unwrap();
        let number_part = &window_size[..window_size.len()-1];
        
        match number_part.parse::<u32>() {
            Ok(num) => {
                match last_char {
                    'm' => {
                        if num < 1 || num > 59 {
                            return Err(InvalidParameter::new(
                                "window_size", 
                                "minutes must be 1-59"
                            ).into());
                        }
                    },
                    'h' => {
                        if num < 1 || num > 23 {
                            return Err(InvalidParameter::new(
                                "window_size", 
                                "hours must be 1-23"
                            ).into());
                        }
                    },
                    'd' => {
                        if num < 1 || num > 7 {
                            return Err(InvalidParameter::new(
                                "window_size", 
                                "days must be 1-7"
                            ).into());
                        }
                    },
                    _ => {
                        return Err(InvalidParameter::new(
                            "window_size", 
                            "must end with 'm' (minutes), 'h' (hours), or 'd' (days)"
                        ).into());
                    }
                }
            },
            Err(_) => {
                return Err(InvalidParameter::new(
                    "window_size", 
                    "must be a number followed by m, h, or d"
                ).into());
            }
        }

        Ok(())
    }
}