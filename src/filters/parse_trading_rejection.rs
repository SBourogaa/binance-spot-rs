use phf::phf_map;
use std::hash::Hash;

/**
 * Specific trading rejection message types for detailed error handling.
 *
 * # Variants
 * - These correspond to specific rejection messages from the matching engine.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradingRejectionMessage {
    UnknownOrderSent,
    DuplicateOrderSent,
    PriceQtyTooLow,
    RequestWouldChangeNoState,
    MarketIsClosed,
    InsufficientBalance,
    ActionDisabledOnAccount,
    TradingDisabledOnAccount,
    SymbolNotPermittedForAccount,
    SymbolRestrictedForAccount,
    RestApiTradingNotEnabled,
    WebSocketApiTradingNotEnabled,
    FixApiTradingNotEnabled,
    MarketOrdersNotSupported,
    IcebergOrdersNotSupported,
    StopLossOrdersNotSupported,
    StopLossLimitOrdersNotSupported,
    TakeProfitOrdersNotSupported,
    TakeProfitLimitOrdersNotSupported,
    OcoOrdersNotSupported,
    QuoteOrderQtyNotSupported,
    TrailingStopOrdersNotSupported,
    UnsupportedOrderCombination,
    IcebergQtyExceedsQty,
    OrderWouldTriggerImmediately,
    OrderWouldImmediatelyMatch,
    IncorrectOcoPriceRelationship,
    OrderAmendNotSupported,
    OrderAmendQuantityIncreaseNotSupported,
    OrderCancelReplaceNotSupported,
    InvalidCancelOrderParams,
    OrderNotCanceledDueToCancelRestrictions,
    OrderBookLiquidityBelowLotSize,
    OrderBookLiquidityBelowMarketLotSize,
    OrderBookLiquidityBelowMinimum,
}

/**
 * Exact trading rejection message patterns from Binance API documentation.
 */
static REJECTION_MAP: phf::Map<&'static str, TradingRejectionMessage> = phf_map! {
    "Unknown order sent." => TradingRejectionMessage::UnknownOrderSent,
    "Duplicate order sent." => TradingRejectionMessage::DuplicateOrderSent,
    "Price * QTY is zero or less." => TradingRejectionMessage::PriceQtyTooLow,
    "The requested action would change no state; rejecting" => TradingRejectionMessage::RequestWouldChangeNoState,
    "Market is closed." => TradingRejectionMessage::MarketIsClosed,
    "Account has insufficient balance for requested action." => TradingRejectionMessage::InsufficientBalance,
    "This action is disabled on this account." => TradingRejectionMessage::ActionDisabledOnAccount,
    "This account may not place or cancel orders." => TradingRejectionMessage::TradingDisabledOnAccount,
    "This symbol is not permitted for this account." => TradingRejectionMessage::SymbolNotPermittedForAccount,
    "This symbol is restricted for this account." => TradingRejectionMessage::SymbolRestrictedForAccount,
    "Rest API trading is not enabled." => TradingRejectionMessage::RestApiTradingNotEnabled,
    "WebSocket API trading is not enabled." => TradingRejectionMessage::WebSocketApiTradingNotEnabled,
    "FIX API trading is not enabled." => TradingRejectionMessage::FixApiTradingNotEnabled,
    "Market orders are not supported for this symbol." => TradingRejectionMessage::MarketOrdersNotSupported,
    "Iceberg orders are not supported for this symbol." => TradingRejectionMessage::IcebergOrdersNotSupported,
    "Stop loss orders are not supported for this symbol." => TradingRejectionMessage::StopLossOrdersNotSupported,
    "Stop loss limit orders are not supported for this symbol." => TradingRejectionMessage::StopLossLimitOrdersNotSupported,
    "Take profit orders are not supported for this symbol." => TradingRejectionMessage::TakeProfitOrdersNotSupported,
    "Take profit limit orders are not supported for this symbol." => TradingRejectionMessage::TakeProfitLimitOrdersNotSupported,
    "OCO orders are not supported for this symbol" => TradingRejectionMessage::OcoOrdersNotSupported,
    "Quote order qty market orders are not support for this symbol." => TradingRejectionMessage::QuoteOrderQtyNotSupported,
    "Trailing stop orders are not supported for this symbol." => TradingRejectionMessage::TrailingStopOrdersNotSupported,
    "Unsupported order combination" => TradingRejectionMessage::UnsupportedOrderCombination,
    "IcebergQty exceeds QTY." => TradingRejectionMessage::IcebergQtyExceedsQty,
    "Order would trigger immediately." => TradingRejectionMessage::OrderWouldTriggerImmediately,
    "Order would immediately match and take." => TradingRejectionMessage::OrderWouldImmediatelyMatch,
    "The relationship of the prices for the orders is not correct." => TradingRejectionMessage::IncorrectOcoPriceRelationship,
    "Order amend is not supported for this symbol." => TradingRejectionMessage::OrderAmendNotSupported,
    "Order amend (quantity increase) is not supported." => TradingRejectionMessage::OrderAmendQuantityIncreaseNotSupported,
    "Order cancel-replace is not supported for this symbol." => TradingRejectionMessage::OrderCancelReplaceNotSupported,
    "Cancel order is invalid. Check origClOrdId and orderId." => TradingRejectionMessage::InvalidCancelOrderParams,
    "Order was not canceled due to cancel restrictions." => TradingRejectionMessage::OrderNotCanceledDueToCancelRestrictions,
    "Order book liquidity is less than LOT_SIZE filter minimum quantity." => TradingRejectionMessage::OrderBookLiquidityBelowLotSize,
    "Order book liquidity is less than MARKET_LOT_SIZE filter minimum quantity." => TradingRejectionMessage::OrderBookLiquidityBelowMarketLotSize,
    "Order book liquidity is less than symbol minimum quantity." => TradingRejectionMessage::OrderBookLiquidityBelowMinimum,
};

/**
 * Parses trading rejection message types from error messages.
 *
 * # Arguments
 * - `msg`: The error message to parse.
 *
 * # Returns
 * - `Option<TradingRejectionMessage>`: The detected rejection type, if any.
 */
pub fn parse_trading_rejection(msg: &str) -> Option<TradingRejectionMessage> {
    REJECTION_MAP
        .entries()
        .find_map(|(k, v)| msg.contains(k).then_some(*v))
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Tests all documented trading rejection messages.
     */
    #[test]
    fn test_all_documented_trading_rejections() {
        let test_cases = [
            (
                "Unknown order sent.",
                TradingRejectionMessage::UnknownOrderSent,
            ),
            (
                "Duplicate order sent.",
                TradingRejectionMessage::DuplicateOrderSent,
            ),
            (
                "Price * QTY is zero or less.",
                TradingRejectionMessage::PriceQtyTooLow,
            ),
            ("Market is closed.", TradingRejectionMessage::MarketIsClosed),
            (
                "Account has insufficient balance for requested action.",
                TradingRejectionMessage::InsufficientBalance,
            ),
            (
                "This action is disabled on this account.",
                TradingRejectionMessage::ActionDisabledOnAccount,
            ),
            (
                "This account may not place or cancel orders.",
                TradingRejectionMessage::TradingDisabledOnAccount,
            ),
            (
                "Market orders are not supported for this symbol.",
                TradingRejectionMessage::MarketOrdersNotSupported,
            ),
            (
                "Iceberg orders are not supported for this symbol.",
                TradingRejectionMessage::IcebergOrdersNotSupported,
            ),
            (
                "Stop loss orders are not supported for this symbol.",
                TradingRejectionMessage::StopLossOrdersNotSupported,
            ),
            (
                "Take profit orders are not supported for this symbol.",
                TradingRejectionMessage::TakeProfitOrdersNotSupported,
            ),
            (
                "OCO orders are not supported for this symbol",
                TradingRejectionMessage::OcoOrdersNotSupported,
            ),
            (
                "Unsupported order combination",
                TradingRejectionMessage::UnsupportedOrderCombination,
            ),
            (
                "IcebergQty exceeds QTY.",
                TradingRejectionMessage::IcebergQtyExceedsQty,
            ),
            (
                "Order would trigger immediately.",
                TradingRejectionMessage::OrderWouldTriggerImmediately,
            ),
            (
                "Order would immediately match and take.",
                TradingRejectionMessage::OrderWouldImmediatelyMatch,
            ),
            (
                "The relationship of the prices for the orders is not correct.",
                TradingRejectionMessage::IncorrectOcoPriceRelationship,
            ),
            (
                "Order amend is not supported for this symbol.",
                TradingRejectionMessage::OrderAmendNotSupported,
            ),
            (
                "Order cancel-replace is not supported for this symbol.",
                TradingRejectionMessage::OrderCancelReplaceNotSupported,
            ),
            (
                "This symbol is not permitted for this account.",
                TradingRejectionMessage::SymbolNotPermittedForAccount,
            ),
            (
                "This symbol is restricted for this account.",
                TradingRejectionMessage::SymbolRestrictedForAccount,
            ),
            (
                "Order was not canceled due to cancel restrictions.",
                TradingRejectionMessage::OrderNotCanceledDueToCancelRestrictions,
            ),
            (
                "Rest API trading is not enabled.",
                TradingRejectionMessage::RestApiTradingNotEnabled,
            ),
            (
                "WebSocket API trading is not enabled.",
                TradingRejectionMessage::WebSocketApiTradingNotEnabled,
            ),
            (
                "FIX API trading is not enabled.",
                TradingRejectionMessage::FixApiTradingNotEnabled,
            ),
            (
                "Order book liquidity is less than LOT_SIZE filter minimum quantity.",
                TradingRejectionMessage::OrderBookLiquidityBelowLotSize,
            ),
            (
                "Order book liquidity is less than MARKET_LOT_SIZE filter minimum quantity.",
                TradingRejectionMessage::OrderBookLiquidityBelowMarketLotSize,
            ),
            (
                "Order book liquidity is less than symbol minimum quantity.",
                TradingRejectionMessage::OrderBookLiquidityBelowMinimum,
            ),
            (
                "Order amend (quantity increase) is not supported.",
                TradingRejectionMessage::OrderAmendQuantityIncreaseNotSupported,
            ),
            (
                "The requested action would change no state; rejecting",
                TradingRejectionMessage::RequestWouldChangeNoState,
            ),
        ];

        for (message, expected) in test_cases {
            let result = parse_trading_rejection(message);
            assert_eq!(result, Some(expected), "Failed for: {}", message);
        }
    }

    /**
     * Tests that unknown strings return None.
     */
    #[test]
    fn test_unknown_strings_return_none() {
        let result = parse_trading_rejection("irrelevant");
        assert!(result.is_none());
    }

    /**
     * Tests trading rejections embedded in larger error messages.
     */
    #[test]
    fn test_embedded_trading_rejections() {
        let error_message = "Order placement failed: Market is closed.";
        let result = parse_trading_rejection(error_message);
        assert_eq!(result, Some(TradingRejectionMessage::MarketIsClosed));
    }
}
