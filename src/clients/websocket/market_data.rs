use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::MarketDataClient,
        websocket::BinanceSpotWebSocketClient
    },
    types::{
        responses::{OrderBook, Trade, AggregateTrade, Kline, AveragePrice},
        requests::{
            Validated,
            OrderBookSpec,
            RecentTradesSpec,
            HistoricalTradesSpec,
            AggregateTradesSpec,
            KlinesSpec,
            AveragePriceSpec,
        },
    },
};

#[async_trait]
impl MarketDataClient for BinanceSpotWebSocketClient {
    async fn order_book(&self, specification: OrderBookSpec<Validated>) -> Result<OrderBook> {
        self.request("depth", specification).await
    }

    async fn recent_trades(&self, specification: RecentTradesSpec<Validated>) -> Result<Vec<Trade>> {
        self.request("trades.recent", specification).await
    }

    async fn historical_trades(&self, specification: HistoricalTradesSpec<Validated>) -> Result<Vec<Trade>> {
        self.request("trades.historical", specification).await
    }

    async fn aggregate_trades(&self, specification: AggregateTradesSpec<Validated>) -> Result<Vec<AggregateTrade>> {
        self.request("trades.aggregate", specification).await
    }

    async fn klines(&self, specification: KlinesSpec<Validated>) -> Result<Vec<Kline>> {
        self.request("klines", specification).await
    }

    async fn ui_klines(&self, specification: KlinesSpec<Validated>) -> Result<Vec<Kline>> {
        self.request("uiKlines", specification).await
    }

    async fn average_price(&self, specification: AveragePriceSpec<Validated>) -> Result<AveragePrice> {
        self.request("avgPrice", specification).await
    }
}