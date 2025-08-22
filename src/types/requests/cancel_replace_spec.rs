use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;
use rust_decimal::Decimal;

use crate::Result;
use crate::{
    errors::InvalidParameter,
    enums::{
        OrderSide, 
        OrderType, 
        TimeInForce, 
        OrderResponseType, 
        SelfTradePreventionMode, 
        CancelRestrictions,
        CancelReplaceMode,
        OrderRateLimitExceededMode,
    },
    types::requests::{Unvalidated, Validated},
};

/**
 * Specification for cancel-replace order operations.
 *
 * Cancel-replace atomically cancels an existing order and places a new order.
 * This operation can have partial success (e.g., cancel succeeds but new order fails).
 * 
 * # Fields
 * - `symbol`: Trading symbol for the order.
 * - `cancel_replace_mode`: Mode for the cancel-replace operation.
 * - `cancel_order_id`: Optional ID of the order to cancel.
 * - `cancel_origin_client_order_id`: Optional original client order ID to cancel.
 * - `cancel_new_client_order_id`: Optional new client order ID for the canceled order.
 * - `cancel_restrictions`: Optional restrictions on the cancellation.
 * - `order_rate_limit_exceeded_mode`: Optional mode for handling order rate limit exceeded.
 * - `side`: Side of the new order (buy/sell).
 * - `order_type`: Type of the new order (e.g., limit, market).
 * - `time_in_force`: Optional time in force for the new order.
 * - `quantity`: Optional quantity for the new order.
 * - `quote_order_quantity`: Optional quote order quantity for the new order.
 * - `price`: Optional price for the new order.
 * - `new_client_order_id`: Optional new client order ID for the new order.
 * - `strategy_id`: Optional strategy ID for the new order.
 * - `strategy_type`: Optional strategy type for the new order. 
 * - `stop_price`: Optional stop price for the new order.
 * - `trailing_delta`: Optional trailing delta for the new order.
 * - `iceberg_quantity`: Optional iceberg quantity for the new order.
 * - `new_order_response_type`: Optional response type for the new order.
 * - `self_trade_prevention_mode`: Optional self-trade prevention mode for the new order.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceSpec<S=Unvalidated> {
    // Cancel-specific fields
    pub symbol: String,
    pub cancel_replace_mode: CancelReplaceMode,
    pub cancel_order_id: Option<u64>,
    #[serde(rename = "cancelOrigClientOrderId")]
    pub cancel_origin_client_order_id: Option<String>,
    pub cancel_new_client_order_id: Option<String>,
    pub cancel_restrictions: Option<CancelRestrictions>,
    pub order_rate_limit_exceeded_mode: Option<OrderRateLimitExceededMode>,
    
    // New order fields
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(rename = "quoteOrderQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_quantity: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u32>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<Decimal>,
    #[serde(rename = "icebergQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_quantity: Option<Decimal>,
    #[serde(rename = "newOrderRespType")]
    pub new_order_response_type: Option<OrderResponseType>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl CancelReplaceSpec<Unvalidated> {
    /**
     * Creates a new cancel-replace specification with required parameters.
     * 
     * # Arguments
     * - `symbol`: Trading symbol for the order.
     * - `cancel_replace_mode`: Mode for the cancel-replace operation.
     * - `side`: Side of the new order (buy/sell).
     * - `order_type`: Type of the new order (e.g., limit, market).
     * 
     * # Returns
     * - `Self`: New cancel-replace specification.
     */
    pub fn new(
        symbol: impl Into<String>,
        cancel_replace_mode: CancelReplaceMode,
        side: OrderSide,
        order_type: OrderType,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            cancel_replace_mode,
            cancel_order_id: None,
            cancel_origin_client_order_id: None,
            cancel_new_client_order_id: None,
            cancel_restrictions: None,
            order_rate_limit_exceeded_mode: None,
            side,
            order_type,
            time_in_force: None,
            quantity: None,
            quote_order_quantity: None,
            price: None,
            new_client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_quantity: None,
            new_order_response_type: None,
            self_trade_prevention_mode: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the order ID to cancel.
     * 
     * # Arguments
     * - `order_id`: The ID of the order to cancel.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_cancel_order_id(mut self, order_id: u64) -> Self {
        self.cancel_order_id = Some(order_id);
        self
    }

    /**
     * Sets the client order ID to cancel.
     * 
     * # Arguments
     * - `original_id`: The original client order ID to cancel.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_cancel_original_client_order_id(mut self, original_id: impl Into<String>) -> Self {
        self.cancel_origin_client_order_id = Some(original_id.into());
        self
    }

    /**
     * Sets the new client order ID for the canceled order.
     * 
     * # Arguments
     * - `new_id`: The new client order ID to assign to the canceled order
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_cancel_new_client_order_id(mut self, new_id: impl Into<String>) -> Self {
        self.cancel_new_client_order_id = Some(new_id.into());
        self
    }

    /**
     * Sets cancel restrictions.
     * 
     * # Arguments
     * - `restrictions`: Restrictions on the cancellation based on order status:
     *      * ONLY_NEW: only cancel if status is NEW, 
     *      * ONLY_PARTIALLY_FILLED: only cancel if partially filled.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_cancel_restrictions(mut self, restrictions: CancelRestrictions) -> Self {
        self.cancel_restrictions = Some(restrictions);
        self
    }

    /**
     * Sets the order rate limit exceeded mode.
     * 
     * # Arguments
     * - `mode`: Mode for handling order rate limit exceeded (e.g., retry,
     *   fail, ignore).
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_order_rate_limit_exceeded_mode(mut self, mode: OrderRateLimitExceededMode) -> Self {
        self.order_rate_limit_exceeded_mode = Some(mode);
        self
    }

    /**
     * Sets the time in force for the new order.
     * 
     * # Arguments
     * - `tif`: Time in force for the new order (e.g., GTC, IOC, FOK).
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /**
     * Sets the quantity for the new order.
     * 
     * # Arguments
     * - `quantity`: Quantity for the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_quantity(mut self, quantity: Decimal) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /**
     * Sets the quote order quantity for the new order.
     * 
     * # Arguments
     * - `quote_quantity`: Quote order quantity for the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_quote_order_quantity(mut self, quote_quantity: Decimal) -> Self {
        self.quote_order_quantity = Some(quote_quantity);
        self
    }

    /**
     * Sets the price for the new order.
     * 
     * # Arguments
     * - `price`: Price for the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    /**
     * Sets the client order ID for the new order.
     * 
     * # Arguments
     * - `new_id`: The new client order ID to assign to the new order
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_new_client_order_id(mut self, new_id: impl Into<String>) -> Self {
        self.new_client_order_id = Some(new_id.into());
        self
    }

    /**
     * Sets the strategy ID for the new order.
     * 
     * # Arguments
     * - `strategy_id`: The strategy ID to assign to the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_strategy_id(mut self, strategy_id: u64) -> Self {
        self.strategy_id = Some(strategy_id);
        self
    }

    /**
     * Sets the strategy type for the new order.
     * 
     * # Arguments
     * - `strategy_type`: The strategy type to assign to the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_strategy_type(mut self, strategy_type: u32) -> Self {
        self.strategy_type = Some(strategy_type);
        self
    }

    /**
     * Sets the stop price for the new order.
     * 
     * # Arguments
     * - `stop_price`: The stop price for the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_stop_price(mut self, stop_price: Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the trailing delta for the new order.
     * 
     * # Arguments
     * - `trailing_delta`: The trailing delta for the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_trailing_delta(mut self, trailing_delta: Decimal) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

    /**
     * Sets the iceberg quantity for the new order.
     * 
     * # Arguments
     * - `iceberg_quantity`: The iceberg quantity for the new order.
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_iceberg_quantity(mut self, iceberg_quantity: Decimal) -> Self {
        self.iceberg_quantity = Some(iceberg_quantity);
        self
    }

    /**
     * Sets the response type for the new order.
     * 
     * # Arguments
     * - `response_type`: The response type for the new order (e.g., ACK, RESULT, FULL).
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_new_order_response_type(mut self, response_type: OrderResponseType) -> Self {
        self.new_order_response_type = Some(response_type);
        self
    }

    /**
     * Sets the self-trade prevention mode for the new order.
     * 
     * # Arguments
     * - `stp_mode`: The self-trade prevention mode for the new order 
     *   (e.g., EXPIRE_TAKER, EXPIRE_MAKER, CANCEL_OLDEST).
     * 
     * # Returns
     * - `Self`: Updated specification.
     */
    pub fn with_self_trade_prevention_mode(mut self, stp_mode: SelfTradePreventionMode) -> Self {
        self.self_trade_prevention_mode = Some(stp_mode);
        self
    }

    /**
     * Builds the cancel-replace specification.
     * 
     * # Returns
     * - `CancelReplaceSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<CancelReplaceSpec<Validated>> {
        self.validate().context("Failed to validate CancelReplaceSpecification")?;

        Ok(CancelReplaceSpec {
            symbol: self.symbol,
            cancel_replace_mode: self.cancel_replace_mode,
            cancel_order_id: self.cancel_order_id,
            cancel_origin_client_order_id: self.cancel_origin_client_order_id,
            cancel_new_client_order_id: self.cancel_new_client_order_id,
            cancel_restrictions: self.cancel_restrictions,
            order_rate_limit_exceeded_mode: self.order_rate_limit_exceeded_mode,
            side: self.side,
            order_type: self.order_type,
            time_in_force: self.time_in_force,
            quantity: self.quantity,
            quote_order_quantity: self.quote_order_quantity,
            price: self.price,
            new_client_order_id: self.new_client_order_id,
            strategy_id: self.strategy_id,
            strategy_type: self.strategy_type,
            stop_price: self.stop_price,
            trailing_delta: self.trailing_delta,
            iceberg_quantity: self.iceberg_quantity,
            new_order_response_type: self.new_order_response_type,
            self_trade_prevention_mode: self.self_trade_prevention_mode,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the cancel-replace specification parameters.
     * 
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {

        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        match (self.cancel_order_id.is_some(), self.cancel_origin_client_order_id.is_some()) {
            (false, false) => {
                return Err(InvalidParameter::required("cancel_order_id or cancel_origin_client_order_id").into());
            }
            _ => {}
        }

        if let Some(strategy_type) = self.strategy_type {
            if strategy_type < 1000000 {
                return Err(InvalidParameter::range(
                    "strategy_type", 1000000, i32::MAX
                ).into());
            }
        }

        if let Some(qty) = self.quantity {
            if qty <= Decimal::ZERO {
                return Err(InvalidParameter::range(
                    "quantity", Decimal::ZERO, Decimal::MAX
                ).into());
            }
        }

        if let Some(quote_qty) = self.quote_order_quantity {
            if quote_qty <= Decimal::ZERO {
                return Err(InvalidParameter::range(
                    "quote_order_quantity", Decimal::ZERO, Decimal::MAX
                ).into());
            }
        }

        if let Some(price) = self.price {
            if price <= Decimal::ZERO {
                return Err(InvalidParameter::range(
                    "price", Decimal::ZERO, Decimal::MAX
                ).into());
            }
        }

        if let Some(stop_price) = self.stop_price {
            if stop_price <= Decimal::ZERO {
                return Err(InvalidParameter::range(
                    "stop_price", Decimal::ZERO, Decimal::MAX
                ).into());
            }
        }

        if let Some(trailing_delta) = self.trailing_delta {
            if trailing_delta <= Decimal::ZERO {
                return Err(InvalidParameter::range(
                    "trailing_delta", Decimal::ZERO, Decimal::MAX
                ).into());
            }
        }

        if let Some(iceberg_qty) = self.iceberg_quantity {
            if iceberg_qty <= Decimal::ZERO {
                return Err(InvalidParameter::range(
                    "iceberg_quantity", Decimal::ZERO, Decimal::MAX
                ).into());
            }
        }

        self.validate_new_order_parameters()?;

        Ok(())
    }

    /**
     * Validates new order parameters based on the order type.
     * 
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate_new_order_parameters(&self) -> Result<()> {
        use OrderType::*;

        match self.order_type {
            Limit => {
                if self.time_in_force.is_none() {
                    return Err(InvalidParameter::required("time_in_force").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::required("price").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::required("quantity").into());
                }
            }
            LimitMaker => {
                if self.price.is_none() {
                    return Err(InvalidParameter::required("price").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::required("quantity").into());
                }
            }
            Market => {
                if self.quantity.is_none() && self.quote_order_quantity.is_none() {
                    return Err(InvalidParameter::required("quantity or quote_order_quantity").into());
                }
                if self.quantity.is_some() && self.quote_order_quantity.is_some() {
                    return Err(InvalidParameter::mutually_exclusive(
                        "quantity", "quote_order_quantity"
                    ).into());
                }
            }
            StopLoss => {
                if self.quantity.is_none() {
                    return Err(InvalidParameter::required("quantity").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::required("stop_price or trailing_delta").into());
                }
            }
            StopLossLimit => {
                if self.time_in_force.is_none() {
                    return Err(InvalidParameter::required("time_in_force").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::required("price").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::required("quantity").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::required("stop_price or trailing_delta").into());
                }
            }
            TakeProfit => {
                if self.quantity.is_none() {
                    return Err(InvalidParameter::required("quantity").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::required("stop_price or trailing_delta").into());
                }
            }
            TakeProfitLimit => {
                if self.time_in_force.is_none() {
                    return Err(InvalidParameter::required("time_in_force").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::required("price").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::required("quantity").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::required("stop_price or trailing_delta").into());
                }
            }
            _ => {} 
        }

        Ok(())
    }
}