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
 * OTOCO (One-Triggers-One-Cancels-Other) order specification builder.
 *
 * OTOCO orders consist of a working order and two pending orders (above/below)
 * that form an OCO pair. When the working order is filled, the OCO pending
 * orders are automatically placed.
 *
 * # Fields
 * - `symbol`: Trading symbol for the order.
 * - `list_client_order_id`: Client-specified order list identifier.
 * - `working_type`: Type of working order.
 * - `working_side`: Side of working order.
 * - `working_client_order_id`: Client-specified identifier for the working order.
 * - `working_price`: Price for the working order.
 * - `working_quantity`: Quantity for the working order.
 * - `working_iceberg_quantity`: Iceberg quantity for the working order.
 * - `working_time_in_force`: Time in force for the working order.
 * - `working_strategy_id`: Strategy ID for the working order.
 * - `working_strategy_type`: Strategy type for the working order.
 * - `pending_side`: Side of pending orders.
 * - `pending_quantity`: Quantity for the pending orders.
 * - `pending_above_type`: Type of pending above order.
 * - `pending_above_client_order_id`: Client-specified identifier for the pending above order.
 * - `pending_above_price`: Price for the pending above order.
 * - `pending_above_stop_price`: Stop price for the pending above order.
 * - `pending_above_trailing_delta`: Trailing delta for the pending above order.
 * - `pending_above_iceberg_quantity`: Iceberg quantity for the pending above order.
 * - `pending_above_time_in_force`: Time in force for the pending above order.
 * - `pending_above_strategy_id`: Strategy ID for the pending above order.
 * - `pending_above_strategy_type`: Strategy type for the pending above order.
 * - `pending_below_type`: Type of pending below order.
 * - `pending_below_client_order_id`: Client-specified identifier for the pending below order.
 * - `pending_below_price`: Price for the pending below order.
 * - `pending_below_stop_price`: Stop price for the pending below order.
 * - `pending_below_trailing_delta`: Trailing delta for the pending below order.
 * - `pending_below_iceberg_quantity`: Iceberg quantity for the pending below order.
 * - `pending_below_time_in_force`: Time in force for the pending below order.
 * - `pending_below_strategy_id`: Strategy ID for the pending below order.
 * - `pending_below_strategy_type`: Strategy type for the pending below order.
 * - `response_type`: Response format - ACK, RESULT, or FULL.
 * - `self_trade_prevention_mode`: Self-trade prevention mode.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtocoOrderSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,

    // Working order
    pub working_type: OrderType,
    pub working_side: OrderSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_client_order_id: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub working_price: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub working_quantity: rust_decimal::Decimal,
    #[serde(
        rename = "workingIcebergQty",
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_strategy_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_strategy_type: Option<u32>,

    // Pending orders
    pub pending_side: OrderSide,
    #[serde(with = "rust_decimal::serde::str")]
    pub pending_quantity: rust_decimal::Decimal,

    // Pending above order
    pub pending_above_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_above_client_order_id: Option<String>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_stop_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_trailing_delta: Option<rust_decimal::Decimal>,
    #[serde(
        rename = "pendingAboveIcebergQty",
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_above_iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_above_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_above_strategy_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_above_strategy_type: Option<u32>,

    // Pending below order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_below_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_below_client_order_id: Option<String>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_stop_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_trailing_delta: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_below_iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_below_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_below_strategy_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_below_strategy_type: Option<u32>,

    // Response
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub response_type: Option<OrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OtocoOrderSpec<Unvalidated> {
    /**
     * Creates a new OTOCO order specification with required parameters.
     *
     * # Arguments
     * - `symbol`: Trading symbol for the order.
     * - `working_type`: Type of working order.
     * - `working_side`: Side of working order.
     * - `working_price`: Price for the working order.
     * - `working_quantity`: Quantity for the working order.
     * - `pending_side`: Side of pending orders.
     * - `pending_quantity`: Quantity for the pending orders.
     * - `pending_above_type`: Type of pending above order.
     *
     * # Returns
     * - `Self`: New OTOCO order specification.
     */
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        symbol: impl Into<String>,
        working_type: OrderType,
        working_side: OrderSide,
        working_price: rust_decimal::Decimal,
        working_quantity: rust_decimal::Decimal,
        pending_side: OrderSide,
        pending_quantity: rust_decimal::Decimal,
        pending_above_type: OrderType,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            list_client_order_id: None,
            working_type,
            working_side,
            working_client_order_id: None,
            working_price,
            working_quantity,
            working_iceberg_quantity: None,
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            pending_side,
            pending_quantity,
            pending_above_type,
            pending_above_client_order_id: None,
            pending_above_price: None,
            pending_above_stop_price: None,
            pending_above_trailing_delta: None,
            pending_above_iceberg_quantity: None,
            pending_above_time_in_force: None,
            pending_above_strategy_id: None,
            pending_above_strategy_type: None,
            pending_below_type: None,
            pending_below_client_order_id: None,
            pending_below_price: None,
            pending_below_stop_price: None,
            pending_below_trailing_delta: None,
            pending_below_iceberg_quantity: None,
            pending_below_time_in_force: None,
            pending_below_strategy_id: None,
            pending_below_strategy_type: None,
            response_type: None,
            self_trade_prevention_mode: None,
            _state: PhantomData,
        }
    }

    /**
     * Sets the list client order ID.
     *
     * # Arguments
     * - `client_id`: Custom client order list ID.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_list_client_order_id(mut self, client_id: impl Into<String>) -> Self {
        self.list_client_order_id = Some(client_id.into());
        self
    }

    /**
     * Sets the pending above order price.
     *
     * # Arguments
     * - `price`: Price for the pending above order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_above_price(mut self, price: rust_decimal::Decimal) -> Self {
        self.pending_above_price = Some(price);
        self
    }

    /**
     * Sets the pending above order stop price.
     *
     * # Arguments
     * - `stop_price`: Stop price for the pending above order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_above_stop_price(mut self, stop_price: rust_decimal::Decimal) -> Self {
        self.pending_above_stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the pending above order time in force.
     *
     * # Arguments
     * - `time_in_force`: Time in force for the pending above order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_above_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.pending_above_time_in_force = Some(time_in_force);
        self
    }

    /**
     * Sets the pending below order type.
     *
     * # Arguments
     * - `below_type`: Type for the pending below order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_below_type(mut self, below_type: OrderType) -> Self {
        self.pending_below_type = Some(below_type);
        self
    }

    /**
     * Sets the pending below order price.
     *
     * # Arguments
     * - `price`: Price for the pending below order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_below_price(mut self, price: rust_decimal::Decimal) -> Self {
        self.pending_below_price = Some(price);
        self
    }

    /**
     * Sets the pending below order stop price.
     *
     * # Arguments
     * - `stop_price`: Stop price for the pending below order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_below_stop_price(mut self, stop_price: rust_decimal::Decimal) -> Self {
        self.pending_below_stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the pending below order time in force.
     *
     * # Arguments
     * - `time_in_force`: Time in force for the pending below order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_pending_below_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.pending_below_time_in_force = Some(time_in_force);
        self
    }

    /**
     * Sets the working order time in force.
     *
     * # Arguments
     * - `time_in_force`: Time in force for the working order.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_working_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.working_time_in_force = Some(time_in_force);
        self
    }

    /**
     * Sets the response type.
     *
     * # Arguments
     * - `response_type`: Response type for the order (ACK, RESULT, FULL).
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_response_type(mut self, response_type: OrderResponseType) -> Self {
        self.response_type = Some(response_type);
        self
    }

    /**
     * Sets the self-trade prevention mode.
     *
     * # Arguments
     * - `stp_mode`: Self-trade prevention mode to use.
     *
     * # Returns
     * - `Self`: Updated OTOCO order specification.
     */
    pub fn with_self_trade_prevention_mode(mut self, stp_mode: SelfTradePreventionMode) -> Self {
        self.self_trade_prevention_mode = Some(stp_mode);
        self
    }

    /**
     * Builds the OTOCO order specification.
     *
     * # Returns
     * - `OtocoOrderSpec<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OtocoOrderSpec<Validated>> {
        self.validate()
            .context("Failed to validate OtocoOrderSpec")?;

        Ok(OtocoOrderSpec {
            symbol: self.symbol,
            list_client_order_id: self.list_client_order_id,
            working_type: self.working_type,
            working_side: self.working_side,
            working_client_order_id: self.working_client_order_id,
            working_price: self.working_price,
            working_quantity: self.working_quantity,
            working_iceberg_quantity: self.working_iceberg_quantity,
            working_time_in_force: self.working_time_in_force,
            working_strategy_id: self.working_strategy_id,
            working_strategy_type: self.working_strategy_type,
            pending_side: self.pending_side,
            pending_quantity: self.pending_quantity,
            pending_above_type: self.pending_above_type,
            pending_above_client_order_id: self.pending_above_client_order_id,
            pending_above_price: self.pending_above_price,
            pending_above_stop_price: self.pending_above_stop_price,
            pending_above_trailing_delta: self.pending_above_trailing_delta,
            pending_above_iceberg_quantity: self.pending_above_iceberg_quantity,
            pending_above_time_in_force: self.pending_above_time_in_force,
            pending_above_strategy_id: self.pending_above_strategy_id,
            pending_above_strategy_type: self.pending_above_strategy_type,
            pending_below_type: self.pending_below_type,
            pending_below_client_order_id: self.pending_below_client_order_id,
            pending_below_price: self.pending_below_price,
            pending_below_stop_price: self.pending_below_stop_price,
            pending_below_trailing_delta: self.pending_below_trailing_delta,
            pending_below_iceberg_quantity: self.pending_below_iceberg_quantity,
            pending_below_time_in_force: self.pending_below_time_in_force,
            pending_below_strategy_id: self.pending_below_strategy_id,
            pending_below_strategy_type: self.pending_below_strategy_type,
            response_type: self.response_type,
            self_trade_prevention_mode: self.self_trade_prevention_mode,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the OTOCO order parameters.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if !matches!(self.working_type, OrderType::Limit | OrderType::LimitMaker) {
            return Err(
                InvalidParameter::new("working_type", "must be LIMIT or LIMIT_MAKER").into(),
            );
        }

        if self.working_type == OrderType::Limit && self.working_time_in_force.is_none() {
            return Err(InvalidParameter::empty("working_time_in_force").into());
        }

        if matches!(self.working_type, OrderType::Limit) && self.working_time_in_force.is_none() {
            return Err(InvalidParameter::empty("working_time_in_force").into());
        }

        if self.working_price <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("working_price", "must be greater than 0").into());
        }

        if self.working_quantity <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("working_quantity", "must be greater than 0").into());
        }

        if self.pending_quantity <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("pending_quantity", "must be greater than 0").into());
        }

        if matches!(self.pending_above_type, OrderType::Unknown) {
            return Err(
                InvalidParameter::new("pending_above_type", "must be a valid order type").into(),
            );
        }

        match self.pending_above_type {
            OrderType::LimitMaker => {
                if self.pending_above_price.is_none() {
                    return Err(InvalidParameter::empty("pending_above_price").into());
                }
            }
            OrderType::StopLoss | OrderType::TakeProfit => {
                if self.pending_above_stop_price.is_none()
                    && self.pending_above_trailing_delta.is_none()
                {
                    return Err(InvalidParameter::empty(
                        "pending_above_stop_price or pending_above_trailing_delta",
                    )
                    .into());
                }
            }
            OrderType::StopLossLimit | OrderType::TakeProfitLimit => {
                if self.pending_above_price.is_none() {
                    return Err(InvalidParameter::empty("pending_above_price").into());
                }
                if self.pending_above_stop_price.is_none()
                    && self.pending_above_trailing_delta.is_none()
                {
                    return Err(InvalidParameter::empty(
                        "pending_above_stop_price or pending_above_trailing_delta",
                    )
                    .into());
                }
                if self.pending_above_time_in_force.is_none() {
                    return Err(InvalidParameter::empty("pending_above_time_in_force").into());
                }
            }
            _ => {
                return Err(InvalidParameter::new("pending_above_type", "must be STOP_LOSS_LIMIT, STOP_LOSS, LIMIT_MAKER, TAKE_PROFIT, or TAKE_PROFIT_LIMIT").into());
            }
        }

        if let Some(below_type) = &self.pending_below_type {
            match below_type {
                OrderType::LimitMaker => {
                    if self.pending_below_price.is_none() {
                        return Err(InvalidParameter::empty("pending_below_price").into());
                    }
                }
                OrderType::StopLoss | OrderType::TakeProfit => {
                    if self.pending_below_stop_price.is_none()
                        && self.pending_below_trailing_delta.is_none()
                    {
                        return Err(InvalidParameter::empty(
                            "pending_below_stop_price or pending_below_trailing_delta",
                        )
                        .into());
                    }
                }
                OrderType::StopLossLimit | OrderType::TakeProfitLimit => {
                    if self.pending_below_price.is_none() {
                        return Err(InvalidParameter::empty("pending_below_price").into());
                    }
                    if self.pending_below_stop_price.is_none()
                        && self.pending_below_trailing_delta.is_none()
                    {
                        return Err(InvalidParameter::empty(
                            "pending_below_stop_price or pending_below_trailing_delta",
                        )
                        .into());
                    }
                    if self.pending_below_time_in_force.is_none() {
                        return Err(InvalidParameter::empty("pending_below_time_in_force").into());
                    }
                }
                _ => {
                    return Err(InvalidParameter::new(
                        "pending_below_type",
                        "must be STOP_LOSS, STOP_LOSS_LIMIT, TAKE_PROFIT, or TAKE_PROFIT_LIMIT",
                    )
                    .into());
                }
            }
        }

        Ok(())
    }
}
