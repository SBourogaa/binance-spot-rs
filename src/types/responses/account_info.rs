use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::responses::{Balance, CommissionRates};

/**
 * Account information response from Binance API.
 *
 * # Fields
 * - `maker_commission`: Maker commission (raw value, deprecated - use commission_rates instead).
 * - `taker_commission`: Taker commission (raw value, deprecated - use commission_rates instead).
 * - `buyer_commission`: Buyer commission (raw value, deprecated - use commission_rates instead).
 * - `seller_commission`: Seller commission (raw value, deprecated - use commission_rates instead).
 * - `commission_rates`: Current commission rates structure.
 * - `can_trade`: Whether account can trade.
 * - `can_withdraw`: Whether account can withdraw.
 * - `can_deposit`: Whether account can deposit.
 * - `brokered`: Whether account is brokered.
 * - `require_self_trade_prevention`: Whether STP is required.
 * - `prevent_sor`: Whether SOR is prevented.
 * - `update_time`: Last account update timestamp.
 * - `account_type`: Type of account.
 * - `balances`: List of asset balances.
 * - `permissions`: Account permissions.
 * - `uid`: Unique account identifier.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AccountInfo {
    pub maker_commission: i32,
    pub taker_commission: i32,
    pub buyer_commission: i32,
    pub seller_commission: i32,
    pub commission_rates: CommissionRates,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub require_self_trade_prevention: bool,
    pub prevent_sor: bool,
    pub update_time: u64,
    pub account_type: String,
    pub balances: Vec<Balance>,
    pub permissions: Vec<String>,
    pub uid: u64,
}

impl AccountInfo {
    /**
     * Gets total balance for a specific asset.
     *
     * # Arguments
     * - `asset`: Asset symbol to query.
     *
     * # Returns
     * - `Option<Decimal>`: Total balance (free + locked) or None if asset not found.
     */
    pub fn total_balance(&self, asset: &str) -> Option<Decimal> {
        self.balances
            .iter()
            .find(|b| b.asset == asset)
            .map(|b| b.free + b.locked)
    }

    /**
     * Gets free balance for a specific asset.
     *
     * # Arguments
     * - `asset`: Asset symbol to query.
     *
     * # Returns
     * - `Option<Decimal>`: Free balance or None if asset not found.
     */
    pub fn free_balance(&self, asset: &str) -> Option<Decimal> {
        self.balances
            .iter()
            .find(|b| b.asset == asset)
            .map(|b| b.free)
    }

    /**
     * Gets locked balance for a specific asset.
     *
     * # Arguments
     * - `asset`: Asset symbol to query.
     *
     * # Returns
     * - `Option<Decimal>`: Locked balance or None if asset not found.
     */
    pub fn locked_balance(&self, asset: &str) -> Option<Decimal> {
        self.balances
            .iter()
            .find(|b| b.asset == asset)
            .map(|b| b.locked)
    }

    /**
     * Returns all assets with non-zero balances.
     *
     * # Returns
     * - `Vec<&Balance>`: Assets with positive total balance.
     */
    pub fn non_zero_balances(&self) -> Vec<&Balance> {
        self.balances
            .iter()
            .filter(|b| (b.free + b.locked) > Decimal::ZERO)
            .collect()
    }

    /**
     * Calculates total account value in a specific quote asset.
     *
     * # Arguments
     * - `quote_asset`: Asset to calculate total value in.
     * - `prices`: Map of asset prices in quote asset.
     *
     * # Returns
     * - `Decimal`: Total account value in quote asset.
     */
    pub fn total_value_in(&self, quote_asset: &str, prices: &HashMap<String, Decimal>) -> Decimal {
        self.balances
            .iter()
            .map(|balance| {
                let total_balance = balance.free + balance.locked;
                if balance.asset == quote_asset {
                    total_balance
                } else {
                    let price = prices.get(&balance.asset).copied().unwrap_or(Decimal::ZERO);
                    total_balance * price
                }
            })
            .sum()
    }
}
