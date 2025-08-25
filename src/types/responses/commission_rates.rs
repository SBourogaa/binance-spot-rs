use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/**
 * Commission rates for trading operations.
 *
 * # Fields
 * - `maker`: Maker commission rate.
 * - `taker`: Taker commission rate.
 * - `buyer`: Buyer commission rate (optional).
 * - `seller`: Seller commission rate (optional).
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommissionRates {
    #[serde(with = "rust_decimal::serde::str")]
    pub maker: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub taker: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub buyer: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub seller: Decimal,
}
