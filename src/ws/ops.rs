//! WebSocket operation helpers.

use serde::Serialize;

use crate::Error;
use crate::api::trade::{AmendOrderRequest, CancelOrderRequest, PlaceOrderRequest};

use super::client::OkxWs;
use super::conn::WsConnector;
use super::request::{
    AmendSpreadOrderRequest, CancelSpreadOrderRequest, MassCancelRequest,
    MassCancelSpreadOrdersRequest, OperationRequest, PlaceSpreadOrderRequest,
};

impl<C: WsConnector> OkxWs<C> {
    /// Send a raw WebSocket operation request.
    ///
    /// The response is returned asynchronously through
    /// [`WsEvent::Operation`](crate::ws::WsEvent::Operation).
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
    pub async fn send_request<A: Serialize>(
        &mut self,
        id: impl Into<String>,
        op: impl Into<String>,
        args: &[A],
    ) -> Result<(), Error> {
        self.send_request_with_expiry(id, op, None, args).await
    }

    /// Send a raw WebSocket operation request with an optional effective deadline.
    ///
    /// `exp_time` is a Unix timestamp in milliseconds. OKX discards the request
    /// when it reaches the matching trading engine after this deadline.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
    pub async fn send_request_with_expiry<A: Serialize>(
        &mut self,
        id: impl Into<String>,
        op: impl Into<String>,
        exp_time: Option<String>,
        args: &[A],
    ) -> Result<(), Error> {
        let payload = operation_payload_with_expiry(id, op, exp_time, args)?;
        self.send_operation_payload(payload).await
    }

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
        self.send_request(id, "mass-cancel", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-order` to place a spread order over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-place-order>
    pub async fn place_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &PlaceSpreadOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-order", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-amend-order` to amend a spread order over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-amend-order>
    pub async fn amend_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &AmendSpreadOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-amend-order", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-cancel-order` to cancel a spread order over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-order>
    pub async fn cancel_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &CancelSpreadOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-cancel-order", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-mass-cancel` to cancel spread orders over WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-all-orders>
    pub async fn mass_cancel_spread_ws(
        &mut self,
        id: impl Into<String>,
        request: &MassCancelSpreadOrdersRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-mass-cancel", std::slice::from_ref(request))
            .await
    }
}

pub(crate) fn operation_payload_with_expiry<A: Serialize>(
    id: impl Into<String>,
    op: impl Into<String>,
    exp_time: Option<String>,
    args: &[A],
) -> Result<String, Error> {
    let mut payload = OperationRequest::new(id, op, args);
    if let Some(exp_time) = exp_time {
        payload = payload.exp_time(exp_time);
    }
    serde_json::to_string(&payload).map_err(Error::encode)
}
