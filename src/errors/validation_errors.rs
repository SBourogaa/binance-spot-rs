use thiserror::Error;

/**
 * Parameter validation error with context.
 * 
 * Used when request parameters fail client-side validation before being sent to the API.
 * Provides specific context about which parameter failed and why.
 */
#[derive(Debug, Error, Clone, PartialEq)]
#[error("Invalid parameter '{param}': {reason}")]
pub struct InvalidParameter {
    /// The name of the parameter that failed validation
    pub param: String,
    /// Human-readable explanation of why the parameter is invalid
    pub reason: String,
}

impl InvalidParameter {
    /**
     * Creates a new parameter validation error.
     */
    pub fn new(param: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            param: param.into(),
            reason: reason.into(),
        }
    }

    /**
     * Helper for empty parameter validation.
     */
    pub fn empty(param: impl Into<String>) -> Self {
        Self::new(param, "cannot be empty")
    }

    /**
     * Helper for invalid range validation.
     */
    pub fn range(param: impl Into<String>, min: impl std::fmt::Display, max: impl std::fmt::Display) -> Self {
        Self::new(param, format!("must be between {} and {}", min, max))
    }

    /**
     * Helper for mutual exclusion validation.
     */
    pub fn mutually_exclusive(param1: impl Into<String>, param2: impl Into<String>) -> Self {
        Self::new(
            format!("{}/{}", param1.into(), param2.into()),
            "cannot be used together"
        )
    }

    /**
     * Helper for required parameter validation.
     */
    pub fn required(param: impl Into<String>) -> Self {
        Self::new(param, "is required")
    }
}

/**
 * Authentication credentials validation error.
 * 
 * Used when API keys, private keys, or signatures fail validation before use.
 */
#[derive(Debug, Error, Clone, PartialEq)]
#[error("Invalid credentials: {reason}")]
pub struct InvalidCredentials {
    /// Human-readable explanation of the credential validation failure
    pub reason: String,
}

impl InvalidCredentials {
    /**
     * Creates a new credentials validation error.
     */
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }

    /**
     * Helper for missing API key.
     */
    pub fn missing_api_key() -> Self {
        Self::new("API key is required for authenticated requests")
    }

    /**
     * Helper for invalid private key format.
     */
    pub fn invalid_private_key(details: impl Into<String>) -> Self {
        Self::new(format!("private key format is invalid: {}", details.into()))
    }

    /**
     * Helper for signature generation failure.
     */
    pub fn signature_failed(details: impl Into<String>) -> Self {
        Self::new(format!("failed to generate signature: {}", details.into()))
    }
}

/**
 * URL validation error.
 * 
 * Used when constructed URLs are malformed or don't meet API requirements.
 */
#[derive(Debug, Error, Clone, PartialEq)]
#[error("Invalid URL '{url}': {reason}")]
pub struct InvalidUrl {
    /// The URL that failed validation
    pub url: String,
    /// Human-readable explanation of why the URL is invalid
    pub reason: String,
}

impl InvalidUrl {
    /**
     * Creates a new URL validation error.
     */
    pub fn new(url: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            reason: reason.into(),
        }
    }

    /**
     * Helper for invalid scheme validation.
     */
    pub fn invalid_scheme(url: impl Into<String>, expected: &str) -> Self {
        Self::new(url, format!("must use {} scheme", expected))
    }

    /**
     * Helper for malformed URL validation.
     */
    pub fn malformed(url: impl Into<String>) -> Self {
        Self::new(url, "URL format is invalid")
    }
}

/**
 * Configuration validation error.
 * 
 * Used when client configuration contains invalid values or incompatible combinations.
 */
#[derive(Debug, Error, Clone, PartialEq)]
#[error("Invalid configuration for '{field}': {reason}")]
pub struct InvalidConfig {
    /// The configuration field that failed validation
    pub field: String,
    /// Human-readable explanation of why the configuration is invalid
    pub reason: String,
}

impl InvalidConfig {
    /**
     * Creates a new configuration validation error.
     */
    pub fn new(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /**
     * Helper for timeout validation.
     */
    pub fn invalid_timeout(field: impl Into<String>) -> Self {
        Self::new(field, "timeout must be greater than zero")
    }

    /**
     * Helper for invalid combination validation.
     */
    pub fn incompatible(field1: impl Into<String>, field2: impl Into<String>) -> Self {
        Self::new(
            format!("{}/{}", field1.into(), field2.into()),
            "configuration combination is not supported"
        )
    }

    /**
     * Helper for out of range validation.
     */
    pub fn out_of_range(field: impl Into<String>, min: impl std::fmt::Display, max: impl std::fmt::Display) -> Self {
        Self::new(field, format!("must be between {} and {}", min, max))
    }
}