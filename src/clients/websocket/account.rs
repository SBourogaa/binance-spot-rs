use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::AccountClient, 
        websocket::BinanceSpotWebSocketClient,
    },
    types::{
        responses::{AccountInfo, SymbolCommissionRates, RateLimit, Order, AccountTrade, PreventedMatch, Allocation},
        requests::{
            Validated,
            QueryOrderSpec,
            AllocationSpec,
            PreventedMatchesSpec,
            CommissionRatesSpec,
            OpenOrdersSpec,
            AllOrdersSpec,
            MyTradesSpec,
        },
    },
};

#[async_trait]
impl AccountClient for BinanceSpotWebSocketClient {
    async fn account_info(&self) -> Result<AccountInfo> {
        self.signed_request("account.status", ()).await
    }

    async fn commission_rates(&self, specification: CommissionRatesSpec<Validated>) -> Result<SymbolCommissionRates> {
        self.signed_request("account.commission", specification).await
    }

    async fn rate_limits(&self) -> Result<Vec<RateLimit>> {
        self.signed_request("account.rateLimits.orders", ()).await
    }

    async fn order_status(&self, specification: QueryOrderSpec<Validated>) -> Result<Order> {
        self.signed_request("order.status", specification).await
    }

    async fn open_orders(&self, specification: OpenOrdersSpec<Validated>) -> Result<Vec<Order>> {
        self.signed_request("openOrders.status", specification).await
    }

    async fn all_orders(&self, specification: AllOrdersSpec<Validated>) -> Result<Vec<Order>> {
        self.signed_request("allOrders", specification).await
    }

    async fn my_trades(&self, specification: MyTradesSpec<Validated>) -> Result<Vec<AccountTrade>> {
        self.signed_request("myTrades", specification).await
    }

    async fn prevented_matches(&self, specification: PreventedMatchesSpec<Validated>) -> Result<Vec<PreventedMatch>> {
        self.signed_request("myPreventedMatches", specification).await
    }

    async fn allocations(&self, specification: AllocationSpec<Validated>) -> Result<Vec<Allocation>> {
        self.signed_request("myAllocations", specification).await
    }
}