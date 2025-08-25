use serde::{Deserialize, Serialize};

use crate::enums::{OrderType, Permission, SelfTradePreventionMode, SymbolStatus};
use crate::filters::{LotSizeFilter, MinNotionalFilter, PriceFilter, SymbolFilter};

/**
 * Symbol information from exchange.
 *
 * # Fields
 * - `symbol`: The trading symbol name (e.g., "BTCUSDT").
 * - `status`: Current trading status of the symbol.
 * - `base_asset`: The base asset of the trading pair.
 * - `base_asset_precision`: Precision for the base asset.
 * - `quote_asset`: The quote asset of the trading pair.
 * - `quote_precision`: Precision for quotes (deprecated, use quoteAssetPrecision).
 * - `quote_asset_precision`: Precision for the quote asset.
 * - `base_commission_precision`: Precision for base asset commissions.
 * - `quote_commission_precision`: Precision for quote asset commissions.
 * - `order_types`: Supported order types for this symbol.
 * - `iceberg_allowed`: Whether iceberg orders are allowed.
 * - `oco_allowed`: Whether OCO orders are allowed.
 * - `oto_allowed`: Whether OTO orders are allowed.
 * - `quote_order_qty_market_allowed`: Whether market orders can use quoteOrderQty.
 * - `allow_trailing_stop`: Whether trailing stop orders are allowed.
 * - `cancel_replace_allowed`: Whether cancel-replace is allowed.
 * - `amend_allowed`: Whether order amendment is allowed.
 * - `is_spot_trading_allowed`: Whether spot trading is allowed.
 * - `is_margin_trading_allowed`: Whether margin trading is allowed.
 * - `filters`: Trading filters applied to this symbol.
 * - `permissions`: Required permissions to trade this symbol.
 * - `permission_sets`: Permission sets for trading (optional).
 * - `default_self_trade_prevention_mode`: Default STP mode.
 * - `allowed_self_trade_prevention_modes`: Allowed STP modes.
 * - `peg_instructions_allowed`: Whether peg instructions are allowed (optional).
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SymbolInfo {
    pub symbol: String,
    pub status: SymbolStatus,
    pub base_asset: String,
    pub base_asset_precision: u8,
    pub quote_asset: String,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: u8,
    pub quote_asset_precision: u8,
    pub base_commission_precision: u8,
    pub quote_commission_precision: u8,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub oto_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub allow_trailing_stop: bool,
    pub cancel_replace_allowed: bool,
    pub amend_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<SymbolFilter>,
    pub permissions: Vec<Permission>,
    pub permission_sets: Option<Vec<Vec<Permission>>>,
    pub default_self_trade_prevention_mode: SelfTradePreventionMode,
    pub allowed_self_trade_prevention_modes: Vec<SelfTradePreventionMode>,
    pub peg_instructions_allowed: Option<bool>,
}

impl SymbolInfo {
    /**
     * Gets the PRICE_FILTER for this symbol, if it exists.
     *
     * # Returns
     * - `Option<&PriceFilter>`: The price filter if present.
     */
    pub fn price_filter(&self) -> Option<&PriceFilter> {
        self.filters.iter().find_map(|f| match f {
            SymbolFilter::PriceFilter(pf) => Some(pf),
            _ => None,
        })
    }

    /**
     * Gets the LOT_SIZE filter for this symbol, if it exists.
     *
     * # Returns
     * - `Option<&LotSizeFilter>`: The lot size filter if present.
     */
    pub fn lot_size_filter(&self) -> Option<&LotSizeFilter> {
        self.filters.iter().find_map(|f| match f {
            SymbolFilter::LotSize(lsf) => Some(lsf),
            _ => None,
        })
    }

    /**
     * Gets the MIN_NOTIONAL filter for this symbol, if it exists.
     *
     * # Returns
     * - `Option<&MinNotionalFilter>`: The min notional filter if present.
     */
    pub fn min_notional_filter(&self) -> Option<&MinNotionalFilter> {
        self.filters.iter().find_map(|f| match f {
            SymbolFilter::MinNotional(mnf) => Some(mnf),
            _ => None,
        })
    }
}
