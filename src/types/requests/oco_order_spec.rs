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
 * OCO (One-Cancels-the-Other) order specification.
 *
 * An OCO has 2 orders called the above order and below order.
 * - One of the orders must be a LIMIT_MAKER/TAKE_PROFIT/TAKE_PROFIT_LIMIT order
 * - The other must be STOP_LOSS or STOP_LOSS_LIMIT order.
 *
 * # Fields
 * - `symbol`: Trading symbol for the order.
 * - `list_client_order_id`: Optional arbitrary unique ID among open order lists.
 * - `side`: Order side - BUY or SELL.
 * - `quantity`: Quantity for both orders of the order list.
 * - `above_type`: Type of above order.
 * - `above_client_order_id`: Optional arbitrary unique ID for the above order.
 * - `above_iceberg_qty`: Optional iceberg quantity for the above order.
 * - `above_price`: Optional price for the above order.
 * - `above_stop_price`: Optional stop price for the above order.
 * - `above_trailing_delta`: Optional trailing delta for the above order.
 * - `above_time_in_force`: Optional time in force for the above order.
 * - `above_strategy_id`: Optional strategy ID for the above order.
 * - `above_strategy_type`: Optional strategy type for the above order.
 * - `below_type`: Type of below order (required).
 * - `below_client_order_id`: Optional arbitrary unique ID for the below order.
 * - `below_iceberg_qty`: Optional iceberg quantity for the below order.
 * - `below_price`: Optional price for the below order.
 * - `below_stop_price`: Optional stop price for the below order.
 * - `below_trailing_delta`: Optional trailing delta for the below order.
 * - `below_time_in_force`: Optional time in force for the below order.
 * - `below_strategy_id`: Optional strategy ID for the below order.
 * - `below_strategy_type`: Optional strategy type for the below order.
 * - `response_type`: Optional response format - ACK, RESULT, or FULL.
 * - `self_trade_prevention_mode`: Optional self-trade prevention mode.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderSpec<S = Unvalidated> {
    pub symbol: String,
    pub list_client_order_id: Option<String>,
    pub side: OrderSide,
    #[serde(with = "rust_decimal::serde::str")]
    pub quantity: rust_decimal::Decimal,

    // Above order fields
    pub above_type: OrderType,
    pub above_client_order_id: Option<String>,
    #[serde(
        rename = "aboveIcebergQty",
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_stop_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub above_trailing_delta: Option<rust_decimal::Decimal>,
    pub above_time_in_force: Option<TimeInForce>,
    pub above_strategy_id: Option<u64>,
    pub above_strategy_type: Option<u32>,

    // Below order fields
    pub below_type: OrderType,
    pub below_client_order_id: Option<String>,
    #[serde(
        rename = "belowIcebergQty",
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_stop_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub below_trailing_delta: Option<rust_decimal::Decimal>,
    pub below_time_in_force: Option<TimeInForce>,
    pub below_strategy_id: Option<u64>,
    pub below_strategy_type: Option<u32>,
    pub new_order_response_type: Option<OrderResponseType>,
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OcoOrderSpec<Unvalidated> {
    /**
     * Creates a new OCO order specification.
     *
     * # Arguments
     * - `symbol`: Symbol to trade
     * - `side`: Order side (BUY or SELL)
     * - `quantity`: Quantity for both orders
     * - `above_type`: Order type for the above order
     * - `below_type`: Order type for the below order
     *
     * # Returns
     * - `Self`: New OCO order specification
     */
    pub fn new(
        symbol: impl Into<String>,
        side: OrderSide,
        quantity: rust_decimal::Decimal,
        above_type: OrderType,
        below_type: OrderType,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            list_client_order_id: None,
            side,
            quantity,
            above_type,
            above_client_order_id: None,
            above_iceberg_quantity: None,
            above_price: None,
            above_stop_price: None,
            above_trailing_delta: None,
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_type,
            below_client_order_id: None,
            below_iceberg_quantity: None,
            below_price: None,
            below_stop_price: None,
            below_trailing_delta: None,
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_response_type: None,
            self_trade_prevention_mode: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the list client order ID.
     */
    pub fn with_list_client_order_id(mut self, id: impl Into<String>) -> Self {
        self.list_client_order_id = Some(id.into());
        self
    }

    /**
     * Sets the above order client ID.
     */
    pub fn with_above_client_order_id(mut self, id: impl Into<String>) -> Self {
        self.above_client_order_id = Some(id.into());
        self
    }

    /**
     * Sets the above order price.
     */
    pub fn with_above_price(mut self, price: rust_decimal::Decimal) -> Self {
        self.above_price = Some(price);
        self
    }

    /**
     * Sets the above order stop price.
     */
    pub fn with_above_stop_price(mut self, stop_price: rust_decimal::Decimal) -> Self {
        self.above_stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the above order time in force.
     */
    pub fn with_above_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.above_time_in_force = Some(tif);
        self
    }

    /**
     * Sets the below order client ID.
     */
    pub fn with_below_client_order_id(mut self, id: impl Into<String>) -> Self {
        self.below_client_order_id = Some(id.into());
        self
    }

    /**
     * Sets the below order price.
     */
    pub fn with_below_price(mut self, price: rust_decimal::Decimal) -> Self {
        self.below_price = Some(price);
        self
    }

    /**
     * Sets the below order stop price.
     */
    pub fn with_below_stop_price(mut self, stop_price: rust_decimal::Decimal) -> Self {
        self.below_stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the below order time in force.
     */
    pub fn with_below_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.below_time_in_force = Some(tif);
        self
    }

    /**
     * Sets the response type.
     */
    pub fn with_response_type(mut self, response_type: OrderResponseType) -> Self {
        self.new_order_response_type = Some(response_type);
        self
    }

    /**
     * Sets the self-trade prevention mode.
     */
    pub fn with_self_trade_prevention_mode(mut self, stp_mode: SelfTradePreventionMode) -> Self {
        self.self_trade_prevention_mode = Some(stp_mode);
        self
    }

    /**
     * Validates and builds the OCO order specification.
     *
     * # Returns
     * - `OcoOrderSpec<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OcoOrderSpec<Validated>> {
        self.validate().context("Failed to validate OcoOrderSpec")?;

        Ok(OcoOrderSpec::<Validated> {
            symbol: self.symbol,
            list_client_order_id: self.list_client_order_id,
            side: self.side,
            quantity: self.quantity,
            above_type: self.above_type,
            above_client_order_id: self.above_client_order_id,
            above_iceberg_quantity: self.above_iceberg_quantity,
            above_price: self.above_price,
            above_stop_price: self.above_stop_price,
            above_trailing_delta: self.above_trailing_delta,
            above_time_in_force: self.above_time_in_force,
            above_strategy_id: self.above_strategy_id,
            above_strategy_type: self.above_strategy_type,
            below_type: self.below_type,
            below_client_order_id: self.below_client_order_id,
            below_iceberg_quantity: self.below_iceberg_quantity,
            below_price: self.below_price,
            below_stop_price: self.below_stop_price,
            below_trailing_delta: self.below_trailing_delta,
            below_time_in_force: self.below_time_in_force,
            below_strategy_id: self.below_strategy_id,
            below_strategy_type: self.below_strategy_type,
            new_order_response_type: self.new_order_response_type,
            self_trade_prevention_mode: self.self_trade_prevention_mode,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the OCO order parameters according to Binance API requirements.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.quantity <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("quantity", "must be greater than 0").into());
        }

        match self.above_type {
            OrderType::StopLossLimit => {
                if self.above_time_in_force.is_none() {
                    return Err(InvalidParameter::new(
                        "above_time_in_force",
                        "required for STOP_LOSS_LIMIT",
                    )
                    .into());
                }
                if self.above_price.is_none() {
                    return Err(InvalidParameter::new(
                        "above_price",
                        "required for STOP_LOSS_LIMIT",
                    )
                    .into());
                }
            }
            OrderType::TakeProfitLimit => {
                if self.above_time_in_force.is_none() {
                    return Err(InvalidParameter::new(
                        "above_time_in_force",
                        "required for TAKE_PROFIT_LIMIT",
                    )
                    .into());
                }
                if self.above_price.is_none() {
                    return Err(InvalidParameter::new(
                        "above_price",
                        "required for TAKE_PROFIT_LIMIT",
                    )
                    .into());
                }
                if self.above_stop_price.is_none() && self.above_trailing_delta.is_none() {
                    return Err(InvalidParameter::new("above_stop_price or above_trailing_delta", "either above_stop_price or above_trailing_delta must be specified for TAKE_PROFIT_LIMIT").into());
                }
            }
            OrderType::LimitMaker => {
                if self.above_price.is_none() {
                    return Err(
                        InvalidParameter::new("above_price", "required for LIMIT_MAKER").into(),
                    );
                }
            }
            OrderType::TakeProfit => {
                if self.above_price.is_none() {
                    return Err(
                        InvalidParameter::new("above_price", "required for TAKE_PROFIT").into(),
                    );
                }
                if self.above_stop_price.is_none() && self.above_trailing_delta.is_none() {
                    return Err(InvalidParameter::new(
                        "above_stop_price or above_trailing_delta",
                        "either above_stop_price or above_trailing_delta must be specified",
                    )
                    .into());
                }
            }
            OrderType::StopLoss => {
                if self.above_stop_price.is_none() && self.above_trailing_delta.is_none() {
                    return Err(InvalidParameter::new(
                        "above_stop_price or above_trailing_delta",
                        "either above_stop_price or above_trailing_delta must be specified",
                    )
                    .into());
                }
            }
            _ => {
                return Err(InvalidParameter::new("above_type", "must be STOP_LOSS_LIMIT, STOP_LOSS, LIMIT_MAKER, TAKE_PROFIT, or TAKE_PROFIT_LIMIT").into());
            }
        }

        match self.below_type {
            OrderType::StopLossLimit => {
                if self.below_time_in_force.is_none() {
                    return Err(InvalidParameter::new(
                        "below_time_in_force",
                        "required for STOP_LOSS_LIMIT",
                    )
                    .into());
                }
                if self.below_price.is_none() {
                    return Err(InvalidParameter::new(
                        "below_price",
                        "required for STOP_LOSS_LIMIT",
                    )
                    .into());
                }
            }
            OrderType::TakeProfitLimit => {
                if self.below_time_in_force.is_none() {
                    return Err(InvalidParameter::new(
                        "below_time_in_force",
                        "required for TAKE_PROFIT_LIMIT",
                    )
                    .into());
                }
                if self.below_price.is_none() {
                    return Err(InvalidParameter::new(
                        "below_price",
                        "required for TAKE_PROFIT_LIMIT",
                    )
                    .into());
                }
                if self.below_stop_price.is_none() && self.below_trailing_delta.is_none() {
                    return Err(InvalidParameter::new("below_stop_price or below_trailing_delta", "either below_stop_price or below_trailing_delta must be specified for TAKE_PROFIT_LIMIT").into());
                }
            }
            OrderType::LimitMaker => {
                if self.below_time_in_force.is_some() {
                    return Err(InvalidParameter::new(
                        "below_time_in_force",
                        "not needed for LIMIT_MAKER",
                    )
                    .into());
                }
                if self.below_price.is_none() {
                    return Err(
                        InvalidParameter::new("below_price", "required for LIMIT_MAKER").into(),
                    );
                }
            }
            OrderType::StopLoss | OrderType::TakeProfit => {
                if self.below_stop_price.is_none() && self.below_trailing_delta.is_none() {
                    return Err(InvalidParameter::new(
                        "below_stop_price or below_trailing_delta",
                        "either below_stop_price or below_trailing_delta must be specified",
                    )
                    .into());
                }
            }
            _ => {
                return Err(InvalidParameter::new(
                    "below_type",
                    "must be STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, or TAKE_PROFIT_LIMIT",
                )
                .into());
            }
        }

        let above_is_maker_take = matches!(
            self.above_type,
            OrderType::LimitMaker | OrderType::TakeProfit | OrderType::TakeProfitLimit
        );
        let below_is_maker_take = matches!(
            self.below_type,
            OrderType::LimitMaker | OrderType::TakeProfit | OrderType::TakeProfitLimit
        );
        let above_is_stop = matches!(
            self.above_type,
            OrderType::StopLoss | OrderType::StopLossLimit
        );
        let below_is_stop = matches!(
            self.below_type,
            OrderType::StopLoss | OrderType::StopLossLimit
        );

        if !((above_is_maker_take && below_is_stop) || (above_is_stop && below_is_maker_take)) {
            return Err(InvalidParameter::new(
                "above_type/below_type",
                "one order must be LIMIT_MAKER/TAKE_PROFIT/TAKE_PROFIT_LIMIT and the other must be STOP_LOSS/STOP_LOSS_LIMIT"
            ).into());
        }

        if let Some(strategy_type) = self.above_strategy_type
            && strategy_type < 1000000
        {
            return Err(InvalidParameter::new("above_strategy_type", "must be >= 1000000").into());
        }

        if let Some(strategy_type) = self.below_strategy_type
            && strategy_type < 1000000
        {
            return Err(InvalidParameter::new("below_strategy_type", "must be >= 1000000").into());
        }

        Ok(())
    }
}
