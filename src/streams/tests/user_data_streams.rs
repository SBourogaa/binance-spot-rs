#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::str::FromStr;

    use tracing::debug;
    use serial_test::serial;
    use rust_decimal::Decimal;

    use crate::Result;

    use crate::streams::client::TypedSubscription;
    use crate::{BinanceConfig, StreamConfig, RestConfig, WebSocketConfig};
    use crate::streams::{BinanceSpotStreamClient, specs::*};
    use crate::streams::events::*;
    use crate::streams::connection::ConnectionStatus;
    use crate::clients::r#trait::{TradingClient, TickerClient, GeneralClient, BinanceSpotClient};
    use crate::types::requests::{OrderSpec, CancelOrderSpec, TickerPriceSpec, ExchangeInfoSpec};
    use crate::types::responses::SymbolInfo;
    use crate::filters::SymbolFilter;
    use crate::enums::{OrderSide, OrderType, TimeInForce};
    use crate::{rest, websocket};

    /**
     * Wraps stream operations with timeout to prevent hanging tests.
     */
    async fn with_timeout<T>(
        future: impl std::future::Future<Output = Result<T>>
    ) -> Result<T> {
        tokio::time::timeout(Duration::from_secs(30), future)
            .await
            .map_err(|_| anyhow::anyhow!("Request timed out"))?
    }

    /**
     * Wraps subscription recv operations with timeout.
     */
    async fn with_recv_timeout<T>(
        future: impl std::future::Future<Output = std::result::Result<T, tokio::sync::broadcast::error::RecvError>>
    ) -> Result<T> {
        tokio::time::timeout(Duration::from_secs(30), future)
            .await
            .map_err(|_| anyhow::anyhow!("Request timed out"))?
            .map_err(|e| anyhow::anyhow!("Receive error: {:?}", e))
    }

    /**
     * Creates a user data stream client for testnet with authentication.
     */
    fn create_user_data_stream_client() -> Result<BinanceSpotStreamClient> {
        let config = BinanceConfig::<StreamConfig>::builder()
            .with_testnet()
            .with_credentials_from_file(
                std::env::var("BINANCE_TESTNET_API_KEY").expect("BINANCE_TESTNET_API_KEY required"),
                std::env::var("BINANCE_TESTNET_PEM_FILE").expect("BINANCE_TESTNET_PEM_FILE required")
            )?
            .with_user_data()
            .build()?;
        crate::streams::client(config)
    }

    /**
     * Creates a REST API client for testnet with authentication.
     */
    fn create_rest_client() -> Result<impl TradingClient> {
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .with_credentials_from_file(
                std::env::var("BINANCE_TESTNET_API_KEY").expect("BINANCE_TESTNET_API_KEY required"),
                std::env::var("BINANCE_TESTNET_PEM_FILE").expect("BINANCE_TESTNET_PEM_FILE required")
            )?
            .build()?;
        rest::client(config)
    }

    /**
     * Creates a WebSocket API client for testnet with authentication.
     */
    async fn create_websocket_client() -> Result<impl TradingClient> {
        let config = BinanceConfig::<WebSocketConfig>::builder()
            .with_testnet()
            .with_credentials_from_file(
                std::env::var("BINANCE_TESTNET_API_KEY").expect("BINANCE_TESTNET_API_KEY required"),
                std::env::var("BINANCE_TESTNET_PEM_FILE").expect("BINANCE_TESTNET_PEM_FILE required")
            )?
            .build()?;
        let client = websocket::client(config)?;
        Ok(client)
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
            let tick_size = Decimal::from_str(&price_filter.tick_size)
                .unwrap_or(Decimal::new(1, 2));
            
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
            let step_size = Decimal::from_str(&lot_size_filter.step_size)
                .unwrap_or(Decimal::new(1, 6));
            let min_qty = Decimal::from_str(&lot_size_filter.min_qty)
                .unwrap_or(Decimal::ZERO);
                
            if step_size > Decimal::ZERO {
                let steps = (quantity / step_size).floor();
                let compliant_qty = steps * step_size;
                return compliant_qty.max(min_qty);
            }
        }
        quantity
    }

    /**
     * Calculates safe order parameters that comply with all symbol filters.
     * Returns (price, quantity) that will pass PRICE_FILTER, LOT_SIZE, and MIN_NOTIONAL.
     */
    fn calculate_safe_order_params(market_price: Decimal, symbol_info: &SymbolInfo) -> (Decimal, Decimal) {
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
     * Creates an authenticated REST client for testnet (needed for market data).
     */
    fn create_authenticated_rest_client() -> Result<impl BinanceSpotClient> {
        let config = BinanceConfig::<RestConfig>::builder()
            .with_testnet()
            .with_credentials_from_file(
                std::env::var("BINANCE_TESTNET_API_KEY").expect("BINANCE_TESTNET_API_KEY required"),
                std::env::var("BINANCE_TESTNET_PEM_FILE").expect("BINANCE_TESTNET_PEM_FILE required")
            )?
            .build()?;
        rest::client(config)
    }

    /**
     * Gets realistic BTCUSDT trading parameters by fetching current market data.
     */
    async fn get_safe_btcusdt_order_params() -> Result<(Decimal, Decimal)> {
        let rest_client = create_authenticated_rest_client()?;
        let test_symbol = "BTCUSDT";
        
        // Get current market price
        let price_spec = TickerPriceSpec::new().with_symbol(test_symbol).build()?;
        let current_prices = rest_client.ticker_price(price_spec).await?;
        let market_price = current_prices[0].price;
        
        // Get symbol info for filters
        let info_spec = ExchangeInfoSpec::new().with_symbol(test_symbol).build()?;
        let exchange_info = rest_client.exchange_info(info_spec).await?;
        let symbol_info = &exchange_info.symbols[0];
        
        Ok(calculate_safe_order_params(market_price, symbol_info))
    }


    /**
     * Validates execution report event structure.
     */
    fn assert_valid_execution_report_event(event: &ExecutionReportEvent) {
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert!(event.order_id > 0, "Order ID should be positive");
        assert!(!event.client_order_id.is_empty(), "Client order ID should not be empty");
        assert!(event.price >= Decimal::ZERO, "Price should be non-negative");
        assert!(event.quantity > Decimal::ZERO, "Quantity should be positive");
        assert!(event.last_executed_quantity >= Decimal::ZERO, "Last executed quantity should be non-negative");
        assert!(event.cumulative_quote_quantity >= Decimal::ZERO, "Cumulative quote quantity should be non-negative");
        assert!(event.transaction_time > 0, "Transaction time should be positive");
    }

    /**
     * Validates outbound account position event structure.
     */
    fn assert_valid_account_position_event(event: &OutboundAccountPositionEvent) {
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.last_update_time > 0, "Last update time should be positive");
        assert!(!event.balances.is_empty(), "Should have at least one balance");
        
        for balance in &event.balances {
            assert!(!balance.asset.is_empty(), "Asset should not be empty");
            assert!(balance.free >= Decimal::ZERO, "Free balance should be non-negative");
            assert!(balance.locked >= Decimal::ZERO, "Locked balance should be non-negative");
        }
    }

    /**
     * Validates balance update event structure.
     */
    fn assert_valid_balance_update_event(event: &BalanceUpdateEvent) {
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(!event.asset.is_empty(), "Asset should not be empty");
        assert!(event.clear_time > 0, "Clear time should be positive");
    }

    /**
     * Waits for a specific user data event type, skipping other event types.
     * This is useful when multiple events are sent by Binance in response to an action.
     */
    async fn wait_for_execution_report(
        subscription: &mut TypedSubscription<UserDataEvent>,
        max_attempts: usize,
    ) -> Result<ExecutionReportEvent> {
        for _attempt in 0..max_attempts {
            match with_recv_timeout(subscription.recv()).await? {
                UserDataEvent::ExecutionReport(event) => return Ok(event),
                _other => {
                    // Skip other event types and continue waiting for ExecutionReport
                }
            }
        }
        Err(anyhow::anyhow!("ExecutionReport not received within {} attempts", max_attempts))
    }

    /**
     * Tests basic user data stream connection and authentication.
     */
    #[tokio::test]
    async fn test_user_data_stream_connection() {
        // Arrange
        let mut client = create_user_data_stream_client().expect("Client creation");
        
        // Act & Assert
        let connection_result = with_timeout(client.wait_for_connection()).await;
        assert!(connection_result.is_ok(), "Connection should succeed with valid credentials");
        assert!(client.is_connected(), "Client should report as connected");
        assert_eq!(client.connection_status(), ConnectionStatus::Connected);
        
        let _ = with_timeout(client.close()).await;
    }

    /**
     * Tests user data stream subscription functionality.
     */
    #[tokio::test]
    async fn test_user_data_stream_subscription() {
        // Arrange
        let mut client = create_user_data_stream_client().expect("Client creation");
        let spec = UserDataStreamSpec::new();
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let _subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        
        // Assert
        let _ = with_timeout(client.close()).await;
    }

    /**
     * Tests execution report events by placing and canceling a REST API order.
     */
    #[tokio::test]
    #[serial]
    async fn test_execution_report_via_rest_order() {
        // Arrange
        let mut stream_client = create_user_data_stream_client().expect("Stream client creation");
        let rest_client = create_rest_client().expect("REST client creation");
        let spec = UserDataStreamSpec::new();
        
        let (safe_price, safe_quantity) = get_safe_btcusdt_order_params().await.expect("Get safe order params");
        
        // Act
        let _ = with_timeout(stream_client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(stream_client.subscribe(&spec)).await.expect("Subscription");
        
        let order_spec = OrderSpec::new("BTCUSDT", OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("Order spec creation");
            
        let order_result = rest_client.place_order(order_spec).await.expect("Order placement");
        
        let execution_report = wait_for_execution_report(&mut subscription, 5).await.expect("Receive execution report");
        
        // Assert
        assert_valid_execution_report_event(&execution_report);
        assert_eq!(execution_report.symbol, "BTCUSDT");
        assert_eq!(execution_report.order_id, order_result.order_id);
        assert_eq!(execution_report.side, OrderSide::Buy);
        assert_eq!(execution_report.order_type, OrderType::Limit);
        
        let cancel_spec = CancelOrderSpec::new("BTCUSDT")
            .with_order_id(order_result.order_id)
            .build()
            .expect("Cancel spec creation");
            
        let _ = rest_client.cancel_order(cancel_spec).await.expect("Order cancellation");
        
        let cancel_execution_report = wait_for_execution_report(&mut subscription, 5).await.expect("Receive cancel execution report");
        
        assert_valid_execution_report_event(&cancel_execution_report);
        assert_eq!(cancel_execution_report.symbol, "BTCUSDT");
        assert_eq!(cancel_execution_report.order_id, order_result.order_id);
        assert!(matches!(cancel_execution_report.order_status, crate::enums::OrderStatus::Canceled));
        
        let _ = with_timeout(stream_client.close()).await;
    }

    /**
     * Tests execution report events by placing a WebSocket API order.
     */
    #[tokio::test]
    async fn test_execution_report_via_websocket_order() {
        // Arrange
        let mut stream_client = create_user_data_stream_client().expect("Stream client creation");
        let ws_client = create_websocket_client().await.expect("WebSocket client creation");
        let spec = UserDataStreamSpec::new();
        
        let (safe_price, safe_quantity) = get_safe_btcusdt_order_params().await.expect("Get safe order params");
        
        // Act
        let _ = with_timeout(stream_client.wait_for_connection()).await.expect("Connection");
        let _subscription = with_timeout(stream_client.subscribe(&spec)).await.expect("Subscription");
        
        let order_spec = OrderSpec::new("BTCUSDT", OrderSide::Buy, OrderType::Limit)
            .with_quantity(safe_quantity)
            .with_price(safe_price)
            .with_time_in_force(TimeInForce::GTC)
            .build()
            .expect("Order spec creation");
            
        let _test_order_result = ws_client.test_order(order_spec).await.expect("Test order placement");
        
        // Assert
        let _ = with_timeout(stream_client.close()).await;
    }

    /**
     * Tests account position update events.
     */
    #[tokio::test]
    async fn test_account_position_events() {
        // Arrange
        let mut stream_client = create_user_data_stream_client().expect("Stream client creation");
        let rest_client = create_rest_client().expect("REST client creation");
        let spec = UserDataStreamSpec::new();
        
        // Act
        let _ = with_timeout(stream_client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(stream_client.subscribe(&spec)).await.expect("Subscription");
        
        let order_spec = OrderSpec::new("BTCUSDT", OrderSide::Buy, OrderType::Market)
            .with_quote_order_quantity(Decimal::new(15, 0))
            .build()
            .expect("Order spec creation");
            
        let _order_result = rest_client.place_order(order_spec).await.expect("Order placement");
        
        let mut received_execution_report = false;
        
        for _ in 0..5 {
            if let Ok(event) = tokio::time::timeout(Duration::from_secs(10), subscription.recv()).await {
                if let Ok(user_data_event) = event {
                    match user_data_event {
                        UserDataEvent::ExecutionReport(execution_report) => {
                            assert_valid_execution_report_event(&execution_report);
                            received_execution_report = true;
                        }
                        UserDataEvent::OutboundAccountPosition(account_position) => {
                            assert_valid_account_position_event(&account_position);
                        }
                        UserDataEvent::BalanceUpdate(balance_update) => {
                            assert_valid_balance_update_event(&balance_update);
                        }
                        other => {
                            debug!(event = ?other, "Received other event type");
                        }
                    }
                    
                    if received_execution_report {
                        break;
                    }
                }
            }
        }
        
        // Assert
        assert!(received_execution_report, "Should receive at least one execution report");
        
        let _ = with_timeout(stream_client.close()).await;
    }

    /**
     * Tests multiple concurrent user data subscriptions.
     */
    #[tokio::test]
    async fn test_multiple_user_data_subscriptions() {
        // Arrange
        let mut client = create_user_data_stream_client().expect("Client creation");
        let spec = UserDataStreamSpec::new();
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let _subscription1 = with_timeout(client.subscribe(&spec)).await.expect("First subscription");
        let _subscription2 = with_timeout(client.subscribe(&spec)).await.expect("Second subscription");
        
        // Assert
        let _ = with_timeout(client.close()).await;
    }

    /**
     * Tests user data stream unsubscription.
     */
    #[tokio::test]
    async fn test_user_data_stream_unsubscribe() {
        // Arrange
        let mut client = create_user_data_stream_client().expect("Client creation");
        let spec = UserDataStreamSpec::new();
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let _subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        
        let _unsubscribe_result = with_timeout(client.unsubscribe(spec)).await.expect("Unsubscription");
        
        // Assert
        let _ = with_timeout(client.close()).await;
    }

    /**
     * Tests graceful client shutdown.
     */
    #[tokio::test]
    async fn test_user_data_client_shutdown() {
        // Arrange
        let mut client = create_user_data_stream_client().expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        assert!(client.is_connected(), "Should be connected");
        
        let close_result = with_timeout(client.close()).await.expect("Close");
        
        // Assert
        let _ = close_result;
    }

    /**
     * Tests connection status monitoring during lifecycle.
     */
    #[tokio::test]
    async fn test_connection_status_lifecycle() {
        // Arrange
        let mut client = create_user_data_stream_client().expect("Client creation");
        
        // Act & Assert
        let initial_status = client.connection_status();
        assert!(matches!(initial_status, ConnectionStatus::Connecting), "Should start in Connecting state");
        assert!(!client.is_connected(), "Should not report as connected initially");
        
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        assert_eq!(client.connection_status(), ConnectionStatus::Connected);
        assert!(client.is_connected(), "Should report as connected");
        
        let _ = with_timeout(client.close()).await.expect("Close");
    }

}
