use async_trait::async_trait;

use crate::Result;
use crate::{
    clients::{
        r#trait::TradingClient,
        websocket::BinanceSpotWebSocketClient
    },
    types::{
        responses::{Order, TestOrder, AmendedOrder, CancelledOrder, CancelReplaceOrder},
        requests::{
            Validated,
            OrderSpec,
            AmendOrderSpec,
            CancelOrderSpec,
            CancelReplaceSpec,
            CancelAllOrdersSpec,
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
}