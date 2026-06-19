//! Standard Trade WebSocket operations.

use crate::Error;
use crate::api::trade::{AmendOrderRequest, CancelOrderRequest, PlaceOrderRequest};
use crate::ws::client::OkxWs;
use crate::ws::conn::WsConnector;
use crate::ws::request::MassCancelRequest;

impl<C: WsConnector> OkxWs<C> {
    /// Send `order` to place one order over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-place-order>
    pub async fn place_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &PlaceOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "order", std::slice::from_ref(request))
            .await
    }

    /// Send `batch-orders` to place multiple orders over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-place-multiple-orders>
    pub async fn place_orders_ws(
        &mut self,
        id: impl Into<String>,
        requests: &[PlaceOrderRequest],
    ) -> Result<(), Error> {
        self.send_request(id, "batch-orders", requests).await
    }

    /// Send `cancel-order` to cancel one order over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-cancel-order>
    pub async fn cancel_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &CancelOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "cancel-order", std::slice::from_ref(request))
            .await
    }

    /// Send `batch-cancel-orders` to cancel multiple orders over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-cancel-multiple-orders>
    pub async fn cancel_orders_ws(
        &mut self,
        id: impl Into<String>,
        requests: &[CancelOrderRequest],
    ) -> Result<(), Error> {
        self.send_request(id, "batch-cancel-orders", requests).await
    }

    /// Send `amend-order` to amend one order over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-amend-order>
    pub async fn amend_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &AmendOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "amend-order", std::slice::from_ref(request))
            .await
    }

    /// Send `batch-amend-orders` to amend multiple orders over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-amend-multiple-orders>
    pub async fn amend_orders_ws(
        &mut self,
        id: impl Into<String>,
        requests: &[AmendOrderRequest],
    ) -> Result<(), Error> {
        self.send_request(id, "batch-amend-orders", requests).await
    }

    /// Send `mass-cancel` to cancel market-maker-protection orders.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-mass-cancel-order>
    pub async fn mass_cancel_ws(
        &mut self,
        id: impl Into<String>,
        request: &MassCancelRequest,
    ) -> Result<(), Error> {
        self.send_request_with_expiry(id, "mass-cancel", None, std::slice::from_ref(request))
            .await
    }
}
