use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{rest::BinanceSpotRestClient, r#trait::AccountClient},
    types::{
        requests::{
            AllOrdersSpec, AllocationSpec, CommissionRatesSpec, MyTradesSpec, OpenOrdersSpec,
            PreventedMatchesSpec, QueryOrderSpec, Validated,
        },
        responses::{
            AccountInfo, AccountTrade, Allocation, Order, PreventedMatch, RateLimit,
            SymbolCommissionRates,
        },
    },
};

#[async_trait]
impl AccountClient for BinanceSpotRestClient {
    async fn account_info(&self) -> Result<AccountInfo> {
        self.signed_request(reqwest::Method::GET, "/api/v3/account", ())
            .await
    }

    async fn commission_rates(
        &self,
        specification: CommissionRatesSpec<Validated>,
    ) -> Result<SymbolCommissionRates> {
        self.signed_request(
            reqwest::Method::GET,
            "/api/v3/account/commission",
            specification,
        )
        .await
    }

    async fn rate_limits(&self) -> Result<Vec<RateLimit>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/rateLimit/order", ())
            .await
    }

    async fn order_status(&self, specification: QueryOrderSpec<Validated>) -> Result<Order> {
        self.signed_request(reqwest::Method::GET, "/api/v3/order", specification)
            .await
    }

    async fn open_orders(&self, specification: OpenOrdersSpec<Validated>) -> Result<Vec<Order>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/openOrders", specification)
            .await
    }

    async fn all_orders(&self, specification: AllOrdersSpec<Validated>) -> Result<Vec<Order>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/allOrders", specification)
            .await
    }

    async fn my_trades(&self, specification: MyTradesSpec<Validated>) -> Result<Vec<AccountTrade>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/myTrades", specification)
            .await
    }

    async fn prevented_matches(
        &self,
        specification: PreventedMatchesSpec<Validated>,
    ) -> Result<Vec<PreventedMatch>> {
        self.signed_request(
            reqwest::Method::GET,
            "/api/v3/myPreventedMatches",
            specification,
        )
        .await
    }

    async fn allocations(
        &self,
        specification: AllocationSpec<Validated>,
    ) -> Result<Vec<Allocation>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/myAllocations", specification)
            .await
    }
}
