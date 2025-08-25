use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{rest::BinanceSpotRestClient, r#trait::MarketDataClient},
    types::{
        requests::{
            AggregateTradesSpec, AveragePriceSpec, HistoricalTradesSpec, KlinesSpec, OrderBookSpec,
            RecentTradesSpec, Validated,
        },
        responses::{AggregateTrade, AveragePrice, Kline, OrderBook, Trade},
    },
};

#[async_trait]
impl MarketDataClient for BinanceSpotRestClient {
    async fn order_book(&self, specification: OrderBookSpec<Validated>) -> Result<OrderBook> {
        self.request(reqwest::Method::GET, "/api/v3/depth", specification)
            .await
    }

    async fn recent_trades(
        &self,
        specification: RecentTradesSpec<Validated>,
    ) -> Result<Vec<Trade>> {
        self.request(reqwest::Method::GET, "/api/v3/trades", specification)
            .await
    }

    async fn historical_trades(
        &self,
        specification: HistoricalTradesSpec<Validated>,
    ) -> Result<Vec<Trade>> {
        self.request(
            reqwest::Method::GET,
            "/api/v3/historicalTrades",
            specification,
        )
        .await
    }

    async fn aggregate_trades(
        &self,
        specification: AggregateTradesSpec<Validated>,
    ) -> Result<Vec<AggregateTrade>> {
        self.request(reqwest::Method::GET, "/api/v3/aggTrades", specification)
            .await
    }

    async fn klines(&self, specification: KlinesSpec<Validated>) -> Result<Vec<Kline>> {
        self.request(reqwest::Method::GET, "/api/v3/klines", specification)
            .await
    }

    async fn ui_klines(&self, specification: KlinesSpec<Validated>) -> Result<Vec<Kline>> {
        self.request(reqwest::Method::GET, "/api/v3/uiKlines", specification)
            .await
    }

    async fn average_price(
        &self,
        specification: AveragePriceSpec<Validated>,
    ) -> Result<AveragePrice> {
        self.request(reqwest::Method::GET, "/api/v3/avgPrice", specification)
            .await
    }
}
