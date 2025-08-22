#[cfg(test)]
mod tests {
    use std::time::Duration;
    use rust_decimal::Decimal;
    
    use crate::Result;
    use crate::{BinanceConfig, StreamConfig};
    use crate::streams::{BinanceSpotStreamClient, specs::*, events::*};
    use crate::streams::connection::ConnectionStatus;

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
     * Creates a dynamic stream client for testnet.
     */
    fn create_dynamic_stream_client() -> Result<BinanceSpotStreamClient> {
        let config = BinanceConfig::<StreamConfig>::builder()
            .with_testnet()
            .with_market_data()
            .with_dynamic_streams()
            .build()?;
        crate::streams::client(config)
    }

    /**
     * Creates a raw stream client for testnet with a single stream specification.
     */
    fn create_raw_stream_client<S: StreamSpec>(spec: &S) -> Result<BinanceSpotStreamClient> {
        let config = BinanceConfig::<StreamConfig>::builder()
            .with_testnet()
            .with_market_data()
            .with_raw_stream(spec)?
            .build()?;
        crate::streams::client(config)
    }

    /**
     * Creates a combined stream client for testnet with multiple stream specifications.
     */
    fn create_combined_stream_client<'a, I, S>(specs: I) -> Result<BinanceSpotStreamClient> 
    where 
        I: IntoIterator<Item = &'a S>,
        S: StreamSpec + 'a,
    {
        let config = BinanceConfig::<StreamConfig>::builder()
            .with_testnet()
            .with_market_data()
            .with_combined_streams(specs)?
            .build()?;
        crate::streams::client(config)
    }

    /**
     * Validates aggregate trade stream event structure.
     */
    fn assert_valid_aggregate_trade_event(event: &AggregateTradeStreamEvent) {
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "aggTrade", "Event type should be aggTrade");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.aggregate_trade.id > 0, "Aggregate trade ID should be positive");
        assert!(event.aggregate_trade.price > Decimal::ZERO, "Price should be positive");
        assert!(event.aggregate_trade.quantity > Decimal::ZERO, "Quantity should be positive");
        assert!(event.aggregate_trade.first_trade_id > 0, "First trade ID should be positive");
        assert!(event.aggregate_trade.last_trade_id > 0, "Last trade ID should be positive");
        assert!(event.aggregate_trade.last_trade_id >= event.aggregate_trade.first_trade_id, "Last trade ID should be >= first trade ID");
        assert!(event.aggregate_trade.timestamp > 0, "Timestamp should be positive");
    }

    /**
     * Validates trade stream event structure.
     */
    fn assert_valid_trade_event(event: &TradeStreamEvent) {
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "trade", "Event type should be trade");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.trade.id > 0, "Trade ID should be positive");
        assert!(event.trade.price > Decimal::ZERO, "Price should be positive");
        assert!(event.trade.quantity > Decimal::ZERO, "Quantity should be positive");
        assert!(event.trade.time > 0, "Trade time should be positive");
    }

    /**
     * Validates book ticker stream event structure.
     */
    fn assert_valid_book_ticker_event(event: &BookTickerStreamEvent) {
        assert!(!event.ticker.symbol.is_empty(), "Symbol should not be empty");
        assert!(event.ticker.bid_price > Decimal::ZERO, "Bid price should be positive");
        assert!(event.ticker.ask_price > Decimal::ZERO, "Ask price should be positive");
        assert!(event.ticker.bid_quantity > Decimal::ZERO, "Bid quantity should be positive");
        assert!(event.ticker.ask_quantity > Decimal::ZERO, "Ask quantity should be positive");
        assert!(event.ticker.ask_price >= event.ticker.bid_price, "Ask price should be >= bid price");
        assert!(event.update_id > 0, "Update ID should be positive");
    }

    /**
     * Validates mini ticker stream event structure.
     */
    fn assert_valid_mini_ticker_event(event: &MiniTickerStreamEvent) {
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "24hrMiniTicker", "Event type should be 24hrMiniTicker");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.close_price > Decimal::ZERO, "Close price should be positive");
        assert!(event.open_price > Decimal::ZERO, "Open price should be positive");
        assert!(event.high_price > Decimal::ZERO, "High price should be positive");
        assert!(event.low_price > Decimal::ZERO, "Low price should be positive");
        assert!(event.volume >= Decimal::ZERO, "Volume should be non-negative");
        assert!(event.quote_volume >= Decimal::ZERO, "Quote volume should be non-negative");
    }

    /**
     * Validates ticker stream event structure.
     */
    fn assert_valid_ticker_event(event: &TickerStreamEvent) {
        assert!(!event.ticker.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "24hrTicker", "Event type should be 24hrTicker");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.ticker.last_price > Decimal::ZERO, "Last price should be positive");
        assert!(event.ticker.volume >= Decimal::ZERO, "Volume should be non-negative");
        assert!(event.ticker.quote_volume >= Decimal::ZERO, "Quote volume should be non-negative");
        assert!(event.ticker.open_time > 0, "Open time should be positive");
        assert!(event.ticker.close_time > 0, "Close time should be positive");
        assert!(event.ticker.close_time >= event.ticker.open_time, "Close time should be >= open time");
    }

    /**
     * Validates kline stream event structure.
     */
    fn assert_valid_kline_event(event: &KlineStreamEvent) {
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "kline", "Event type should be kline");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(!event.kline.interval.is_empty(), "Interval should not be empty");
        assert!(event.kline.kline.open_time > 0, "Open time should be positive");
        assert!(event.kline.kline.close_time > 0, "Close time should be positive");
        assert!(event.kline.kline.close_time > event.kline.kline.open_time, "Close time should be after open time");
        assert!(event.kline.kline.open_price > Decimal::ZERO, "Open price should be positive");
        assert!(event.kline.kline.high_price > Decimal::ZERO, "High price should be positive");
        assert!(event.kline.kline.low_price > Decimal::ZERO, "Low price should be positive");
        assert!(event.kline.kline.close_price > Decimal::ZERO, "Close price should be positive");
        assert!(event.kline.kline.volume >= Decimal::ZERO, "Volume should be non-negative");
        assert!(event.kline.kline.quote_asset_volume >= Decimal::ZERO, "Quote asset volume should be non-negative");
    }

    /**
     * Validates average price stream event structure.
     */
    fn assert_valid_average_price_event(event: &AveragePriceStreamEvent) {
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "avgPrice", "Event type should be avgPrice");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(!event.interval.is_empty(), "Interval should not be empty");
        assert!(event.price > Decimal::ZERO, "Price should be positive");
        assert!(event.last_trade_time > 0, "Last trade time should be positive");
    }

    /**
     * Validates rolling window ticker stream event structure.
     */
    fn assert_valid_rolling_window_ticker_event(event: &RollingWindowTickerStreamEvent) {
        assert!(!event.ticker.symbol.is_empty(), "Symbol should not be empty");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.ticker.last_price > Decimal::ZERO, "Last price should be positive");
        assert!(event.ticker.volume >= Decimal::ZERO, "Volume should be non-negative");
        assert!(event.ticker.quote_volume >= Decimal::ZERO, "Quote volume should be non-negative");
        assert!(event.ticker.open_time > 0, "Open time should be positive");
        assert!(event.ticker.close_time > 0, "Close time should be positive");
    }

    /**
     * Validates partial book depth stream event structure.
     */
    fn assert_valid_partial_book_depth_event(event: &PartialBookDepthStreamEvent) {
        assert!(event.last_update_id > 0, "Last update ID should be positive");
        assert!(!event.bids.is_empty(), "Should have bids");
        assert!(!event.asks.is_empty(), "Should have asks");
        
        // Validate first few bids and asks
        for bid in &event.bids[..3.min(event.bids.len())] {
            assert!(bid.price() > Decimal::ZERO, "Bid price should be positive");
            assert!(bid.quantity() > Decimal::ZERO, "Bid quantity should be positive");
        }
        
        for ask in &event.asks[..3.min(event.asks.len())] {
            assert!(ask.price() > Decimal::ZERO, "Ask price should be positive");
            assert!(ask.quantity() > Decimal::ZERO, "Ask quantity should be positive");
        }
    }

    /**
     * Validates diff depth stream event structure.
     */
    fn assert_valid_diff_depth_event(event: &DiffDepthStreamEvent) {
        assert!(!event.symbol.is_empty(), "Symbol should not be empty");
        assert_eq!(event.event_type, "depthUpdate", "Event type should be depthUpdate");
        assert!(event.event_time > 0, "Event time should be positive");
        assert!(event.first_update_id > 0, "First update ID should be positive");
        assert!(event.final_update_id > 0, "Final update ID should be positive");
        assert!(event.final_update_id >= event.first_update_id, "Final update ID should be >= first update ID");
        
        // Note: bids and asks can be empty in diff depth updates, but if present, should be valid
        for bid in &event.bids {
            assert!(bid.price() > Decimal::ZERO, "Bid price should be positive");
            assert!(bid.quantity() >= Decimal::ZERO, "Bid quantity should be non-negative (0 means remove)");
        }
        
        for ask in &event.asks {
            assert!(ask.price() > Decimal::ZERO, "Ask price should be positive");
            assert!(ask.quantity() >= Decimal::ZERO, "Ask quantity should be non-negative (0 means remove)");
        }
    }

    /**
     * Validates all mini tickers stream event structure.
     */
    fn assert_valid_all_mini_tickers_event(event: &AllMiniTickersStreamEvent) {
        assert!(!event.tickers.is_empty(), "Should have at least one ticker");
        
        // Validate first few tickers
        for ticker in &event.tickers[..3.min(event.tickers.len())] {
            assert!(!ticker.symbol.is_empty(), "Symbol should not be empty");
            assert_eq!(ticker.event_type, "24hrMiniTicker", "Event type should be 24hrMiniTicker");
            assert!(ticker.event_time > 0, "Event time should be positive");
            assert!(ticker.close_price > Decimal::ZERO, "Close price should be positive");
            assert!(ticker.open_price > Decimal::ZERO, "Open price should be positive");
            assert!(ticker.volume >= Decimal::ZERO, "Volume should be non-negative");
        }
    }

    /**
     * Validates all tickers stream event structure.
     */
    fn assert_valid_all_tickers_event(event: &AllTickersStreamEvent) {
        assert!(!event.tickers.is_empty(), "Should have at least one ticker");
        
        // Validate first few tickers
        for ticker in &event.tickers[..3.min(event.tickers.len())] {
            assert!(!ticker.ticker.symbol.is_empty(), "Symbol should not be empty");
            assert_eq!(ticker.event_type, "24hrTicker", "Event type should be 24hrTicker");
            assert!(ticker.event_time > 0, "Event time should be positive");
            assert!(ticker.ticker.last_price > Decimal::ZERO, "Last price should be positive");
            assert!(ticker.ticker.volume >= Decimal::ZERO, "Volume should be non-negative");
            assert!(ticker.ticker.open_time > 0, "Open time should be positive");
        }
    }

    /**
     * Validates all rolling window tickers stream event structure.
     */
    fn assert_valid_all_rolling_window_tickers_event(event: &AllRollingWindowTickersStreamEvent) {
        assert!(!event.tickers.is_empty(), "Should have at least one ticker");
        
        // Validate first few tickers
        for ticker in &event.tickers[..3.min(event.tickers.len())] {
            assert!(!ticker.ticker.symbol.is_empty(), "Symbol should not be empty");
            assert!(ticker.event_time > 0, "Event time should be positive");
            assert!(ticker.ticker.last_price > Decimal::ZERO, "Last price should be positive");
            assert!(ticker.ticker.volume >= Decimal::ZERO, "Volume should be non-negative");
            assert!(ticker.ticker.open_time > 0, "Open time should be positive");
        }
    }

    /**
     * Tests aggregate trade stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_aggregate_trade_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = AggregateTradeStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_aggregate_trade_event(&event);
        assert_eq!(event.symbol, test_symbol);
    }

    /**
     * Tests aggregate trade stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_aggregate_trade_stream_combined() {
        // Arrange
        let first_test_symbol = "BTCUSDT";
        let second_test_symbol = "ETHUSDT";
        let first_spec = AggregateTradeStreamSpec::new(first_test_symbol);
        let second_spec = AggregateTradeStreamSpec::new(second_test_symbol);
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut first_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("First subscription");
        let first_event = with_recv_timeout(first_subscription.recv()).await.expect("Receive first event");
        
        let mut second_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("Second subscription");
        let second_event = with_recv_timeout(second_subscription.recv()).await.expect("Receive second event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_aggregate_trade_event(&first_event);
        assert_eq!(first_event.symbol, first_test_symbol);
        
        assert_valid_aggregate_trade_event(&second_event);
        assert_eq!(second_event.symbol, second_test_symbol);
    }

    /**
     * Tests aggregate trade stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_aggregate_trade_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let agg_trade_symbol = "BTCUSDT";
        let trade_symbol = "ETHUSDT";
        let book_ticker_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let agg_trade_spec = AggregateTradeStreamSpec::new(agg_trade_symbol);
        let mut agg_trade_subscription = with_timeout(client.subscribe(&agg_trade_spec)).await.expect("Aggregate trade subscription");
        let agg_trade_event = with_recv_timeout(agg_trade_subscription.recv()).await.expect("Receive aggregate trade event");
        
        let trade_spec = TradeStreamSpec::new(trade_symbol);
        let mut trade_subscription = with_timeout(client.subscribe(&trade_spec)).await.expect("Trade subscription");
        let trade_event = with_recv_timeout(trade_subscription.recv()).await.expect("Receive trade event");
        
        let book_ticker_spec = BookTickerStreamSpec::new(book_ticker_symbol);
        let mut book_ticker_subscription = with_timeout(client.subscribe(&book_ticker_spec)).await.expect("Book ticker subscription");
        let book_ticker_event = with_recv_timeout(book_ticker_subscription.recv()).await.expect("Receive book ticker event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_aggregate_trade_event(&agg_trade_event);
        assert_eq!(agg_trade_event.symbol, agg_trade_symbol);
        
        assert_valid_trade_event(&trade_event);
        assert_eq!(trade_event.symbol, trade_symbol);
        
        assert_valid_book_ticker_event(&book_ticker_event);
        assert_eq!(book_ticker_event.ticker.symbol, book_ticker_symbol);
    }

    /**
     * Tests all mini tickers stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_all_mini_tickers_stream_raw() {
        // Arrange
        let spec = AllMiniTickersStreamSpec::new();
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_mini_tickers_event(&event);
        assert!(event.tickers.len() > 10, "Should have many tickers");
    }

    /**
     * Tests all mini tickers stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_all_mini_tickers_stream_combined() {
        // Arrange
        let first_spec = AllMiniTickersStreamSpec::new();
        let specs = [&first_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut all_mini_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("All mini tickers subscription");
        let all_mini_event = with_recv_timeout(all_mini_subscription.recv()).await.expect("Receive all mini tickers event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_mini_tickers_event(&all_mini_event);
        assert!(all_mini_event.tickers.len() > 10, "Should have many tickers");
    }

    /**
     * Tests all mini tickers stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_all_mini_tickers_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let trade_symbol = "BTCUSDT";
        let kline_symbol = "ETHUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let all_mini_spec = AllMiniTickersStreamSpec::new();
        let mut all_mini_subscription = with_timeout(client.subscribe(&all_mini_spec)).await.expect("All mini tickers subscription");
        let all_mini_event = with_recv_timeout(all_mini_subscription.recv()).await.expect("Receive all mini tickers event");
        
        let trade_spec = TradeStreamSpec::new(trade_symbol);
        let mut trade_subscription = with_timeout(client.subscribe(&trade_spec)).await.expect("Trade subscription");
        let trade_event = with_recv_timeout(trade_subscription.recv()).await.expect("Receive trade event");
        
        let kline_spec = KlineStreamSpec::one_minute(kline_symbol);
        let mut kline_subscription = with_timeout(client.subscribe(&kline_spec)).await.expect("Kline subscription");
        let kline_event = with_recv_timeout(kline_subscription.recv()).await.expect("Receive kline event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_mini_tickers_event(&all_mini_event);
        assert!(all_mini_event.tickers.len() > 10, "Should have many tickers");
        
        assert_valid_trade_event(&trade_event);
        assert_eq!(trade_event.symbol, trade_symbol);
        
        assert_valid_kline_event(&kline_event);
        assert_eq!(kline_event.symbol, kline_symbol);
        assert_eq!(kline_event.kline.interval, "1m");
    }

    /**
     * Tests all rolling window tickers stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_all_rolling_window_tickers_stream_raw() {
        // Arrange
        let spec = AllRollingWindowTickersStreamSpec::hourly(); // Test 1-hour window
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_rolling_window_tickers_event(&event);
        assert!(event.tickers.len() > 10, "Should have many tickers");
    }

    /**
     * Tests all rolling window tickers stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_all_rolling_window_tickers_stream_combined() {
        // Arrange
        let first_spec = AllRollingWindowTickersStreamSpec::hourly();
        let second_spec = AllRollingWindowTickersStreamSpec::four_hourly();
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut hourly_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("Hourly all tickers subscription");
        let hourly_event = with_recv_timeout(hourly_subscription.recv()).await.expect("Receive hourly event");
        
        let mut four_hourly_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("4-hourly all tickers subscription");
        let four_hourly_event = with_recv_timeout(four_hourly_subscription.recv()).await.expect("Receive 4-hourly event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_rolling_window_tickers_event(&hourly_event);
        assert!(hourly_event.tickers.len() > 10, "Should have many hourly tickers");
        
        assert_valid_all_rolling_window_tickers_event(&four_hourly_event);
        assert!(four_hourly_event.tickers.len() > 10, "Should have many 4-hourly tickers");
    }

    /**
     * Tests all rolling window tickers stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_all_rolling_window_tickers_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let agg_trade_symbol = "BTCUSDT";
        let book_ticker_symbol = "ETHUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to all rolling window tickers stream (1-day window)
        let all_rolling_spec = AllRollingWindowTickersStreamSpec::daily();
        let mut all_rolling_subscription = with_timeout(client.subscribe(&all_rolling_spec)).await.expect("All rolling window tickers subscription");
        let all_rolling_event = with_recv_timeout(all_rolling_subscription.recv()).await.expect("Receive all rolling window event");
        
        // Subscribe to aggregate trade stream (different spec type)
        let agg_trade_spec = AggregateTradeStreamSpec::new(agg_trade_symbol);
        let mut agg_trade_subscription = with_timeout(client.subscribe(&agg_trade_spec)).await.expect("Aggregate trade subscription");
        let agg_trade_event = with_recv_timeout(agg_trade_subscription.recv()).await.expect("Receive aggregate trade event");
        
        // Subscribe to book ticker stream (different spec type)
        let book_ticker_spec = BookTickerStreamSpec::new(book_ticker_symbol);
        let mut book_ticker_subscription = with_timeout(client.subscribe(&book_ticker_spec)).await.expect("Book ticker subscription");
        let book_ticker_event = with_recv_timeout(book_ticker_subscription.recv()).await.expect("Receive book ticker event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_rolling_window_tickers_event(&all_rolling_event);
        assert!(all_rolling_event.tickers.len() > 10, "Should have many rolling window tickers");
        
        assert_valid_aggregate_trade_event(&agg_trade_event);
        assert_eq!(agg_trade_event.symbol, agg_trade_symbol);
        
        assert_valid_book_ticker_event(&book_ticker_event);
        assert_eq!(book_ticker_event.ticker.symbol, book_ticker_symbol);
    }

    /**
     * Tests all tickers stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_all_tickers_stream_raw() {
        // Arrange
        let spec = AllTickersStreamSpec::new();
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_tickers_event(&event);
        assert!(event.tickers.len() > 10, "Should have many tickers");
    }

    /**
     * Tests all tickers stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_all_tickers_stream_combined() {
        let first_spec = TickerStreamSpec::new("BTCUSDT");
        let second_spec = TickerStreamSpec::new("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");

        let mut btc_ticker_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC ticker subscription");
        let btc_ticker_event = with_recv_timeout(btc_ticker_subscription.recv()).await.expect("Receive BTC ticker event");
        
        let mut eth_ticker_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH ticker subscription");
        let eth_ticker_event = with_recv_timeout(eth_ticker_subscription.recv()).await.expect("Receive ETH ticker event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_ticker_event(&btc_ticker_event);
        assert_eq!(btc_ticker_event.ticker.symbol, "BTCUSDT");
        
        assert_valid_ticker_event(&eth_ticker_event);
        assert_eq!(eth_ticker_event.ticker.symbol, "ETHUSDT");
    }

    /**
     * Tests all tickers stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_all_tickers_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let trade_symbol = "BTCUSDT";
        let mini_ticker_symbol = "ETHUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to all tickers stream
        let all_tickers_spec = AllTickersStreamSpec::new();
        let mut all_tickers_subscription = with_timeout(client.subscribe(&all_tickers_spec)).await.expect("All tickers subscription");
        let all_tickers_event = with_recv_timeout(all_tickers_subscription.recv()).await.expect("Receive all tickers event");
        
        // Subscribe to trade stream (different spec type)
        let trade_spec = TradeStreamSpec::new(trade_symbol);
        let mut trade_subscription = with_timeout(client.subscribe(&trade_spec)).await.expect("Trade subscription");
        let trade_event = with_recv_timeout(trade_subscription.recv()).await.expect("Receive trade event");
        
        // Subscribe to mini ticker stream (different spec type)
        let mini_ticker_spec = MiniTickerStreamSpec::new(mini_ticker_symbol);
        let mut mini_ticker_subscription = with_timeout(client.subscribe(&mini_ticker_spec)).await.expect("Mini ticker subscription");
        let mini_ticker_event = with_recv_timeout(mini_ticker_subscription.recv()).await.expect("Receive mini ticker event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_all_tickers_event(&all_tickers_event);
        assert!(all_tickers_event.tickers.len() > 10, "Should have many tickers");
        
        assert_valid_trade_event(&trade_event);
        assert_eq!(trade_event.symbol, trade_symbol);
        
        assert_valid_mini_ticker_event(&mini_ticker_event);
        assert_eq!(mini_ticker_event.symbol, mini_ticker_symbol);
    }

    /**
     * Tests average price stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_average_price_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = AveragePriceStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_average_price_event(&event);
        assert_eq!(event.symbol, test_symbol);
    }

    /**
     * Tests average price stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_average_price_stream_combined() {
        // Arrange
        let first_spec = AveragePriceStreamSpec::new("BTCUSDT");
        let second_spec = AveragePriceStreamSpec::new("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC average price subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH average price subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_average_price_event(&btc_event);
        assert_eq!(btc_event.symbol, "BTCUSDT");
        
        assert_valid_average_price_event(&eth_event);
        assert_eq!(eth_event.symbol, "ETHUSDT");
    }

    /**
     * Tests average price stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_average_price_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let avg_price_symbol = "BTCUSDT";
        let kline_symbol = "ETHUSDT";
        let ticker_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to average price stream
        let avg_price_spec = AveragePriceStreamSpec::new(avg_price_symbol);
        let mut avg_price_subscription = with_timeout(client.subscribe(&avg_price_spec)).await.expect("Average price subscription");
        let avg_price_event = with_recv_timeout(avg_price_subscription.recv()).await.expect("Receive average price event");
        
        // Subscribe to kline stream (different spec type)
        let kline_spec = KlineStreamSpec::five_minutes(kline_symbol);
        let mut kline_subscription = with_timeout(client.subscribe(&kline_spec)).await.expect("Kline subscription");
        let kline_event = with_recv_timeout(kline_subscription.recv()).await.expect("Receive kline event");
        
        // Subscribe to ticker stream (different spec type)
        let ticker_spec = TickerStreamSpec::new(ticker_symbol);
        let mut ticker_subscription = with_timeout(client.subscribe(&ticker_spec)).await.expect("Ticker subscription");
        let ticker_event = with_recv_timeout(ticker_subscription.recv()).await.expect("Receive ticker event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_average_price_event(&avg_price_event);
        assert_eq!(avg_price_event.symbol, avg_price_symbol);
        
        assert_valid_kline_event(&kline_event);
        assert_eq!(kline_event.symbol, kline_symbol);
        assert_eq!(kline_event.kline.interval, "5m");
        
        assert_valid_ticker_event(&ticker_event);
        assert_eq!(ticker_event.ticker.symbol, ticker_symbol);
    }

    /**
     * Tests book ticker stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_book_ticker_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = BookTickerStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_book_ticker_event(&event);
        assert_eq!(event.ticker.symbol, test_symbol);
    }

    /**
     * Tests book ticker stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_book_ticker_stream_combined() {
        // Arrange
        let first_spec = BookTickerStreamSpec::new("BTCUSDT");
        let second_spec = BookTickerStreamSpec::new("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC book ticker subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH book ticker subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_book_ticker_event(&btc_event);
        assert_eq!(btc_event.ticker.symbol, "BTCUSDT");
        
        assert_valid_book_ticker_event(&eth_event);
        assert_eq!(eth_event.ticker.symbol, "ETHUSDT");
    }

    /**
     * Tests book ticker stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_book_ticker_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let book_ticker_symbol = "BTCUSDT";
        let trade_symbol = "ETHUSDT";
        let avg_price_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to book ticker stream
        let book_ticker_spec = BookTickerStreamSpec::new(book_ticker_symbol);
        let mut book_ticker_subscription = with_timeout(client.subscribe(&book_ticker_spec)).await.expect("Book ticker subscription");
        let book_ticker_event = with_recv_timeout(book_ticker_subscription.recv()).await.expect("Receive book ticker event");
        
        // Subscribe to trade stream (different spec type)
        let trade_spec = TradeStreamSpec::new(trade_symbol);
        let mut trade_subscription = with_timeout(client.subscribe(&trade_spec)).await.expect("Trade subscription");
        let trade_event = with_recv_timeout(trade_subscription.recv()).await.expect("Receive trade event");
        
        // Subscribe to average price stream (different spec type)
        let avg_price_spec = AveragePriceStreamSpec::new(avg_price_symbol);
        let mut avg_price_subscription = with_timeout(client.subscribe(&avg_price_spec)).await.expect("Average price subscription");
        let avg_price_event = with_recv_timeout(avg_price_subscription.recv()).await.expect("Receive average price event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_book_ticker_event(&book_ticker_event);
        assert_eq!(book_ticker_event.ticker.symbol, book_ticker_symbol);
        
        assert_valid_trade_event(&trade_event);
        assert_eq!(trade_event.symbol, trade_symbol);
        
        assert_valid_average_price_event(&avg_price_event);
        assert_eq!(avg_price_event.symbol, avg_price_symbol);
    }

    /**
     * Tests diff depth stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_diff_depth_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = DiffDepthStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_diff_depth_event(&event);
        assert_eq!(event.symbol, test_symbol);
    }

    /**
     * Tests diff depth stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_diff_depth_stream_combined() {
        // Arrange
        let first_spec = DiffDepthStreamSpec::standard("BTCUSDT");
        let second_spec = DiffDepthStreamSpec::fast("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC diff depth subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH diff depth subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_diff_depth_event(&btc_event);
        assert_eq!(btc_event.symbol, "BTCUSDT");
        
        assert_valid_diff_depth_event(&eth_event);
        assert_eq!(eth_event.symbol, "ETHUSDT");
    }

    /**
     * Tests diff depth stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_diff_depth_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let diff_depth_symbol = "BTCUSDT";
        let book_ticker_symbol = "ETHUSDT";
        let agg_trade_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to diff depth stream (fast updates)
        let diff_depth_spec = DiffDepthStreamSpec::with_fast_updates(diff_depth_symbol);
        let mut diff_depth_subscription = with_timeout(client.subscribe(&diff_depth_spec)).await.expect("Diff depth subscription");
        let diff_depth_event = with_recv_timeout(diff_depth_subscription.recv()).await.expect("Receive diff depth event");
        
        // Subscribe to book ticker stream (different spec type)
        let book_ticker_spec = BookTickerStreamSpec::new(book_ticker_symbol);
        let mut book_ticker_subscription = with_timeout(client.subscribe(&book_ticker_spec)).await.expect("Book ticker subscription");
        let book_ticker_event = with_recv_timeout(book_ticker_subscription.recv()).await.expect("Receive book ticker event");
        
        // Subscribe to aggregate trade stream (different spec type)
        let agg_trade_spec = AggregateTradeStreamSpec::new(agg_trade_symbol);
        let mut agg_trade_subscription = with_timeout(client.subscribe(&agg_trade_spec)).await.expect("Aggregate trade subscription");
        let agg_trade_event = with_recv_timeout(agg_trade_subscription.recv()).await.expect("Receive aggregate trade event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_diff_depth_event(&diff_depth_event);
        assert_eq!(diff_depth_event.symbol, diff_depth_symbol);
        
        assert_valid_book_ticker_event(&book_ticker_event);
        assert_eq!(book_ticker_event.ticker.symbol, book_ticker_symbol);
        
        assert_valid_aggregate_trade_event(&agg_trade_event);
        assert_eq!(agg_trade_event.symbol, agg_trade_symbol);
    }

    /**
     * Tests kline stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_kline_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = KlineStreamSpec::one_minute(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_kline_event(&event);
        assert_eq!(event.symbol, test_symbol);
        assert_eq!(event.kline.interval, "1m");
    }

    /**
     * Tests kline stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_kline_stream_combined() {
        // Arrange
        let first_spec = KlineStreamSpec::five_minutes("BTCUSDT");
        let second_spec = KlineStreamSpec::hourly("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC kline subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH kline subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_kline_event(&btc_event);
        assert_eq!(btc_event.symbol, "BTCUSDT");
        assert_eq!(btc_event.kline.interval, "5m");
        
        assert_valid_kline_event(&eth_event);
        assert_eq!(eth_event.symbol, "ETHUSDT");
        assert_eq!(eth_event.kline.interval, "1h");
    }

    /**
     * Tests kline stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_kline_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let kline_symbol = "BTCUSDT";
        let trade_symbol = "ETHUSDT";
        let diff_depth_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to kline stream (daily interval)
        let kline_spec = KlineStreamSpec::daily(kline_symbol);
        let mut kline_subscription = with_timeout(client.subscribe(&kline_spec)).await.expect("Kline subscription");
        let kline_event = with_recv_timeout(kline_subscription.recv()).await.expect("Receive kline event");
        
        // Subscribe to trade stream (different spec type)
        let trade_spec = TradeStreamSpec::new(trade_symbol);
        let mut trade_subscription = with_timeout(client.subscribe(&trade_spec)).await.expect("Trade subscription");
        let trade_event = with_recv_timeout(trade_subscription.recv()).await.expect("Receive trade event");
        
        // Subscribe to diff depth stream (different spec type)
        let diff_depth_spec = DiffDepthStreamSpec::standard(diff_depth_symbol);
        let mut diff_depth_subscription = with_timeout(client.subscribe(&diff_depth_spec)).await.expect("Diff depth subscription");
        let diff_depth_event = with_recv_timeout(diff_depth_subscription.recv()).await.expect("Receive diff depth event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_kline_event(&kline_event);
        assert_eq!(kline_event.symbol, kline_symbol);
        assert_eq!(kline_event.kline.interval, "1d");
        
        assert_valid_trade_event(&trade_event);
        assert_eq!(trade_event.symbol, trade_symbol);
        
        assert_valid_diff_depth_event(&diff_depth_event);
        assert_eq!(diff_depth_event.symbol, diff_depth_symbol);
    }

    /**
     * Tests kline with timezone stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_kline_with_timezone_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = KlineWithTimezoneStreamSpec::one_minute_utc_plus_8(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_kline_event(&event);
        assert_eq!(event.symbol, test_symbol);
        assert_eq!(event.kline.interval, "1m");
    }

    /**
     * Tests kline with timezone stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_kline_with_timezone_stream_combined() {
        // Arrange
        let first_spec = KlineWithTimezoneStreamSpec::fifteen_minutes_utc_plus_8("BTCUSDT");
        let second_spec = KlineWithTimezoneStreamSpec::hourly_utc_plus_8("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC kline subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH kline subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_kline_event(&btc_event);
        assert_eq!(btc_event.symbol, "BTCUSDT");
        assert_eq!(btc_event.kline.interval, "15m");
        
        assert_valid_kline_event(&eth_event);
        assert_eq!(eth_event.symbol, "ETHUSDT");
        assert_eq!(eth_event.kline.interval, "1h");
    }

    /**
     * Tests kline with timezone stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_kline_with_timezone_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let kline_tz_symbol = "BTCUSDT";
        let avg_price_symbol = "ETHUSDT";
        let book_ticker_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to kline with timezone stream (daily UTC+8)
        let kline_tz_spec = KlineWithTimezoneStreamSpec::daily_utc_plus_8(kline_tz_symbol);
        let mut kline_tz_subscription = with_timeout(client.subscribe(&kline_tz_spec)).await.expect("Kline timezone subscription");
        let kline_tz_event = with_recv_timeout(kline_tz_subscription.recv()).await.expect("Receive kline timezone event");
        
        // Subscribe to average price stream (different spec type)
        let avg_price_spec = AveragePriceStreamSpec::new(avg_price_symbol);
        let mut avg_price_subscription = with_timeout(client.subscribe(&avg_price_spec)).await.expect("Average price subscription");
        let avg_price_event = with_recv_timeout(avg_price_subscription.recv()).await.expect("Receive average price event");
        
        // Subscribe to book ticker stream (different spec type)
        let book_ticker_spec = BookTickerStreamSpec::new(book_ticker_symbol);
        let mut book_ticker_subscription = with_timeout(client.subscribe(&book_ticker_spec)).await.expect("Book ticker subscription");
        let book_ticker_event = with_recv_timeout(book_ticker_subscription.recv()).await.expect("Receive book ticker event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_kline_event(&kline_tz_event);
        assert_eq!(kline_tz_event.symbol, kline_tz_symbol);
        assert_eq!(kline_tz_event.kline.interval, "1d");
        
        assert_valid_average_price_event(&avg_price_event);
        assert_eq!(avg_price_event.symbol, avg_price_symbol);
        
        assert_valid_book_ticker_event(&book_ticker_event);
        assert_eq!(book_ticker_event.ticker.symbol, book_ticker_symbol);
    }

    /**
     * Tests mini ticker stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_mini_ticker_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = MiniTickerStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_mini_ticker_event(&event);
        assert_eq!(event.symbol, test_symbol);
    }

    /**
     * Tests mini ticker stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_mini_ticker_stream_combined() {
        // Arrange
        let first_spec = MiniTickerStreamSpec::new("BTCUSDT");
        let second_spec = MiniTickerStreamSpec::new("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC mini ticker subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH mini ticker subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_mini_ticker_event(&btc_event);
        assert_eq!(btc_event.symbol, "BTCUSDT");
        
        assert_valid_mini_ticker_event(&eth_event);
        assert_eq!(eth_event.symbol, "ETHUSDT");
    }

    /**
     * Tests mini ticker stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_mini_ticker_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let mini_ticker_symbol = "BTCUSDT";
        let kline_symbol = "ETHUSDT";
        let diff_depth_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to mini ticker stream
        let mini_ticker_spec = MiniTickerStreamSpec::new(mini_ticker_symbol);
        let mut mini_ticker_subscription = with_timeout(client.subscribe(&mini_ticker_spec)).await.expect("Mini ticker subscription");
        let mini_ticker_event = with_recv_timeout(mini_ticker_subscription.recv()).await.expect("Receive mini ticker event");
        
        // Subscribe to kline stream (different spec type)
        let kline_spec = KlineStreamSpec::three_minutes(kline_symbol);
        let mut kline_subscription = with_timeout(client.subscribe(&kline_spec)).await.expect("Kline subscription");
        let kline_event = with_recv_timeout(kline_subscription.recv()).await.expect("Receive kline event");
        
        // Subscribe to diff depth stream (different spec type)
        let diff_depth_spec = DiffDepthStreamSpec::fast(diff_depth_symbol);
        let mut diff_depth_subscription = with_timeout(client.subscribe(&diff_depth_spec)).await.expect("Diff depth subscription");
        let diff_depth_event = with_recv_timeout(diff_depth_subscription.recv()).await.expect("Receive diff depth event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_mini_ticker_event(&mini_ticker_event);
        assert_eq!(mini_ticker_event.symbol, mini_ticker_symbol);
        
        assert_valid_kline_event(&kline_event);
        assert_eq!(kline_event.symbol, kline_symbol);
        assert_eq!(kline_event.kline.interval, "3m");
        
        assert_valid_diff_depth_event(&diff_depth_event);
        assert_eq!(diff_depth_event.symbol, diff_depth_symbol);
    }

    /**
     * Tests partial book depth stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_partial_book_depth_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = PartialBookDepthStreamSpec::levels_5(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_partial_book_depth_event(&event);
        assert!(event.bids.len() <= 5, "Should have at most 5 bids");
        assert!(event.asks.len() <= 5, "Should have at most 5 asks");
    }

    /**
     * Tests partial book depth stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_partial_book_depth_stream_combined() {
        // Arrange
        let first_spec = PartialBookDepthStreamSpec::levels_10("BTCUSDT");
        let second_spec = PartialBookDepthStreamSpec::levels_20_fast("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC partial depth subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH partial depth subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_partial_book_depth_event(&btc_event);
        assert!(btc_event.bids.len() <= 10, "Should have at most 10 bids");
        assert!(btc_event.asks.len() <= 10, "Should have at most 10 asks");
        
        assert_valid_partial_book_depth_event(&eth_event);
        assert!(eth_event.bids.len() <= 20, "Should have at most 20 bids");
        assert!(eth_event.asks.len() <= 20, "Should have at most 20 asks");
    }

    /**
     * Tests partial book depth stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_partial_book_depth_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let partial_depth_symbol = "BTCUSDT";
        let mini_ticker_symbol = "ETHUSDT";
        let agg_trade_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to partial book depth stream (20 levels, fast updates)
        let partial_depth_spec = PartialBookDepthStreamSpec::levels_20_fast(partial_depth_symbol);
        let mut partial_depth_subscription = with_timeout(client.subscribe(&partial_depth_spec)).await.expect("Partial depth subscription");
        let partial_depth_event = with_recv_timeout(partial_depth_subscription.recv()).await.expect("Receive partial depth event");
        
        // Subscribe to mini ticker stream (different spec type)
        let mini_ticker_spec = MiniTickerStreamSpec::new(mini_ticker_symbol);
        let mut mini_ticker_subscription = with_timeout(client.subscribe(&mini_ticker_spec)).await.expect("Mini ticker subscription");
        let mini_ticker_event = with_recv_timeout(mini_ticker_subscription.recv()).await.expect("Receive mini ticker event");
        
        // Subscribe to aggregate trade stream (different spec type)
        let agg_trade_spec = AggregateTradeStreamSpec::new(agg_trade_symbol);
        let mut agg_trade_subscription = with_timeout(client.subscribe(&agg_trade_spec)).await.expect("Aggregate trade subscription");
        let agg_trade_event = with_recv_timeout(agg_trade_subscription.recv()).await.expect("Receive aggregate trade event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_partial_book_depth_event(&partial_depth_event);
        assert!(partial_depth_event.bids.len() <= 20, "Should have at most 20 bids");
        assert!(partial_depth_event.asks.len() <= 20, "Should have at most 20 asks");
        
        assert_valid_mini_ticker_event(&mini_ticker_event);
        assert_eq!(mini_ticker_event.symbol, mini_ticker_symbol);
        
        assert_valid_aggregate_trade_event(&agg_trade_event);
        assert_eq!(agg_trade_event.symbol, agg_trade_symbol);
    }

    /**
     * Tests rolling window ticker stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_rolling_window_ticker_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = RollingWindowTickerStreamSpec::hourly(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_rolling_window_ticker_event(&event);
        assert_eq!(event.ticker.symbol, test_symbol);
    }

    /**
     * Tests rolling window ticker stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_rolling_window_ticker_stream_combined() {
        // Arrange
        let first_spec = RollingWindowTickerStreamSpec::four_hourly("BTCUSDT");
        let second_spec = RollingWindowTickerStreamSpec::daily("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC rolling window subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH rolling window subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_rolling_window_ticker_event(&btc_event);
        assert_eq!(btc_event.ticker.symbol, "BTCUSDT");
        
        assert_valid_rolling_window_ticker_event(&eth_event);
        assert_eq!(eth_event.ticker.symbol, "ETHUSDT");
    }

    /**
     * Tests rolling window ticker stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_rolling_window_ticker_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let rolling_window_symbol = "BTCUSDT";
        let book_ticker_symbol = "ETHUSDT";
        let kline_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to rolling window ticker stream (1-hour window)
        let rolling_window_spec = RollingWindowTickerStreamSpec::hourly(rolling_window_symbol);
        let mut rolling_window_subscription = with_timeout(client.subscribe(&rolling_window_spec)).await.expect("Rolling window ticker subscription");
        let rolling_window_event = with_recv_timeout(rolling_window_subscription.recv()).await.expect("Receive rolling window event");
        
        // Subscribe to book ticker stream (different spec type)
        let book_ticker_spec = BookTickerStreamSpec::new(book_ticker_symbol);
        let mut book_ticker_subscription = with_timeout(client.subscribe(&book_ticker_spec)).await.expect("Book ticker subscription");
        let book_ticker_event = with_recv_timeout(book_ticker_subscription.recv()).await.expect("Receive book ticker event");
        
        // Subscribe to kline stream (different spec type)
        let kline_spec = KlineStreamSpec::thirty_minutes(kline_symbol);
        let mut kline_subscription = with_timeout(client.subscribe(&kline_spec)).await.expect("Kline subscription");
        let kline_event = with_recv_timeout(kline_subscription.recv()).await.expect("Receive kline event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_rolling_window_ticker_event(&rolling_window_event);
        assert_eq!(rolling_window_event.ticker.symbol, rolling_window_symbol);
        
        assert_valid_book_ticker_event(&book_ticker_event);
        assert_eq!(book_ticker_event.ticker.symbol, book_ticker_symbol);
        
        assert_valid_kline_event(&kline_event);
        assert_eq!(kline_event.symbol, kline_symbol);
        assert_eq!(kline_event.kline.interval, "30m");
    }

    /**
     * Tests ticker stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_ticker_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = TickerStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_ticker_event(&event);
        assert_eq!(event.ticker.symbol, test_symbol);
    }

    /**
     * Tests ticker stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_ticker_stream_combined() {
        // Arrange
        let first_spec = TickerStreamSpec::new("BTCUSDT");
        let second_spec = TickerStreamSpec::new("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC ticker subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH ticker subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_ticker_event(&btc_event);
        assert_eq!(btc_event.ticker.symbol, "BTCUSDT");
        
        assert_valid_ticker_event(&eth_event);
        assert_eq!(eth_event.ticker.symbol, "ETHUSDT");
    }

    /**
     * Tests ticker stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_ticker_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let ticker_symbol = "BTCUSDT";
        let partial_depth_symbol = "ETHUSDT";
        let avg_price_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to ticker stream
        let ticker_spec = TickerStreamSpec::new(ticker_symbol);
        let mut ticker_subscription = with_timeout(client.subscribe(&ticker_spec)).await.expect("Ticker subscription");
        let ticker_event = with_recv_timeout(ticker_subscription.recv()).await.expect("Receive ticker event");
        
        // Subscribe to partial book depth stream (different spec type)
        let partial_depth_spec = PartialBookDepthStreamSpec::levels_10(partial_depth_symbol);
        let mut partial_depth_subscription = with_timeout(client.subscribe(&partial_depth_spec)).await.expect("Partial depth subscription");
        let partial_depth_event = with_recv_timeout(partial_depth_subscription.recv()).await.expect("Receive partial depth event");
        
        // Subscribe to average price stream (different spec type)
        let avg_price_spec = AveragePriceStreamSpec::new(avg_price_symbol);
        let mut avg_price_subscription = with_timeout(client.subscribe(&avg_price_spec)).await.expect("Average price subscription");
        let avg_price_event = with_recv_timeout(avg_price_subscription.recv()).await.expect("Receive average price event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_ticker_event(&ticker_event);
        assert_eq!(ticker_event.ticker.symbol, ticker_symbol);
        
        assert_valid_partial_book_depth_event(&partial_depth_event);
        assert!(partial_depth_event.bids.len() <= 10, "Should have at most 10 bids");
        assert!(partial_depth_event.asks.len() <= 10, "Should have at most 10 asks");
        
        assert_valid_average_price_event(&avg_price_event);
        assert_eq!(avg_price_event.symbol, avg_price_symbol);
    }

    /**
     * Tests trade stream with raw stream mode.
     */
    #[tokio::test]
    async fn test_trade_stream_raw() {
        // Arrange
        let test_symbol = "BTCUSDT";
        let spec = TradeStreamSpec::new(test_symbol);
        let mut client = create_raw_stream_client(&spec).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        let mut subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        let event = with_recv_timeout(subscription.recv()).await.expect("Receive event");
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_trade_event(&event);
        assert_eq!(event.symbol, test_symbol);
    }

    /**
     * Tests trade stream with combined stream mode.
     */
    #[tokio::test]
    async fn test_trade_stream_combined() {
        // Arrange
        let first_spec = TradeStreamSpec::new("BTCUSDT");
        let second_spec = TradeStreamSpec::new("ETHUSDT");
        let specs = [&first_spec, &second_spec];
        let mut client = create_combined_stream_client(specs).expect("Client creation");
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        let mut btc_subscription = with_timeout(client.subscribe(&first_spec)).await.expect("BTC trade subscription");
        let btc_event = with_recv_timeout(btc_subscription.recv()).await.expect("Receive BTC event");
        
        let mut eth_subscription = with_timeout(client.subscribe(&second_spec)).await.expect("ETH trade subscription");
        let eth_event = with_recv_timeout(eth_subscription.recv()).await.expect("Receive ETH event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_trade_event(&btc_event);
        assert_eq!(btc_event.symbol, "BTCUSDT");
        
        assert_valid_trade_event(&eth_event);
        assert_eq!(eth_event.symbol, "ETHUSDT");
    }

    /**
     * Tests trade stream with dynamic mode involving multiple different specs.
     */
    #[tokio::test]
    async fn test_trade_stream_dynamic() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let trade_symbol = "BTCUSDT";
        let rolling_window_symbol = "ETHUSDT";
        let kline_tz_symbol = "ADAUSDT";
        
        // Act
        let _ = with_timeout(client.wait_for_connection()).await.expect("Connection");
        
        // Subscribe to trade stream
        let trade_spec = TradeStreamSpec::new(trade_symbol);
        let mut trade_subscription = with_timeout(client.subscribe(&trade_spec)).await.expect("Trade subscription");
        let trade_event = with_recv_timeout(trade_subscription.recv()).await.expect("Receive trade event");
        
        // Subscribe to rolling window ticker stream (different spec type)
        let rolling_window_spec = RollingWindowTickerStreamSpec::four_hourly(rolling_window_symbol);
        let mut rolling_window_subscription = with_timeout(client.subscribe(&rolling_window_spec)).await.expect("Rolling window subscription");
        let rolling_window_event = with_recv_timeout(rolling_window_subscription.recv()).await.expect("Receive rolling window event");
        
        // Subscribe to kline with timezone stream (different spec type)
        let kline_tz_spec = KlineWithTimezoneStreamSpec::five_minutes_utc_plus_8(kline_tz_symbol);
        let mut kline_tz_subscription = with_timeout(client.subscribe(&kline_tz_spec)).await.expect("Kline timezone subscription");
        let kline_tz_event = with_recv_timeout(kline_tz_subscription.recv()).await.expect("Receive kline timezone event");
        
        let _ = with_timeout(client.close()).await;
        
        // Assert
        assert_valid_trade_event(&trade_event);
        assert_eq!(trade_event.symbol, trade_symbol);
        
        assert_valid_rolling_window_ticker_event(&rolling_window_event);
        assert_eq!(rolling_window_event.ticker.symbol, rolling_window_symbol);
        
        assert_valid_kline_event(&kline_tz_event);
        assert_eq!(kline_tz_event.symbol, kline_tz_symbol);
        assert_eq!(kline_tz_event.kline.interval, "5m");
    }

    #[tokio::test]
    async fn test_connection_status_initial_state() {
        // Arrange
        let client = create_dynamic_stream_client().expect("Client creation");

        // Act
        let status = client.connection_status();

        // Assert
        assert_eq!(status, ConnectionStatus::Connecting, "Initial status should be Connecting");
    }

    #[tokio::test]
    async fn test_connection_status_after_successful_connection() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");

        // Act
        with_timeout(client.wait_for_connection()).await.expect("Connection");
        let status = client.connection_status();

        // Assert
        assert_eq!(status, ConnectionStatus::Connected, "Status should be Connected after successful connection");
    }

    #[tokio::test]
    async fn test_is_connected_after_successful_connection() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");

        // Act
        with_timeout(client.wait_for_connection()).await.expect("Connection");
        let is_connected = client.is_connected();

        // Assert
        assert!(is_connected, "Client should report as connected");
    }

    #[tokio::test]
    async fn test_connection_status_transitions() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");

        // Act & Assert - Initial state
        assert_eq!(client.connection_status(), ConnectionStatus::Connecting);
        assert!(!client.is_connected());

        // Act & Assert - After connection
        with_timeout(client.wait_for_connection()).await.expect("Connection");
        assert_eq!(client.connection_status(), ConnectionStatus::Connected);
        assert!(client.is_connected());

        // Act & Assert - After close
        with_timeout(client.close()).await.expect("Close");
        assert!(!client.is_connected());
    }

    #[tokio::test] 
    async fn test_connection_status_max_retries_exceeded() {
        // Arrange - Create config with limited retries and invalid endpoint
        let config = BinanceConfig::<StreamConfig>::builder()
            .with_market_data_url("wss://invalid.nonexistent.endpoint.test")  // Invalid endpoint to force failures
            .with_stream_config(
                StreamConfig::builder()
                    .with_max_reconnects(2)
                    .with_connection_timeout(Duration::from_millis(500))
                    .with_initial_retry_delay(Duration::from_millis(100))
                    .build()
            )
            .with_market_data()
            .with_dynamic_streams()
            .build()
            .expect("Config creation");
        let client = crate::streams::client(config)
            .expect("Client creation");

        // Act
        tokio::time::sleep(Duration::from_secs(5)).await;
        let status = client.connection_status();

        // Assert
        assert_eq!(status, ConnectionStatus::Failed, "Should be Failed after max retries");
    }

    #[tokio::test]
    async fn test_connection_status_during_subscription() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        let spec = AggregateTradeStreamSpec::new("BTCUSDT");

        // Act
        with_timeout(client.wait_for_connection()).await.expect("Connection");
        assert_eq!(client.connection_status(), ConnectionStatus::Connected);

        // Subscribe while connected
        let _subscription = with_timeout(client.subscribe(&spec)).await.expect("Subscription");
        
        // Assert - Status should remain connected during subscription
        assert_eq!(client.connection_status(), ConnectionStatus::Connected);
    }

    #[tokio::test]
    async fn test_wait_for_connection_already_connected() {
        // Arrange
        let mut client = create_dynamic_stream_client().expect("Client creation");
        
        // Act - Connect first time
        with_timeout(client.wait_for_connection()).await.expect("First connection");
        
        // Act - Call wait_for_connection again when already connected
        let result = with_timeout(client.wait_for_connection()).await;
        
        // Assert - Should return immediately since already connected
        assert!(result.is_ok());
        assert_eq!(client.connection_status(), ConnectionStatus::Connected);
    }
}