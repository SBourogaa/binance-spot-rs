use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::GeneralClient, 
        websocket::BinanceSpotWebSocketClient
    },
    types::{
        responses::{
            ServerTime, 
            ExchangeInfo
        },
        requests::{
            Validated,
            ExchangeInfoSpec,
        },
    },
};

#[async_trait]
impl GeneralClient for BinanceSpotWebSocketClient {
    async fn ping(&self) -> Result<()> {
        let _: serde_json::Value = self.request("ping", ()).await?;
        Ok(())
    }


    async fn server_time(&self) -> Result<ServerTime> {
        self.request("time", ()).await
    }

    async fn exchange_info(&self, specification: ExchangeInfoSpec<Validated>) -> Result<ExchangeInfo> {
        self.request("exchangeInfo", specification).await
    }
}