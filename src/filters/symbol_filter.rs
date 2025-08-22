use serde::{Deserialize, Serialize};

use super::{
    PriceFilter, 
    LotSizeFilter, 
    MinNotionalFilter, 
    PercentPriceFilter,
    PercentPriceBySideFilter, 
    NotionalFilter, 
    IcebergPartsFilter, 
    MarketLotSizeFilter, 
    MaxNumOrdersFilter, 
    MaxNumOrderAmendsFilter,
    MaxNumOrderListsFilter,
    MaxNumAlgoOrdersFilter, 
    MaxNumIcebergOrdersFilter, 
    MaxPositionFilter, 
    TrailingDeltaFilter
};

/**
 * Symbol-level filters.
 *
 * # Variants
 * Each variant wraps a struct containing the concrete rule fields.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "filterType")]
pub enum SymbolFilter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter(PriceFilter),
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice(PercentPriceFilter),
    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    PercentPriceBySide(PercentPriceBySideFilter),
    #[serde(rename = "LOT_SIZE")]
    LotSize(LotSizeFilter),
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional(MinNotionalFilter),
    #[serde(rename = "NOTIONAL")]
    Notional(NotionalFilter),
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts(IcebergPartsFilter),
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize(MarketLotSizeFilter),
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders(MaxNumOrdersFilter),
    #[serde(rename = "MAX_NUM_ORDER_AMENDS")]
    MaxNumOrderAmends(MaxNumOrderAmendsFilter),
    #[serde(rename = "MAX_NUM_ORDER_LISTS")]
    MaxNumOrderLists(MaxNumOrderListsFilter),
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders(MaxNumAlgoOrdersFilter),
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders(MaxNumIcebergOrdersFilter),
    #[serde(rename = "MAX_POSITION")]
    MaxPosition(MaxPositionFilter),
    #[serde(rename = "TRAILING_DELTA")]
    TrailingDelta(TrailingDeltaFilter),
}