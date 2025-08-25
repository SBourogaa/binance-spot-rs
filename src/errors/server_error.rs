use super::error_categories::impl_from_code;

impl_from_code!(ServerError,
    // General Server or Network Issues (10xx series)
    // These errors occur due to server or network problems.

    -1000 => Unknown,
    -1001 => Disconnected,
    -1002 => Unauthorized,
    -1003 => TooManyRequests,
    -1006 => UnexpectedResponse,
    -1007 => Timeout,
    -1008 => ServerBusy,
    -1010 => ErrorMsgReceived,
    -1013 => InvalidMessage,
    -1014 => UnknownOrderComposition,
    -1015 => TooManyOrders,
    -1016 => ServiceShuttingDown,
    -1020 => UnsupportedOperation,
    -1021 => InvalidTimestamp,
    -1022 => InvalidSignature,
    -1033 => CompIdInUse,
    -1034 => TooManyConnections,
    -1035 => LoggedOut,
);

impl ServerError {
    /**
     * Returns whether this error indicates a temporary server issue.
     */
    pub fn is_temporary(&self) -> bool {
        matches!(
            self,
            Self::Disconnected
                | Self::Timeout
                | Self::ServerBusy
                | Self::UnexpectedResponse
                | Self::TooManyRequests
                | Self::TooManyOrders
                | Self::TooManyConnections
        )
    }

    /**
     * Returns whether this error indicates an authentication problem.
     */
    pub fn is_auth_related(&self) -> bool {
        matches!(
            self,
            Self::Unauthorized | Self::InvalidTimestamp | Self::InvalidSignature
        )
    }

    /**
     * Returns suggested retry delay in seconds for temporary errors.
     */
    pub fn retry_delay_seconds(&self) -> Option<u64> {
        match self {
            Self::TooManyRequests => Some(60),   // Rate limited - wait longer
            Self::ServerBusy => Some(30),        // Server busy - moderate delay
            Self::Timeout => Some(10),           // Timeout - short delay
            Self::Disconnected => Some(5),       // Connection issue - quick retry
            Self::UnexpectedResponse => Some(5), // Unexpected response - quick retry
            _ => None,                           // Not retryable
        }
    }

    /**
     * Returns user-friendly error message.
     */
    pub fn user_message(&self) -> &'static str {
        match self {
            Self::Unknown => "An unknown server error occurred",
            Self::Disconnected => "Connection to server was lost",
            Self::Unauthorized => "Authentication failed - check API credentials",
            Self::TooManyRequests => "Too many requests - please reduce request frequency",
            Self::UnexpectedResponse => "Server returned unexpected response",
            Self::Timeout => "Request timed out - server may be busy",
            Self::ServerBusy => "Server is temporarily overloaded",
            Self::ErrorMsgReceived => "Request was rejected by the server",
            Self::InvalidMessage => "Request format was invalid",
            Self::UnknownOrderComposition => "Unsupported order combination",
            Self::TooManyOrders => "Too many orders - some orders may need to be canceled",
            Self::ServiceShuttingDown => "Service is temporarily unavailable",
            Self::UnsupportedOperation => "This operation is not supported",
            Self::InvalidTimestamp => "Request timestamp is invalid - check system clock",
            Self::InvalidSignature => "Request signature is invalid - check API secret",
            Self::CompIdInUse => "Connection ID is already in use",
            Self::TooManyConnections => "Too many concurrent connections",
            Self::LoggedOut => "Session has been logged out",
            Self::Other(_) => "Unknown server error occurred",
        }
    }
}
