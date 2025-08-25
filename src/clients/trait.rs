use async_trait::async_trait;

use crate::Result;
use crate::{
    types::responses::{
        ServerTime, 
        OrderBook, 
        ExchangeInfo, 
        Trade, 
        AggregateTrade,
        Kline,
        AveragePrice,
        TickerStatistics,
        TickerPrice,
        TickerBook,
        AccountInfo,
        Order,
        TestOrder,
        CancelledOrder,
        CancelReplaceOrder,
        AmendedOrder,
        SymbolCommissionRates,
        RateLimit,
        AccountTrade,
        PreventedMatch,
        Allocation,
        OrderList,
    },
    types::requests::{
        Validated,
        OrderSpec,
        AllocationSpec,
        AmendOrderSpec,
        QueryOrderSpec,
        CancelOrderSpec,
        CancelReplaceSpec,
        PreventedMatchesSpec,
        OrderBookSpec,
        Ticker24HrSpec,
        RecentTradesSpec,
        KlinesSpec,
        ExchangeInfoSpec,
        HistoricalTradesSpec,
        AggregateTradesSpec,
        AveragePriceSpec,
        TickerPriceSpec,
        TickerBookSpec,
        TickerRollingWindowSpec,
        TickerTradingDaySpec,
        CommissionRatesSpec,
        OpenOrdersSpec,
        AllOrdersSpec,
        MyTradesSpec,
        CancelAllOrdersSpec,
        OcoOrderSpec,
        OtoOrderSpec,
        OtocoOrderSpec,
        CancelOrderListSpec,
        OrderListStatusSpec,
        AllOrderListsSpec,
        OpenOrderListsSpec,
        SorOrderSpec,
    }
};

/**
 * General client operations for connectivity and exchange metadata.
 */
#[async_trait]
pub trait GeneralClient {
    /**
     * Tests connectivity to the Binance API.
     * 
     * # Returns
     * - `()`: Ok if connection successful, error otherwise.
     */
    async fn ping(&self) -> Result<()>;

    /**
     * Gets the current server time from Binance.
     * 
     * # Returns
     * - `ServerTime`: Server timestamp.
     */
    async fn server_time(&self) -> Result<ServerTime>;

    /**
     * Gets exchange trading rules, rate limits, and symbol information.
     * 
     * # Arguments
     * - `specification`: Exchange info query specification.
     * 
     * # Returns
     * - `ExchangeInfo`: Exchange information.
     */
    async fn exchange_info(&self, specification: ExchangeInfoSpec<Validated>) -> Result<ExchangeInfo>;
}

/**
 * Market data operations for order books, trades, klines, and price data.
 */
#[async_trait]
pub trait MarketDataClient {
    /**
     * Gets current order book for a symbol.
     * 
     * # Arguments
     * - `specification`: Order book query specification.
     * 
     * # Returns
     * - `OrderBook`: Order book with bids and asks.
     */
    async fn order_book(&self, specification: OrderBookSpec<Validated>) -> Result<OrderBook>;

    /**
     * Gets recent trades for a symbol.
     * 
     * # Arguments
     * - `specification`: Recent trades query specification.
     * 
     * # Returns
     * - `Vec<Trade>`: Vector of recent trades.
     */
    async fn recent_trades(&self, specification: RecentTradesSpec<Validated>) -> Result<Vec<Trade>>;

    /**
     * Gets historical trades for a symbol.
     * 
     * # Arguments
     * - `specification`: Historical trades query specification.
     * 
     * # Returns
     * - `Vec<Trade>`: Vector of historical trades.
     */
    async fn historical_trades(&self, specification: HistoricalTradesSpec<Validated>) -> Result<Vec<Trade>>;

    /**
     * Gets compressed/aggregate trades for a symbol.
     * 
     * # Arguments
     * - `specification`: Aggregate trades query specification.
     * 
     * # Returns
     * - `Vec<AggregateTrade>`: Vector of aggregate trades.
     */
    async fn aggregate_trades(&self, specification: AggregateTradesSpec<Validated>) -> Result<Vec<AggregateTrade>>;

    /**
     * Gets kline/candlestick data for a symbol.
     * 
     * # Arguments
     * - `specification`: Klines query specification.
     * 
     * # Returns
     * - `Vec<Kline>`: Array of kline data.
     */
    async fn klines(&self, specification: KlinesSpec<Validated>) -> Result<Vec<Kline>>;

    /**
     * Gets UI-optimized kline/candlestick data for a symbol.
     * 
     * # Arguments
     * - `specification`: Klines query specification.
     * 
     * # Returns
     * - `Vec<Kline>`: Array of UI-optimized kline data.
     */
    async fn ui_klines(&self, specification: KlinesSpec<Validated>) -> Result<Vec<Kline>>;

    /**
     * Gets current average price for a symbol.
     * 
     * # Arguments
     * - `specification`: Average price query specification.
     * 
     * # Returns
     * - `AveragePrice`: Average price information.
     */
    async fn average_price(&self, specification: AveragePriceSpec<Validated>) -> Result<AveragePrice>;
}

/**
 * Ticker operations for current market statistics and price information.
 */
#[async_trait]
pub trait TickerClient {
    /**
     * Gets 24hr ticker price change statistics.
     * 
     * # Arguments
     * - `specification`: 24hr ticker query specification.
     * 
     * # Returns
     * - `Vec<TickerStatistics>`: Vector of ticker statistics.
     */
    async fn ticker_24hr(&self, specification: Ticker24HrSpec<Validated>) -> Result<Vec<TickerStatistics>>;

    /**
     * Gets latest price for symbol(s).
     * 
     * # Arguments
     * - `specification`: Ticker price query specification.
     * 
     * # Returns
     * - `Vec<TickerPrice>`: Vector of ticker prices.
     */
    async fn ticker_price(&self, specification: TickerPriceSpec<Validated>) -> Result<Vec<TickerPrice>>;

    /**
     * Gets best bid/ask prices for symbol(s).
     * 
     * # Arguments
     * - `specification`: Ticker book query specification.
     * 
     * # Returns
     * - `Vec<TickerBook>`: Vector of ticker book prices.
     */
    async fn ticker_book(&self, specification: TickerBookSpec<Validated>) -> Result<Vec<TickerBook>>;

    /**
     * Gets rolling window price change statistics.
     * 
     * # Arguments
     * - `specification`: Rolling window ticker query specification.
     * 
     * # Returns
     * - `Vec<TickerStatistics>`: Vector of rolling window ticker statistics.
     */
    async fn ticker_rolling_window(&self, specification: TickerRollingWindowSpec<Validated>) -> Result<Vec<TickerStatistics>>;

    /**
     * Gets trading day ticker statistics.
     * 
     * # Arguments
     * - `specification`: Trading day ticker query specification.
     * 
     * # Returns
     * - `Vec<TickerStatistics>`: Vector of trading day ticker statistics.
     */
    async fn ticker_trading_day(&self, specification: TickerTradingDaySpec<Validated>) -> Result<Vec<TickerStatistics>>;
}

/**
 * Account and trading-related client operations.
 */
#[async_trait]
pub trait AccountClient {
    /**
     * Gets current account information including balances and permissions.
     * 
     * # Returns
     * - `AccountInfo`: Account information.
     */
    async fn account_info(&self) -> Result<AccountInfo>;

    /**
     * Gets commission rates for a specific trading symbol.
     * 
     * # Arguments
     * - `specification`: Commission rates query specification.
     * 
     * # Returns
     * - `SymbolCommissionRates`: Commission rates for the symbol.
     */
    async fn commission_rates(&self, specification: CommissionRatesSpec<Validated>) -> Result<SymbolCommissionRates>;

    /**
     * Gets current unfilled order count rate limits for the account.
     * 
     * # Returns
     * - `Vec<RateLimit>`: List of rate limits.
     */
    async fn rate_limits(&self) -> Result<Vec<RateLimit>>;

    /**
     * Gets the status of a specific order.
     * 
     * # Arguments
     * - `specification`: Query order specification.
     * 
     * # Returns
     * - `Order`: Order status information.
     */
    async fn order_status(&self, specification: QueryOrderSpec<Validated>) -> Result<Order>;

    /**
     * Gets all open orders for a symbol or all symbols.
     *
     * # Arguments
     * - `specification`: Open orders query specification.
     *
     * # Returns
     * - `Vec<Order>`: List of open orders.
     */
    async fn open_orders(&self, specification: OpenOrdersSpec<Validated>) -> Result<Vec<Order>>;

    /**
     * Gets all account orders (active, canceled, or filled) for a symbol.
     *
     * # Arguments
     * - `specification`: All orders query specification.
     *
     * # Returns
     * - `Vec<Order>`: List of all orders for the symbol.
     */
    async fn all_orders(&self, specification: AllOrdersSpec<Validated>) -> Result<Vec<Order>>;

    /**
     * Gets trade history for a specific account and symbol.
     *
     * # Arguments
     * - `specification`: My trades query specification.
     *
     * # Returns
     * - `Vec<AccountTrade>`: List of trades for the account.
     */
    async fn my_trades(&self, specification: MyTradesSpec<Validated>) -> Result<Vec<AccountTrade>>;

    /**
     * Gets prevented matches for orders expired due to STP.
     *
     * # Arguments
     * - `specification`: Prevented matches query specification.
     *
     * # Returns
     * - `Vec<PreventedMatch>`: List of prevented matches.
     */
    async fn prevented_matches(&self, specification: PreventedMatchesSpec<Validated>) -> Result<Vec<PreventedMatch>>;

    /**
     * Gets account allocations resulting from Smart Order Routing.
     *
     * # Arguments
     * - `specification`: Allocation query specification.
     *
     * # Returns
     * - `Vec<Allocation>`: List of allocations.
     */
    async fn allocations(&self, specification: AllocationSpec<Validated>) -> Result<Vec<Allocation>>;
}

/**
 * Trading operations for order placement, cancellation, and management.
 */
#[async_trait]
pub trait TradingClient {
    /**
     * Places a new order on the exchange.
     * 
     * # Arguments
     * - `specification`: Order specification.
     * 
     * # Returns
     * - `Order`: Order placement result.
     */
    async fn place_order(&self, specification: OrderSpec<Validated>) -> Result<Order>;

    /**
     * Tests order placement without actually executing the order.
     * 
     * # Arguments
     * - `specification`: Order specification.
     * 
     * # Returns
     * - `TestOrder`: Test order result.
     */
    async fn test_order(&self, specification: OrderSpec<Validated>) -> Result<TestOrder>;

    /**
     * Cancels an active order on the exchange.
     * 
     * # Arguments
     * - `specification`: Cancel order specification.
     * 
     * # Returns
     * - `Order`: Cancelled order information.
     */
    async fn cancel_order(&self, specification: CancelOrderSpec<Validated>) -> Result<Order>;

    /**
     * Cancels all active orders on a symbol.
     * 
     * # Arguments
     * - `specification`: Cancel all orders specification.
     * 
     * # Returns
     * - `Vec<CancelledOrder>`: List of cancelled orders.
     */
    async fn cancel_all_orders(&self, specification: CancelAllOrdersSpec<Validated>) -> Result<Vec<CancelledOrder>>;

    /**
     * Cancels an existing order and immediately places a new order.
     *
     * # Arguments
     * - `specification`: Cancel-replace order specification.
     *
     * # Returns
     * - `CancelReplaceOrder`: Cancel-replaced order.
     */
    async fn cancel_replace_order(&self, specification: CancelReplaceSpec<Validated>) -> Result<CancelReplaceOrder>;

    /**
     * Amends an existing order by reducing its quantity while keeping its priority.
     *
     * # Arguments
     * - `specification`: Amendment specification.
     *
     * # Returns
     * - `AmendedOrder`: Amended order.
     */
    async fn amend_order(&self, specification: AmendOrderSpec<Validated>) -> Result<AmendedOrder>;

    /**
     * Places a new OCO (One-Cancels-Other) order.
     * 
     * # Arguments
     * - `specification`: OCO order specification.
     * 
     * # Returns
     * - `OrderList`: OCO order list result.
     */
    async fn place_oco_order(&self, specification: OcoOrderSpec<Validated>) -> Result<OrderList>;

    /**
     * Places a new OTO (One-Triggers-Other) order.
     * 
     * # Arguments
     * - `specification`: OTO order specification.
     * 
     * # Returns
     * - `OrderList`: OTO order list result.
     */
    async fn place_oto_order(&self, specification: OtoOrderSpec<Validated>) -> Result<OrderList>;

    /**
     * Places a new OTOCO (One-Triggers-One-Cancels-Other) order.
     * 
     * # Arguments
     * - `specification`: OTOCO order specification.
     * 
     * # Returns
     * - `OrderList`: OTOCO order list result.
     */
    async fn place_otoco_order(&self, specification: OtocoOrderSpec<Validated>) -> Result<OrderList>;

    /**
     * Cancels an order list (OCO/OTO/OTOCO).
     * 
     * # Arguments
     * - `specification`: Cancel order list specification.
     * 
     * # Returns
     * - `OrderList`: Cancelled order list.
     */
    async fn cancel_order_list(&self, specification: CancelOrderListSpec<Validated>) -> Result<OrderList>;

    /**
     * Gets the status of a specific order list.
     * 
     * # Arguments
     * - `specification`: Order list status query specification.
     * 
     * # Returns
     * - `OrderList`: Order list information.
     */
    async fn order_list_status(&self, specification: OrderListStatusSpec<Validated>) -> Result<OrderList>;

    /**
     * Gets all order lists for the account.
     * 
     * # Arguments
     * - `specification`: All order lists query specification.
     * 
     * # Returns
     * - `Vec<OrderList>`: List of all order lists.
     */
    async fn all_order_lists(&self, specification: AllOrderListsSpec<Validated>) -> Result<Vec<OrderList>>;

    /**
     * Gets all open order lists for the account.
     * 
     * # Returns
     * - `Vec<OrderList>`: List of open order lists.
     */
    async fn open_order_lists(&self, specification: OpenOrderListsSpec<Validated>) -> Result<Vec<OrderList>>;

    /**
     * Places a new SOR (Smart Order Routing) order.
     * 
     * # Arguments
     * - `specification`: SOR order specification.
     * 
     * # Returns
     * - `Order`: SOR order result.
     */
    async fn place_sor_order(&self, specification: SorOrderSpec<Validated>) -> Result<Order>;

    /**
     * Tests SOR order placement without execution.
     * 
     * # Arguments
     * - `specification`: SOR order specification.
     * 
     * # Returns
     * - `TestOrder`: Test SOR order result.
     */
    async fn test_sor_order(&self, specification: SorOrderSpec<Validated>) -> Result<TestOrder>;
}

/**
 * Main client trait that combines all Binance API functionality.
 */
pub trait BinanceSpotClient: GeneralClient + MarketDataClient + TickerClient + AccountClient + TradingClient {}

/**
 * Blanket implementation of BinanceClient for any type that implements all component traits.
 */
impl<T> BinanceSpotClient for T where T: GeneralClient + MarketDataClient + TickerClient + AccountClient + TradingClient {}