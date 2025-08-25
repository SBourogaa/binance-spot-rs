use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{error, warn};

use crate::errors::{ErrorCategory, RequestError, ServerError, TradingError};
use crate::filters::{
    FilterFailure, TradingRejectionMessage, parse_filter_failure, parse_trading_rejection,
};

/**
 * API error returned by Binance with comprehensive categorization.
 *
 * This structure represents errors returned directly by the Binance API,
 * with additional parsing and categorization for easier error handling.
 *
 * # Fields
 * - `code`: Official Binance error code
 * - `msg`: Error message from Binance
 * - `category`: Categorized error type for easier handling
 * - `server_error`: Specific server error type (if applicable)
 * - `request_error`: Specific request error type (if applicable)  
 * - `trading_error`: Specific trading error type (if applicable)
 * - `filter_failure`: Parsed filter failure type (if applicable)
 * - `trading_rejection`: Parsed trading rejection message (if applicable)
 */
#[derive(Debug, Error, Clone, Serialize, Deserialize, PartialEq)]
#[error("Binance API error {code}: {msg}")]
pub struct ApiError {
    /// Official Binance error code
    pub code: i32,
    /// Error message from Binance
    pub msg: String,

    /// Categorized error type for easier handling (not serialized)
    #[serde(skip)]
    pub category: ErrorCategory,

    /// Specific server error type if this is a 10xx error (not serialized)
    #[serde(skip)]
    pub server_error: Option<ServerError>,

    /// Specific request error type if this is a 11xx error (not serialized)
    #[serde(skip)]
    pub request_error: Option<RequestError>,

    /// Specific trading error type if this is a 20xx error (not serialized)
    #[serde(skip)]
    pub trading_error: Option<TradingError>,

    /// Parsed filter failure information (not serialized)
    #[serde(skip)]
    pub filter_failure: Option<FilterFailure>,

    /// Parsed trading rejection message (not serialized)
    #[serde(skip)]
    pub trading_rejection: Option<TradingRejectionMessage>,
}

impl ApiError {
    /**
     * Creates a new API error with automatic categorization and message parsing.
     *
     * # Arguments
     * - `code`: The Binance error code
     * - `msg`: The error message from Binance
     *
     * # Returns
     * - `Self`: A new ApiError with proper categorization and parsed details
     */
    pub fn new(code: i32, msg: impl Into<String>) -> Self {
        let msg = msg.into();
        let category = ErrorCategory::from_code(code);

        match category {
            ErrorCategory::ServerOrNetwork => {
                error!(
                    error_code = code,
                    category = ?category,
                    "Binance server/network error"
                );
            }
            ErrorCategory::RequestIssues => {
                // Check if it's an auth error based on code
                if matches!(code, -1002 | -1021 | -1022 | -2014 | -2015) {
                    warn!(
                        error_code = code,
                        category = ?category,
                        "Binance authentication/authorization error"
                    );
                } else if code == -1003 {
                    warn!(error_code = code, "Binance rate limit hit");
                } else {
                    error!(
                        error_code = code,
                        category = ?category,
                        "Binance request error"
                    );
                }
            }
            _ => {
                error!(
                    error_code = code,
                    category = ?category,
                    "Binance API error"
                );
            }
        }

        Self {
            code,
            category,
            server_error: ServerError::maybe(code),
            request_error: RequestError::maybe(code),
            trading_error: TradingError::maybe(code),
            filter_failure: parse_filter_failure(&msg),
            trading_rejection: parse_trading_rejection(&msg),
            msg,
        }
    }

    /**
     * Checks if the error is related to rate limiting (code -1003).
     */
    pub fn is_rate_limit(&self) -> bool {
        self.code == -1003
    }

    /**
     * Checks if the error is an authentication/authorization issue.
     */
    pub fn is_auth_error(&self) -> bool {
        matches!(self.code, -1002 | -1021 | -1022 | -2014 | -2015)
    }

    /**
     * Checks if the error is retryable (typically server/network issues).
     */
    pub fn is_retryable(&self) -> bool {
        !self.is_auth_error()
            && (matches!(self.category, ErrorCategory::ServerOrNetwork)
                || matches!(self.code, -1003 | -1007 | -1008))
    }

    /**
     * Gets retry delay in seconds for rate limit errors.
     */
    pub fn retry_after_seconds(&self) -> Option<u64> {
        if self.is_rate_limit() {
            Some(60)
        } else if self.is_retryable() {
            Some(5)
        } else {
            None
        }
    }

    /**
     * Checks if this is a filter validation failure.
     */
    pub fn is_filter_failure(&self) -> bool {
        self.filter_failure.is_some()
    }

    /**
     * Checks if this is a trading rejection.
     */
    pub fn is_trading_rejection(&self) -> bool {
        self.trading_rejection.is_some()
    }
}
