use serde::{Deserialize, Serialize};

use crate::filters::ExchangeFilter;
use crate::enums::{RateLimitType, SymbolStatus, Permission};
use crate::types::responses::{RateLimit, SymbolInfo};

/**
 * Smart Order Routing (SOR) information for a base asset.
 *
 * Contains information about which symbols support SOR for a specific base asset.
 * This data is only present when SOR is available on the exchange.
 *
 * # Fields
 * - `base_asset`: The base asset that supports SOR (e.g., "BTC").
 * - `symbols`: List of trading symbols that support SOR for this base asset.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SorInfo {
    pub base_asset: String,
    pub symbols: Vec<String>,
}

/**
 * Exchange information response from Binance API.
 *
 * Contains comprehensive trading rules, rate limits, and symbol information.
 *
 * # Fields
 * - `timezone`: Exchange timezone (typically "UTC").
 * - `server_time`: Current server timestamp in milliseconds.
 * - `rate_limits`: Global rate limits for the exchange.
 * - `exchange_filters`: Global exchange filters.
 * - `symbols`: List of all available trading symbols and their rules.
 * - `sors`: Smart Order Routing information (when available).
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<SymbolInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sors: Vec<SorInfo>,
}

impl ExchangeInfo {
    /**
     * Finds symbol information by symbol name.
     * 
     * # Arguments
     * - `symbol_name`: The symbol to search for (e.g., "BTCUSDT").
     * 
     * # Returns
     * - `Option<&SymbolInfo>`: Symbol information if found.
     */
    pub fn find_symbol(&self, symbol_name: &str) -> Option<&SymbolInfo> {
        self.symbols.iter().find(|s| s.symbol == symbol_name)
    }

    /**
     * Gets all symbols with a specific trading status.
     * 
     * # Arguments
     * - `status`: The status to filter by.
     * 
     * # Returns
     * - `Vec<&SymbolInfo>`: All symbols with the specified status.
     */
    pub fn symbols_with_status(&self, status: SymbolStatus) -> Vec<&SymbolInfo> {
        self.symbols.iter().filter(|s| s.status == status).collect()
    }

    /**
     * Gets all symbols that require specific permissions.
     * 
     * # Arguments
     * - `required_permission`: The permission to filter by.
     * 
     * # Returns
     * - `Vec<&SymbolInfo>`: All symbols requiring the specified permission.
     */
    pub fn symbols_with_permission(&self, required_permission: Permission) -> Vec<&SymbolInfo> {
        self.symbols.iter()
            .filter(|s| s.permissions.contains(&required_permission))
            .collect()
    }

    /**
     * Gets rate limit information by type.
     * 
     * # Arguments
     * - `limit_type`: The rate limit type to search for.
     * 
     * # Returns
     * - `Option<&RateLimit>`: Rate limit information if found.
     */
    pub fn get_rate_limit(&self, limit_type: RateLimitType) -> Option<&RateLimit> {
        self.rate_limits.iter().find(|rl| rl.rate_limit_type == limit_type)
    }

    /**
     * Gets exchange filter by type.
     * 
     * # Arguments
     * - `filter_type`: A function to match the desired filter type.
     * 
     * # Returns
     * - `Option<&ExchangeFilter>`: Exchange filter if found.
     */
    pub fn get_exchange_filter<F>(&self, filter_type: F) -> Option<&ExchangeFilter>
    where
        F: Fn(&ExchangeFilter) -> bool,
    {
        self.exchange_filters.iter().find(|&f| filter_type(f))
    }

    /**
     * Gets the maximum number of orders allowed on the exchange.
     * 
     * # Returns
     * - `Option<u32>`: Maximum orders if the filter exists.
     */
    pub fn max_orders_limit(&self) -> Option<u32> {
        self.get_exchange_filter(|f| matches!(f, ExchangeFilter::ExchangeMaxNumOrders(_)))
            .and_then(|f| match f {
                ExchangeFilter::ExchangeMaxNumOrders(filter) => Some(filter.max_num_orders),
                _ => None,
            })
    }
}