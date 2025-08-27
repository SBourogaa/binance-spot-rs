#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;
    use serial_test::serial;

    use crate::{
        clients::{
            tests::helpers::*,
            r#trait::{GeneralClient, TickerClient, TradingClient},
        },
        enums::{OrderListOrderStatus, OrderSide, OrderStatus, OrderType, TimeInForce},
        errors::{BinanceError, ErrorCategory, RequestError},
        filters::SymbolFilter,
        types::{
            requests::{
                AllOrderListsSpec, CancelOrderListSpec, CancelOrderSpec, ExchangeInfoSpec,
                OcoOrderSpec, OpenOrderListsSpec, OrderListStatusSpec, OtoOrderSpec,
                OtocoOrderSpec, SorOrderSpec, TickerPriceSpec,
            },
            responses::{Order, OrderList, SymbolInfo, TestOrder},
        },
    };

    /**
     * Gets SOR-enabled symbols from exchange info.
     *
     * Returns the first available SOR symbol or None if SOR is not available.
     */
    async fn get_sor_enabled_symbol() -> Option<String> {
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let exchange_info_spec = ExchangeInfoSpec::new()
            .build()
            .expect("Exchange info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(exchange_info_spec))
            .await
            .expect("Get exchange info");

        for sor_info in &exchange_info.sors {
            if !sor_info.symbols.is_empty() {
                return Some(sor_info.symbols[0].clone());
            }
        }
        None
    }

    /**
     * Gets the minimum notional value for a symbol from MIN_NOTIONAL or NOTIONAL filters.
     */
    fn get_min_notional(symbol_info: &SymbolInfo) -> Decimal {
        if let Some(min_notional_filter) = symbol_info.min_notional_filter() {
            return Decimal::from_str(&min_notional_filter.min_notional)
                .unwrap_or(Decimal::new(10, 0)); // Default $10
        }

        for filter in &symbol_info.filters {
            if let SymbolFilter::Notional(notional_filter) = filter {
                return Decimal::from_str(&notional_filter.min_notional)
                    .unwrap_or(Decimal::new(10, 0)); // Default $10
            }
        }

        Decimal::new(10, 0)
    }

    fn calculate_oco_safe_quantity(
        take_profit_price: Decimal,
        stop_loss_price: Decimal,
        symbol_info: &SymbolInfo,
    ) -> Decimal {
        let min_notional = get_min_notional(symbol_info);

        let lower_price = if stop_loss_price < take_profit_price {
            stop_loss_price
        } else {
            take_profit_price
        };

        let min_quantity_for_notional = (min_notional / lower_price) * Decimal::new(15, 1); // 1.5x margin

        make_quantity_step_compliant(min_quantity_for_notional, symbol_info)
    }

    fn calculate_otoco_safe_quantity(
        working_price: Decimal,
        limit_maker_price: Decimal,
        stop_loss_price: Decimal,
        symbol_info: &SymbolInfo,
    ) -> Decimal {
        let min_notional = get_min_notional(symbol_info);
        
        let lowest_price = working_price.min(limit_maker_price.min(stop_loss_price));
        
        let min_quantity_for_notional = (min_notional / lowest_price) * Decimal::new(15, 1); // 1.5x margin
        
        make_quantity_step_compliant(min_quantity_for_notional, symbol_info)
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
     *
     * Returns (price, quantity) that will pass PRICE_FILTER, LOT_SIZE, and MIN_NOTIONAL filters.
     */
    fn calculate_safe_order_params(
        market_price: Decimal,
        symbol_info: &SymbolInfo,
    ) -> (Decimal, Decimal) {
        let target_price = market_price * Decimal::new(95, 2); // 0.95
        let safe_price = make_price_tick_compliant(target_price, symbol_info);

        let min_notional = get_min_notional(symbol_info);

        let min_quantity_for_notional = (min_notional / safe_price) * Decimal::new(12, 1);

        let safe_quantity = make_quantity_step_compliant(min_quantity_for_notional, symbol_info);

        let final_notional = safe_price * safe_quantity;
        if final_notional < min_notional {
            let adjusted_quantity = (min_notional / safe_price) * Decimal::new(15, 1);
            let final_safe_quantity = make_quantity_step_compliant(adjusted_quantity, symbol_info);
            return (safe_price, final_safe_quantity);
        }

        (safe_price, safe_quantity)
    }

    /**
     * Validates SOR order response structure.
     */
    fn assert_valid_sor_order(order: &Order) {
        assert!(!order.symbol.is_empty(), "Symbol should not be empty");
        assert!(order.order_id > 0, "Order ID should be positive");
        assert!(
            !order.client_order_id.is_empty(),
            "Client order ID should not be empty"
        );

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
     * Validates SOR test order response structure.
     */
    fn assert_valid_sor_test_order(test_order: &TestOrder) {
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
     * Validates OCO order list response structure.
     */
    fn assert_valid_oco_order_list(order_list: &OrderList) {
        assert!(
            order_list.order_list_id > 0,
            "Order list ID should be positive"
        );
        assert!(!order_list.symbol.is_empty(), "Symbol should not be empty");
        assert!(
            !order_list.orders.is_empty(),
            "Order list should contain orders"
        );

        assert_eq!(
            order_list.orders.len(),
            2,
            "OCO should have exactly 2 orders"
        );

        for order in &order_list.orders {
            assert!(order.order_id > 0, "Order ID should be positive");
            assert!(!order.symbol.is_empty(), "Order symbol should not be empty");
        }
    }

    /**
     * Validates order list response structure.
     */
    fn assert_valid_order_list(order_list: &OrderList) {
        assert!(
            order_list.order_list_id > 0,
            "Order list ID should be positive"
        );
        assert!(!order_list.symbol.is_empty(), "Symbol should not be empty");
        assert!(
            !order_list.orders.is_empty(),
            "Order list should contain orders"
        );

        for order in &order_list.orders {
            assert!(order.order_id > 0, "Order ID should be positive");
            assert!(!order.symbol.is_empty(), "Order symbol should not be empty");
        }
    }

    /**
     * Validates a vector of order lists.
     */
    fn assert_valid_order_lists(order_lists: &[OrderList]) {
        for order_list in order_lists {
            assert_valid_order_list(order_list);
        }
    }

    /**
     * Validates that an order list is in an open/active state.
     */
    fn assert_valid_open_order_list(order_list: &OrderList) {
        assert_valid_order_list(order_list);

        assert_eq!(
            order_list.list_order_status,
            OrderListOrderStatus::Executing,
            "Open order list should be in executing state"
        );
    }

    /**
     * Helper function to cleanup all open order lists after tests.
     */
    async fn cleanup_all_open_order_lists() {
        let rest_client = match create_authenticated_rest_client() {
            Ok(client) => client,
            Err(_) => return,
        };

        let open_spec = match OpenOrderListsSpec::new().build() {
            Ok(spec) => spec,
            Err(_) => return,
        };

        let open_lists = match with_timeout(rest_client.open_order_lists(open_spec)).await {
            Ok(lists) => lists,
            Err(_) => return,
        };

        if open_lists.is_empty() {
            return;
        }

        println!("Cleaning up {} open order lists...", open_lists.len());

        let mut cancelled_count = 0;
        for order_list in &open_lists {
            let cancel_spec = match CancelOrderListSpec::new(&order_list.symbol)
                .with_order_list_id(order_list.order_list_id)
                .build()
            {
                Ok(spec) => spec,
                Err(_) => continue,
            };

            if (with_timeout(rest_client.cancel_order_list(cancel_spec)).await).is_ok() {
                cancelled_count += 1;
            }
        }

        if cancelled_count > 0 {
            println!(
                "🧹 Cleanup complete: {}/{} order lists cancelled.",
                cancelled_count,
                open_lists.len()
            );
        }
    }

    /**
     * Tests SOR order placement with limit order.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_sor_order_limit() {
        // Arrange
        let sor_symbol = match get_sor_enabled_symbol().await {
            Some(symbol) => symbol,
            None => {
                println!("Skipping SOR test - no SOR-enabled symbols found on testnet");
                return;
            }
        };

        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        let price_spec = TickerPriceSpec::new()
            .with_symbol(&sor_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(&sor_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(format!("rest_sor_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let ws_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(format!("ws_sor_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let rest_order = with_timeout(rest_client.place_sor_order(rest_spec))
            .await
            .expect("REST place SOR order");
        let ws_order = with_timeout(ws_client.place_sor_order(ws_spec))
            .await
            .expect("WebSocket place SOR order");

        // Assert
        assert_valid_sor_order(&rest_order);
        assert_valid_sor_order(&ws_order);
        assert_eq!(rest_order.symbol, sor_symbol);
        assert_eq!(ws_order.symbol, sor_symbol);

        // Cleanup
        let rest_cancel_spec = CancelOrderSpec::new(&sor_symbol)
            .with_order_id(rest_order.order_id)
            .build()
            .expect("Cancel spec validation");
        let ws_cancel_spec = CancelOrderSpec::new(&sor_symbol)
            .with_order_id(ws_order.order_id)
            .build()
            .expect("Cancel spec validation");

        let _ = with_timeout(rest_client.cancel_order(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests SOR order placement with market orders.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_sor_order_market() {
        // Arrange
        let sor_symbol = match get_sor_enabled_symbol().await {
            Some(symbol) => symbol,
            None => {
                println!("Skipping SOR test - no SOR-enabled symbols found on testnet");
                return;
            }
        };

        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(&sor_symbol)
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

        let rest_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(format!("rest_sor_market_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let ws_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(format!("ws_sor_market_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let rest_order = with_timeout(rest_client.place_sor_order(rest_spec))
            .await
            .expect("REST place SOR order");
        let ws_order = with_timeout(ws_client.place_sor_order(ws_spec))
            .await
            .expect("WebSocket place SOR order");

        // Assert
        assert_valid_sor_order(&rest_order);
        assert_valid_sor_order(&ws_order);
        assert_eq!(rest_order.symbol, sor_symbol);
        assert_eq!(ws_order.symbol, sor_symbol);

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests SOR order validation with limit orders without execution.
     */
    #[tokio::test]
    #[serial]
    async fn test_test_sor_order_limit() {
        // Arrange
        let sor_symbol = match get_sor_enabled_symbol().await {
            Some(symbol) => symbol,
            None => {
                println!("Skipping SOR test - no SOR-enabled symbols found on testnet");
                return;
            }
        };

        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        let price_spec = TickerPriceSpec::new()
            .with_symbol(&sor_symbol)
            .build()
            .expect("Price spec validation");
        let current_prices = with_timeout(rest_client.ticker_price(price_spec))
            .await
            .expect("Get current price");
        let market_price = current_prices[0].price;

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(&sor_symbol)
            .build()
            .expect("Info spec validation");
        let exchange_info = with_timeout(rest_client.exchange_info(info_spec))
            .await
            .expect("Get exchange info");
        let symbol_info = &exchange_info.symbols[0];

        let (safe_price, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(format!("rest_test_sor_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let ws_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .with_client_order_id(format!("ws_test_sor_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let rest_test_order = with_timeout(rest_client.test_sor_order(rest_spec))
            .await
            .expect("REST test SOR order");
        let ws_test_order = with_timeout(ws_client.test_sor_order(ws_spec))
            .await
            .expect("WebSocket test SOR order");

        // Assert
        assert_valid_sor_test_order(&rest_test_order);
        assert_valid_sor_test_order(&ws_test_order);
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

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests SOR order validation with market orders without execution.
     */
    #[tokio::test]
    #[serial]
    async fn test_test_sor_order_market() {
        // Arrange
        let sor_symbol = match get_sor_enabled_symbol().await {
            Some(symbol) => symbol,
            None => {
                println!("Skipping SOR test - no SOR-enabled symbols found on testnet");
                return;
            }
        };

        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        let info_spec = ExchangeInfoSpec::new()
            .with_symbol(&sor_symbol)
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

        let rest_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(format!("rest_test_sor_market_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let ws_spec = SorOrderSpec::new(&sor_symbol, OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(quote_quantity)
            .with_client_order_id(format!("ws_test_sor_market_{}", base_id))
            .build()
            .expect("SOR order spec validation");

        let rest_test_order = with_timeout(rest_client.test_sor_order(rest_spec))
            .await
            .expect("REST test SOR order");
        let ws_test_order = with_timeout(ws_client.test_sor_order(ws_spec))
            .await
            .expect("WebSocket test SOR order");

        // Assert
        assert_valid_sor_test_order(&rest_test_order);
        assert_valid_sor_test_order(&ws_test_order);
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

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests SOR order validation error handling with invalid symbol.
     */
    #[tokio::test]
    #[serial]
    async fn test_test_sor_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = SorOrderSpec::new("INVALID", OrderSide::Buy, OrderType::Limit)
            .with_quantity(Decimal::new(1, 4))
            .with_price(Decimal::new(20000, 0))
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("SOR order spec validation");

        let ws_spec = SorOrderSpec::new("INVALID", OrderSide::Buy, OrderType::Limit)
            .with_quantity(Decimal::new(1, 4))
            .with_price(Decimal::new(20000, 0))
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("SOR order spec validation");

        let rest_result = with_timeout(rest_client.test_sor_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.test_sor_order(ws_spec)).await;

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

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OCO order placement with TAKE_PROFIT_LIMIT above and STOP_LOSS_LIMIT below.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_oco_order_take_profit_stop_loss() {
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

        let take_profit_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let safe_quantity =
            calculate_oco_safe_quantity(take_profit_price, stop_loss_price, symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::TakeProfitLimit,
            OrderType::StopLossLimit,
        )
        .with_above_price(take_profit_price)
        .with_above_stop_price(take_profit_price)
        .with_above_time_in_force(TimeInForce::GTC)
        .with_below_price(stop_loss_price)
        .with_below_stop_price(stop_loss_price)
        .with_below_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("rest_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let ws_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::TakeProfitLimit,
            OrderType::StopLossLimit,
        )
        .with_above_price(take_profit_price)
        .with_above_stop_price(take_profit_price)
        .with_above_time_in_force(TimeInForce::GTC)
        .with_below_price(stop_loss_price)
        .with_below_stop_price(stop_loss_price)
        .with_below_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("ws_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_spec))
            .await
            .expect("WebSocket place OCO order");

        // Assert
        assert_valid_oco_order_list(&rest_order_list);
        assert_valid_oco_order_list(&ws_order_list);
        assert_eq!(rest_order_list.symbol, test_symbol);
        assert_eq!(ws_order_list.symbol, test_symbol);

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OCO order placement with LIMIT_MAKER above and STOP_LOSS below.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_oco_order_limit_maker_stop_loss() {
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

        let (_, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);

        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("rest_oco_lm_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let ws_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("ws_oco_lm_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_spec))
            .await
            .expect("WebSocket place OCO order");

        // Assert
        assert_valid_oco_order_list(&rest_order_list);
        assert_valid_oco_order_list(&ws_order_list);
        assert_eq!(rest_order_list.symbol, test_symbol);
        assert_eq!(ws_order_list.symbol, test_symbol);

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OCO order placement error handling with invalid symbol.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_oco_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OcoOrderSpec::new(
            "INVALID",
            OrderSide::Sell,
            Decimal::new(1, 4),
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(Decimal::new(50000, 0))
        .with_below_stop_price(Decimal::new(30000, 0))
        .build()
        .expect("OCO order spec validation");

        let ws_spec = OcoOrderSpec::new(
            "INVALID",
            OrderSide::Sell,
            Decimal::new(1, 4),
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(Decimal::new(50000, 0))
        .with_below_stop_price(Decimal::new(30000, 0))
        .build()
        .expect("OCO order spec validation");

        let rest_result = with_timeout(rest_client.place_oco_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.place_oco_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order_list) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order_list
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

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OTO order placement with LIMIT working order triggering MARKET contingent order.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_oto_order_limit_market() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BNBUSDT";

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
        let min_notional = get_min_notional(symbol_info);
        let contingent_quote_qty =
            round_to_quote_precision(min_notional * Decimal::new(15, 1), symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OtoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            safe_price,
            safe_quantity,
            OrderType::Market,
            OrderSide::Buy,
            contingent_quote_qty,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("rest_oto_{}", base_id))
        .build()
        .expect("OTO order spec validation");

        let ws_spec = OtoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            safe_price,
            safe_quantity,
            OrderType::Market,
            OrderSide::Buy,
            contingent_quote_qty,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("ws_oto_{}", base_id))
        .build()
        .expect("OTO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oto_order(rest_spec))
            .await
            .expect("REST place OTO order");
        let ws_order_list = with_timeout(ws_client.place_oto_order(ws_spec))
            .await
            .expect("WebSocket place OTO order");

        // Assert
        assert_valid_oco_order_list(&rest_order_list);
        assert_valid_oco_order_list(&ws_order_list);
        assert_eq!(rest_order_list.symbol, test_symbol);
        assert_eq!(ws_order_list.symbol, test_symbol);

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OTO order placement with LIMIT working order triggering LIMIT contingent order.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_oto_order_limit_limit() {
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
        let contingent_price =
            make_price_tick_compliant(market_price * Decimal::new(102, 2), symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OtoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            safe_price,
            safe_quantity,
            OrderType::Limit,
            OrderSide::Buy,
            safe_quantity,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_price(contingent_price)
        .with_pending_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("rest_oto_ll_{}", base_id))
        .build()
        .expect("OTO order spec validation");

        let ws_spec = OtoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            safe_price,
            safe_quantity,
            OrderType::Limit,
            OrderSide::Buy,
            safe_quantity,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_price(contingent_price)
        .with_pending_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("ws_oto_ll_{}", base_id))
        .build()
        .expect("OTO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oto_order(rest_spec))
            .await
            .expect("REST place OTO order");
        let ws_order_list = with_timeout(ws_client.place_oto_order(ws_spec))
            .await
            .expect("WebSocket place OTO order");

        // Assert
        assert_valid_oco_order_list(&rest_order_list);
        assert_valid_oco_order_list(&ws_order_list);
        assert_eq!(rest_order_list.symbol, test_symbol);
        assert_eq!(ws_order_list.symbol, test_symbol);

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OTO order placement error handling with invalid symbol.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_oto_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OtoOrderSpec::new(
            "INVALID",
            OrderType::Limit,
            OrderSide::Buy,
            Decimal::new(50000, 0),
            Decimal::new(1, 4),
            OrderType::Market,
            OrderSide::Buy,
            Decimal::new(50, 0),
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .build()
        .expect("OTO order spec validation");

        let ws_spec = OtoOrderSpec::new(
            "INVALID",
            OrderType::Limit,
            OrderSide::Buy,
            Decimal::new(50000, 0),
            Decimal::new(1, 4),
            OrderType::Market,
            OrderSide::Buy,
            Decimal::new(50, 0),
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .build()
        .expect("OTO order spec validation");

        let rest_result = with_timeout(rest_client.place_oto_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.place_oto_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order_list) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order_list
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

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OTOCO order placement with LIMIT working order triggering TAKE_PROFIT_LIMIT/STOP_LOSS_LIMIT OCO.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_otoco_order_limit_take_profit_stop_loss() {
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

        let (working_price, working_quantity) =
            calculate_safe_order_params(market_price, symbol_info);
        let take_profit_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OtocoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            working_price,
            working_quantity,
            OrderSide::Sell,
            working_quantity,
            OrderType::TakeProfitLimit,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_above_price(take_profit_price)
        .with_pending_above_stop_price(take_profit_price)
        .with_pending_above_time_in_force(TimeInForce::GTC)
        .with_pending_below_type(OrderType::StopLossLimit)
        .with_pending_below_price(stop_loss_price)
        .with_pending_below_stop_price(stop_loss_price)
        .with_pending_below_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("rest_otoco_{}", base_id))
        .build()
        .expect("OTOCO order spec validation");

        let ws_spec = OtocoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            working_price,
            working_quantity,
            OrderSide::Sell,
            working_quantity,
            OrderType::TakeProfitLimit,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_above_price(take_profit_price)
        .with_pending_above_stop_price(take_profit_price)
        .with_pending_above_time_in_force(TimeInForce::GTC)
        .with_pending_below_type(OrderType::StopLossLimit)
        .with_pending_below_price(stop_loss_price)
        .with_pending_below_stop_price(stop_loss_price)
        .with_pending_below_time_in_force(TimeInForce::GTC)
        .with_list_client_order_id(format!("ws_otoco_{}", base_id))
        .build()
        .expect("OTOCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_otoco_order(rest_spec))
            .await
            .expect("REST place OTOCO order");
        let ws_order_list = with_timeout(ws_client.place_otoco_order(ws_spec))
            .await
            .expect("WebSocket place OTOCO order");

        // Assert
        assert!(
            rest_order_list.order_list_id > 0,
            "Order list ID should be positive"
        );
        assert_eq!(rest_order_list.symbol, test_symbol);
        assert_eq!(
            rest_order_list.orders.len(),
            3,
            "OTOCO should have exactly 3 orders"
        );

        assert!(
            ws_order_list.order_list_id > 0,
            "Order list ID should be positive"
        );
        assert_eq!(ws_order_list.symbol, test_symbol);
        assert_eq!(
            ws_order_list.orders.len(),
            3,
            "OTOCO should have exactly 3 orders"
        );

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OTOCO order placement with LIMIT working order triggering LIMIT_MAKER/STOP_LOSS OCO.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_otoco_order_limit_maker_stop_loss() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "DOGEUSDT";

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

        let (working_price, _) = calculate_safe_order_params(market_price, symbol_info);
        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);
        
        let working_quantity = calculate_otoco_safe_quantity(working_price, limit_maker_price, stop_loss_price, symbol_info);

        // Act
        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_spec = OtocoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            working_price,
            working_quantity,
            OrderSide::Sell,
            working_quantity,
            OrderType::LimitMaker,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_above_price(limit_maker_price)
        .with_pending_below_type(OrderType::StopLoss)
        .with_pending_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("rest_otoco_lm_{}", base_id))
        .build()
        .expect("OTOCO order spec validation");

        let ws_spec = OtocoOrderSpec::new(
            test_symbol,
            OrderType::Limit,
            OrderSide::Buy,
            working_price,
            working_quantity,
            OrderSide::Sell,
            working_quantity,
            OrderType::LimitMaker,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_above_price(limit_maker_price)
        .with_pending_below_type(OrderType::StopLoss)
        .with_pending_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("ws_otoco_lm_{}", base_id))
        .build()
        .expect("OTOCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_otoco_order(rest_spec))
            .await
            .expect("REST place OTOCO order");
        let ws_order_list = with_timeout(ws_client.place_otoco_order(ws_spec))
            .await
            .expect("WebSocket place OTOCO order");

        // Assert
        assert!(
            rest_order_list.order_list_id > 0,
            "Order list ID should be positive"
        );
        assert_eq!(rest_order_list.symbol, test_symbol);
        assert_eq!(
            rest_order_list.orders.len(),
            3,
            "OTOCO should have exactly 3 orders"
        );

        assert!(
            ws_order_list.order_list_id > 0,
            "Order list ID should be positive"
        );
        assert_eq!(ws_order_list.symbol, test_symbol);
        assert_eq!(
            ws_order_list.orders.len(),
            3,
            "OTOCO should have exactly 3 orders"
        );

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests OTOCO order placement error handling with invalid symbol.
     */
    #[tokio::test]
    #[serial]
    async fn test_place_otoco_order_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OtocoOrderSpec::new(
            "INVALID",
            OrderType::Limit,
            OrderSide::Buy,
            Decimal::new(50000, 0),
            Decimal::new(1, 4),
            OrderSide::Sell,
            Decimal::new(1, 4),
            OrderType::LimitMaker,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_above_price(Decimal::new(55000, 0))
        .with_pending_below_type(OrderType::StopLoss)
        .with_pending_below_stop_price(Decimal::new(45000, 0))
        .build()
        .expect("OTOCO order spec validation");

        let ws_spec = OtocoOrderSpec::new(
            "INVALID",
            OrderType::Limit,
            OrderSide::Buy,
            Decimal::new(50000, 0),
            Decimal::new(1, 4),
            OrderSide::Sell,
            Decimal::new(1, 4),
            OrderType::LimitMaker,
        )
        .with_working_time_in_force(TimeInForce::GTC)
        .with_pending_above_price(Decimal::new(55000, 0))
        .with_pending_below_type(OrderType::StopLoss)
        .with_pending_below_stop_price(Decimal::new(45000, 0))
        .build()
        .expect("OTOCO order spec validation");

        let rest_result = with_timeout(rest_client.place_otoco_order(rest_spec)).await;
        let ws_result = with_timeout(ws_client.place_otoco_order(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order_list) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order_list
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

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests cancel order list functionality by creating and canceling OCO orders.
     */
    #[tokio::test]
    #[serial]
    async fn test_cancel_order_list_oco() {
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

        let (_, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("rest_cancel_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let ws_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("ws_cancel_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_oco_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_oco_spec))
            .await
            .expect("WebSocket place OCO order");

        // Act
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let rest_cancelled = with_timeout(rest_client.cancel_order_list(rest_cancel_spec))
            .await
            .expect("REST cancel order list");
        let ws_cancelled = with_timeout(ws_client.cancel_order_list(ws_cancel_spec))
            .await
            .expect("WebSocket cancel order list");

        // Assert
        assert_valid_oco_order_list(&rest_cancelled);
        assert_valid_oco_order_list(&ws_cancelled);
        assert_eq!(rest_cancelled.symbol, test_symbol);
        assert_eq!(ws_cancelled.symbol, test_symbol);
        assert_eq!(rest_cancelled.order_list_id, rest_order_list.order_list_id);
        assert_eq!(ws_cancelled.order_list_id, ws_order_list.order_list_id);

        assert_eq!(
            rest_cancelled.list_order_status,
            OrderListOrderStatus::AllDone
        );
        assert_eq!(
            ws_cancelled.list_order_status,
            OrderListOrderStatus::AllDone
        );

        if !rest_cancelled.order_reports.is_empty() {
            for order in &rest_cancelled.order_reports {
                assert_eq!(order.status, Some(OrderStatus::Canceled));
            }
        }
        if !ws_cancelled.order_reports.is_empty() {
            for order in &ws_cancelled.order_reports {
                assert_eq!(order.status, Some(OrderStatus::Canceled));
            }
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests cancel order list functionality with client order ID.
     */
    #[tokio::test]
    #[serial]
    async fn test_cancel_order_list_by_client_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "SUIUSDT";

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

        let (_, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let base_id = chrono::Utc::now().timestamp_millis();
        let rest_client_id = format!("rest_cancel_client_{}", base_id);
        let ws_client_id = format!("ws_cancel_client_{}", base_id);

        let rest_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(&rest_client_id)
        .build()
        .expect("OCO order spec validation");

        let ws_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(&ws_client_id)
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_oco_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_oco_spec))
            .await
            .expect("WebSocket place OCO order");

        // Act
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_list_client_order_id(&rest_client_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_list_client_order_id(&ws_client_id)
            .build()
            .expect("Cancel order list spec validation");

        let rest_cancelled = with_timeout(rest_client.cancel_order_list(rest_cancel_spec))
            .await
            .expect("REST cancel order list");
        let ws_cancelled = with_timeout(ws_client.cancel_order_list(ws_cancel_spec))
            .await
            .expect("WebSocket cancel order list");

        // Assert
        assert_valid_oco_order_list(&rest_cancelled);
        assert_valid_oco_order_list(&ws_cancelled);
        assert_eq!(rest_cancelled.symbol, test_symbol);
        assert_eq!(ws_cancelled.symbol, test_symbol);
        assert_eq!(rest_cancelled.order_list_id, rest_order_list.order_list_id);
        assert_eq!(ws_cancelled.order_list_id, ws_order_list.order_list_id);

        assert_eq!(
            rest_cancelled.list_order_status,
            OrderListOrderStatus::AllDone
        );
        assert_eq!(
            ws_cancelled.list_order_status,
            OrderListOrderStatus::AllDone
        );

        if !rest_cancelled.order_reports.is_empty() {
            for order in &rest_cancelled.order_reports {
                assert_eq!(order.status, Some(OrderStatus::Canceled));
            }
        }
        if !ws_cancelled.order_reports.is_empty() {
            for order in &ws_cancelled.order_reports {
                assert_eq!(order.status, Some(OrderStatus::Canceled));
            }
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests cancel order list error handling with invalid order list ID.
     */
    #[tokio::test]
    #[serial]
    async fn test_cancel_order_list_invalid_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_cancel_spec = CancelOrderListSpec::new("BTCUSDT")
            .with_order_list_id(999999999)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new("BTCUSDT")
            .with_order_list_id(999999999)
            .build()
            .expect("Cancel order list spec validation");

        let rest_result = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let ws_result = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order_list) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order_list
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -2011
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests order list status query by order list ID.
     */
    #[tokio::test]
    #[serial]
    async fn test_order_list_status_by_id() {
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

        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let safe_quantity =
            calculate_oco_safe_quantity(limit_maker_price, stop_loss_price, symbol_info);

        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("rest_status_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let ws_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("ws_status_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_oco_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_oco_spec))
            .await
            .expect("WebSocket place OCO order");

        // Act
        let rest_status_spec = OrderListStatusSpec::new()
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Order list status spec validation");
        let ws_status_spec = OrderListStatusSpec::new()
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Order list status spec validation");

        let rest_status = with_timeout(rest_client.order_list_status(rest_status_spec))
            .await
            .expect("REST order list status");
        let ws_status = with_timeout(ws_client.order_list_status(ws_status_spec))
            .await
            .expect("WebSocket order list status");

        // Assert
        assert_valid_oco_order_list(&rest_status);
        assert_valid_oco_order_list(&ws_status);
        assert_eq!(rest_status.symbol, test_symbol);
        assert_eq!(ws_status.symbol, test_symbol);
        assert_eq!(rest_status.order_list_id, rest_order_list.order_list_id);
        assert_eq!(ws_status.order_list_id, ws_order_list.order_list_id);

        assert_eq!(
            rest_status.list_order_status,
            OrderListOrderStatus::Executing
        );
        assert_eq!(ws_status.list_order_status, OrderListOrderStatus::Executing);

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests order list status query by client order ID.
     */
    #[tokio::test]
    #[serial]
    async fn test_order_list_status_by_client_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "TRXUSDT";

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

        let (_, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let base_id = chrono::Utc::now().timestamp_millis();
        let rest_client_id = format!("rest_status_client_{}", base_id);
        let ws_client_id = format!("ws_status_client_{}", base_id);

        let rest_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(&rest_client_id)
        .build()
        .expect("OCO order spec validation");

        let ws_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(&ws_client_id)
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_oco_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_oco_spec))
            .await
            .expect("WebSocket place OCO order");

        // Act
        let rest_status_spec = OrderListStatusSpec::new()
            .with_original_client_order_id(&rest_client_id)
            .build()
            .expect("Order list status spec validation");
        let ws_status_spec = OrderListStatusSpec::new()
            .with_original_client_order_id(&ws_client_id)
            .build()
            .expect("Order list status spec validation");

        let rest_status = with_timeout(rest_client.order_list_status(rest_status_spec))
            .await
            .expect("REST order list status");
        let ws_status = with_timeout(ws_client.order_list_status(ws_status_spec))
            .await
            .expect("WebSocket order list status");

        // Assert
        assert_valid_oco_order_list(&rest_status);
        assert_valid_oco_order_list(&ws_status);
        assert_eq!(rest_status.symbol, test_symbol);
        assert_eq!(ws_status.symbol, test_symbol);
        assert_eq!(rest_status.order_list_id, rest_order_list.order_list_id);
        assert_eq!(ws_status.order_list_id, ws_order_list.order_list_id);

        assert_eq!(
            rest_status.list_order_status,
            OrderListOrderStatus::Executing
        );
        assert_eq!(ws_status.list_order_status, OrderListOrderStatus::Executing);

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests order list status error handling with invalid order list ID.
     */
    #[tokio::test]
    #[serial]
    async fn test_order_list_status_invalid_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_status_spec = OrderListStatusSpec::new()
            .with_order_list_id(999999999)
            .build()
            .expect("Order list status spec validation");
        let ws_status_spec = OrderListStatusSpec::new()
            .with_order_list_id(999999999)
            .build()
            .expect("Order list status spec validation");

        let rest_result = with_timeout(rest_client.order_list_status(rest_status_spec)).await;
        let ws_result = with_timeout(ws_client.order_list_status(ws_status_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order_list) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order_list
                ),
                Err(err) => {
                    let downcast = err.downcast_ref::<BinanceError>();
                    assert!(
                        matches!(
                            downcast,
                            Some(BinanceError::Api(api_err))
                                if api_err.code == -2018
                        ),
                        "Unexpected {} error: {:#?}",
                        client_name,
                        downcast
                    );
                }
            }
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests all order lists query with default parameters.
     */
    #[tokio::test]
    #[serial]
    async fn test_all_order_lists_default() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "ARBUSDT";

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

        let (_, safe_quantity) = calculate_safe_order_params(market_price, symbol_info);
        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("rest_all_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let ws_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("ws_all_oco_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_oco_spec))
            .await
            .expect("REST place OCO order");
        let ws_order_list = with_timeout(ws_client.place_oco_order(ws_oco_spec))
            .await
            .expect("WebSocket place OCO order");

        // Act
        let rest_all_spec = AllOrderListsSpec::new()
            .build()
            .expect("All order lists spec validation");
        let ws_all_spec = AllOrderListsSpec::new()
            .build()
            .expect("All order lists spec validation");

        let rest_all_lists = with_timeout(rest_client.all_order_lists(rest_all_spec))
            .await
            .expect("REST all order lists");
        let ws_all_lists = with_timeout(ws_client.all_order_lists(ws_all_spec))
            .await
            .expect("WebSocket all order lists");

        // Assert
        assert_valid_order_lists(&rest_all_lists);
        assert_valid_order_lists(&ws_all_lists);

        let rest_contains_our_order = rest_all_lists
            .iter()
            .any(|ol| ol.order_list_id == rest_order_list.order_list_id);
        let ws_contains_our_order = ws_all_lists
            .iter()
            .any(|ol| ol.order_list_id == ws_order_list.order_list_id);
        assert!(
            rest_contains_our_order,
            "REST all order lists should contain our created order list"
        );
        assert!(
            ws_contains_our_order,
            "WebSocket all order lists should contain our created order list"
        );

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");
        let ws_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(ws_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;
        let _ = with_timeout(ws_client.cancel_order_list(ws_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests all order lists query with time range filter.
     */
    #[tokio::test]
    #[serial]
    async fn test_all_order_lists_with_time_range() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        let now = chrono::Utc::now().timestamp_millis() as u64;
        let one_hour_ago = now - 3600000;

        // Act
        let rest_all_spec = AllOrderListsSpec::new()
            .with_start_time(one_hour_ago)
            .with_end_time(now)
            .build()
            .expect("All order lists spec validation");
        let ws_all_spec = AllOrderListsSpec::new()
            .with_start_time(one_hour_ago)
            .with_end_time(now)
            .build()
            .expect("All order lists spec validation");

        let rest_all_lists = with_timeout(rest_client.all_order_lists(rest_all_spec))
            .await
            .expect("REST all order lists");
        let ws_all_lists = with_timeout(ws_client.all_order_lists(ws_all_spec))
            .await
            .expect("WebSocket all order lists");

        // Assert
        assert_valid_order_lists(&rest_all_lists);
        assert_valid_order_lists(&ws_all_lists);

        for order_list in &rest_all_lists {
            assert!(
                order_list.transaction_time >= one_hour_ago && order_list.transaction_time <= now,
                "Order list transaction time {} should be within range {} to {}",
                order_list.transaction_time,
                one_hour_ago,
                now
            );
        }
        for order_list in &ws_all_lists {
            assert!(
                order_list.transaction_time >= one_hour_ago && order_list.transaction_time <= now,
                "Order list transaction time {} should be within range {} to {}",
                order_list.transaction_time,
                one_hour_ago,
                now
            );
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests all order lists query with limit parameter.
     */
    #[tokio::test]
    #[serial]
    async fn test_all_order_lists_with_limit() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_all_spec = AllOrderListsSpec::new()
            .with_limit(5)
            .build()
            .expect("All order lists spec validation");
        let ws_all_spec = AllOrderListsSpec::new()
            .with_limit(5)
            .build()
            .expect("All order lists spec validation");

        let rest_all_lists = with_timeout(rest_client.all_order_lists(rest_all_spec))
            .await
            .expect("REST all order lists");
        let ws_all_lists = with_timeout(ws_client.all_order_lists(ws_all_spec))
            .await
            .expect("WebSocket all order lists");

        // Assert
        assert_valid_order_lists(&rest_all_lists);
        assert_valid_order_lists(&ws_all_lists);

        if !rest_all_lists.is_empty() {
            assert!(
                rest_all_lists.len() <= 5,
                "REST result should respect the limit of 5"
            );
        }
        if !ws_all_lists.is_empty() {
            assert!(
                ws_all_lists.len() <= 5,
                "WebSocket result should respect the limit of 5"
            );
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests open order lists query with default parameters.
     */
    #[tokio::test]
    #[serial]
    async fn test_open_order_lists_default() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_open_spec = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");
        let ws_open_spec = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");

        let rest_open_lists = with_timeout(rest_client.open_order_lists(rest_open_spec))
            .await
            .expect("REST open order lists");
        let ws_open_lists = with_timeout(ws_client.open_order_lists(ws_open_spec))
            .await
            .expect("WebSocket open order lists");

        // Assert
        assert_valid_order_lists(&rest_open_lists);
        assert_valid_order_lists(&ws_open_lists);

        for order_list in &rest_open_lists {
            assert_valid_open_order_list(order_list);
        }
        for order_list in &ws_open_lists {
            assert_valid_open_order_list(order_list);
        }

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests open order lists query with active OCO order in place.
     */
    #[tokio::test]
    #[serial]
    async fn test_open_order_lists_with_active_order() {
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

        let limit_maker_price =
            make_price_tick_compliant(market_price * Decimal::new(105, 2), symbol_info);
        let stop_loss_price =
            make_price_tick_compliant(market_price * Decimal::new(90, 2), symbol_info);

        let safe_quantity =
            calculate_oco_safe_quantity(limit_maker_price, stop_loss_price, symbol_info);

        let base_id = chrono::Utc::now().timestamp_millis();

        let rest_oco_spec = OcoOrderSpec::new(
            test_symbol,
            OrderSide::Sell,
            safe_quantity,
            OrderType::LimitMaker,
            OrderType::StopLoss,
        )
        .with_above_price(limit_maker_price)
        .with_below_stop_price(stop_loss_price)
        .with_list_client_order_id(format!("rest_open_test_{}", base_id))
        .build()
        .expect("OCO order spec validation");

        let rest_order_list = with_timeout(rest_client.place_oco_order(rest_oco_spec))
            .await
            .expect("REST OCO order creation");

        // Act
        let rest_open_spec = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");
        let ws_open_spec = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");

        let rest_open_lists = with_timeout(rest_client.open_order_lists(rest_open_spec))
            .await
            .expect("REST open order lists");
        let ws_open_lists = with_timeout(ws_client.open_order_lists(ws_open_spec))
            .await
            .expect("WebSocket open order lists");

        // Assert
        assert_valid_order_lists(&rest_open_lists);
        assert_valid_order_lists(&ws_open_lists);

        for order_list in &rest_open_lists {
            assert_valid_open_order_list(order_list);
        }
        for order_list in &ws_open_lists {
            assert_valid_open_order_list(order_list);
        }

        let rest_contains_our_order = rest_open_lists
            .iter()
            .any(|ol| ol.order_list_id == rest_order_list.order_list_id);
        assert!(
            rest_contains_our_order,
            "REST open order lists should contain our created order list"
        );

        // Cleanup
        let rest_cancel_spec = CancelOrderListSpec::new(test_symbol)
            .with_order_list_id(rest_order_list.order_list_id)
            .build()
            .expect("Cancel order list spec validation");

        let _ = with_timeout(rest_client.cancel_order_list(rest_cancel_spec)).await;

        cleanup_all_open_order_lists().await;
    }

    /**
     * Tests open order lists query behavior.
     */
    #[tokio::test]
    #[serial]
    async fn test_open_order_lists_behavior() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_open_spec1 = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");
        let rest_open_spec2 = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");
        let ws_open_spec = OpenOrderListsSpec::new()
            .build()
            .expect("Open order lists spec validation");

        let rest_open_lists1 = with_timeout(rest_client.open_order_lists(rest_open_spec1))
            .await
            .expect("REST open order lists 1");
        let rest_open_lists2 = with_timeout(rest_client.open_order_lists(rest_open_spec2))
            .await
            .expect("REST open order lists 2");
        let ws_open_lists = with_timeout(ws_client.open_order_lists(ws_open_spec))
            .await
            .expect("WebSocket open order lists");

        // Assert
        assert_valid_order_lists(&rest_open_lists1);
        assert_valid_order_lists(&rest_open_lists2);
        assert_valid_order_lists(&ws_open_lists);

        for order_list in &rest_open_lists1 {
            assert_valid_open_order_list(order_list);
        }
        for order_list in &rest_open_lists2 {
            assert_valid_open_order_list(order_list);
        }
        for order_list in &ws_open_lists {
            assert_valid_open_order_list(order_list);
        }

        cleanup_all_open_order_lists().await;
    }
}
