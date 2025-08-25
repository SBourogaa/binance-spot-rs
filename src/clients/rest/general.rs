use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{rest::BinanceSpotRestClient, r#trait::GeneralClient},
    types::{
        requests::{ExchangeInfoSpec, Validated},
        responses::{ExchangeInfo, ServerTime},
    },
};

#[async_trait]
impl GeneralClient for BinanceSpotRestClient {
    async fn ping(&self) -> Result<()> {
        let _: serde_json::Value = self
            .request(reqwest::Method::GET, "/api/v3/ping", ())
            .await?;
        Ok(())
    }

    async fn server_time(&self) -> Result<ServerTime> {
        self.request(reqwest::Method::GET, "/api/v3/time", ()).await
    }

    async fn exchange_info(
        &self,
        specification: ExchangeInfoSpec<Validated>,
    ) -> Result<ExchangeInfo> {
        self.request(reqwest::Method::GET, "/api/v3/exchangeInfo", specification)
            .await
    }
}
