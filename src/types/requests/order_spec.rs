use std::marker::PhantomData;

use anyhow::Context;
use serde::Serialize;

use crate::Result;
use crate::{
    enums::{OrderResponseType, OrderSide, OrderType, SelfTradePreventionMode, TimeInForce},
    errors::InvalidParameter,
    types::requests::{Unvalidated, Validated},
};

/**
 * Order specification builder for clean parameter specification.
 *
 * # Fields
 * - `symbol`: Trading symbol for the order (required).
 * - `side`: Order side - BUY or SELL (required).
 * - `order_type`: Type of order - LIMIT, MARKET, STOP_LOSS, etc. (required).
 * - `time_in_force`: Time in force - GTC, IOC, FOK (required for some order types).
 * - `quantity`: Order quantity in base asset (required for most order types).
 * - `quote_order_quantity`: Order quantity in quote asset (alternative to quantity for MARKET orders).
 * - `price`: Order price (required for LIMIT orders).
 * - `client_order_id`: Custom client order ID for tracking.
 * - `stop_price`: Stop price for stop orders.
 * - `trailing_delta`: Trailing delta for trailing stop orders.
 * - `iceberg_quantity`: Visible quantity for iceberg orders.
 * - `response_type`: Response format - ACK, RESULT, or FULL.
 * - `strategy_id`: Strategy ID for order labeling.
 * - `strategy_type`: Strategy type (must be >= 1000000).
 * - `self_trade_prevention_mode`: Self-trade prevention mode.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderSpec<S = Unvalidated> {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub time_in_force: Option<TimeInForce>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<rust_decimal::Decimal>,
    #[serde(rename = "quoteOrderQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_order_quantity: Option<rust_decimal::Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<rust_decimal::Decimal>,
    #[serde(rename = "newClientOrderId")]
    pub client_order_id: Option<String>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<rust_decimal::Decimal>,
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_delta: Option<rust_decimal::Decimal>,
    #[serde(rename = "icebergQty")]
    #[serde(with = "rust_decimal::serde::str_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(rename = "newOrderRespType")]
    pub response_type: Option<OrderResponseType>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u32>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OrderSpec<Unvalidated> {
    /**
     * Creates a new order specification with required parameters.
     *
     * # Arguments
     * - `symbol`: Trading symbol for the order.
     * - `side`: Order side (buy/sell).
     * - `order_type`: Type of order (limit, market, etc.).
     *
     * # Returns
     * - `Self`: New order specification.
     */
    pub fn new(symbol: impl Into<String>, side: OrderSide, order_type: OrderType) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            order_type,
            time_in_force: None,
            quantity: None,
            quote_order_quantity: None,
            price: None,
            client_order_id: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_quantity: None,
            response_type: None,
            strategy_id: None,
            strategy_type: None,
            self_trade_prevention_mode: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the time in force for the order.
     *
     * # Arguments
     * - `time_in_force`: Time in force for the order (GTC, IOC, FOK).
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    /**
     * Sets the order quantity.
     *
     * # Arguments
     * - `quantity`: Quantity of the asset to order.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_quantity(mut self, quantity: rust_decimal::Decimal) -> Self {
        self.quantity = Some(quantity);
        self
    }

    /**
     * Sets the quote order quantity (alternative to quantity for market orders).
     *
     * # Arguments
     * - `quote_quantity`: Quote quantity for the order.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_quote_order_quantity(mut self, quote_quantity: rust_decimal::Decimal) -> Self {
        self.quote_order_quantity = Some(quote_quantity);
        self
    }

    /**
     * Sets the order price.
     *
     * # Arguments
     * - `price`: Price for limit orders.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_price(mut self, price: rust_decimal::Decimal) -> Self {
        self.price = Some(price);
        self
    }

    /**
     * Sets the client order ID.
     *
     * # Arguments
     * - `client_id`: Custom client order ID.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_client_order_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_id.into());
        self
    }

    /**
     * Sets the stop price for stop orders.
     *
     * # Arguments
     * - `stop_price`: Stop price for stop loss/take profit orders.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_stop_price(mut self, stop_price: rust_decimal::Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the trailing delta for trailing stop orders.
     *
     * # Arguments
     * - `trailing_delta`: Trailing delta for trailing stop orders.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_trailing_delta(mut self, trailing_delta: rust_decimal::Decimal) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

    /**
     * Sets the iceberg quantity for iceberg orders.
     *
     * # Arguments
     * - `iceberg_quantity`: Quantity for iceberg orders.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_iceberg_quantity(mut self, iceberg_quantity: rust_decimal::Decimal) -> Self {
        self.iceberg_quantity = Some(iceberg_quantity);
        self
    }

    /**
     * Sets the response type.
     *
     * # Arguments
     * - `response_type`: Response type for the order (ACK, RESULT, FULL).
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_response_type(mut self, response_type: OrderResponseType) -> Self {
        self.response_type = Some(response_type);
        self
    }

    /**
     * Sets the strategy ID.
     *
     * # Arguments
     * - `strategy_id`: ID of the strategy to associate with the order.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_strategy_id(mut self, strategy_id: u64) -> Self {
        self.strategy_id = Some(strategy_id);
        self
    }

    /**
     * Sets the strategy type.
     *
     * # Arguments
     * - `strategy_type`: Type of the strategy (must be >= 1000000).
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_strategy_type(mut self, strategy_type: u32) -> Self {
        self.strategy_type = Some(strategy_type);
        self
    }

    /**
     * Sets the self-trade prevention mode.
     *
     * # Arguments
     * - `stp_mode`: Self-trade prevention mode to use.
     *
     * # Returns
     * - `Self`: Updated order specification.
     */
    pub fn with_self_trade_prevention_mode(mut self, stp_mode: SelfTradePreventionMode) -> Self {
        self.self_trade_prevention_mode = Some(stp_mode);
        self
    }

    /**
     * Builds the order specification.
     *
     * # Returns
     * - `OrderSpecification<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OrderSpec<Validated>> {
        self.validate()
            .context("Failed to validate OrderSpecification")?;

        Ok(OrderSpec {
            symbol: self.symbol,
            side: self.side,
            order_type: self.order_type,
            time_in_force: self.time_in_force,
            quantity: self.quantity,
            quote_order_quantity: self.quote_order_quantity,
            price: self.price,
            client_order_id: self.client_order_id,
            stop_price: self.stop_price,
            trailing_delta: self.trailing_delta,
            iceberg_quantity: self.iceberg_quantity,
            response_type: self.response_type,
            strategy_id: self.strategy_id,
            strategy_type: self.strategy_type,
            self_trade_prevention_mode: self.self_trade_prevention_mode,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the order parameters based on order type requirements.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.quantity.is_some() && self.quote_order_quantity.is_some() {
            return Err(
                InvalidParameter::mutually_exclusive("quantity", "quote_order_quantity").into(),
            );
        }

        if let Some(qty) = self.quantity
            && qty <= rust_decimal::Decimal::ZERO
        {
            return Err(InvalidParameter::new("quantity", "must be greater than 0").into());
        }

        if let Some(quote_qty) = self.quote_order_quantity
            && quote_qty <= rust_decimal::Decimal::ZERO
        {
            return Err(
                InvalidParameter::new("quote_order_quantity", "must be greater than 0").into(),
            );
        }

        if let Some(price) = self.price
            && price <= rust_decimal::Decimal::ZERO
        {
            return Err(InvalidParameter::new("price", "must be greater than 0").into());
        }

        if let Some(stop_price) = self.stop_price
            && stop_price <= rust_decimal::Decimal::ZERO
        {
            return Err(InvalidParameter::new("stop_price", "must be greater than 0").into());
        }

        if let Some(trailing_delta) = self.trailing_delta
            && trailing_delta <= rust_decimal::Decimal::ZERO
        {
            return Err(InvalidParameter::new("trailing_delta", "must be greater than 0").into());
        }

        if let Some(iceberg_qty) = self.iceberg_quantity
            && iceberg_qty <= rust_decimal::Decimal::ZERO
        {
            return Err(InvalidParameter::new("iceberg_quantity", "must be greater than 0").into());
        }

        if let Some(strat_type) = self.strategy_type
            && strat_type < 1000000
        {
            return Err(InvalidParameter::range("strategy_type", 1000000, u32::MAX).into());
        }

        if self.iceberg_quantity.is_some()
            && !matches!(self.time_in_force, Some(TimeInForce::GTC))
            && !matches!(self.order_type, OrderType::LimitMaker)
        {
            return Err(InvalidParameter::new(
                "iceberg_quantity",
                "requires time_in_force to be GTC or order_type to be LIMIT_MAKER",
            )
            .into());
        }

        match self.order_type {
            OrderType::Limit => {
                if self.time_in_force.is_none() {
                    return Err(InvalidParameter::empty("time_in_force").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::empty("price").into());
                }
            }
            OrderType::Market => {
                if self.quantity.is_none() && self.quote_order_quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity or quote_order_quantity").into());
                }
            }
            OrderType::StopLoss => {
                if self.quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::empty("stop_price or trailing_delta").into());
                }
            }
            OrderType::StopLossLimit => {
                if self.time_in_force.is_none() {
                    return Err(InvalidParameter::empty("time_in_force").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::empty("price").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::empty("stop_price or trailing_delta").into());
                }
            }
            OrderType::TakeProfit => {
                if self.quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::empty("stop_price or trailing_delta").into());
                }
            }
            OrderType::TakeProfitLimit => {
                if self.time_in_force.is_none() {
                    return Err(InvalidParameter::empty("time_in_force").into());
                }
                if self.quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::empty("price").into());
                }
                if self.stop_price.is_none() && self.trailing_delta.is_none() {
                    return Err(InvalidParameter::empty("stop_price or trailing_delta").into());
                }
            }
            OrderType::LimitMaker => {
                if self.quantity.is_none() {
                    return Err(InvalidParameter::empty("quantity").into());
                }
                if self.price.is_none() {
                    return Err(InvalidParameter::empty("price").into());
                }
            }
            OrderType::Unknown => {
                return Err(
                    InvalidParameter::new("order_type", "must be a valid order type").into(),
                );
            }
        }

        Ok(())
    }
}
