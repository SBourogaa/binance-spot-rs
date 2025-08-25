use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{r#trait::TickerClient, websocket::BinanceSpotWebSocketClient},
    types::{
        requests::{
            Ticker24HrSpec, TickerBookSpec, TickerPriceSpec, TickerRollingWindowSpec,
            TickerTradingDaySpec, Validated,
        },
        responses::{TickerBook, TickerPrice, TickerStatistics},
    },
};

#[async_trait]
impl TickerClient for BinanceSpotWebSocketClient {
    async fn ticker_price(
        &self,
        specification: TickerPriceSpec<Validated>,
    ) -> Result<Vec<TickerPrice>> {
        self.request("ticker.price", specification).await
    }

    async fn ticker_book(
        &self,
        specification: TickerBookSpec<Validated>,
    ) -> Result<Vec<TickerBook>> {
        self.request("ticker.book", specification).await
    }

    async fn ticker_24hr(
        &self,
        specification: Ticker24HrSpec<Validated>,
    ) -> Result<Vec<TickerStatistics>> {
        self.request("ticker.24hr", specification).await
    }

    async fn ticker_trading_day(
        &self,
        specification: TickerTradingDaySpec<Validated>,
    ) -> Result<Vec<TickerStatistics>> {
        self.request("ticker.tradingDay", specification).await
    }

    async fn ticker_rolling_window(
        &self,
        specification: TickerRollingWindowSpec<Validated>,
    ) -> Result<Vec<TickerStatistics>> {
        self.request("ticker", specification).await
    }
}
