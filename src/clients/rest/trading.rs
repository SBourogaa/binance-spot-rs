use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{rest::BinanceSpotRestClient, r#trait::TradingClient},
    types::{
        requests::{
            AllOrderListsSpec, AmendOrderSpec, CancelAllOrdersSpec, CancelOrderListSpec,
            CancelOrderSpec, CancelReplaceSpec, OcoOrderSpec, OpenOrderListsSpec,
            OrderListStatusSpec, OrderSpec, OtoOrderSpec, OtocoOrderSpec, SorOrderSpec, Validated,
        },
        responses::{
            AmendedOrder, CancelReplaceOrder, CancelledOrder, Order, OrderList, TestOrder,
        },
    },
};

#[async_trait]
impl TradingClient for BinanceSpotRestClient {
    async fn place_order(&self, specification: OrderSpec<Validated>) -> Result<Order> {
        self.signed_request(reqwest::Method::POST, "/api/v3/order", specification)
            .await
    }

    async fn test_order(&self, specification: OrderSpec<Validated>) -> Result<TestOrder> {
        self.signed_request(reqwest::Method::POST, "/api/v3/order/test", specification)
            .await
    }

    async fn cancel_order(&self, specification: CancelOrderSpec<Validated>) -> Result<Order> {
        self.signed_request(reqwest::Method::DELETE, "/api/v3/order", specification)
            .await
    }

    async fn cancel_all_orders(
        &self,
        specification: CancelAllOrdersSpec<Validated>,
    ) -> Result<Vec<CancelledOrder>> {
        self.signed_request(reqwest::Method::DELETE, "/api/v3/openOrders", specification)
            .await
    }

    async fn cancel_replace_order(
        &self,
        specification: CancelReplaceSpec<Validated>,
    ) -> Result<CancelReplaceOrder> {
        self.signed_request(
            reqwest::Method::POST,
            "/api/v3/order/cancelReplace",
            specification,
        )
        .await
    }

    async fn amend_order(&self, specification: AmendOrderSpec<Validated>) -> Result<AmendedOrder> {
        self.signed_request(
            reqwest::Method::PUT,
            "/api/v3/order/amend/keepPriority",
            specification,
        )
        .await
    }

    async fn place_oco_order(&self, specification: OcoOrderSpec<Validated>) -> Result<OrderList> {
        self.signed_request(
            reqwest::Method::POST,
            "/api/v3/orderList/oco",
            specification,
        )
        .await
    }

    async fn place_oto_order(&self, specification: OtoOrderSpec<Validated>) -> Result<OrderList> {
        self.signed_request(
            reqwest::Method::POST,
            "/api/v3/orderList/oto",
            specification,
        )
        .await
    }

    async fn place_otoco_order(
        &self,
        specification: OtocoOrderSpec<Validated>,
    ) -> Result<OrderList> {
        self.signed_request(
            reqwest::Method::POST,
            "/api/v3/orderList/otoco",
            specification,
        )
        .await
    }

    async fn cancel_order_list(
        &self,
        specification: CancelOrderListSpec<Validated>,
    ) -> Result<OrderList> {
        self.signed_request(reqwest::Method::DELETE, "/api/v3/orderList", specification)
            .await
    }

    async fn order_list_status(
        &self,
        specification: OrderListStatusSpec<Validated>,
    ) -> Result<OrderList> {
        self.signed_request(reqwest::Method::GET, "/api/v3/orderList", specification)
            .await
    }

    async fn all_order_lists(
        &self,
        specification: AllOrderListsSpec<Validated>,
    ) -> Result<Vec<OrderList>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/allOrderList", specification)
            .await
    }

    async fn open_order_lists(
        &self,
        specification: OpenOrderListsSpec<Validated>,
    ) -> Result<Vec<OrderList>> {
        self.signed_request(reqwest::Method::GET, "/api/v3/openOrderList", specification)
            .await
    }

    async fn place_sor_order(&self, specification: SorOrderSpec<Validated>) -> Result<Order> {
        self.signed_request(reqwest::Method::POST, "/api/v3/sor/order", specification)
            .await
    }

    async fn test_sor_order(&self, specification: SorOrderSpec<Validated>) -> Result<TestOrder> {
        self.signed_request(
            reqwest::Method::POST,
            "/api/v3/sor/order/test",
            specification,
        )
        .await
    }
}
