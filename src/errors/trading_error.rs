use super::error_categories::impl_from_code;

impl_from_code!(TradingError,
    // Trading-specific rejection errors (20xx series)
    // These errors occur when a trading operation is rejected by the Binance Matching Engine.
    
    -2010 => NewOrderRejected,
    -2011 => CancelRejected,
    -2013 => NoSuchOrder,
    -2014 => BadApiKeyFormat,
    -2015 => RejectedMbxKey,
    -2016 => NoTradingWindow,
    -2021 => CancelReplacePartiallyFailed,
    -2022 => CancelReplaceFailed,
    -2026 => OrderArchived,
    -2035 => SubscriptionActive,
    -2036 => SubscriptionInactive,
    -2038 => OrderAmendRejected,
    -2039 => ClientOrderIdInvalid,
);

impl TradingError {
    /**
     * Returns whether this error is related to order placement.
     */
    pub fn is_order_placement(&self) -> bool {
        matches!(self,
            Self::NewOrderRejected
        )
    }

    /**
     * Returns whether this error is related to order cancellation.
     */
    pub fn is_order_cancellation(&self) -> bool {
        matches!(self,
            Self::CancelRejected |
            Self::CancelReplacePartiallyFailed |
            Self::CancelReplaceFailed
        )
    }

    /**
     * Returns whether this error is related to order modification.
     */
    pub fn is_order_modification(&self) -> bool {
        matches!(self,
            Self::OrderAmendRejected |
            Self::CancelReplacePartiallyFailed |
            Self::CancelReplaceFailed
        )
    }

    /**
     * Returns whether this error is related to authentication/authorization.
     */
    pub fn is_auth_related(&self) -> bool {
        matches!(self,
            Self::BadApiKeyFormat |
            Self::RejectedMbxKey
        )
    }

    /**
     * Returns whether this error indicates the order was not found.
     */
    pub fn is_order_not_found(&self) -> bool {
        matches!(self,
            Self::NoSuchOrder |
            Self::OrderArchived |
            Self::ClientOrderIdInvalid
        )
    }

    /**
     * Returns whether this error is related to data streams.
     */
    pub fn is_stream_related(&self) -> bool {
        matches!(self,
            Self::SubscriptionActive |
            Self::SubscriptionInactive
        )
    }

    /**
     * Returns user-friendly error message with guidance.
     */
    pub fn user_message(&self) -> &'static str {
        match self {
            Self::NewOrderRejected => "Order was rejected - check order parameters and account status",
            Self::CancelRejected => "Order cancellation was rejected - order may already be filled or canceled",
            Self::NoSuchOrder => "Order not found - it may have been filled, canceled, or expired",
            Self::BadApiKeyFormat => "API key format is invalid",
            Self::RejectedMbxKey => "API key is invalid or lacks required permissions",
            Self::NoTradingWindow => "No trading window available for this symbol",
            Self::CancelReplacePartiallyFailed => "Cancel-replace partially failed - check order status",
            Self::CancelReplaceFailed => "Cancel-replace failed completely",
            Self::OrderArchived => "Order is too old and has been archived",
            Self::SubscriptionActive => "Data stream subscription is already active",
            Self::SubscriptionInactive => "Data stream subscription is not active",
            Self::OrderAmendRejected => "Order amendment was rejected",
            Self::ClientOrderIdInvalid => "Client order ID does not match this order",
            Self::Other(_) => "Trading operation failed",
        }
    }

    /**
     * Returns suggested action for resolving this error.
     */
    pub fn suggested_action(&self) -> &'static str {
        match self {
            Self::NewOrderRejected => "Review order parameters and try again",
            Self::CancelRejected => "Check order status before attempting to cancel",
            Self::NoSuchOrder => "Verify order ID and check order history",
            Self::BadApiKeyFormat => "Check API key format and regenerate if necessary",
            Self::RejectedMbxKey => "Verify API key permissions and IP restrictions",
            Self::NoTradingWindow => "Use 24hr ticker endpoint instead",
            Self::CancelReplacePartiallyFailed => "Check which operation failed and retry if needed",
            Self::CancelReplaceFailed => "Retry with corrected parameters",
            Self::OrderArchived => "Order is too old to query - check order history",
            Self::SubscriptionActive => "Use existing subscription or close before creating new one",
            Self::SubscriptionInactive => "Start subscription before using data stream",
            Self::OrderAmendRejected => "Check amendment parameters and symbol support",
            Self::ClientOrderIdInvalid => "Use correct client order ID for this order",
            Self::Other(_) => "Check Binance API documentation for this error code",
        }
    }
}