use std::hash::Hash;

/**
 * Categorization of Binance API errors for easier error handling.
 *
 * This enum groups related error codes into logical categories,
 * making it easier to handle different types of errors appropriately.
 *
 * # Variants
 * - `ServerOrNetwork`: 10xx series - General server or network issues
 * - `RequestIssues`: 11xx series - Problems with request format or parameters
 * - `TradingRejected`: 20xx series - Trading-specific rejections
 * - `FilterFailure`: Order filter validation failures
 * - `CancelReplace`: Cancel-replace operation specific errors
 * - `Unknown`: Unrecognized error codes
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ErrorCategory {
    /// Server or network related issues (10xx series)
    ServerOrNetwork,
    /// Request format or parameter issues (11xx series)  
    RequestIssues,
    /// Trading operation rejections (20xx series)
    TradingRejected,
    /// Order filter validation failures
    FilterFailure,
    /// Cancel-replace operation errors
    CancelReplace,
    /// Unknown or unrecognized error codes
    #[default]
    Unknown,
}

impl ErrorCategory {
    /**
     * Categorizes an error code into the appropriate error category.
     *
     * # Arguments
     * - `code`: The Binance error code to categorize
     *
     * # Returns
     * - `Self`: The appropriate category for the error code
     */
    pub const fn from_code(code: i32) -> Self {
        match code {
            -2021 | -2022 => Self::CancelReplace,
            -1013 => Self::FilterFailure,
            -1099..=-1000 => Self::ServerOrNetwork,
            -1199..=-1100 => Self::RequestIssues,
            -2099..=-2000 => Self::TradingRejected,
            _ => Self::Unknown,
        }
    }

    /**
     * Returns a human-readable description of the error category.
     */
    pub fn description(&self) -> &'static str {
        match self {
            Self::ServerOrNetwork => "Server or network issue",
            Self::RequestIssues => "Request format or parameter issue",
            Self::TradingRejected => "Trading operation rejected",
            Self::FilterFailure => "Order filter validation failed",
            Self::CancelReplace => "Cancel-replace operation issue",
            Self::Unknown => "Unknown error type",
        }
    }

    /**
     * Returns suggested action for handling this category of error.
     */
    pub fn suggested_action(&self) -> &'static str {
        match self {
            Self::ServerOrNetwork => "Retry after delay",
            Self::RequestIssues => "Fix request parameters",
            Self::TradingRejected => "Check trading rules and account status",
            Self::FilterFailure => "Adjust order parameters to meet filters",
            Self::CancelReplace => "Check cancel-replace operation parameters",
            Self::Unknown => "Contact support if error persists",
        }
    }
}

/**
 * Macro to generate error enums with from_code and maybe methods.
 *
 * Creates enums with specific variants for known error codes and Other(i32) for unknown codes.
 * Provides both exhaustive mapping (from_code) and optional mapping (maybe) methods.
 *
 * # Arguments
 * - `$name`: The name of the enum to create
 * - `$($code:literal => $variant:ident),+`: Pairs of error codes and their corresponding enum variants
 *
 * # Generated Methods
 * - `from_code(code: i32) -> Self`: Maps any code to enum variant (uses Other for unknown)
 * - `maybe(code: i32) -> Option<Self>`: Maps only known codes (returns None for unknown)
 * - `From<i32>` implementation using from_code
 */
macro_rules! impl_from_code {
    ($name:ident, $($code:literal => $variant:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)+
            Other(i32)
        }

        impl $name {
            /**
             * Creates enum variant from error code, using Other for unknown codes.
             *
             * # Arguments
             * - `code`: The error code
             *
             * # Returns
             * - `Self`: The appropriate enum variant
             */
            pub const fn from_code(code: i32) -> Self {
                match code {
                    $($code => Self::$variant,)+
                    other => Self::Other(other)
                }
            }

            /**
             * Creates enum variant from error code if known, None for unknown codes.
             *
             * # Arguments
             * - `code`: The error code
             *
             * # Returns
             * - `Option<Self>`: Some(variant) if known, None if unknown
             */
            pub const fn maybe(code: i32) -> Option<Self> {
                match code {
                    $($code => Some(Self::$variant),)+
                    _ => None
                }
            }

            /**
             * Returns the error code for this variant.
             */
            pub const fn code(&self) -> i32 {
                match self {
                    $(Self::$variant => $code,)+
                    Self::Other(code) => *code,
                }
            }

            /**
             * Returns a human-readable description of this error.
             */
            pub fn description(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)+
                    Self::Other(_) => "Unknown error code",
                }
            }
        }

        impl From<i32> for $name {
            fn from(code: i32) -> Self {
                Self::from_code(code)
            }
        }
    };
}

// Export the macro for use in other modules
pub(super) use impl_from_code;
