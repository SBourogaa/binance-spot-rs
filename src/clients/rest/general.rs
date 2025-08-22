use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::GeneralClient, 
        rest::BinanceSpotRestClient
    },
    types::{
        responses::{ServerTime, ExchangeInfo},
        requests::{ExchangeInfoSpec, Validated},
    },
};

#[async_trait]
impl GeneralClient for BinanceSpotRestClient {
    async fn ping(&self) -> Result<()> {
        let _: serde_json::Value = self.request(reqwest::Method::GET, "/api/v3/ping", ()).await?;
        Ok(())
    }

    async fn server_time(&self) -> Result<ServerTime> {
        self.request(reqwest::Method::GET, "/api/v3/time", ()).await
    }

    async fn exchange_info(&self, specification: ExchangeInfoSpec<Validated>) -> Result<ExchangeInfo> {
        self.request(reqwest::Method::GET, "/api/v3/exchangeInfo", specification).await
    }
}