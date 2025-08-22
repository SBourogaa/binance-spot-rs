#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use crate::{
        clients::{
            r#trait::TickerClient,
            tests::helpers::*,
        },
        types::{
            requests::{
                Ticker24HrSpec,
                TickerPriceSpec,
                TickerBookSpec,
                TickerRollingWindowSpec,
                TickerTradingDaySpec,
            },
            responses::{TickerStatistics, TickerPrice, TickerBook},
        },
        errors::{BinanceError, ErrorCategory, RequestError},
    };

    /**
     * Validates ticker price response structure.
     */
    fn assert_valid_ticker_price(price: &TickerPrice) {
        assert!(!price.symbol.is_empty(), "Symbol should not be empty");
        assert!(price.price > Decimal::ZERO, "Price should be positive");
    }

    /**
     * Validates ticker book response structure.
     */
    fn assert_valid_ticker_book(book: &TickerBook) {
        assert!(!book.symbol.is_empty(), "Symbol should not be empty");
        assert!(book.bid_price > Decimal::ZERO, "Bid price should be positive");
        assert!(book.ask_price > Decimal::ZERO, "Ask price should be positive");
        assert!(book.bid_quantity > Decimal::ZERO, "Bid quantity should be positive");
        assert!(book.ask_quantity > Decimal::ZERO, "Ask quantity should be positive");
        assert!(book.ask_price >= book.bid_price, "Ask price should be >= bid price");
    }

    /**
     * Validates ticker statistics response structure.
     */
    fn assert_valid_ticker_statistics(stats: &TickerStatistics) {
        match stats {
            TickerStatistics::Full(full) => {
                assert!(!full.symbol.is_empty(), "Symbol should not be empty");
                assert!(full.last_price > Decimal::ZERO, "Last price should be positive");
                assert!(full.volume >= Decimal::ZERO, "Volume should be non-negative");
                assert!(full.quote_volume >= Decimal::ZERO, "Quote volume should be non-negative");
                assert!(full.open_time > 0, "Open time should be positive");
                assert!(full.close_time > 0, "Close time should be positive");
                assert!(full.close_time >= full.open_time, "Close time should be >= open time");
            }
            TickerStatistics::Mini(mini) => {
                assert!(!mini.symbol.is_empty(), "Symbol should not be empty");
                assert!(mini.last_price > Decimal::ZERO, "Last price should be positive");
                assert!(mini.volume >= Decimal::ZERO, "Volume should be non-negative");
                assert!(mini.quote_volume >= Decimal::ZERO, "Quote volume should be non-negative");
                assert!(mini.open_time > 0, "Open time should be positive");
                assert!(mini.close_time > 0, "Close time should be positive");
                assert!(mini.close_time >= mini.open_time, "Close time should be >= open time");
            }
        }
    }

    /**
     * Tests trading day ticker error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_ticker_trading_day_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = TickerTradingDaySpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.ticker_trading_day(rest_spec)).await;

        let ws_spec = TickerTradingDaySpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.ticker_trading_day(ws_spec)).await;

        // Assert both
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(stats) => panic!("Expected {} error, got successful response: {:?}", client_name, stats),
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
     * Tests rolling window ticker error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_ticker_rolling_window_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = TickerRollingWindowSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.ticker_rolling_window(rest_spec)).await;

        let ws_spec = TickerRollingWindowSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.ticker_rolling_window(ws_spec)).await;

        // Assert both
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(stats) => panic!("Expected {} error, got successful response: {:?}", client_name, stats),
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
     * Tests ticker price retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_ticker_price_all_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        
        // Act
        let rest_spec = TickerPriceSpec::new().build().expect("Spec validation");
        let rest_prices = with_timeout(rest_client.ticker_price(rest_spec)).await.expect("REST ticker price");
        
        let ws_spec = TickerPriceSpec::new().build().expect("Spec validation");
        let ws_prices = with_timeout(ws_client.ticker_price(ws_spec)).await.expect("WebSocket ticker price");
        
        // Assert
        assert!(!rest_prices.is_empty(), "REST should return at least one price");
        assert!(!ws_prices.is_empty(), "WebSocket should return at least one price");
        
        for price in &rest_prices[..5.min(rest_prices.len())] {
            assert_valid_ticker_price(price);
        }
        
        for price in &ws_prices[..5.min(ws_prices.len())] {
            assert_valid_ticker_price(price);
        }
        
        assert_eq!(rest_prices.len(), ws_prices.len(), "Price count should match");
    }

    /**
     * Tests ticker price retrieval with specific symbol filter.
     */
    #[tokio::test]
    async fn test_ticker_price_specific_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = TickerPriceSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let rest_prices = with_timeout(rest_client.ticker_price(rest_spec)).await.expect("REST ticker price");
        
        let ws_spec = TickerPriceSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let ws_prices = with_timeout(ws_client.ticker_price(ws_spec)).await.expect("WebSocket ticker price");
        
        // Assert
        assert_eq!(rest_prices.len(), 1, "Should return exactly one price");
        assert_eq!(ws_prices.len(), 1, "Should return exactly one price");
        assert_eq!(rest_prices[0].symbol, test_symbol);
        assert_eq!(ws_prices[0].symbol, test_symbol);
        
        assert_valid_ticker_price(&rest_prices[0]);
        assert_valid_ticker_price(&ws_prices[0]);
    }

    /**
     * Tests ticker price retrieval with multiple symbols filter.
     */
    #[tokio::test]
    async fn test_ticker_price_multiple_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbols = vec!["BTCUSDT", "ETHUSDT"];
        
        // Act
        let rest_spec = TickerPriceSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let rest_prices = with_timeout(rest_client.ticker_price(rest_spec)).await.expect("REST ticker price");
        
        let ws_spec = TickerPriceSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let ws_prices = with_timeout(ws_client.ticker_price(ws_spec)).await.expect("WebSocket ticker price");
        
        // Assert
        assert_eq!(rest_prices.len(), 2, "Should return exactly two prices");
        assert_eq!(ws_prices.len(), 2, "Should return exactly two prices");
        assert!(rest_prices.iter().any(|p| p.symbol == "BTCUSDT"));
        assert!(rest_prices.iter().any(|p| p.symbol == "ETHUSDT"));
        assert!(ws_prices.iter().any(|p| p.symbol == "BTCUSDT"));
        assert!(ws_prices.iter().any(|p| p.symbol == "ETHUSDT"));
        
        for price in &rest_prices {
            assert_valid_ticker_price(price);
        }
        
        for price in &ws_prices {
            assert_valid_ticker_price(price);
        }
    }

    /**
     * Tests ticker book retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_ticker_book_all_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        
        // Act
        let rest_spec = TickerBookSpec::new().build().expect("Spec validation");
        let rest_books = with_timeout(rest_client.ticker_book(rest_spec)).await.expect("REST ticker book");
        
        let ws_spec = TickerBookSpec::new().build().expect("Spec validation");
        let ws_books = with_timeout(ws_client.ticker_book(ws_spec)).await.expect("WebSocket ticker book");
        
        // Assert
        assert!(!rest_books.is_empty(), "REST should return at least one book ticker");
        assert!(!ws_books.is_empty(), "WebSocket should return at least one book ticker");
        
        for book in &rest_books[..5.min(rest_books.len())] {
            assert_valid_ticker_book(book);
        }
        
        for book in &ws_books[..5.min(ws_books.len())] {
            assert_valid_ticker_book(book);
        }
        
        assert_eq!(rest_books.len(), ws_books.len(), "Book ticker count should match");
    }

    /**
     * Tests ticker book retrieval with specific symbol filter.
     */
    #[tokio::test]
    async fn test_ticker_book_specific_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = TickerBookSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let rest_books = with_timeout(rest_client.ticker_book(rest_spec)).await.expect("REST ticker book");
        
        let ws_spec = TickerBookSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let ws_books = with_timeout(ws_client.ticker_book(ws_spec)).await.expect("WebSocket ticker book");
        
        // Assert
        assert_eq!(rest_books.len(), 1, "Should return exactly one book ticker");
        assert_eq!(ws_books.len(), 1, "Should return exactly one book ticker");
        assert_eq!(rest_books[0].symbol, test_symbol);
        assert_eq!(ws_books[0].symbol, test_symbol);
        
        assert_valid_ticker_book(&rest_books[0]);
        assert_valid_ticker_book(&ws_books[0]);
    }

    /**
     * Tests ticker book retrieval with multiple symbols filter.
     */
    #[tokio::test]
    async fn test_ticker_book_multiple_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbols = vec!["BTCUSDT", "ETHUSDT"];
        
        // Act
        let rest_spec = TickerBookSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let rest_books = with_timeout(rest_client.ticker_book(rest_spec)).await.expect("REST ticker book");
        
        let ws_spec = TickerBookSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let ws_books = with_timeout(ws_client.ticker_book(ws_spec)).await.expect("WebSocket ticker book");
        
        // Assert
        assert_eq!(rest_books.len(), 2, "Should return exactly two book tickers");
        assert_eq!(ws_books.len(), 2, "Should return exactly two book tickers");
        assert!(rest_books.iter().any(|b| b.symbol == "BTCUSDT"));
        assert!(rest_books.iter().any(|b| b.symbol == "ETHUSDT"));
        assert!(ws_books.iter().any(|b| b.symbol == "BTCUSDT"));
        assert!(ws_books.iter().any(|b| b.symbol == "ETHUSDT"));
        
        for book in &rest_books {
            assert_valid_ticker_book(book);
        }
        
        for book in &ws_books {
            assert_valid_ticker_book(book);
        }
    }

    /**
     * Tests 24hr ticker statistics retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_ticker_24hr_specific_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = Ticker24HrSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_24hr(rest_spec)).await.expect("REST 24hr ticker");
        
        let ws_spec = Ticker24HrSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_24hr(ws_spec)).await.expect("WebSocket 24hr ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        // Check symbol matches
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests rolling window ticker statistics with multiple symbols filter.
     */
    #[tokio::test]
    async fn test_ticker_rolling_window_multiple_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbols = vec!["BTCUSDT", "ETHUSDT"];
        
        // Act
        let rest_spec = TickerRollingWindowSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_rolling_window(rest_spec)).await.expect("REST rolling window ticker");
        
        let ws_spec = TickerRollingWindowSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_rolling_window(ws_spec)).await.expect("WebSocket rolling window ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 2, "Should return exactly two ticker statistics");
        assert_eq!(ws_stats.len(), 2, "Should return exactly two ticker statistics");
        
        for stat in &rest_stats {
            assert_valid_ticker_statistics(stat);
        }
        
        for stat in &ws_stats {
            assert_valid_ticker_statistics(stat);
        }
        
        let rest_symbols: Vec<String> = rest_stats.iter().map(|s| match s {
            TickerStatistics::Full(full) => full.symbol.clone(),
            TickerStatistics::Mini(mini) => mini.symbol.clone(),
        }).collect();
        
        let ws_symbols: Vec<String> = ws_stats.iter().map(|s| match s {
            TickerStatistics::Full(full) => full.symbol.clone(),
            TickerStatistics::Mini(mini) => mini.symbol.clone(),
        }).collect();
        
        assert!(rest_symbols.contains(&"BTCUSDT".to_string()));
        assert!(rest_symbols.contains(&"ETHUSDT".to_string()));
        assert!(ws_symbols.contains(&"BTCUSDT".to_string()));
        assert!(ws_symbols.contains(&"ETHUSDT".to_string()));
    }

    /**
     * Tests rolling window ticker statistics with MINI ticker type.
     */
    #[tokio::test]
    async fn test_ticker_rolling_window_mini_type() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = TickerRollingWindowSpec::new()
            .with_symbol(test_symbol)
            .with_ticker_type("MINI")
            .build()
            .expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_rolling_window(rest_spec)).await.expect("REST rolling window ticker");
        
        let ws_spec = TickerRollingWindowSpec::new()
            .with_symbol(test_symbol)
            .with_ticker_type("MINI")
            .build()
            .expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_rolling_window(ws_spec)).await.expect("WebSocket rolling window ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests trading day ticker statistics with multiple symbols filter.
     */
    #[tokio::test]
    async fn test_ticker_trading_day_multiple_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbols = vec!["BTCUSDT", "ETHUSDT"];
        
        // Act
        let rest_spec = TickerTradingDaySpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_trading_day(rest_spec)).await.expect("REST trading day ticker");
        
        let ws_spec = TickerTradingDaySpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_trading_day(ws_spec)).await.expect("WebSocket trading day ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 2, "Should return exactly two ticker statistics");
        assert_eq!(ws_stats.len(), 2, "Should return exactly two ticker statistics");
        
        for stat in &rest_stats {
            assert_valid_ticker_statistics(stat);
        }
        
        for stat in &ws_stats {
            assert_valid_ticker_statistics(stat);
        }
        
        let rest_symbols: Vec<String> = rest_stats.iter().map(|s| match s {
            TickerStatistics::Full(full) => full.symbol.clone(),
            TickerStatistics::Mini(mini) => mini.symbol.clone(),
        }).collect();
        
        let ws_symbols: Vec<String> = ws_stats.iter().map(|s| match s {
            TickerStatistics::Full(full) => full.symbol.clone(),
            TickerStatistics::Mini(mini) => mini.symbol.clone(),
        }).collect();
        
        assert!(rest_symbols.contains(&"BTCUSDT".to_string()));
        assert!(rest_symbols.contains(&"ETHUSDT".to_string()));
        assert!(ws_symbols.contains(&"BTCUSDT".to_string()));
        assert!(ws_symbols.contains(&"ETHUSDT".to_string()));
    }

    /**
     * Tests trading day ticker statistics with MINI ticker type.
     */
    #[tokio::test]
    async fn test_ticker_trading_day_mini_type() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = TickerTradingDaySpec::new()
            .with_symbol(test_symbol)
            .with_ticker_type("MINI")
            .build()
            .expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_trading_day(rest_spec)).await.expect("REST trading day ticker");
        
        let ws_spec = TickerTradingDaySpec::new()
            .with_symbol(test_symbol)
            .with_ticker_type("MINI")
            .build()
            .expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_trading_day(ws_spec)).await.expect("WebSocket trading day ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests 24hr ticker statistics with multiple symbols filter.
     */
    #[tokio::test]
    async fn test_ticker_24hr_multiple_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbols = vec!["BTCUSDT", "ETHUSDT"];
        
        // Act
        let rest_spec = Ticker24HrSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_24hr(rest_spec)).await.expect("REST 24hr ticker");
        
        let ws_spec = Ticker24HrSpec::new().with_symbols(test_symbols.clone()).build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_24hr(ws_spec)).await.expect("WebSocket 24hr ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 2, "Should return exactly two ticker statistics");
        assert_eq!(ws_stats.len(), 2, "Should return exactly two ticker statistics");
        
        for stat in &rest_stats {
            assert_valid_ticker_statistics(stat);
        }
        
        for stat in &ws_stats {
            assert_valid_ticker_statistics(stat);
        }
        
        // Check both symbols are present
        let rest_symbols: Vec<String> = rest_stats.iter().map(|s| match s {
            TickerStatistics::Full(full) => full.symbol.clone(),
            TickerStatistics::Mini(mini) => mini.symbol.clone(),
        }).collect();
        
        let ws_symbols: Vec<String> = ws_stats.iter().map(|s| match s {
            TickerStatistics::Full(full) => full.symbol.clone(),
            TickerStatistics::Mini(mini) => mini.symbol.clone(),
        }).collect();
        
        assert!(rest_symbols.contains(&"BTCUSDT".to_string()));
        assert!(rest_symbols.contains(&"ETHUSDT".to_string()));
        assert!(ws_symbols.contains(&"BTCUSDT".to_string()));
        assert!(ws_symbols.contains(&"ETHUSDT".to_string()));
    }

    /**
     * Tests 24hr ticker statistics retrieval for all symbols.
     */
    #[tokio::test]
    async fn test_ticker_24hr_all_symbols() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        
        // Act
        let rest_spec = Ticker24HrSpec::new().build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_24hr(rest_spec)).await.expect("REST 24hr ticker");
        
        let ws_spec = Ticker24HrSpec::new().build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_24hr(ws_spec)).await.expect("WebSocket 24hr ticker");
        
        // Assert
        assert!(!rest_stats.is_empty(), "REST should return at least one ticker statistic");
        assert!(!ws_stats.is_empty(), "WebSocket should return at least one ticker statistic");
        
        for stat in &rest_stats[..5.min(rest_stats.len())] {
            assert_valid_ticker_statistics(stat);
        }
        
        for stat in &ws_stats[..5.min(ws_stats.len())] {
            assert_valid_ticker_statistics(stat);
        }
        
        assert_eq!(rest_stats.len(), ws_stats.len(), "Ticker statistics count should match");
    }

    /**
     * Tests trading day ticker statistics retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_ticker_trading_day_specific_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = TickerTradingDaySpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_trading_day(rest_spec)).await.expect("REST trading day ticker");
        
        let ws_spec = TickerTradingDaySpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_trading_day(ws_spec)).await.expect("WebSocket trading day ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        // Check symbol matches
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests trading day ticker statistics with custom timezone.
     */
    #[tokio::test]
    async fn test_ticker_trading_day_custom_timezone() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let timezone = "8"; // GMT+8
        
        // Act
        let rest_spec = TickerTradingDaySpec::new()
            .with_symbol(test_symbol)
            .with_time_zone(timezone)
            .build()
            .expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_trading_day(rest_spec)).await.expect("REST trading day ticker");
        
        let ws_spec = TickerTradingDaySpec::new()
            .with_symbol(test_symbol)
            .with_time_zone(timezone)
            .build()
            .expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_trading_day(ws_spec)).await.expect("WebSocket trading day ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        // Check symbol matches
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests 24hr ticker statistics with MINI ticker type.
     */
    #[tokio::test]
    async fn test_ticker_24hr_mini_type() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = Ticker24HrSpec::new()
            .with_symbol(test_symbol)
            .with_ticker_type("MINI")
            .build()
            .expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_24hr(rest_spec)).await.expect("REST 24hr ticker");
        
        let ws_spec = Ticker24HrSpec::new()
            .with_symbol(test_symbol)
            .with_ticker_type("MINI")
            .build()
            .expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_24hr(ws_spec)).await.expect("WebSocket 24hr ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        // Verify MINI type was returned (though API may still return FULL)
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests rolling window ticker statistics retrieval for both REST and WebSocket clients.
     */
    #[tokio::test]
    async fn test_ticker_rolling_window_specific_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        
        // Act
        let rest_spec = TickerRollingWindowSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_rolling_window(rest_spec)).await.expect("REST rolling window ticker");
        
        let ws_spec = TickerRollingWindowSpec::new().with_symbol(test_symbol).build().expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_rolling_window(ws_spec)).await.expect("WebSocket rolling window ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        // Check symbol matches
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests rolling window ticker statistics with custom window size.
     */
    #[tokio::test]
    async fn test_ticker_rolling_window_custom_window_size() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");
        let test_symbol = "BTCUSDT";
        let window_size = "7d";
        
        // Act
        let rest_spec = TickerRollingWindowSpec::new()
            .with_symbol(test_symbol)
            .with_window_size(window_size)
            .build()
            .expect("Spec validation");
        let rest_stats = with_timeout(rest_client.ticker_rolling_window(rest_spec)).await.expect("REST rolling window ticker");
        
        let ws_spec = TickerRollingWindowSpec::new()
            .with_symbol(test_symbol)
            .with_window_size(window_size)
            .build()
            .expect("Spec validation");
        let ws_stats = with_timeout(ws_client.ticker_rolling_window(ws_spec)).await.expect("WebSocket rolling window ticker");
        
        // Assert
        assert_eq!(rest_stats.len(), 1, "Should return exactly one ticker statistic");
        assert_eq!(ws_stats.len(), 1, "Should return exactly one ticker statistic");
        
        assert_valid_ticker_statistics(&rest_stats[0]);
        assert_valid_ticker_statistics(&ws_stats[0]);
        
        // Check symbol matches
        match &rest_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
        
        match &ws_stats[0] {
            TickerStatistics::Full(full) => assert_eq!(full.symbol, test_symbol),
            TickerStatistics::Mini(mini) => assert_eq!(mini.symbol, test_symbol),
        }
    }

    /**
     * Tests ticker price error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_ticker_price_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = TickerPriceSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.ticker_price(rest_spec)).await;

        let ws_spec = TickerPriceSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.ticker_price(ws_spec)).await;

        // Assert REST
        match rest_result {
            Ok(prices) => panic!("Expected REST error, got successful response: {:?}", prices),
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
                    "Unexpected REST error: {:#?}",
                    downcast
                );
            }
        }

        // Assert WS
        match ws_result {
            Ok(prices) => panic!("Expected WebSocket error, got successful response: {:?}", prices),
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
                    "Unexpected WebSocket error: {:#?}",
                    downcast
                );
            }
        }
    }

    /**
     * Tests ticker book error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_ticker_book_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = TickerBookSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.ticker_book(rest_spec)).await;

        let ws_spec = TickerBookSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.ticker_book(ws_spec)).await;

        // Assert both should return the same error pattern
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(books) => panic!("Expected {} error, got successful response: {:?}", client_name, books),
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
     * Tests 24hr ticker statistics error handling with invalid symbol.
     */
    #[tokio::test]
    async fn test_ticker_24hr_invalid_symbol() {
        // Arrange
        let rest_client = create_rest_client().expect("REST client creation");
        let ws_client = create_websocket_client().expect("WebSocket client creation");

        // Act
        let rest_spec = Ticker24HrSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let rest_result = with_timeout(rest_client.ticker_24hr(rest_spec)).await;

        let ws_spec = Ticker24HrSpec::new()
            .with_symbol("INVALID")
            .build()
            .expect("Spec validation");
        let ws_result = with_timeout(ws_client.ticker_24hr(ws_spec)).await;

        // Assert both should return the same error pattern
        for (client_name, result) in [("REST", rest_result), ("WebSocket", ws_result)] {
            match result {
                Ok(stats) => panic!("Expected {} error, got successful response: {:?}", client_name, stats),
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