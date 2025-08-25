#[cfg(test)]
mod tests {
    use crate::{
        clients::{tests::helpers::*, r#trait::AccountClient},
        enums::RateLimitType,
        errors::{BinanceError, ErrorCategory, RequestError},
        types::{
            requests::{
                AllOrdersSpec, AllocationSpec, CommissionRatesSpec, MyTradesSpec, OpenOrdersSpec,
                PreventedMatchesSpec, QueryOrderSpec,
            },
            responses::{
                AccountInfo, AccountTrade, Allocation, Order, PreventedMatch, RateLimit,
                SymbolCommissionRates,
            },
        },
    };
    use rust_decimal::Decimal;
    use tracing::warn;

    /**
     * Validates account info response structure.
     */
    fn assert_valid_account_info(account_info: &AccountInfo) {
        assert!(
            account_info.maker_commission >= 0,
            "Maker commission should be non-negative"
        );
        assert!(
            account_info.taker_commission >= 0,
            "Taker commission should be non-negative"
        );
        assert!(
            account_info.buyer_commission >= 0,
            "Buyer commission should be non-negative"
        );
        assert!(
            account_info.seller_commission >= 0,
            "Seller commission should be non-negative"
        );
        assert!(
            account_info.update_time > 0,
            "Update time should be positive"
        );
        assert!(
            !account_info.account_type.is_empty(),
            "Account type should not be empty"
        );
        assert!(
            !account_info.permissions.is_empty(),
            "Permissions should not be empty"
        );

        assert!(
            account_info.commission_rates.maker >= Decimal::ZERO,
            "Maker commission rate should be non-negative"
        );
        assert!(
            account_info.commission_rates.taker >= Decimal::ZERO,
            "Taker commission rate should be non-negative"
        );

        assert!(
            account_info.commission_rates.buyer >= Decimal::ZERO,
            "Buyer commission rate should be non-negative."
        );
        assert!(
            account_info.commission_rates.seller >= Decimal::ZERO,
            "Seller commission rate should be non-negative."
        );

        for balance in &account_info.balances {
            assert!(!balance.asset.is_empty(), "Asset name should not be empty");
            assert!(
                balance.free >= Decimal::ZERO,
                "Free balance should be non-negative"
            );
            assert!(
                balance.locked >= Decimal::ZERO,
                "Locked balance should be non-negative"
            );
        }
    }

    /**
     * Validates symbol commission rates response structure.
     */
    fn assert_valid_symbol_commission_rates(rates: &SymbolCommissionRates) {
        assert!(!rates.symbol.is_empty(), "Symbol should not be empty");

        assert!(
            rates.standard_commission.maker >= Decimal::ZERO,
            "Standard maker commission should be non-negative"
        );
        assert!(
            rates.standard_commission.taker >= Decimal::ZERO,
            "Standard taker commission should be non-negative"
        );
        assert!(
            rates.standard_commission.buyer >= Decimal::ZERO,
            "Standard buyer commission should be non-negative"
        );
        assert!(
            rates.standard_commission.seller >= Decimal::ZERO,
            "Standard seller commission should be non-negative"
        );

        assert!(
            rates.tax_commission.maker >= Decimal::ZERO,
            "Tax maker commission should be non-negative"
        );
        assert!(
            rates.tax_commission.taker >= Decimal::ZERO,
            "Tax taker commission should be non-negative"
        );
        assert!(
            rates.tax_commission.buyer >= Decimal::ZERO,
            "Tax buyer commission should be non-negative"
        );
        assert!(
            rates.tax_commission.seller >= Decimal::ZERO,
            "Tax seller commission should be non-negative"
        );

        assert!(
            rates.discount.discount >= Decimal::ZERO,
            "Discount should be non-negative"
        );
        assert!(
            rates.discount.discount <= Decimal::ONE,
            "Discount should not exceed 1.0"
        );
    }

    /**
     * Validates rate limit response structure.
     */
    fn assert_valid_rate_limit(rate_limit: &RateLimit) {
        assert!(
            rate_limit.interval_num > 0,
            "Interval number should be positive"
        );
        assert!(rate_limit.limit > 0, "Limit should be positive");

        if let Some(count) = rate_limit.count {
            assert!(count <= rate_limit.limit, "Count should not exceed limit");
        }
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

        if let Some(price) = order.price {
            assert!(
                price >= Decimal::ZERO,
                "Price should be positive when present"
            );
        }

        if let Some(origin_quantity) = order.original_quantity {
            assert!(
                origin_quantity > Decimal::ZERO,
                "Origin quantity should be positive when present"
            );
        }

        if let Some(executed_quantity) = order.executed_quantity {
            assert!(
                executed_quantity >= Decimal::ZERO,
                "Executed quantity should be non-negative when present"
            );
        }

        if let Some(cumulative_quote_qty) = order.cumulative_quote_quantity {
            assert!(
                cumulative_quote_qty >= Decimal::ZERO,
                "Cumulative quote quantity should be non-negative when present"
            );
        }
    }

    /**
     * Validates account trade response structure.
     */
    fn assert_valid_account_trade(trade: &AccountTrade) {
        assert!(!trade.symbol.is_empty(), "Symbol should not be empty");
        assert!(trade.id > 0, "Trade ID should be positive");
        assert!(trade.order_id > 0, "Order ID should be positive");
        assert!(trade.price > Decimal::ZERO, "Price should be positive");
        assert!(
            trade.quantity > Decimal::ZERO,
            "Quantity should be positive"
        );
        assert!(
            trade.quote_quantity > Decimal::ZERO,
            "Quote quantity should be positive"
        );
        assert!(
            trade.commission >= Decimal::ZERO,
            "Commission should be non-negative"
        );
        assert!(
            !trade.commission_asset.is_empty(),
            "Commission asset should not be empty"
        );
        assert!(trade.time > 0, "Trade time should be positive");
    }

    /**
     * Validates prevented match response structure.
     */
    fn assert_valid_prevented_match(prevented_match: &PreventedMatch) {
        assert!(
            !prevented_match.symbol.is_empty(),
            "Symbol should not be empty"
        );
        assert!(
            prevented_match.prevented_match_id > 0,
            "Prevented match ID should be positive"
        );
        assert!(
            prevented_match.taker_order_id > 0,
            "Taker order ID should be positive"
        );
        assert!(
            !prevented_match.maker_symbol.is_empty(),
            "Maker symbol should not be empty"
        );
        assert!(
            prevented_match.maker_order_id > 0,
            "Maker order ID should be positive"
        );
        assert!(
            prevented_match.price > Decimal::ZERO,
            "Price should be positive"
        );
        assert!(
            prevented_match.maker_prevented_quantity > Decimal::ZERO,
            "Maker prevented quantity should be positive"
        );
        assert!(
            prevented_match.transaction_time > 0,
            "Transaction time should be positive"
        );
    }

    /**
     * Validates allocation response structure.
     */
    fn assert_valid_allocation(allocation: &Allocation) {
        assert!(!allocation.symbol.is_empty(), "Symbol should not be empty");
        assert!(allocation.order_id > 0, "Order ID should be positive");
        assert!(allocation.price > Decimal::ZERO, "Price should be positive");
        assert!(
            allocation.quantity > Decimal::ZERO,
            "Quantity should be positive"
        );
        assert!(
            allocation.quote_quantity > Decimal::ZERO,
            "Quote quantity should be positive"
        );
        assert!(
            allocation.commission >= Decimal::ZERO,
            "Commission should be non-negative"
        );
        assert!(
            !allocation.commission_asset.is_empty(),
            "Commission asset should not be empty"
        );
        assert!(allocation.time > 0, "Allocation time should be positive");
    }

    /**
     * Tests account info retrieval for authenticated clients.
     */
    #[tokio::test]
    async fn test_account_info() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_account_info = with_timeout(rest_client.account_info())
            .await
            .expect("REST account info");
        let ws_account_info = with_timeout(ws_client.account_info())
            .await
            .expect("WebSocket account info");

        // Assert
        assert_valid_account_info(&rest_account_info);
        assert_valid_account_info(&ws_account_info);
        assert_eq!(rest_account_info.account_type, ws_account_info.account_type);
        assert_eq!(rest_account_info.can_trade, ws_account_info.can_trade);
        assert_eq!(rest_account_info.can_withdraw, ws_account_info.can_withdraw);
        assert_eq!(rest_account_info.can_deposit, ws_account_info.can_deposit);
    }

    /**
     * Tests commission rates retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_commission_rates() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = CommissionRatesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_rates = with_timeout(rest_client.commission_rates(rest_spec))
            .await
            .expect("REST commission rates");

        let ws_spec = CommissionRatesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_rates = with_timeout(ws_client.commission_rates(ws_spec))
            .await
            .expect("WebSocket commission rates");

        // Assert
        assert_valid_symbol_commission_rates(&rest_rates);
        assert_valid_symbol_commission_rates(&ws_rates);
        assert_eq!(rest_rates.symbol, test_symbol);
        assert_eq!(ws_rates.symbol, test_symbol);
        assert_eq!(
            rest_rates.standard_commission.maker,
            ws_rates.standard_commission.maker
        );
        assert_eq!(
            rest_rates.standard_commission.taker,
            ws_rates.standard_commission.taker
        );
    }

    /**
     * Tests commission rates error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_commission_rates_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = CommissionRatesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.commission_rates(rest_spec)).await;

        let ws_spec = CommissionRatesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.commission_rates(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(rates) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, rates
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
     * Tests rate limits retrieval for authenticated clients.
     */
    #[tokio::test]
    async fn test_rate_limits() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_rate_limits = with_timeout(rest_client.rate_limits())
            .await
            .expect("REST rate limits");
        let ws_rate_limits = with_timeout(ws_client.rate_limits())
            .await
            .expect("WebSocket rate limits");

        // Assert
        assert!(
            !rest_rate_limits.is_empty(),
            "REST should return at least one rate limit"
        );
        assert!(
            !ws_rate_limits.is_empty(),
            "WebSocket should return at least one rate limit"
        );
        for rate_limit in &rest_rate_limits {
            assert_valid_rate_limit(rate_limit);
        }
        for rate_limit in &ws_rate_limits {
            assert_valid_rate_limit(rate_limit);
        }
        assert!(
            rest_rate_limits
                .iter()
                .any(|rl| matches!(rl.rate_limit_type, RateLimitType::Orders)),
            "Should have ORDERS rate limit"
        );
        assert!(
            ws_rate_limits
                .iter()
                .any(|rl| matches!(rl.rate_limit_type, RateLimitType::Orders)),
            "Should have ORDERS rate limit"
        );
    }

    /**
     * Tests order status retrieval by order ID.
     * Note: This test requires an existing order ID, so it may fail if no orders exist.
     * In practice, you'd need to create an order first or use a known order ID.
     */
    #[tokio::test]
    async fn test_order_status_by_order_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let all_orders_spec = AllOrdersSpec::new(test_symbol)
            .with_limit(1)
            .build()
            .expect("Spec validation");
        let orders = with_timeout(rest_client.all_orders(all_orders_spec))
            .await
            .expect("Get orders to find order ID");
        if orders.is_empty() {
            warn!(symbol = %test_symbol, "Skipping order_status test - no existing orders found");
            return;
        }

        let order_id = orders[0].order_id;

        // Act
        let rest_spec = QueryOrderSpec::new(test_symbol)
            .with_order_id(order_id)
            .build()
            .expect("Spec validation");
        let rest_order = with_timeout(rest_client.order_status(rest_spec))
            .await
            .expect("REST order status");

        let ws_spec = QueryOrderSpec::new(test_symbol)
            .with_order_id(order_id)
            .build()
            .expect("Spec validation");
        let ws_order = with_timeout(ws_client.order_status(ws_spec))
            .await
            .expect("WebSocket order status");

        // Assert
        assert_valid_order(&rest_order);
        assert_valid_order(&ws_order);
        assert_eq!(rest_order.order_id, order_id);
        assert_eq!(ws_order.order_id, order_id);
        assert_eq!(rest_order.symbol, test_symbol);
        assert_eq!(ws_order.symbol, test_symbol);
        assert_eq!(rest_order.order_id, ws_order.order_id);
        assert_eq!(rest_order.client_order_id, ws_order.client_order_id);
    }

    /**
     * Tests order status retrieval by client order ID.
     * Note: This test requires an existing client order ID, so it may fail if no orders exist.
     */
    #[tokio::test]
    async fn test_order_status_by_client_order_id() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let all_orders_spec = AllOrdersSpec::new(test_symbol)
            .with_limit(1)
            .build()
            .expect("Spec validation");
        let orders = with_timeout(rest_client.all_orders(all_orders_spec))
            .await
            .expect("Get orders to find client order ID");

        if orders.is_empty() {
            warn!(symbol = %test_symbol, "Skipping order_status by client order ID test - no existing orders found");
            return;
        }

        let client_order_id = orders[0].client_order_id.clone();

        // Act
        let rest_spec = QueryOrderSpec::new(test_symbol)
            .with_original_client_order_id(client_order_id.clone())
            .build()
            .expect("Spec validation");
        let rest_order = with_timeout(rest_client.order_status(rest_spec))
            .await
            .expect("REST order status");

        let ws_spec = QueryOrderSpec::new(test_symbol)
            .with_original_client_order_id(client_order_id.clone())
            .build()
            .expect("Spec validation");
        let ws_order = with_timeout(ws_client.order_status(ws_spec))
            .await
            .expect("WebSocket order status");

        // Assert
        assert_valid_order(&rest_order);
        assert_valid_order(&ws_order);
        assert_eq!(rest_order.client_order_id, client_order_id);
        assert_eq!(ws_order.client_order_id, client_order_id);
        assert_eq!(rest_order.symbol, test_symbol);
        assert_eq!(ws_order.symbol, test_symbol);
        assert_eq!(rest_order.order_id, ws_order.order_id);
        assert_eq!(rest_order.client_order_id, ws_order.client_order_id);
    }

    /**
     * Tests order status error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_order_status_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let fake_order_id = 999999999;

        // Act
        let rest_spec = QueryOrderSpec::new("INVALID")
            .with_order_id(fake_order_id)
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.order_status(rest_spec)).await;

        let ws_spec = QueryOrderSpec::new("INVALID")
            .with_order_id(fake_order_id)
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.order_status(ws_spec)).await;

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
     * Tests open orders retrieval with no symbol filter.
     */
    #[tokio::test]
    async fn test_open_orders_all_symbols() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OpenOrdersSpec::new().build().expect("Spec validation");
        let rest_open_orders = with_timeout(rest_client.open_orders(rest_spec))
            .await
            .expect("REST open orders");

        let ws_spec = OpenOrdersSpec::new().build().expect("Spec validation");
        let ws_open_orders = with_timeout(ws_client.open_orders(ws_spec))
            .await
            .expect("WebSocket open orders");

        // Assert
        for order in &rest_open_orders {
            assert_valid_order(order);
        }
        for order in &ws_open_orders {
            assert_valid_order(order);
        }
        assert_eq!(
            rest_open_orders.len(),
            ws_open_orders.len(),
            "Open orders count should match"
        );
    }

    /**
     * Tests open orders retrieval with specific symbol filter.
     */
    #[tokio::test]
    async fn test_open_orders_specific_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = OpenOrdersSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_open_orders = with_timeout(rest_client.open_orders(rest_spec))
            .await
            .expect("REST open orders");

        let ws_spec = OpenOrdersSpec::new()
            .with_symbol(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_open_orders = with_timeout(ws_client.open_orders(ws_spec))
            .await
            .expect("WebSocket open orders");

        // Assert
        for order in &rest_open_orders {
            assert_valid_order(order);
            assert_eq!(
                order.symbol, test_symbol,
                "Order symbol should match filter"
            );
        }
        for order in &ws_open_orders {
            assert_valid_order(order);
            assert_eq!(
                order.symbol, test_symbol,
                "Order symbol should match filter"
            );
        }
        assert_eq!(
            rest_open_orders.len(),
            ws_open_orders.len(),
            "Open orders count should match"
        );
    }

    /**
     * Tests open orders error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_open_orders_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OpenOrdersSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.open_orders(rest_spec)).await;

        let ws_spec = OpenOrdersSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.open_orders(ws_spec)).await;

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
     * Tests all orders retrieval with default parameters.
     */
    #[tokio::test]
    async fn test_all_orders_default_params() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = AllOrdersSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_all_orders = with_timeout(rest_client.all_orders(rest_spec))
            .await
            .expect("REST all orders");

        let ws_spec = AllOrdersSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_all_orders = with_timeout(ws_client.all_orders(ws_spec))
            .await
            .expect("WebSocket all orders");

        // Assert
        for order in &rest_all_orders[..5.min(rest_all_orders.len())] {
            assert_valid_order(order);
            assert_eq!(order.symbol, test_symbol, "Order symbol should match");
        }

        for order in &ws_all_orders[..5.min(ws_all_orders.len())] {
            assert_valid_order(order);
            assert_eq!(order.symbol, test_symbol, "Order symbol should match");
        }
        assert!(
            rest_all_orders.len() <= 500,
            "REST orders should not exceed default limit"
        );
        assert!(
            ws_all_orders.len() <= 500,
            "WebSocket orders should not exceed default limit"
        );
    }

    /**
     * Tests all orders retrieval with custom limit.
     */
    #[tokio::test]
    async fn test_all_orders_custom_limit() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let limit = 100;

        // Act
        let rest_spec = AllOrdersSpec::new(test_symbol)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let rest_all_orders = with_timeout(rest_client.all_orders(rest_spec))
            .await
            .expect("REST all orders");

        let ws_spec = AllOrdersSpec::new(test_symbol)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let ws_all_orders = with_timeout(ws_client.all_orders(ws_spec))
            .await
            .expect("WebSocket all orders");

        // Assert
        for order in &rest_all_orders {
            assert_valid_order(order);
            assert_eq!(order.symbol, test_symbol, "Order symbol should match");
        }
        for order in &ws_all_orders {
            assert_valid_order(order);
            assert_eq!(order.symbol, test_symbol, "Order symbol should match");
        }
        assert!(
            rest_all_orders.len() <= limit as usize,
            "REST orders should not exceed custom limit"
        );
        assert!(
            ws_all_orders.len() <= limit as usize,
            "WebSocket orders should not exceed custom limit"
        );
    }

    /**
     * Tests all orders error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_all_orders_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = AllOrdersSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.all_orders(rest_spec)).await;

        let ws_spec = AllOrdersSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.all_orders(ws_spec)).await;

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
     * Tests my trades retrieval with default parameters.
     */
    #[tokio::test]
    async fn test_my_trades_default_params() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = MyTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_my_trades = with_timeout(rest_client.my_trades(rest_spec))
            .await
            .expect("REST my trades");

        let ws_spec = MyTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_my_trades = with_timeout(ws_client.my_trades(ws_spec))
            .await
            .expect("WebSocket my trades");

        // Assert
        for trade in &rest_my_trades[..5.min(rest_my_trades.len())] {
            assert_valid_account_trade(trade);
            assert_eq!(trade.symbol, test_symbol, "Trade symbol should match");
        }
        for trade in &ws_my_trades[..5.min(ws_my_trades.len())] {
            assert_valid_account_trade(trade);
            assert_eq!(trade.symbol, test_symbol, "Trade symbol should match");
        }
        assert!(
            rest_my_trades.len() <= 500,
            "REST trades should not exceed default limit"
        );
        assert!(
            ws_my_trades.len() <= 500,
            "WebSocket trades should not exceed default limit"
        );
    }

    /**
     * Tests my trades retrieval with time range.
     */
    #[tokio::test]
    async fn test_my_trades_time_range() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let end_time = chrono::Utc::now().timestamp_millis() as u64;
        let start_time = end_time - (24 * 60 * 60 * 1000);

        // Act
        let rest_spec = MyTradesSpec::new(test_symbol)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .build()
            .expect("Spec validation");
        let rest_my_trades = with_timeout(rest_client.my_trades(rest_spec))
            .await
            .expect("REST my trades");

        let ws_spec = MyTradesSpec::new(test_symbol)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .build()
            .expect("Spec validation");
        let ws_my_trades = with_timeout(ws_client.my_trades(ws_spec))
            .await
            .expect("WebSocket my trades");

        // Assert
        for trade in &rest_my_trades {
            assert_valid_account_trade(trade);
            assert_eq!(trade.symbol, test_symbol, "Trade symbol should match");
            assert!(
                trade.time >= start_time,
                "Trade time should be >= start_time"
            );
            assert!(trade.time <= end_time, "Trade time should be <= end_time");
        }
        for trade in &ws_my_trades {
            assert_valid_account_trade(trade);
            assert_eq!(trade.symbol, test_symbol, "Trade symbol should match");
            assert!(
                trade.time >= start_time,
                "Trade time should be >= start_time"
            );
            assert!(trade.time <= end_time, "Trade time should be <= end_time");
        }
    }

    /**
     * Tests my trades error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_my_trades_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = MyTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.my_trades(rest_spec)).await;

        let ws_spec = MyTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.my_trades(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(trades) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, trades
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
     * Tests prevented matches retrieval with order ID filter.
     */
    #[tokio::test]
    async fn test_prevented_matches_order_id_filter() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let all_orders_spec = AllOrdersSpec::new(test_symbol)
            .with_limit(1)
            .build()
            .expect("Spec validation");
        let orders = with_timeout(rest_client.all_orders(all_orders_spec))
            .await
            .expect("Get orders to find order ID");
        if orders.is_empty() {
            warn!(symbol = %test_symbol, "Skipping prevented matches test - no existing orders found");
            return;
        }
        let order_id = orders[0].order_id;

        // Act
        let rest_spec = PreventedMatchesSpec::new(test_symbol)
            .with_order_id(order_id)
            .build()
            .expect("Spec validation");
        let rest_prevented_matches = with_timeout(rest_client.prevented_matches(rest_spec))
            .await
            .expect("REST prevented matches");

        let ws_spec = PreventedMatchesSpec::new(test_symbol)
            .with_order_id(order_id)
            .build()
            .expect("Spec validation");
        let ws_prevented_matches = with_timeout(ws_client.prevented_matches(ws_spec))
            .await
            .expect("WebSocket prevented matches");

        // Assert
        for prevented_match in &rest_prevented_matches {
            assert_valid_prevented_match(prevented_match);
            assert_eq!(
                prevented_match.symbol, test_symbol,
                "Prevented match symbol should match"
            );
        }
        for prevented_match in &ws_prevented_matches {
            assert_valid_prevented_match(prevented_match);
            assert_eq!(
                prevented_match.symbol, test_symbol,
                "Prevented match symbol should match"
            );
        }
        assert!(
            rest_prevented_matches.len() <= 500,
            "REST prevented matches should not exceed default limit"
        );
        assert!(
            ws_prevented_matches.len() <= 500,
            "WebSocket prevented matches should not exceed default limit"
        );
    }

    /**
     * Tests prevented matches retrieval with custom limit.
     */
    #[tokio::test]
    async fn test_prevented_matches_custom_limit() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let limit = 100;

        let all_orders_spec = AllOrdersSpec::new(test_symbol)
            .with_limit(1)
            .build()
            .expect("Spec validation");
        let orders = with_timeout(rest_client.all_orders(all_orders_spec))
            .await
            .expect("Get orders to find order ID");
        if orders.is_empty() {
            warn!(symbol = %test_symbol, "Skipping prevented matches custom limit test - no existing orders found");
            return;
        }
        let order_id = orders[0].order_id;

        // Act
        let rest_spec = PreventedMatchesSpec::new(test_symbol)
            .with_order_id(order_id)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let rest_prevented_matches = with_timeout(rest_client.prevented_matches(rest_spec))
            .await
            .expect("REST prevented matches");

        let ws_spec = PreventedMatchesSpec::new(test_symbol)
            .with_order_id(order_id)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let ws_prevented_matches = with_timeout(ws_client.prevented_matches(ws_spec))
            .await
            .expect("WebSocket prevented matches");

        // Assert
        for prevented_match in &rest_prevented_matches {
            assert_valid_prevented_match(prevented_match);
            assert_eq!(
                prevented_match.symbol, test_symbol,
                "Prevented match symbol should match"
            );
        }
        for prevented_match in &ws_prevented_matches {
            assert_valid_prevented_match(prevented_match);
            assert_eq!(
                prevented_match.symbol, test_symbol,
                "Prevented match symbol should match"
            );
        }
        assert!(
            rest_prevented_matches.len() <= limit as usize,
            "REST prevented matches should not exceed custom limit"
        );
        assert!(
            ws_prevented_matches.len() <= limit as usize,
            "WebSocket prevented matches should not exceed custom limit"
        );
    }

    /**
     * Tests prevented matches error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_prevented_matches_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let fake_order_id = 999999999;

        // Act
        let rest_spec = PreventedMatchesSpec::new("INVALID")
            .with_order_id(fake_order_id)
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.prevented_matches(rest_spec)).await;

        let ws_spec = PreventedMatchesSpec::new("INVALID")
            .with_order_id(fake_order_id)
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.prevented_matches(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(matches) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, matches
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
     * Tests allocations retrieval with symbol filter.
     */
    #[tokio::test]
    async fn test_allocations_symbol_filter() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = AllocationSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_allocations = with_timeout(rest_client.allocations(rest_spec))
            .await
            .expect("REST allocations");

        let ws_spec = AllocationSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_allocations = with_timeout(ws_client.allocations(ws_spec))
            .await
            .expect("WebSocket allocations");

        // Assert
        for allocation in &rest_allocations {
            assert_valid_allocation(allocation);
            assert_eq!(
                allocation.symbol, test_symbol,
                "Allocation symbol should match"
            );
        }
        for allocation in &ws_allocations {
            assert_valid_allocation(allocation);
            assert_eq!(
                allocation.symbol, test_symbol,
                "Allocation symbol should match"
            );
        }
        assert!(
            rest_allocations.len() <= 500,
            "REST allocations should not exceed default limit"
        );
        assert!(
            ws_allocations.len() <= 500,
            "WebSocket allocations should not exceed default limit"
        );
    }

    /**
     * Tests allocations retrieval with time range.
     */
    #[tokio::test]
    async fn test_allocations_time_range() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let end_time = chrono::Utc::now().timestamp_millis() as u64;
        let start_time = end_time - (24 * 60 * 60 * 1000);

        // Act
        let rest_spec = AllocationSpec::new(test_symbol)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .build()
            .expect("Spec validation");
        let rest_allocations = with_timeout(rest_client.allocations(rest_spec))
            .await
            .expect("REST allocations");

        let ws_spec = AllocationSpec::new(test_symbol)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .build()
            .expect("Spec validation");
        let ws_allocations = with_timeout(ws_client.allocations(ws_spec))
            .await
            .expect("WebSocket allocations");

        // Assert
        for allocation in &rest_allocations {
            assert_valid_allocation(allocation);
            assert_eq!(
                allocation.symbol, test_symbol,
                "Allocation symbol should match"
            );
            assert!(
                allocation.time >= start_time,
                "Allocation time should be >= start_time"
            );
            assert!(
                allocation.time <= end_time,
                "Allocation time should be <= end_time"
            );
        }
        for allocation in &ws_allocations {
            assert_valid_allocation(allocation);
            assert_eq!(
                allocation.symbol, test_symbol,
                "Allocation symbol should match"
            );
            assert!(
                allocation.time >= start_time,
                "Allocation time should be >= start_time"
            );
            assert!(
                allocation.time <= end_time,
                "Allocation time should be <= end_time"
            );
        }
    }

    /**
     * Tests allocations error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_allocations_invalid_symbol() {
        // Arrange
        let rest_client = create_authenticated_rest_client().expect("REST client creation");
        let ws_client = create_authenticated_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = AllocationSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.allocations(rest_spec)).await;

        let ws_spec = AllocationSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.allocations(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(allocations) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, allocations
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
