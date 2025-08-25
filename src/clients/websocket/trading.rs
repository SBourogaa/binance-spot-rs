use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::TradingClient,
        websocket::BinanceSpotWebSocketClient
    },
    types::{
        responses::{Order, TestOrder, AmendedOrder, CancelledOrder, CancelReplaceOrder, OrderList},
        requests::{
            Validated,
            OrderSpec,
            AmendOrderSpec,
            CancelOrderSpec,
            CancelReplaceSpec,
            CancelAllOrdersSpec,
            OcoOrderSpec,
            OtoOrderSpec,
            OtocoOrderSpec,
            CancelOrderListSpec,
            OrderListStatusSpec,
            AllOrderListsSpec,
            OpenOrderListsSpec,
            SorOrderSpec,
        },
    },
};

#[async_trait]
impl TradingClient for BinanceSpotWebSocketClient {
    async fn place_order(&self, specification: OrderSpec<Validated>) -> Result<Order> {
        self.signed_request("order.place", specification).await
    }

    async fn test_order(&self, specification: OrderSpec<Validated>) -> Result<TestOrder> {
        self.signed_request("order.test", specification).await
    }

    async fn cancel_order(&self, specification: CancelOrderSpec<Validated>) -> Result<Order> {
        self.signed_request("order.cancel", specification).await
    }

    async fn cancel_all_orders(&self, specification: CancelAllOrdersSpec<Validated>) -> Result<Vec<CancelledOrder>> {
        self.signed_request("openOrders.cancelAll", specification).await
    }

    async fn cancel_replace_order(&self, specification: CancelReplaceSpec<Validated>) -> Result<CancelReplaceOrder> {
        self.signed_request("order.cancelReplace", specification).await
    }

    async fn amend_order(&self, specification: AmendOrderSpec<Validated>) -> Result<AmendedOrder> {
        self.signed_request("order.amend.keepPriority", specification).await
    }

    async fn place_oco_order(&self, specification: OcoOrderSpec<Validated>) -> Result<OrderList> {
        self.signed_request("orderList.place.oco", specification).await
    }

    async fn place_oto_order(&self, specification: OtoOrderSpec<Validated>) -> Result<OrderList> {
        self.signed_request("orderList.place.oto", specification).await
    }

    async fn place_otoco_order(&self, specification: OtocoOrderSpec<Validated>) -> Result<OrderList> {
        self.signed_request("orderList.place.otoco", specification).await
    }

    async fn cancel_order_list(&self, specification: CancelOrderListSpec<Validated>) -> Result<OrderList> {
        self.signed_request("orderList.cancel", specification).await
    }

    async fn order_list_status(&self, specification: OrderListStatusSpec<Validated>) -> Result<OrderList> {
        self.signed_request("orderList.status", specification).await
    }

    async fn all_order_lists(&self, specification: AllOrderListsSpec<Validated>) -> Result<Vec<OrderList>> {
        self.signed_request("allOrderLists", specification).await
    }

    async fn open_order_lists(&self, specification: OpenOrderListsSpec<Validated>) -> Result<Vec<OrderList>> {
        self.signed_request("openOrderLists.status", specification).await
    }

    async fn place_sor_order(&self, specification: SorOrderSpec<Validated>) -> Result<Order> {
        self.signed_request("sor.order.place", specification).await
    }

    async fn test_sor_order(&self, specification: SorOrderSpec<Validated>) -> Result<TestOrder> {
        self.signed_request("sor.order.test", specification).await
    }
}