use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::TickerClient, 
        rest::BinanceSpotRestClient
    },
    types::{
        responses::{TickerStatistics, TickerPrice, TickerBook},
        requests::{
            Validated,
            Ticker24HrSpec,
            TickerPriceSpec,
            TickerBookSpec,
            TickerRollingWindowSpec,
            TickerTradingDaySpec,
        },
    },
};

#[async_trait]
impl TickerClient for BinanceSpotRestClient {

    async fn ticker_price(&self, specification: TickerPriceSpec<Validated>) -> Result<Vec<TickerPrice>> {
        self.request(reqwest::Method::GET, "/api/v3/ticker/price", specification).await
    }

    async fn ticker_book(&self, specification: TickerBookSpec<Validated>) -> Result<Vec<TickerBook>> {
        self.request(reqwest::Method::GET, "/api/v3/ticker/bookTicker", specification).await
    }

    async fn ticker_24hr(&self, specification: Ticker24HrSpec<Validated>) -> Result<Vec<TickerStatistics>> {
        self.request(reqwest::Method::GET, "/api/v3/ticker/24hr", specification).await
    }

    async fn ticker_trading_day(&self, specification: TickerTradingDaySpec<Validated>) -> Result<Vec<TickerStatistics>> {
        self.request(reqwest::Method::GET, "/api/v3/ticker/tradingDay", specification).await
    }

    async fn ticker_rolling_window(&self, specification: TickerRollingWindowSpec<Validated>) -> Result<Vec<TickerStatistics>> {
        self.request(reqwest::Method::GET, "/api/v3/ticker", specification).await
    }
}