use thiserror::Error;
use tracing::debug;

use super::{
    ApiError,
    InvalidParameter,
    InvalidCredentials,
    InvalidUrl,
    InvalidConfig,
};

/**
 * Main error type for the Binance API client.
 *
 * This enum represents all possible errors that can occur when using the Binance API client.
 * It focuses on library-specific errors while allowing underlying errors (network, parsing, etc.)
 * to bubble up naturally through anyhow's error chaining.
 *
 * # Design Principles
 * - API errors: Direct mapping of Binance error responses
 * - Validation errors: Client-side parameter validation with context
 * - No wrapping: Let reqwest, serde, etc. errors bubble up via anyhow
 * - Rich context: Each error provides meaningful debugging information
 */
#[derive(Debug, Error)]
pub enum BinanceError {
    /**
     * API errors returned directly by Binance servers.
     * 
     * These errors include all official Binance error codes from the API specification,
     * including rate limiting (-1003), authentication failures, and trading rejections.
     */
    #[error(transparent)]
    Api(#[from] ApiError),

    /**
     * Parameter validation errors detected client-side.
     * 
     * These occur when request parameters fail validation before being sent to the API.
     * Catching these early provides better user experience and reduces unnecessary API calls.
     */
    #[error(transparent)]
    InvalidParameter(#[from] InvalidParameter),

    /**
     * Authentication credential validation errors.
     * 
     * These occur when API keys, signatures, or other authentication data is malformed
     * or fails validation before being used in requests.
     */
    #[error(transparent)]
    InvalidCredentials(#[from] InvalidCredentials),

    /**
     * URL validation errors.
     * 
     * These occur when constructed URLs are malformed or don't meet Binance API requirements.
     */
    #[error(transparent)]
    InvalidUrl(#[from] InvalidUrl),

    /**
     * Configuration validation errors.
     * 
     * These occur when client configuration (timeouts, connection settings, etc.)
     * contains invalid values or incompatible combinations.
     */
    #[error(transparent)]
    InvalidConfig(#[from] InvalidConfig),
}

impl BinanceError {
    /**
     * Returns the error code if this is an API error.
     */
    pub fn api_code(&self) -> Option<i32> {
        match self {
            BinanceError::Api(api_error) => Some(api_error.code),
            _ => None,
        }
    }

    /**
     * Returns the API error details if this is an API error.
     */
    pub fn api_error(&self) -> Option<&ApiError> {
        match self {
            BinanceError::Api(api_error) => Some(api_error),
            _ => None,
        }
    }

    /**
     * Checks if this error represents a rate limiting issue.
     */
    pub fn is_rate_limit(&self) -> bool {
        match self {
            BinanceError::Api(api_error) => api_error.is_rate_limit(),
            _ => false,
        }
    }

    /**
     * Checks if this error represents an authentication issue.
     */
    pub fn is_auth_error(&self) -> bool {
        match self {
            BinanceError::Api(api_error) => api_error.is_auth_error(),
            BinanceError::InvalidCredentials(_) => true,
            _ => false,
        }
    }

    /**
     * Checks if this error might be retryable.
     */
    pub fn is_retryable(&self) -> bool {
        let retryable = match self {
            BinanceError::Api(api_error) => api_error.is_retryable(),
            _ => false,
        };
        
        debug!(
            error_type = std::any::type_name::<Self>(),
            is_retryable = retryable,
            "Determined error retryability"
        );
        
        retryable
    }
}