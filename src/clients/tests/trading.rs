#[cfg(test)]
mod tests {
    use crate::{
        clients::{
            tests::helpers::*,
            r#trait::{GeneralClient, TickerClient, TradingClient},
        },
        enums::{
            CancelReplaceMode, CancelRestrictions, OrderResponseType, OrderSide, OrderStatus,
            OrderType, TimeInForce,
        },
        errors::{BinanceError, ErrorCategory, RequestError},
        filters::SymbolFilter,
        types::{
            requests::{
                AmendOrderSpec, CancelAllOrdersSpec, CancelOrderSpec, CancelReplaceSpec,
                ExchangeInfoSpec, OrderSpec, TickerPriceSpec,
            },
            responses::{
                AmendedOrder, CancelReplaceOrder, CancelledOrder, Order, SymbolInfo, TestOrder,
            },
        },
    };
    use rust_decimal::Decimal;
    use std::str::FromStr;

    /**
     * Generates a unique client order ID using timestamp.
     */
    fn generate_unique_client_order_id() -> String {
        format!("test_{}", chrono::Utc::now().timestamp_millis())
    }

    /**
     * Gets the minimum notional value for a symbol from MIN_NOTIONAL or NOTIONAL filters.
     */
    fn get_min_notional(symbol_info: &SymbolInfo) -> Decimal {
        // First try MIN_NOTIONAL filter
        if let Some(min_notional_filter) = symbol_info.min_notional_filter() {
            return Decimal::from_str(&min_notional_filter.min_notional)
                .unwrap_or(Decimal::new(10, 0)); // Default $10
        }

        // Then try NOTIONAL filter
        for filter in &symbol_info.filters {
            if let SymbolFilter::Notional(notional_filter) = filter {
                return Decimal::from_str(&notional_filter.min_notional)
                    .unwrap_or(Decimal::new(10, 0)); // Default $10
            }
        }

        Decimal::new(10, 0)
    }

    /**
     * Makes a price compliant with the PRICE_FILTER tick_size requirement.
     */
    fn make_price_tick_compliant(price: Decimal, symbol_info: &SymbolInfo) -> Decimal {
        if let Some(price_filter) = symbol_info.price_filter() {
            let tick_size =
                Decimal::from_str(&price_filter.tick_size).unwrap_or(Decimal::new(1, 2));

            if tick_size > Decimal::ZERO {
                let ticks = (price / tick_size).floor();
                return ticks * tick_size;
            }
        }
        price
    }

    /**
     * Makes a quantity compliant with the LOT_SIZE step_size requirement.
     */
    fn make_quantity_step_compliant(quantity: Decimal, symbol_info: &SymbolInfo) -> Decimal {
        if let Some(lot_size_filter) = symbol_info.lot_size_filter() {
            let step_size =
                Decimal::from_str(&lot_size_filter.step_size).unwrap_or(Decimal::new(1, 6));
            let min_qty = Decimal::from_str(&lot_size_filter.min_qty).unwrap_or(Decimal::ZERO);

            if step_size > Decimal::ZERO {
                let steps = (quantity / step_size).floor();
                let compliant_qty = steps * step_size;
                return compliant_qty.max(min_qty);
            }
        }
        quantity
    }

    /**
     * Rounds a decimal value to the quote asset precision.
     */
    fn round_to_quote_precision(value: Decimal, symbol_info: &SymbolInfo) -> Decimal {
        let precision = symbol_info.quote_asset_precision as u32;
        let scale_factor = Decimal::new(10_i64.pow(precision), 0);
        (value * scale_factor).round() / scale_factor
    }

    /**
     * Calculates safe order parameters that comply with all symbol filters.
     * Returns (price, quantity) that will pass PRICE_FILTER, LOT_SIZE, and MIN_NOTIONAL.
     */
    fn calculate_safe_order_params(
        market_price: Decimal,
        symbol_info: &SymbolInfo,
    ) -> (Decimal, Decimal) {
        // Calculate a conservative price (95% of market for buy orders)
        let target_price = market_price * Decimal::new(95, 2); // 0.95
        let safe_price = make_price_tick_compliant(target_price, symbol_info);

        // Get minimum notional requirement
        let min_notional = get_min_notional(symbol_info);

        // Calculate minimum quantity needed for MIN_NOTIONAL (with 20% buffer)
        let min_quantity_for_notional = (min_notional / safe_price) * Decimal::new(12, 1);

        // Make quantity step compliant
        let safe_quantity = make_quantity_step_compliant(min_quantity_for_notional, symbol_info);

        // Verify final notional meets requirement
        let final_notional = safe_price * safe_quantity;
        if final_notional < min_notional {
            let adjusted_quantity = (min_notional / safe_price) * Decimal::new(15, 1);
            let final_safe_quantity = make_quantity_step_compliant(adjusted_quantity, symbol_info);
            return (safe_price, final_safe_quantity);
        }

        (safe_price, safe_quantity)
    }

    /**
     * Validates order response structure.
     */
    fn assert_valid_order(order: &Order) {
        assert!(!order.symbol.is_empty(), "Symbol should not be empty");
        assert!(order.order_id > 0, "Order ID should be positive");
        assert!(
            !order.client_order_id.is_empty(),
            "Client order ID should not be empty"
        );

        if let Some(status) = order.status {
            assert!(
                matches!(
                    status,
                    OrderStatus::New
                        | OrderStatus::PartiallyFilled
                        | OrderStatus::Filled
                        | OrderStatus::Canceled
                        | OrderStatus::Rejected
                        | OrderStatus::Expired
                        | OrderStatus::ExpiredInMatch
                ),
                "Order status should be valid"
            );
        }

        if let Some(fills) = &order.fills {
            for fill in fills {
                assert!(fill.trade_id > 0, "Trade ID should be positive");
                assert!(
                    fill.quantity > Decimal::ZERO,
                    "Fill quantity should be positive"
                );
                assert!(fill.price > Decimal::ZERO, "Fill price should be positive");
                assert!(
                    !fill.commission_asset.is_empty(),
                    "Commission asset should not be empty"
                );
            }
        }
    }

    /**
     * Validates test order response structure.
     */
    fn assert_valid_test_order(test_order: &TestOrder) {
        if let Some(commission) = &test_order.standard_commission_for_order {
            assert!(
                commission.maker >= Decimal::ZERO,
                "Maker commission should be non-negative"
            );
            assert!(
                commission.taker >= Decimal::ZERO,
                "Taker commission should be non-negative"
            );
        }
    }

    /**
     * Validates cancelled order response structure.
     */
    fn assert_valid_cancelled_order(cancelled_order: &CancelledOrder) {
        match cancelled_order {
            CancelledOrder::Individual(order) => {
                assert_valid_order(order);
            }
            CancelledOrder::OrderList(order_list) => {
                assert!(
                    !order_list.orders.is_empty(),
                    "Order list should not be empty"
                );
                for order_summary in &order_list.orders {
                    assert!(order_summary.order_id > 0, "Order ID should be positive");
                    assert!(
                        !order_summary.symbol.is_empty(),
                        "Symbol should not be empty"
                    );
                }
            }
        }
    }

    /**
     * Validates cancel replace order response structure.
     */
    fn assert_valid_cancel_replace_order(cancel_replace: &CancelReplaceOrder) {
        if let Some(cancelled_order) = &cancel_replace.cancel_order {
            assert_valid_order(cancelled_order);
        }

        if let Some(new_order) = &cancel_replace.new_order {
            assert_valid_order(new_order);
        }

        assert!(
            cancel_replace.cancel_order.is_some() || cancel_replace.new_order.is_some(),
            "Should have at least cancel order data or new order data"
        );
    }

    /**
     * Validates amended order response structure.
     */
    fn assert_valid_amended_order(amended_order: &AmendedOrder) {
        assert!(
            amended_order.transaction_time > 0,
            "Transaction time should be positive"
        );
        assert!(
            amended_order.execution_id > 0,
            "Execution ID should be positive"
        );
        assert_valid_order(&amended_order.amended_order);
    }

    /**
     * Tests order placement with limit orders using filter-compliant parameters.
     */
    #[tokio::test]
    async fn test_place_order_limit() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_order = with_timeout(rest_client.place_order(rest_spec))
            .await
            .expect("REST place order");
        let ws_order = with_timeout(ws_client.place_order(ws_spec))
            .await
            .expect("WebSocket place order");

        // Assert
        assert_valid_order(&rest_order);
        assert_valid_order(&ws_order);
        assert_eq!(rest_order.symbol, test_symbol);
        assert_eq!(ws_order.symbol, test_symbol);
        assert_eq!(rest_order.status, Some(OrderStatus::New));
        assert_eq!(ws_order.status, Some(OrderStatus::New));

        let rest_cancel_spec = CancelOrderSpec::new(test_symbol)
            .with_order_id(rest_order.order_id)
            .build()
            .expect("Cancel spec validation");
        let ws_cancel_spec = CancelOrderSpec::new(test_symbol)
            .with_order_id(ws_order.order_id)
            .build()
            .expect("Cancel spec validation");

        let _ = with_timeout(rest_client.cancel_order(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order(ws_cancel_spec)).await;
    }

    /**
     * Tests order placement with market orders using proper MIN_NOTIONAL compliance.
     */
    #[tokio::test]
    async fn test_place_order_market() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];
        let min_notional = get_min_notional(symbol_info);

        let target_quote_quantity = min_notional * Decimal::new(15, 1);
        let quote_quantity = round_to_quote_precision(target_quote_quantity, symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(&format!("rest_market_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(&format!("ws_market_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_order = with_timeout(rest_client.place_order(rest_spec))
            .await
            .expect("REST place order");
        let ws_order = with_timeout(ws_client.place_order(ws_spec))
            .await
            .expect("WebSocket place order");

        // Assert
        assert_valid_order(&rest_order);
        assert_valid_order(&ws_order);
        assert_eq!(rest_order.symbol, test_symbol);
        assert_eq!(ws_order.symbol, test_symbol);
        assert_eq!(rest_order.status, Some(OrderStatus::Filled));
        assert_eq!(ws_order.status, Some(OrderStatus::Filled));
    }

    /**
     * Tests order placement error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_place_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OrderSpec::new("INVALID", OrderSide::Buy, OrderType::Limit)
            .with_quantity(Decimal::new(1, 4))
            .with_price(Decimal::new(20000, 0))
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("Order spec validation");

        let ws_spec = OrderSpec::new("INVALID", OrderSide::Buy, OrderType::Limit)
            .with_quantity(Decimal::new(1, 4))
            .with_price(Decimal::new(20000, 0))
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("Order spec validation");

        let rest_result = with_timeout(rest_client.place_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.place_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -1121
                                && api_err.category == ErrorCategory::RequestIssues
                                && api_err.request_error == Some(RequestError::BadSymbol)
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }
    }

    /**
     * Tests order validation with limit orders using filter-compliant parameters.
     */
    #[tokio::test]
    async fn test_test_order_limit() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);

        // Act
        let rest_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&generate_unique_client_order_id())
            .build()
            .expect("Order spec validation");

        let ws_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&generate_unique_client_order_id())
            .build()
            .expect("Order spec validation");

        let rest_test_order = with_timeout(rest_client.test_order(rest_spec))
            .await
            .expect("REST test order");
        let ws_test_order = with_timeout(ws_client.test_order(ws_spec))
            .await
            .expect("WebSocket test order");

        // Assert
        assert_valid_test_order(&rest_test_order);
        assert_valid_test_order(&ws_test_order);
        if let Some(commission) = &rest_test_order.standard_commission_for_order {
            assert!(
                commission.maker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
            assert!(
                commission.taker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
        }

        if let Some(commission) = &ws_test_order.standard_commission_for_order {
            assert!(
                commission.maker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
            assert!(
                commission.taker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
        }
    }

    /**
     * Tests order validation with market orders using proper MIN_NOTIONAL compliance.
     */
    #[tokio::test]
    async fn test_test_order_market() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];
        let min_notional = get_min_notional(symbol_info);

        let target_quote_quantity = min_notional * Decimal::new(15, 1);
        let quote_quantity = round_to_quote_precision(target_quote_quantity, symbol_info);

        // Act
        let rest_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(&generate_unique_client_order_id())
            .build()
            .expect("Order spec validation");

        let ws_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(&generate_unique_client_order_id())
            .build()
            .expect("Order spec validation");

        let rest_test_order = with_timeout(rest_client.test_order(rest_spec))
            .await
            .expect("REST test order");
        let ws_test_order = with_timeout(ws_client.test_order(ws_spec))
            .await
            .expect("WebSocket test order");

        // Assert
        assert_valid_test_order(&rest_test_order);
        assert_valid_test_order(&ws_test_order);
        if let Some(commission) = &rest_test_order.standard_commission_for_order {
            assert!(
                commission.maker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
            assert!(
                commission.taker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
        }

        if let Some(commission) = &ws_test_order.standard_commission_for_order {
            assert!(
                commission.maker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
            assert!(
                commission.taker >= Decimal::ZERO,
                "Commission rates should be non-negative"
            );
        }
    }

    /**
     * Tests order validation error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_test_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OrderSpec::new("INVALID", OrderSide::Buy, OrderType::Limit)
            .with_quantity(Decimal::new(1, 4))
            .with_price(Decimal::new(20000, 0))
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("Order spec validation");

        let ws_spec = OrderSpec::new("INVALID", OrderSide::Buy, OrderType::Limit)
            .with_quantity(Decimal::new(1, 4))
            .with_price(Decimal::new(20000, 0))
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("Order spec validation");

        let rest_result = with_timeout(rest_client.test_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.test_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(test_order) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, test_order
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -1121
                                && api_err.category == ErrorCategory::RequestIssues
                                && api_err.request_error == Some(RequestError::BadSymbol)
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }
    }

    /**
     * Tests cancellation of all orders for a symbol.
     */
    #[tokio::test]
    async fn test_cancel_all_orders_default() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "ETHUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_order_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_cancel_all_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_order_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_cancel_all_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_order = with_timeout(rest_client.place_order(rest_order_spec))
            .await
            .expect("REST place order");
        let ws_order = with_timeout(ws_client.place_order(ws_order_spec))
            .await
            .expect("WebSocket place order");

        // Act
        let rest_cancel_spec = CancelAllOrdersSpec::new(test_symbol)
            .build()
            .expect("Cancel all spec validation");
        let ws_cancel_spec = CancelAllOrdersSpec::new(test_symbol)
            .build()
            .expect("Cancel all spec validation");

        let rest_cancelled_orders = with_timeout(rest_client.cancel_all_orders(rest_cancel_spec))
            .await
            .expect("REST cancel all orders");

        let ws_result = with_timeout(ws_client.cancel_all_orders(ws_cancel_spec)).await;

        // Assert
        assert!(
            !rest_cancelled_orders.is_empty(),
            "REST should cancel at least some orders"
        );

        let mut rest_cancelled_ids = Vec::new();
        for cancelled_order in &rest_cancelled_orders {
            assert_valid_cancelled_order(cancelled_order);
            match cancelled_order {
                CancelledOrder::Individual(order) => {
                    rest_cancelled_ids.push(order.order_id);
                    assert_eq!(order.symbol, test_symbol);
                    assert_eq!(order.status, Some(OrderStatus::Canceled));
                }
                CancelledOrder::OrderList(order_list) => {
                    for order_summary in &order_list.orders {
                        rest_cancelled_ids.push(order_summary.order_id);
                    }
                }
            }
        }

        assert!(
            rest_cancelled_ids.contains(&rest_order.order_id),
            "REST should cancel its placed order"
        );
        assert!(
            rest_cancelled_ids.contains(&ws_order.order_id),
            "REST should also cancel WebSocket's order (same account)"
        );

        match ws_result {
            Ok(ws_cancelled_orders) => {
                assert!(
                    ws_cancelled_orders.is_empty(),
                    "WebSocket should return empty since REST already cancelled everything"
                );
            }
            Err(err) => {
                let downcast = err.downcast_ref::<BinanceError>();
                assert!(
                    matches!(
                        downcast,
                        Some(BinanceError::Api(api_err)) if api_err.code == -2011
                    ),
                    "WebSocket should get 'Unknown order sent' error since no orders left: {:#?}",
                    downcast
                );
            }
        }
    }

    /**
     * Tests cancellation of all orders when no orders exist.
     */
    #[tokio::test]
    async fn test_cancel_all_orders_no_orders() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BNBUSDT";

        let rest_cancel_spec = CancelAllOrdersSpec::new(test_symbol)
            .build()
            .expect("Cancel all spec validation");
        let ws_cancel_spec = CancelAllOrdersSpec::new(test_symbol)
            .build()
            .expect("Cancel all spec validation");

        // Act
        let rest_result = with_timeout(rest_client.cancel_all_orders(rest_cancel_spec)).await;
        let ws_result = with_timeout(ws_client.cancel_all_orders(ws_cancel_spec)).await;

        // Assert
        match rest_result {
            Ok(cancelled_orders) => {
                assert!(
                    cancelled_orders.is_empty(),
                    "REST should return empty when no orders exist"
                );
            }
            Err(err) => {
                let downcast = err.downcast_ref::<BinanceError>();
                assert!(
                    matches!(
                        downcast,
                        Some(BinanceError::Api(api_err)) if api_err.code == -2011
                    ),
                    "REST should return 'Unknown order sent' error when no orders exist: {:#?}",
                    downcast
                );
            }
        }

        match ws_result {
            Ok(cancelled_orders) => {
                assert!(
                    cancelled_orders.is_empty(),
                    "WebSocket should return empty when no orders exist"
                );
            }
            Err(err) => {
                let downcast = err.downcast_ref::<BinanceError>();
                assert!(
                    matches!(
                        downcast,
                        Some(BinanceError::Api(api_err)) if api_err.code == -2011
                    ),
                    "WebSocket should return 'Unknown order sent' error when no orders exist: {:#?}",
                    downcast
                );
            }
        }
    }

    /**
     * Tests cancellation of all orders with comprehensive validation.
     */
    #[tokio::test]
    async fn test_cancel_all_orders_multiple_orders() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "SOLUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let safe_price_2 = make_price_tick_compliant(safe_price * Decimal::new(99, 2), symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_order1_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_multi_1_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_order2_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price_2)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_multi_2_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_order_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price_2)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_multi_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_order1 = with_timeout(rest_client.place_order(rest_order1_spec))
            .await
            .expect("REST place order 1");
        let rest_order2 = with_timeout(rest_client.place_order(rest_order2_spec))
            .await
            .expect("REST place order 2");
        let ws_order = with_timeout(ws_client.place_order(ws_order_spec))
            .await
            .expect("WebSocket place order");

        // Act
        let rest_cancel_spec = CancelAllOrdersSpec::new(test_symbol)
            .build()
            .expect("Cancel all spec validation");
        let ws_cancel_spec = CancelAllOrdersSpec::new(test_symbol)
            .build()
            .expect("Cancel all spec validation");

        let rest_cancelled_orders = with_timeout(rest_client.cancel_all_orders(rest_cancel_spec))
            .await
            .expect("REST cancel all orders");
        let ws_result = with_timeout(ws_client.cancel_all_orders(ws_cancel_spec)).await;

        // Assert
        assert!(
            !rest_cancelled_orders.is_empty(),
            "REST should cancel orders"
        );

        let mut rest_cancelled_ids = Vec::new();
        for cancelled_order in &rest_cancelled_orders {
            assert_valid_cancelled_order(cancelled_order);
            match cancelled_order {
                CancelledOrder::Individual(order) => {
                    rest_cancelled_ids.push(order.order_id);
                    assert_eq!(order.symbol, test_symbol);
                }
                CancelledOrder::OrderList(order_list) => {
                    for order_summary in &order_list.orders {
                        rest_cancelled_ids.push(order_summary.order_id);
                    }
                }
            }
        }

        assert!(
            rest_cancelled_ids.contains(&rest_order1.order_id),
            "REST should cancel its first order"
        );
        assert!(
            rest_cancelled_ids.contains(&rest_order2.order_id),
            "REST should cancel its second order"
        );
        assert!(
            rest_cancelled_ids.contains(&ws_order.order_id),
            "REST should cancel WebSocket's order (same account)"
        );

        match ws_result {
            Ok(ws_cancelled_orders) => {
                assert!(
                    ws_cancelled_orders.is_empty(),
                    "WebSocket should return empty since REST already cancelled everything"
                );
            }
            Err(err) => {
                let downcast = err.downcast_ref::<BinanceError>();
                assert!(
                    matches!(
                        downcast,
                        Some(BinanceError::Api(api_err)) if api_err.code == -2011
                    ),
                    "WebSocket should get 'Unknown order sent' error since no orders left: {:#?}",
                    downcast
                );
            }
        }
    }

    /**
     * Tests cancel all orders error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_cancel_all_orders_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = CancelAllOrdersSpec::new("INVALID")
            .build()
            .expect("Cancel all spec validation");
        let ws_spec = CancelAllOrdersSpec::new("INVALID")
            .build()
            .expect("Cancel all spec validation");

        let rest_result = with_timeout(rest_client.cancel_all_orders(rest_spec)).await;
        let ws_result = with_timeout(ws_client.cancel_all_orders(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(orders) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, orders
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -1121
                                && api_err.category == ErrorCategory::RequestIssues
                                && api_err.request_error == Some(RequestError::BadSymbol)
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }
    }

    /**
     * Tests basic cancel-replace order functionality with STOP_ON_FAILURE mode.
     */
    #[tokio::test]
    async fn test_cancel_replace_order_stop_on_failure() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "LTCUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (initial_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let new_price = make_price_tick_compliant(initial_price * Decimal::new(98, 2), symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_cr_initial_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_cr_initial_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_initial_order = with_timeout(rest_client.place_order(rest_initial_spec))
            .await
            .expect("REST place initial order");
        let ws_initial_order = with_timeout(ws_client.place_order(ws_initial_spec))
            .await
            .expect("WebSocket place initial order");

        // Act
        let rest_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol,
            CancelReplaceMode::StopOnFailure,
            OrderSide::Buy,
            OrderType::Limit,
        )
        .with_cancel_order_id(rest_initial_order.order_id)
        .with_time_in_force(TimeInForce::GTC)
        .with_quantity(safe_quantity)
        .with_price(new_price)
        .with_new_client_order_id(format!("rest_cr_new_{}", base_id))
        .build()
        .expect("Cancel replace spec validation");

        let ws_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol,
            CancelReplaceMode::StopOnFailure,
            OrderSide::Buy,
            OrderType::Limit,
        )
        .with_cancel_order_id(ws_initial_order.order_id)
        .with_time_in_force(TimeInForce::GTC)
        .with_quantity(safe_quantity)
        .with_price(new_price)
        .with_new_client_order_id(format!("ws_cr_new_{}", base_id))
        .build()
        .expect("Cancel replace spec validation");

        let rest_cancel_replace =
            with_timeout(rest_client.cancel_replace_order(rest_cancel_replace_spec))
                .await
                .expect("REST cancel replace order");
        let ws_cancel_replace =
            with_timeout(ws_client.cancel_replace_order(ws_cancel_replace_spec))
                .await
                .expect("WebSocket cancel replace order");

        // Assert
        assert_valid_cancel_replace_order(&rest_cancel_replace);
        assert_valid_cancel_replace_order(&ws_cancel_replace);

        if let Some(cancelled_order) = &rest_cancel_replace.cancel_order {
            assert_eq!(cancelled_order.order_id, rest_initial_order.order_id);
            assert_eq!(cancelled_order.symbol, test_symbol);
            assert_eq!(cancelled_order.status, Some(OrderStatus::Canceled));
        }

        if let Some(new_order) = &rest_cancel_replace.new_order {
            assert_eq!(new_order.symbol, test_symbol);
            assert_eq!(new_order.side, Some(OrderSide::Buy));
            assert_eq!(new_order.order_type, Some(OrderType::Limit));
        }

        if let Some(cancelled_order) = &ws_cancel_replace.cancel_order {
            assert_eq!(cancelled_order.order_id, ws_initial_order.order_id);
            assert_eq!(cancelled_order.symbol, test_symbol);
            assert_eq!(cancelled_order.status, Some(OrderStatus::Canceled));
        }

        if let Some(new_order) = &ws_cancel_replace.new_order {
            assert_eq!(new_order.symbol, test_symbol);
            assert_eq!(new_order.side, Some(OrderSide::Buy));
            assert_eq!(new_order.order_type, Some(OrderType::Limit));
        }
    }

    /**
     * Tests cancel-replace order functionality with ALLOW_FAILURE mode.
     */
    #[tokio::test]
    async fn test_cancel_replace_order_allow_failure() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "DOTUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (initial_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let new_price = make_price_tick_compliant(initial_price * Decimal::new(97, 2), symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_cr_af_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_cr_af_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_initial_order = with_timeout(rest_client.place_order(rest_initial_spec))
            .await
            .expect("REST place initial order");
        let ws_initial_order = with_timeout(ws_client.place_order(ws_initial_spec))
            .await
            .expect("WebSocket place initial order");

        // Act
        let rest_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol.to_string(),
            CancelReplaceMode::AllowFailure,
            OrderSide::Buy,
            OrderType::Limit,
        )
        .with_cancel_order_id(rest_initial_order.order_id)
        .with_time_in_force(TimeInForce::GTC)
        .with_quantity(safe_quantity)
        .with_price(new_price)
        .with_new_client_order_id(format!("rest_cr_af_new_{}", base_id))
        .build()
        .expect("Cancel replace spec validation");

        let ws_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol.to_string(),
            CancelReplaceMode::AllowFailure,
            OrderSide::Buy,
            OrderType::Limit,
        )
        .with_cancel_order_id(ws_initial_order.order_id)
        .with_time_in_force(TimeInForce::GTC)
        .with_quantity(safe_quantity)
        .with_price(new_price)
        .with_new_client_order_id(format!("ws_cr_af_new_{}", base_id))
        .build()
        .expect("Cancel replace spec validation");

        let rest_cancel_replace =
            with_timeout(rest_client.cancel_replace_order(rest_cancel_replace_spec))
                .await
                .expect("REST cancel replace order");
        let ws_cancel_replace =
            with_timeout(ws_client.cancel_replace_order(ws_cancel_replace_spec))
                .await
                .expect("WebSocket cancel replace order");

        // Assert
        assert_valid_cancel_replace_order(&rest_cancel_replace);
        assert_valid_cancel_replace_order(&ws_cancel_replace);

        if let Some(cancelled_order) = &rest_cancel_replace.cancel_order {
            assert_eq!(cancelled_order.order_id, rest_initial_order.order_id);
            assert_eq!(cancelled_order.symbol, test_symbol);
        }

        if let Some(new_order) = &rest_cancel_replace.new_order {
            assert_eq!(new_order.symbol, test_symbol);
            assert_eq!(new_order.side, Some(OrderSide::Buy));
            assert_eq!(new_order.order_type, Some(OrderType::Limit));
        }

        if let Some(cancelled_order) = &ws_cancel_replace.cancel_order {
            assert_eq!(cancelled_order.order_id, ws_initial_order.order_id);
            assert_eq!(cancelled_order.symbol, test_symbol);
        }

        if let Some(new_order) = &ws_cancel_replace.new_order {
            assert_eq!(new_order.symbol, test_symbol);
            assert_eq!(new_order.side, Some(OrderSide::Buy));
            assert_eq!(new_order.order_type, Some(OrderType::Limit));
        }
    }

    /**
     * Tests cancel-replace with additional parameters and restrictions.
     */
    #[tokio::test]
    async fn test_cancel_replace_order_with_restrictions() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "ADAUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (initial_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let new_price = make_price_tick_compliant(initial_price * Decimal::new(96, 2), symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_cr_restr_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_cr_restr_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_initial_order = with_timeout(rest_client.place_order(rest_initial_spec))
            .await
            .expect("REST place initial order");
        let ws_initial_order = with_timeout(ws_client.place_order(ws_initial_spec))
            .await
            .expect("WebSocket place initial order");

        // Act
        let rest_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol.to_string(),
            CancelReplaceMode::StopOnFailure,
            OrderSide::Buy,
            OrderType::Limit,
        )
        .with_cancel_order_id(rest_initial_order.order_id)
        .with_cancel_new_client_order_id(format!("rest_cr_cancel_{}", base_id))
        .with_cancel_restrictions(CancelRestrictions::OnlyNew)
        .with_time_in_force(TimeInForce::GTC)
        .with_quantity(safe_quantity)
        .with_price(new_price)
        .with_new_client_order_id(format!("rest_cr_restr_new_{}", base_id))
        .with_strategy_id(12345)
        .with_strategy_type(1000001)
        .with_new_order_response_type(OrderResponseType::Full)
        .build()
        .expect("Cancel replace spec validation");

        let ws_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol.to_string(),
            CancelReplaceMode::AllowFailure,
            OrderSide::Buy,
            OrderType::Limit,
        )
        .with_cancel_order_id(ws_initial_order.order_id)
        .with_cancel_new_client_order_id(format!("ws_cr_cancel_{}", base_id))
        .with_cancel_restrictions(CancelRestrictions::OnlyNew)
        .with_time_in_force(TimeInForce::GTC)
        .with_quantity(safe_quantity)
        .with_price(new_price)
        .with_new_client_order_id(format!("ws_cr_restr_new_{}", base_id))
        .with_strategy_id(54321)
        .with_strategy_type(1000002)
        .with_new_order_response_type(OrderResponseType::Result)
        .build()
        .expect("Cancel replace spec validation");

        let rest_result =
            with_timeout(rest_client.cancel_replace_order(rest_cancel_replace_spec)).await;
        let ws_result = with_timeout(ws_client.cancel_replace_order(ws_cancel_replace_spec)).await;

        // Assert
        match rest_result {
            Ok(cancel_replace_order) => {
                assert_valid_cancel_replace_order(&cancel_replace_order);
                if let Some(cancelled_order) = &cancel_replace_order.cancel_order {
                    assert_eq!(cancelled_order.order_id, rest_initial_order.order_id);
                    assert_eq!(cancelled_order.symbol, test_symbol);
                }
                if let Some(new_order) = &cancel_replace_order.new_order {
                    assert_eq!(new_order.symbol, test_symbol);
                    assert_eq!(new_order.side, Some(OrderSide::Buy));
                }
            }
            Err(err) => {
                let downcast = err.downcast_ref::<BinanceError>();
                assert!(
                    matches!(
                        downcast,
                        Some(BinanceError::Api(api_err)) if matches!(api_err.code, -2011 | -2021 | -2022)
                    ),
                    "REST should get known cancel-replace error: {:#?}",
                    downcast
                );
            }
        }

        match ws_result {
            Ok(cancel_replace_order) => {
                assert_valid_cancel_replace_order(&cancel_replace_order);
                if let Some(cancelled_order) = &cancel_replace_order.cancel_order {
                    assert_eq!(cancelled_order.order_id, ws_initial_order.order_id);
                    assert_eq!(cancelled_order.symbol, test_symbol);
                }
                if let Some(new_order) = &cancel_replace_order.new_order {
                    assert_eq!(new_order.symbol, test_symbol);
                    assert_eq!(new_order.side, Some(OrderSide::Buy));
                }
            }
            Err(err) => {
                let downcast = err.downcast_ref::<BinanceError>();
                assert!(
                    matches!(
                        downcast,
                        Some(BinanceError::Api(api_err)) if matches!(api_err.code, -2011 | -2021 | -2022)
                    ),
                    "WebSocket should get known cancel-replace error: {:#?}",
                    downcast
                );
            }
        }
    }

    /**
     * Tests cancel-replace with market order for new order.
     */
    #[tokio::test]
    async fn test_cancel_replace_order_market_new_order() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "XRPUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (initial_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let min_notional = get_min_notional(symbol_info);
        let quote_quantity =
            round_to_quote_precision(min_notional * Decimal::new(15, 1), symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_cr_market_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(initial_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_cr_market_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_initial_order = with_timeout(rest_client.place_order(rest_initial_spec))
            .await
            .expect("REST place initial order");
        let ws_initial_order = with_timeout(ws_client.place_order(ws_initial_spec))
            .await
            .expect("WebSocket place initial order");

        // Act
        let rest_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol.to_string(),
            CancelReplaceMode::AllowFailure,
            OrderSide::Buy,
            OrderType::Market,
        )
        .with_cancel_order_id(rest_initial_order.order_id)
        .with_quote_order_quantity(quote_quantity)
        .with_new_client_order_id(format!("rest_cr_market_new_{}", base_id))
        .build()
        .expect("Cancel replace spec validation");

        let ws_cancel_replace_spec = CancelReplaceSpec::new(
            test_symbol.to_string(),
            CancelReplaceMode::AllowFailure,
            OrderSide::Buy,
            OrderType::Market,
        )
        .with_cancel_order_id(ws_initial_order.order_id)
        .with_quote_order_quantity(quote_quantity)
        .with_new_client_order_id(format!("ws_cr_market_new_{}", base_id))
        .build()
        .expect("Cancel replace spec validation");

        let rest_cancel_replace =
            with_timeout(rest_client.cancel_replace_order(rest_cancel_replace_spec))
                .await
                .expect("REST cancel replace order");
        let ws_cancel_replace =
            with_timeout(ws_client.cancel_replace_order(ws_cancel_replace_spec))
                .await
                .expect("WebSocket cancel replace order");

        // Assert
        assert_valid_cancel_replace_order(&rest_cancel_replace);
        assert_valid_cancel_replace_order(&ws_cancel_replace);

        if let Some(cancelled_order) = &rest_cancel_replace.cancel_order {
            assert_eq!(cancelled_order.order_id, rest_initial_order.order_id);
            assert_eq!(cancelled_order.symbol, test_symbol);
            assert_eq!(cancelled_order.status, Some(OrderStatus::Canceled));
        }

        if let Some(new_order) = &rest_cancel_replace.new_order {
            assert_eq!(new_order.symbol, test_symbol);
            assert_eq!(new_order.side, Some(OrderSide::Buy));
            assert_eq!(new_order.order_type, Some(OrderType::Market));
            assert_eq!(new_order.status, Some(OrderStatus::Filled));
        }

        if let Some(cancelled_order) = &ws_cancel_replace.cancel_order {
            assert_eq!(cancelled_order.order_id, ws_initial_order.order_id);
            assert_eq!(cancelled_order.symbol, test_symbol);
            assert_eq!(cancelled_order.status, Some(OrderStatus::Canceled));
        }

        if let Some(new_order) = &ws_cancel_replace.new_order {
            assert_eq!(new_order.symbol, test_symbol);
            assert_eq!(new_order.side, Some(OrderSide::Buy));
            assert_eq!(new_order.order_type, Some(OrderType::Market));
            assert_eq!(new_order.status, Some(OrderStatus::Filled));
        }
    }

    /**
     * Tests cancel-replace error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_cancel_replace_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let fake_order_id = 999999999;

        // Act
        let rest_spec = CancelReplaceSpec::new(
            "INVALID".to_string(),
            CancelReplaceMode::StopOnFailure,
            OrderSide::Buy,
            OrderType::Market,
        )
        .with_cancel_order_id(fake_order_id)
        .with_quote_order_quantity(Decimal::new(10, 0))
        .build()
        .expect("Cancel replace spec validation");

        let ws_spec = CancelReplaceSpec::new(
            "INVALID".to_string(),
            CancelReplaceMode::StopOnFailure,
            OrderSide::Buy,
            OrderType::Market,
        )
        .with_cancel_order_id(fake_order_id)
        .with_quote_order_quantity(Decimal::new(10, 0))
        .build()
        .expect("Cancel replace spec validation");

        let rest_result = with_timeout(rest_client.cancel_replace_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.cancel_replace_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(cancel_replace) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, cancel_replace
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -1121
                                && api_err.category == ErrorCategory::RequestIssues
                                && api_err.request_error == Some(RequestError::BadSymbol)
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }
    }

    /**
     * Tests basic order amendment functionality - reduces quantity using order_id.
     */
    #[tokio::test]
    async fn test_amend_order_basic() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "UNIUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, _) = calculate_safe_order_params(market_price, symbol_info);
        let min_notional = get_min_notional(symbol_info);
        let target_initial_quantity = (min_notional / safe_price) * Decimal::new(3, 0);
        let safe_quantity = make_quantity_step_compliant(target_initial_quantity, symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("rest_amend_basic_{}", base_id))
            .build()
            .expect("Order spec validation");

        let ws_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&format!("ws_amend_basic_{}", base_id))
            .build()
            .expect("Order spec validation");

        let rest_initial_order = with_timeout(rest_client.place_order(rest_initial_spec))
            .await
            .expect("REST place initial order");
        let ws_initial_order = with_timeout(ws_client.place_order(ws_initial_spec))
            .await
            .expect("WebSocket place initial order");

        let target_amended_quantity = (min_notional / safe_price) * Decimal::new(15, 1);
        let amended_quantity = make_quantity_step_compliant(target_amended_quantity, symbol_info);

        // Act
        let rest_amend_spec = AmendOrderSpec::new(test_symbol, amended_quantity)
            .with_order_id(rest_initial_order.order_id)
            .build()
            .expect("Amend spec validation");

        let ws_amend_spec = AmendOrderSpec::new(test_symbol, amended_quantity)
            .with_order_id(ws_initial_order.order_id)
            .build()
            .expect("Amend spec validation");

        let rest_amended_order = with_timeout(rest_client.amend_order(rest_amend_spec))
            .await
            .expect("REST amend order");
        let ws_amended_order = with_timeout(ws_client.amend_order(ws_amend_spec))
            .await
            .expect("WebSocket amend order");

        // Assert
        assert_valid_amended_order(&rest_amended_order);
        assert_valid_amended_order(&ws_amended_order);
        assert_eq!(rest_amended_order.amended_order.symbol, test_symbol);
        assert_eq!(ws_amended_order.amended_order.symbol, test_symbol);
        assert_eq!(
            rest_amended_order.amended_order.order_id,
            rest_initial_order.order_id
        );
        assert_eq!(
            ws_amended_order.amended_order.order_id,
            ws_initial_order.order_id
        );
        assert_eq!(
            rest_amended_order.amended_order.status,
            Some(OrderStatus::New)
        );
        assert_eq!(
            ws_amended_order.amended_order.status,
            Some(OrderStatus::New)
        );

        if let Some(orig_qty) = rest_amended_order.amended_order.original_quantity {
            assert_eq!(orig_qty, amended_quantity);
        }
        if let Some(orig_qty) = ws_amended_order.amended_order.original_quantity {
            assert_eq!(orig_qty, amended_quantity);
        }

        let rest_cancel_spec = CancelOrderSpec::new(test_symbol)
            .with_order_id(rest_initial_order.order_id)
            .build()
            .expect("Cancel spec validation");
        let ws_cancel_spec = CancelOrderSpec::new(test_symbol)
            .with_order_id(ws_initial_order.order_id)
            .build()
            .expect("Cancel spec validation");

        let _ = with_timeout(rest_client.cancel_order(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order(ws_cancel_spec)).await;
    }

    /**
     * Tests order amendment using original_client_order_id for identification.
     */
    #[tokio::test]
    async fn test_amend_order_by_client_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "ATOMUSDT";

        let price_spec = TickerPriceSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, _base_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let min_notional = get_min_notional(symbol_info);
        let target_initial_quantity = (min_notional / safe_price) * Decimal::new(3, 0);
        let safe_quantity = make_quantity_step_compliant(target_initial_quantity, symbol_info);
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_client_id = format!("rest_amend_client_{}", base_id);
        let ws_client_id = format!("ws_amend_client_{}", base_id);

        let rest_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&rest_client_id)
            .build()
            .expect("Order spec validation");

        let ws_initial_spec = OrderSpec::new(test_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(&ws_client_id)
            .build()
            .expect("Order spec validation");

        let rest_initial_order = with_timeout(rest_client.place_order(rest_initial_spec))
            .await
            .expect("REST place initial order");
        let ws_initial_order = with_timeout(ws_client.place_order(ws_initial_spec))
            .await
            .expect("WebSocket place initial order");

        let target_amended_quantity = (min_notional / safe_price) * Decimal::new(15, 1);
        let amended_quantity = make_quantity_step_compliant(target_amended_quantity, symbol_info);

        // Act
        let rest_amend_spec = AmendOrderSpec::new(test_symbol, amended_quantity)
            .with_original_client_order_id(rest_client_id.clone())
            .build()
            .expect("Amend spec validation");

        let ws_amend_spec = AmendOrderSpec::new(test_symbol, amended_quantity)
            .with_original_client_order_id(ws_client_id.clone())
            .build()
            .expect("Amend spec validation");

        let rest_amended_order = with_timeout(rest_client.amend_order(rest_amend_spec))
            .await
            .expect("REST amend order");
        let ws_amended_order = with_timeout(ws_client.amend_order(ws_amend_spec))
            .await
            .expect("WebSocket amend order");

        // Assert
        assert_valid_amended_order(&rest_amended_order);
        assert_valid_amended_order(&ws_amended_order);
        assert_eq!(
            rest_amended_order.amended_order.order_id,
            rest_initial_order.order_id
        );
        assert_eq!(
            ws_amended_order.amended_order.order_id,
            ws_initial_order.order_id
        );

        if let Some(orig_qty) = rest_amended_order.amended_order.original_quantity {
            assert_eq!(orig_qty, amended_quantity);
        }
        if let Some(orig_qty) = ws_amended_order.amended_order.original_quantity {
            assert_eq!(orig_qty, amended_quantity);
        }

        let rest_cancel_spec = CancelOrderSpec::new(test_symbol)
            .with_order_id(rest_initial_order.order_id)
            .build()
            .expect("Cancel spec validation");
        let ws_cancel_spec = CancelOrderSpec::new(test_symbol)
            .with_order_id(ws_initial_order.order_id)
            .build()
            .expect("Cancel spec validation");

        let _ = with_timeout(rest_client.cancel_order(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order(ws_cancel_spec)).await;
    }

    /**
     * Tests order amendment error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_amend_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = AmendOrderSpec::new("INVALID", Decimal::new(1, 0))
            .with_order_id(123456789)
            .build()
            .expect("Amend spec validation");

        let ws_spec = AmendOrderSpec::new("INVALID", Decimal::new(1, 0))
            .with_order_id(123456789)
            .build()
            .expect("Amend spec validation");

        let rest_result = with_timeout(rest_client.amend_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.amend_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(amended_order) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, amended_order
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -1121
                                && api_err.category == ErrorCategory::RequestIssues
                                && api_err.request_error == Some(RequestError::BadSymbol)
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }
    }
}
