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
 * OTO (One-Triggers-Other) order specification builder.
 *
 * OTO orders consist of a working order and a pending order. When the working
 * order is filled, the pending order is automatically placed.
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
 * - `pending_type`: Type of pending order.
 * - `pending_side`: Side of pending order.
 * - `pending_client_order_id`: Client-specified identifier for the pending order.
 * - `pending_quantity`: Quantity for the pending order.
 * - `pending_price`: Price for the pending order.
 * - `pending_stop_price`: Stop price for pending stop orders.
 * - `pending_trailing_delta`: Trailing delta for pending trailing orders.
 * - `pending_iceberg_quantity`: Iceberg quantity for the pending order.
 * - `pending_time_in_force`: Time in force for the pending order.
 * - `pending_strategy_id`: Strategy ID for the pending order.
 * - `pending_strategy_type`: Strategy type for the pending order.
 * - `response_type`: Response format.
 * - `self_trade_prevention_mode`: Self-trade prevention mode.
 */
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OtoOrderSpec<S = Unvalidated> {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_client_order_id: Option<String>,
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
    pub pending_type: OrderType,
    pub pending_side: OrderSide,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_client_order_id: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub pending_quantity: rust_decimal::Decimal,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_stop_price: Option<rust_decimal::Decimal>,
    #[serde(
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_trailing_delta: Option<rust_decimal::Decimal>,
    #[serde(
        rename = "pendingIcebergQty",
        with = "rust_decimal::serde::str_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_iceberg_quantity: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_strategy_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_strategy_type: Option<u32>,
    #[serde(rename = "newOrderRespType", skip_serializing_if = "Option::is_none")]
    pub response_type: Option<OrderResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_mode: Option<SelfTradePreventionMode>,
    #[serde(skip)]
    _state: PhantomData<S>,
}

impl OtoOrderSpec<Unvalidated> {
    /**
     * Creates a new OTO order specification with required parameters.
     *
     * # Arguments
     * - `symbol`: Trading symbol for the order.
     * - `working_type`: Type of working order.
     * - `working_side`: Side of working order.
     * - `working_price`: Price for the working order.
     * - `working_quantity`: Quantity for the working order.
     * - `pending_type`: Type of pending order.
     * - `pending_side`: Side of pending order.
     * - `pending_quantity`: Quantity for the pending order.
     *
     * # Returns
     * - `Self`: New OTO order specification.
     */
    pub fn new(
        symbol: impl Into<String>,
        working_type: OrderType,
        working_side: OrderSide,
        working_price: rust_decimal::Decimal,
        working_quantity: rust_decimal::Decimal,
        pending_type: OrderType,
        pending_side: OrderSide,
        pending_quantity: rust_decimal::Decimal,
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
            pending_type,
            pending_side,
            pending_client_order_id: None,
            pending_quantity,
            pending_price: None,
            pending_stop_price: None,
            pending_trailing_delta: None,
            pending_iceberg_quantity: None,
            pending_time_in_force: None,
            pending_strategy_id: None,
            pending_strategy_type: None,
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
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_list_client_order_id(mut self, client_id: impl Into<String>) -> Self {
        self.list_client_order_id = Some(client_id.into());
        self
    }

    /**
     * Sets the working order client ID.
     *
     * # Arguments
     * - `client_id`: Custom client order ID for the working order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_working_client_order_id(mut self, client_id: impl Into<String>) -> Self {
        self.working_client_order_id = Some(client_id.into());
        self
    }

    /**
     * Sets the working order iceberg quantity.
     *
     * # Arguments
     * - `iceberg_quantity`: Iceberg quantity for the working order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_working_iceberg_quantity(
        mut self,
        iceberg_quantity: rust_decimal::Decimal,
    ) -> Self {
        self.working_iceberg_quantity = Some(iceberg_quantity);
        self
    }

    /**
     * Sets the working order time in force.
     *
     * # Arguments
     * - `time_in_force`: Time in force for the working order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_working_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.working_time_in_force = Some(time_in_force);
        self
    }

    /**
     * Sets the working order strategy ID.
     *
     * # Arguments
     * - `strategy_id`: Strategy ID for the working order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_working_strategy_id(mut self, strategy_id: u64) -> Self {
        self.working_strategy_id = Some(strategy_id);
        self
    }

    /**
     * Sets the working order strategy type.
     *
     * # Arguments
     * - `strategy_type`: Strategy type for the working order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_working_strategy_type(mut self, strategy_type: u32) -> Self {
        self.working_strategy_type = Some(strategy_type);
        self
    }

    /**
     * Sets the pending order client ID.
     *
     * # Arguments
     * - `client_id`: Custom client order ID for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_client_order_id(mut self, client_id: impl Into<String>) -> Self {
        self.pending_client_order_id = Some(client_id.into());
        self
    }

    /**
     * Sets the pending order price.
     *
     * # Arguments
     * - `price`: Price for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_price(mut self, price: rust_decimal::Decimal) -> Self {
        self.pending_price = Some(price);
        self
    }

    /**
     * Sets the pending order stop price.
     *
     * # Arguments
     * - `stop_price`: Stop price for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_stop_price(mut self, stop_price: rust_decimal::Decimal) -> Self {
        self.pending_stop_price = Some(stop_price);
        self
    }

    /**
     * Sets the pending order trailing delta.
     *
     * # Arguments
     * - `trailing_delta`: Trailing delta for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_trailing_delta(mut self, trailing_delta: rust_decimal::Decimal) -> Self {
        self.pending_trailing_delta = Some(trailing_delta);
        self
    }

    /**
     * Sets the pending order iceberg quantity.
     *
     * # Arguments
     * - `iceberg_quantity`: Iceberg quantity for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_iceberg_quantity(
        mut self,
        iceberg_quantity: rust_decimal::Decimal,
    ) -> Self {
        self.pending_iceberg_quantity = Some(iceberg_quantity);
        self
    }

    /**
     * Sets the pending order time in force.
     *
     * # Arguments
     * - `time_in_force`: Time in force for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.pending_time_in_force = Some(time_in_force);
        self
    }

    /**
     * Sets the pending order strategy ID.
     *
     * # Arguments
     * - `strategy_id`: Strategy ID for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_strategy_id(mut self, strategy_id: u64) -> Self {
        self.pending_strategy_id = Some(strategy_id);
        self
    }

    /**
     * Sets the pending order strategy type.
     *
     * # Arguments
     * - `strategy_type`: Strategy type for the pending order.
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_pending_strategy_type(mut self, strategy_type: u32) -> Self {
        self.pending_strategy_type = Some(strategy_type);
        self
    }

    /**
     * Sets the response type.
     *
     * # Arguments
     * - `response_type`: Response type for the order (ACK, RESULT, FULL).
     *
     * # Returns
     * - `Self`: Updated OTO order specification.
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
     * - `Self`: Updated OTO order specification.
     */
    pub fn with_self_trade_prevention_mode(mut self, stp_mode: SelfTradePreventionMode) -> Self {
        self.self_trade_prevention_mode = Some(stp_mode);
        self
    }

    /**
     * Builds the OTO order specification.
     *
     * # Returns
     * - `OtoOrderSpec<Validated>`: Validated specification or error if validation fails.
     */
    pub fn build(self) -> Result<OtoOrderSpec<Validated>> {
        self.validate().context("Failed to validate OtoOrderSpec")?;

        Ok(OtoOrderSpec {
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
            pending_type: self.pending_type,
            pending_side: self.pending_side,
            pending_client_order_id: self.pending_client_order_id,
            pending_quantity: self.pending_quantity,
            pending_price: self.pending_price,
            pending_stop_price: self.pending_stop_price,
            pending_trailing_delta: self.pending_trailing_delta,
            pending_iceberg_quantity: self.pending_iceberg_quantity,
            pending_time_in_force: self.pending_time_in_force,
            pending_strategy_id: self.pending_strategy_id,
            pending_strategy_type: self.pending_strategy_type,
            response_type: self.response_type,
            self_trade_prevention_mode: self.self_trade_prevention_mode,
            _state: PhantomData::<Validated>,
        })
    }

    /**
     * Validates the OTO order parameters according to Binance API requirements.
     *
     * # Returns
     * - `()`: Ok if valid, error if invalid parameters.
     */
    fn validate(&self) -> Result<()> {
        if self.symbol.trim().is_empty() {
            return Err(InvalidParameter::empty("symbol").into());
        }

        if self.working_quantity <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("working_quantity", "must be greater than 0").into());
        }

        if self.working_price <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("working_price", "must be greater than 0").into());
        }

        if self.pending_quantity <= rust_decimal::Decimal::ZERO {
            return Err(InvalidParameter::new("pending_quantity", "must be greater than 0").into());
        }

        if !matches!(self.working_type, OrderType::Limit | OrderType::LimitMaker) {
            return Err(
                InvalidParameter::new("working_type", "must be LIMIT or LIMIT_MAKER").into(),
            );
        }

        if let Some(working_iceberg_qty) = self.working_iceberg_quantity {
            if working_iceberg_qty <= rust_decimal::Decimal::ZERO {
                return Err(InvalidParameter::new(
                    "working_iceberg_quantity",
                    "must be greater than 0",
                )
                .into());
            }
        }

        if let Some(strategy_type) = self.working_strategy_type {
            if strategy_type < 1000000 {
                return Err(
                    InvalidParameter::range("working_strategy_type", 1000000, u32::MAX).into(),
                );
            }
        }

        match self.pending_type {
            OrderType::Limit => {
                if self.pending_price.is_none() {
                    return Err(InvalidParameter::empty("pending_price").into());
                }
                if self.pending_time_in_force.is_none() {
                    return Err(InvalidParameter::empty("pending_time_in_force").into());
                }
            }
            OrderType::Market => {
                // Market orders don't need price or time_in_force
            }
            OrderType::StopLoss => {
                if self.pending_stop_price.is_none() && self.pending_trailing_delta.is_none() {
                    return Err(InvalidParameter::required(
                        "pending_stop_price or pending_trailing_delta",
                    )
                    .into());
                }
            }
            OrderType::StopLossLimit => {
                if self.pending_price.is_none() {
                    return Err(InvalidParameter::empty("pending_price").into());
                }
                if self.pending_stop_price.is_none() && self.pending_trailing_delta.is_none() {
                    return Err(InvalidParameter::required(
                        "pending_stop_price or pending_trailing_delta",
                    )
                    .into());
                }
                if self.pending_time_in_force.is_none() {
                    return Err(InvalidParameter::empty("pending_time_in_force").into());
                }
            }
            OrderType::TakeProfit => {
                if self.pending_stop_price.is_none() && self.pending_trailing_delta.is_none() {
                    return Err(InvalidParameter::required(
                        "pending_stop_price or pending_trailing_delta",
                    )
                    .into());
                }
            }
            OrderType::TakeProfitLimit => {
                if self.pending_price.is_none() {
                    return Err(InvalidParameter::empty("pending_price").into());
                }
                if self.pending_stop_price.is_none() && self.pending_trailing_delta.is_none() {
                    return Err(InvalidParameter::required(
                        "pending_stop_price or pending_trailing_delta",
                    )
                    .into());
                }
                if self.pending_time_in_force.is_none() {
                    return Err(InvalidParameter::empty("pending_time_in_force").into());
                }
            }
            OrderType::LimitMaker => {
                if self.pending_price.is_none() {
                    return Err(InvalidParameter::empty("pending_price").into());
                }
            }
            _ => {
                return Err(InvalidParameter::new(
                    "pending_type",
                    "must be a valid order type for OTO pending orders",
                )
                .into());
            }
        }

        if let Some(price) = self.pending_price {
            if price <= rust_decimal::Decimal::ZERO {
                return Err(
                    InvalidParameter::new("pending_price", "must be greater than 0").into(),
                );
            }
        }

        if let Some(stop_price) = self.pending_stop_price {
            if stop_price <= rust_decimal::Decimal::ZERO {
                return Err(
                    InvalidParameter::new("pending_stop_price", "must be greater than 0").into(),
                );
            }
        }

        if let Some(trailing_delta) = self.pending_trailing_delta {
            if trailing_delta <= rust_decimal::Decimal::ZERO {
                return Err(InvalidParameter::new(
                    "pending_trailing_delta",
                    "must be greater than 0",
                )
                .into());
            }
        }

        if let Some(iceberg_qty) = self.pending_iceberg_quantity {
            if iceberg_qty <= rust_decimal::Decimal::ZERO {
                return Err(InvalidParameter::new(
                    "pending_iceberg_quantity",
                    "must be greater than 0",
                )
                .into());
            }
        }

        if let Some(strategy_type) = self.pending_strategy_type {
            if strategy_type < 1000000 {
                return Err(
                    InvalidParameter::range("pending_strategy_type", 1000000, u32::MAX).into(),
                );
            }
        }

        Ok(())
    }
}
