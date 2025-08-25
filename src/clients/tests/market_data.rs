#[cfg(test)]
mod tests {
    use crate::{
        clients::{tests::helpers::*, r#trait::MarketDataClient},
        errors::{BinanceError, ErrorCategory, RequestError},
        types::{
            requests::{
                AggregateTradesSpec, AveragePriceSpec, HistoricalTradesSpec, KlinesSpec,
                OrderBookSpec, RecentTradesSpec,
            },
            responses::{AggregateTrade, AveragePrice, Kline, OrderBook, Trade},
        },
    };
    use rust_decimal::Decimal;

    /**
     * Validates order book response structure.
     */
    fn assert_valid_order_book(order_book: &OrderBook) {
        assert!(!order_book.bids.is_empty(), "Order book should have bids");
        assert!(!order_book.asks.is_empty(), "Order book should have asks");
        assert!(
            order_book.last_update_id > 0,
            "Last update ID should be positive"
        );

        // Validate first few bids and asks
        for bid in &order_book.bids[..5.min(order_book.bids.len())] {
            assert!(bid.price > Decimal::ZERO, "Bid price should be positive");
            assert!(
                bid.quantity > Decimal::ZERO,
                "Bid quantity should be positive"
            );
        }

        for ask in &order_book.asks[..5.min(order_book.asks.len())] {
            assert!(ask.price > Decimal::ZERO, "Ask price should be positive");
            assert!(
                ask.quantity > Decimal::ZERO,
                "Ask quantity should be positive"
            );
        }

        // Ensure bids are in descending order and asks in ascending order
        if order_book.bids.len() > 1 {
            assert!(
                order_book.bids[0].price >= order_book.bids[1].price,
                "Bids should be in descending price order"
            );
        }
        if order_book.asks.len() > 1 {
            assert!(
                order_book.asks[0].price <= order_book.asks[1].price,
                "Asks should be in ascending price order"
            );
        }
    }

    /**
     * Validates trade response structure.
     */
    fn assert_valid_trade(trade: &Trade) {
        assert!(trade.id > 0, "Trade ID should be positive");
        assert!(
            trade.price > Decimal::ZERO,
            "Trade price should be positive"
        );
        assert!(
            trade.quantity > Decimal::ZERO,
            "Trade quantity should be positive"
        );
        if trade.quote_quantity.is_some() {
            assert!(
                trade.quote_quantity > Some(Decimal::ZERO),
                "Quote quantity should be positive"
            );
        }
        assert!(trade.time > 0, "Trade time should be positive");
    }

    /**
     * Validates aggregate trade response structure.
     */
    fn assert_valid_aggregate_trade(agg_trade: &AggregateTrade) {
        assert!(agg_trade.id > 0, "Aggregate trade ID should be positive");
        assert!(agg_trade.price > Decimal::ZERO, "Price should be positive");
        assert!(
            agg_trade.quantity > Decimal::ZERO,
            "Quantity should be positive"
        );
        assert!(
            agg_trade.first_trade_id > 0,
            "First trade ID should be positive"
        );
        assert!(
            agg_trade.last_trade_id > 0,
            "Last trade ID should be positive"
        );
        assert!(
            agg_trade.last_trade_id >= agg_trade.first_trade_id,
            "Last trade ID should be >= first trade ID"
        );
        assert!(agg_trade.timestamp > 0, "Timestamp should be positive");
    }

    /**
     * Validates kline response structure.
     */
    fn assert_valid_kline(kline: &Kline) {
        assert!(kline.open_time > 0, "Open time should be positive");
        assert!(kline.close_time > 0, "Close time should be positive");
        assert!(
            kline.close_time > kline.open_time,
            "Close time should be after open time"
        );
        assert!(
            kline.open_price > Decimal::ZERO,
            "Open price should be positive"
        );
        assert!(
            kline.high_price > Decimal::ZERO,
            "High price should be positive"
        );
        assert!(
            kline.low_price > Decimal::ZERO,
            "Low price should be positive"
        );
        assert!(
            kline.close_price > Decimal::ZERO,
            "Close price should be positive"
        );
        assert!(
            kline.volume >= Decimal::ZERO,
            "Volume should be non-negative"
        );
        assert!(
            kline.quote_asset_volume >= Decimal::ZERO,
            "Quote asset volume should be non-negative"
        );
        assert!(
            kline.taker_buy_base_asset_volume >= Decimal::ZERO,
            "Taker buy base volume should be non-negative"
        );
        assert!(
            kline.taker_buy_quote_asset_volume >= Decimal::ZERO,
            "Taker buy quote volume should be non-negative"
        );

        // Validate price relationships
        assert!(
            kline.high_price >= kline.open_price,
            "High should be >= open"
        );
        assert!(
            kline.high_price >= kline.close_price,
            "High should be >= close"
        );
        assert!(kline.low_price <= kline.open_price, "Low should be <= open");
        assert!(
            kline.low_price <= kline.close_price,
            "Low should be <= close"
        );
    }

    /**
     * Validates average price response structure.
     */
    fn assert_valid_average_price(avg_price: &AveragePrice) {
        assert!(avg_price.minutes > 0, "Minutes should be positive");
        assert!(
            avg_price.price > Decimal::ZERO,
            "Average price should be positive"
        );
        assert!(avg_price.close_time > 0, "Close time should be positive");
    }

    /**
     * Tests order book retrieval with default limit.
     */
    #[tokio::test]
    async fn test_order_book_default_limit() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = OrderBookSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_order_book = with_timeout(rest_client.order_book(rest_spec))
            .await
            .expect("REST order book");

        let ws_spec = OrderBookSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_order_book = with_timeout(ws_client.order_book(ws_spec))
            .await
            .expect("WebSocket order book");

        // Assert
        assert_valid_order_book(&rest_order_book);
        assert_valid_order_book(&ws_order_book);

        assert!(
            rest_order_book.bids.len() <= 100,
            "Bids should not exceed default limit"
        );
        assert!(
            rest_order_book.asks.len() <= 100,
            "Asks should not exceed default limit"
        );
        assert!(
            ws_order_book.bids.len() <= 100,
            "Bids should not exceed default limit"
        );
        assert!(
            ws_order_book.asks.len() <= 100,
            "Asks should not exceed default limit"
        );
    }

    /**
     * Tests order book retrieval with custom limit.
     */
    #[tokio::test]
    async fn test_order_book_custom_limit() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let limit = 50;

        // Act
        let rest_spec = OrderBookSpec::new(test_symbol)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let rest_order_book = with_timeout(rest_client.order_book(rest_spec))
            .await
            .expect("REST order book");

        let ws_spec = OrderBookSpec::new(test_symbol)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let ws_order_book = with_timeout(ws_client.order_book(ws_spec))
            .await
            .expect("WebSocket order book");

        // Assert
        assert_valid_order_book(&rest_order_book);
        assert_valid_order_book(&ws_order_book);

        assert!(
            rest_order_book.bids.len() <= limit as usize,
            "Bids should not exceed custom limit"
        );
        assert!(
            rest_order_book.asks.len() <= limit as usize,
            "Asks should not exceed custom limit"
        );
        assert!(
            ws_order_book.bids.len() <= limit as usize,
            "Bids should not exceed custom limit"
        );
        assert!(
            ws_order_book.asks.len() <= limit as usize,
            "Asks should not exceed custom limit"
        );
    }

    /**
     * Tests order book error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_order_book_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = OrderBookSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.order_book(rest_spec)).await;

        let ws_spec = OrderBookSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.order_book(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(order_book) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, order_book
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
     * Tests recent trades retrieval with default limit.
     */
    #[tokio::test]
    async fn test_recent_trades_default_limit() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = RecentTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_trades = with_timeout(rest_client.recent_trades(rest_spec))
            .await
            .expect("REST recent trades");

        let ws_spec = RecentTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_trades = with_timeout(ws_client.recent_trades(ws_spec))
            .await
            .expect("WebSocket recent trades");

        // Assert
        assert!(!rest_trades.is_empty(), "REST should return recent trades");
        assert!(
            !ws_trades.is_empty(),
            "WebSocket should return recent trades"
        );

        for trade in &rest_trades[..5.min(rest_trades.len())] {
            assert_valid_trade(trade);
        }

        for trade in &ws_trades[..5.min(ws_trades.len())] {
            assert_valid_trade(trade);
        }

        assert!(
            rest_trades.len() <= 500,
            "REST trades should not exceed default limit"
        );
        assert!(
            ws_trades.len() <= 500,
            "WebSocket trades should not exceed default limit"
        );
    }

    /**
     * Tests recent trades retrieval with custom limit.
     */
    #[tokio::test]
    async fn test_recent_trades_custom_limit() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let limit = 100;

        // Act
        let rest_spec = RecentTradesSpec::new(test_symbol)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let rest_trades = with_timeout(rest_client.recent_trades(rest_spec))
            .await
            .expect("REST recent trades");

        let ws_spec = RecentTradesSpec::new(test_symbol)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let ws_trades = with_timeout(ws_client.recent_trades(ws_spec))
            .await
            .expect("WebSocket recent trades");

        // Assert
        assert!(!rest_trades.is_empty(), "REST should return recent trades");
        assert!(
            !ws_trades.is_empty(),
            "WebSocket should return recent trades"
        );

        for trade in &rest_trades {
            assert_valid_trade(trade);
        }

        for trade in &ws_trades {
            assert_valid_trade(trade);
        }

        assert!(
            rest_trades.len() <= limit as usize,
            "REST trades should not exceed custom limit"
        );
        assert!(
            ws_trades.len() <= limit as usize,
            "WebSocket trades should not exceed custom limit"
        );
    }

    /**
     * Tests recent trades error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_recent_trades_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = RecentTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.recent_trades(rest_spec)).await;

        let ws_spec = RecentTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.recent_trades(ws_spec)).await;

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
     * Tests historical trades retrieval with default limit.
     */
    #[tokio::test]
    async fn test_historical_trades_default_limit() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = HistoricalTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_trades = with_timeout(rest_client.historical_trades(rest_spec))
            .await
            .expect("REST historical trades");

        let ws_spec = HistoricalTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_trades = with_timeout(ws_client.historical_trades(ws_spec))
            .await
            .expect("WebSocket historical trades");

        // Assert
        assert!(
            !rest_trades.is_empty(),
            "REST should return historical trades"
        );
        assert!(
            !ws_trades.is_empty(),
            "WebSocket should return historical trades"
        );

        for trade in &rest_trades[..5.min(rest_trades.len())] {
            assert_valid_trade(trade);
        }

        for trade in &ws_trades[..5.min(ws_trades.len())] {
            assert_valid_trade(trade);
        }

        assert!(
            rest_trades.len() <= 500,
            "REST trades should not exceed default limit"
        );
        assert!(
            ws_trades.len() <= 500,
            "WebSocket trades should not exceed default limit"
        );
    }

    /**
     * Tests historical trades retrieval with from_id parameter.
     */
    #[tokio::test]
    async fn test_historical_trades_from_id() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let from_id = 1000000;

        // Act
        let rest_spec = HistoricalTradesSpec::new(test_symbol)
            .with_from_id(from_id)
            .build()
            .expect("Spec validation");
        let rest_trades = with_timeout(rest_client.historical_trades(rest_spec))
            .await
            .expect("REST historical trades");

        let ws_spec = HistoricalTradesSpec::new(test_symbol)
            .with_from_id(from_id)
            .build()
            .expect("Spec validation");
        let ws_trades = with_timeout(ws_client.historical_trades(ws_spec))
            .await
            .expect("WebSocket historical trades");

        // Assert
        assert!(
            !rest_trades.is_empty(),
            "REST should return historical trades"
        );
        assert!(
            !ws_trades.is_empty(),
            "WebSocket should return historical trades"
        );

        for trade in &rest_trades {
            assert_valid_trade(trade);
            assert!(
                trade.id >= from_id,
                "Trade ID should be >= from_id parameter"
            );
        }

        for trade in &ws_trades {
            assert_valid_trade(trade);
            assert!(
                trade.id >= from_id,
                "Trade ID should be >= from_id parameter"
            );
        }
    }

    /**
     * Tests historical trades error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_historical_trades_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = HistoricalTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.historical_trades(rest_spec)).await;

        let ws_spec = HistoricalTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.historical_trades(ws_spec)).await;

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
     * Tests aggregate trades retrieval with default limit.
     */
    #[tokio::test]
    async fn test_aggregate_trades_default_limit() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = AggregateTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_agg_trades = with_timeout(rest_client.aggregate_trades(rest_spec))
            .await
            .expect("REST aggregate trades");

        let ws_spec = AggregateTradesSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_agg_trades = with_timeout(ws_client.aggregate_trades(ws_spec))
            .await
            .expect("WebSocket aggregate trades");

        // Assert
        assert!(
            !rest_agg_trades.is_empty(),
            "REST should return aggregate trades"
        );
        assert!(
            !ws_agg_trades.is_empty(),
            "WebSocket should return aggregate trades"
        );

        for agg_trade in &rest_agg_trades[..5.min(rest_agg_trades.len())] {
            assert_valid_aggregate_trade(agg_trade);
        }

        for agg_trade in &ws_agg_trades[..5.min(ws_agg_trades.len())] {
            assert_valid_aggregate_trade(agg_trade);
        }

        assert!(
            rest_agg_trades.len() <= 500,
            "REST aggregate trades should not exceed default limit"
        );
        assert!(
            ws_agg_trades.len() <= 500,
            "WebSocket aggregate trades should not exceed default limit"
        );
    }

    /**
     * Tests aggregate trades retrieval with time range.
     */
    #[tokio::test]
    async fn test_aggregate_trades_time_range() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        let end_time = chrono::Utc::now().timestamp_millis() as u64;
        let start_time = end_time - (60 * 60 * 1000);

        // Act
        let rest_spec = AggregateTradesSpec::new(test_symbol)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .build()
            .expect("Spec validation");
        let rest_agg_trades = with_timeout(rest_client.aggregate_trades(rest_spec))
            .await
            .expect("REST aggregate trades");

        let ws_spec = AggregateTradesSpec::new(test_symbol)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .build()
            .expect("Spec validation");
        let ws_agg_trades = with_timeout(ws_client.aggregate_trades(ws_spec))
            .await
            .expect("WebSocket aggregate trades");

        // Assert
        assert!(
            !rest_agg_trades.is_empty(),
            "REST should return aggregate trades in time range"
        );
        assert!(
            !ws_agg_trades.is_empty(),
            "WebSocket should return aggregate trades in time range"
        );

        for agg_trade in &rest_agg_trades {
            assert_valid_aggregate_trade(agg_trade);
            assert!(
                agg_trade.timestamp >= start_time,
                "Trade timestamp should be >= start_time"
            );
            assert!(
                agg_trade.timestamp <= end_time,
                "Trade timestamp should be <= end_time"
            );
        }

        for agg_trade in &ws_agg_trades {
            assert_valid_aggregate_trade(agg_trade);
            assert!(
                agg_trade.timestamp >= start_time,
                "Trade timestamp should be >= start_time"
            );
            assert!(
                agg_trade.timestamp <= end_time,
                "Trade timestamp should be <= end_time"
            );
        }
    }

    /**
     * Tests aggregate trades error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_aggregate_trades_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = AggregateTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.aggregate_trades(rest_spec)).await;

        let ws_spec = AggregateTradesSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.aggregate_trades(ws_spec)).await;

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
     * Tests klines retrieval with default parameters.
     */
    #[tokio::test]
    async fn test_klines_default_params() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let interval = "1h";

        // Act
        let rest_spec = KlinesSpec::new(test_symbol, interval)
            .build()
            .expect("Spec validation");
        let rest_klines = with_timeout(rest_client.klines(rest_spec))
            .await
            .expect("REST klines");

        let ws_spec = KlinesSpec::new(test_symbol, interval)
            .build()
            .expect("Spec validation");
        let ws_klines = with_timeout(ws_client.klines(ws_spec))
            .await
            .expect("WebSocket klines");

        // Assert
        assert!(!rest_klines.is_empty(), "REST should return klines");
        assert!(!ws_klines.is_empty(), "WebSocket should return klines");

        for kline in &rest_klines[..5.min(rest_klines.len())] {
            assert_valid_kline(kline);
        }

        for kline in &ws_klines[..5.min(ws_klines.len())] {
            assert_valid_kline(kline);
        }

        assert!(
            rest_klines.len() <= 500,
            "REST klines should not exceed default limit"
        );
        assert!(
            ws_klines.len() <= 500,
            "WebSocket klines should not exceed default limit"
        );
    }

    /**
     * Tests klines retrieval with custom limit and time range.
     */
    #[tokio::test]
    async fn test_klines_custom_params() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let interval = "5m";
        let limit = 100;

        let end_time = chrono::Utc::now().timestamp_millis() as u64;
        let start_time = end_time - (24 * 60 * 60 * 1000);

        // Act
        let rest_spec = KlinesSpec::new(test_symbol, interval)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let rest_klines = with_timeout(rest_client.klines(rest_spec))
            .await
            .expect("REST klines");

        let ws_spec = KlinesSpec::new(test_symbol, interval)
            .with_start_time(start_time)
            .with_end_time(end_time)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let ws_klines = with_timeout(ws_client.klines(ws_spec))
            .await
            .expect("WebSocket klines");

        // Assert
        assert!(!rest_klines.is_empty(), "REST should return klines");
        assert!(!ws_klines.is_empty(), "WebSocket should return klines");

        for kline in &rest_klines {
            assert_valid_kline(kline);
            assert!(
                kline.open_time >= start_time,
                "Kline open time should be >= start_time"
            );
            assert!(
                kline.close_time <= end_time,
                "Kline close time should be <= end_time"
            );
        }

        for kline in &ws_klines {
            assert_valid_kline(kline);
            assert!(
                kline.open_time >= start_time,
                "Kline open time should be >= start_time"
            );
            assert!(
                kline.close_time <= end_time,
                "Kline close time should be <= end_time"
            );
        }

        assert!(
            rest_klines.len() <= limit as usize,
            "REST klines should not exceed custom limit"
        );
        assert!(
            ws_klines.len() <= limit as usize,
            "WebSocket klines should not exceed custom limit"
        );
    }

    /**
     * Tests klines error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_klines_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = KlinesSpec::new("INVALID", "1h")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.klines(rest_spec)).await;

        let ws_spec = KlinesSpec::new("INVALID", "1h")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.klines(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(klines) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, klines
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
     * Tests UI klines retrieval with default parameters.
     */
    #[tokio::test]
    async fn test_ui_klines_default_params() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let interval = "1d";

        // Act
        let rest_spec = KlinesSpec::new(test_symbol, interval)
            .build()
            .expect("Spec validation");
        let rest_ui_klines = with_timeout(rest_client.ui_klines(rest_spec))
            .await
            .expect("REST UI klines");

        let ws_spec = KlinesSpec::new(test_symbol, interval)
            .build()
            .expect("Spec validation");
        let ws_ui_klines = with_timeout(ws_client.ui_klines(ws_spec))
            .await
            .expect("WebSocket UI klines");

        // Assert
        assert!(!rest_ui_klines.is_empty(), "REST should return UI klines");
        assert!(
            !ws_ui_klines.is_empty(),
            "WebSocket should return UI klines"
        );

        for kline in &rest_ui_klines[..5.min(rest_ui_klines.len())] {
            assert_valid_kline(kline);
        }

        for kline in &ws_ui_klines[..5.min(ws_ui_klines.len())] {
            assert_valid_kline(kline);
        }

        assert!(
            rest_ui_klines.len() <= 500,
            "REST UI klines should not exceed default limit"
        );
        assert!(
            ws_ui_klines.len() <= 500,
            "WebSocket UI klines should not exceed default limit"
        );
    }

    /**
     * Tests UI klines retrieval with timezone parameter.
     */
    #[tokio::test]
    async fn test_ui_klines_with_timezone() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let interval = "1d";
        let timezone = "8";
        let limit = 30;

        // Act
        let rest_spec = KlinesSpec::new(test_symbol, interval)
            .with_time_zone(timezone)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let rest_ui_klines = with_timeout(rest_client.ui_klines(rest_spec))
            .await
            .expect("REST UI klines");

        let ws_spec = KlinesSpec::new(test_symbol, interval)
            .with_time_zone(timezone)
            .with_limit(limit)
            .build()
            .expect("Spec validation");
        let ws_ui_klines = with_timeout(ws_client.ui_klines(ws_spec))
            .await
            .expect("WebSocket UI klines");

        // Assert
        assert!(
            !rest_ui_klines.is_empty(),
            "REST should return UI klines with timezone"
        );
        assert!(
            !ws_ui_klines.is_empty(),
            "WebSocket should return UI klines with timezone"
        );

        for kline in &rest_ui_klines {
            assert_valid_kline(kline);
        }

        for kline in &ws_ui_klines {
            assert_valid_kline(kline);
        }

        assert!(
            rest_ui_klines.len() <= limit as usize,
            "REST UI klines should not exceed custom limit"
        );
        assert!(
            ws_ui_klines.len() <= limit as usize,
            "WebSocket UI klines should not exceed custom limit"
        );
    }

    /**
     * Tests UI klines error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_ui_klines_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = KlinesSpec::new("INVALID", "1d")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.ui_klines(rest_spec)).await;

        let ws_spec = KlinesSpec::new("INVALID", "1d")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.ui_klines(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(klines) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, klines
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
     * Tests average price retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_average_price() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";

        // Act
        let rest_spec = AveragePriceSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let rest_avg_price = with_timeout(rest_client.average_price(rest_spec))
            .await
            .expect("REST average price");

        let ws_spec = AveragePriceSpec::new(test_symbol)
            .build()
            .expect("Spec validation");
        let ws_avg_price = with_timeout(ws_client.average_price(ws_spec))
            .await
            .expect("WebSocket average price");

        // Assert
        assert_valid_average_price(&rest_avg_price);
        assert_valid_average_price(&ws_avg_price);

        let price_diff = (rest_avg_price.price - ws_avg_price.price).abs();
        let price_ratio = price_diff / rest_avg_price.price;
        assert!(
            price_ratio < Decimal::new(1, 2),
            "REST and WebSocket average prices should be close"
        );
    }

    /**
     * Tests average price error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_average_price_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = AveragePriceSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.average_price(rest_spec)).await;

        let ws_spec = AveragePriceSpec::new("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.average_price(ws_spec)).await;

        // Assert
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(avg_price) => panic!(
                    "Expected {} error, got successful response: {:?}",
                    client_name, avg_price
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
