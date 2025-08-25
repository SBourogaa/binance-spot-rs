use phf::phf_map;
use std::hash::Hash;

/**
 * Filter validation failure types for order placement.
 *
 * # Variants
 * - Each variant corresponds to a specific filter failure message.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterFailure {
    PriceFilter,
    PercentPrice,
    PercentPriceBySide,
    LotSize,
    MinNotional,
    Notional,
    IcebergParts,
    MarketLotSize,
    MaxPosition,
    MaxNumOrders,
    MaxNumAlgoOrders,
    MaxNumIcebergOrders,
    TrailingDelta,
    ExchangeMaxNumOrders,
    ExchangeMaxNumAlgoOrders,
    ExchangeMaxNumIcebergOrders,
}

/**
 * Exact filter failure message patterns from Binance API.
 */
static FILTER_MAP: phf::Map<&'static str, FilterFailure> = phf_map! {
    "Filter failure: PRICE_FILTER" => FilterFailure::PriceFilter,
    "Filter failure: PERCENT_PRICE" => FilterFailure::PercentPrice,
    "Filter failure: PERCENT_PRICE_BY_SIDE" => FilterFailure::PercentPriceBySide,
    "Filter failure: LOT_SIZE" => FilterFailure::LotSize,
    "Filter failure: MIN_NOTIONAL" => FilterFailure::MinNotional,
    "Filter failure: NOTIONAL" => FilterFailure::Notional,
    "Filter failure: ICEBERG_PARTS" => FilterFailure::IcebergParts,
    "Filter failure: MARKET_LOT_SIZE" => FilterFailure::MarketLotSize,
    "Filter failure: MAX_POSITION" => FilterFailure::MaxPosition,
    "Filter failure: MAX_NUM_ORDERS" => FilterFailure::MaxNumOrders,
    "Filter failure: MAX_NUM_ALGO_ORDERS" => FilterFailure::MaxNumAlgoOrders,
    "Filter failure: MAX_NUM_ICEBERG_ORDERS" => FilterFailure::MaxNumIcebergOrders,
    "Filter failure: TRAILING_DELTA" => FilterFailure::TrailingDelta,
    "Filter failure: EXCHANGE_MAX_NUM_ORDERS" => FilterFailure::ExchangeMaxNumOrders,
    "Filter failure: EXCHANGE_MAX_NUM_ALGO_ORDERS" => FilterFailure::ExchangeMaxNumAlgoOrders,
    "Filter failure: EXCHANGE_MAX_NUM_ICEBERG_ORDERS" => FilterFailure::ExchangeMaxNumIcebergOrders,
};

/**
 * Parses filter failure types from error messages.
 *
 * # Arguments
 * - `msg`: The error message to parse.
 *
 * # Returns
 * - `Option<FilterFailure>`: The detected filter failure type, if any.
 */
pub fn parse_filter_failure(msg: &str) -> Option<FilterFailure> {
    FILTER_MAP
        .entries()
        .find_map(|(k, v)| msg.contains(k).then_some(*v))
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Tests all documented filter failure messages.
     */
    #[test]
    fn test_all_documented_filter_failures() {
        let test_cases = [
            ("Filter failure: PRICE_FILTER", FilterFailure::PriceFilter),
            ("Filter failure: PERCENT_PRICE", FilterFailure::PercentPrice),
            ("Filter failure: LOT_SIZE", FilterFailure::LotSize),
            ("Filter failure: MIN_NOTIONAL", FilterFailure::MinNotional),
            ("Filter failure: NOTIONAL", FilterFailure::Notional),
            ("Filter failure: ICEBERG_PARTS", FilterFailure::IcebergParts),
            (
                "Filter failure: MARKET_LOT_SIZE",
                FilterFailure::MarketLotSize,
            ),
            ("Filter failure: MAX_POSITION", FilterFailure::MaxPosition),
            (
                "Filter failure: MAX_NUM_ORDERS",
                FilterFailure::MaxNumOrders,
            ),
            (
                "Filter failure: MAX_NUM_ALGO_ORDERS",
                FilterFailure::MaxNumAlgoOrders,
            ),
            (
                "Filter failure: MAX_NUM_ICEBERG_ORDERS",
                FilterFailure::MaxNumIcebergOrders,
            ),
            (
                "Filter failure: TRAILING_DELTA",
                FilterFailure::TrailingDelta,
            ),
            (
                "Filter failure: EXCHANGE_MAX_NUM_ORDERS",
                FilterFailure::ExchangeMaxNumOrders,
            ),
            (
                "Filter failure: EXCHANGE_MAX_NUM_ALGO_ORDERS",
                FilterFailure::ExchangeMaxNumAlgoOrders,
            ),
            (
                "Filter failure: EXCHANGE_MAX_NUM_ICEBERG_ORDERS",
                FilterFailure::ExchangeMaxNumIcebergOrders,
            ),
        ];

        for (message, expected) in test_cases {
            let result = parse_filter_failure(message);
            assert_eq!(result, Some(expected), "Failed for: {}", message);
        }
    }

    /**
     * Tests that unknown strings return None.
     */
    #[test]
    fn test_unknown_strings_return_none() {
        let result = parse_filter_failure("irrelevant");
        assert!(result.is_none());
    }

    /**
     * Tests filter failures embedded in larger error messages.
     */
    #[test]
    fn test_embedded_filter_failures() {
        let error_message = "Order rejected: Filter failure: LOT_SIZE";
        let result = parse_filter_failure(error_message);
        assert_eq!(result, Some(FilterFailure::LotSize));
    }
}
