//! WebSocket operation helpers.

use serde::Serialize;

use crate::Error;
use crate::api::trade::{AmendOrderRequest, CancelOrderRequest, PlaceOrderRequest};
use crate::model::RequestParams;

use super::client::OkxWs;
use super::conn::WsConnector;

impl<C: WsConnector> OkxWs<C> {
    /// Send a raw WebSocket operation request.
    ///
    /// The response is returned asynchronously through
    /// [`WsEvent::Operation`](crate::ws::WsEvent::Operation).
    pub async fn send_request<A: Serialize>(
        &mut self,
        id: impl Into<String>,
        op: impl Into<String>,
        args: &[A],
    ) -> Result<(), Error> {
        let payload = operation_payload(id, op, args)?;
        self.send_operation_payload(payload).await
    }

    /// Send `order` to place one order over WebSocket.
    pub async fn place_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &PlaceOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "order", std::slice::from_ref(request))
            .await
    }

    /// Send `batch-orders` to place multiple orders over WebSocket.
    pub async fn place_orders_ws(
        &mut self,
        id: impl Into<String>,
        requests: &[PlaceOrderRequest],
    ) -> Result<(), Error> {
        self.send_request(id, "batch-orders", requests).await
    }

    /// Send `cancel-order` to cancel one order over WebSocket.
    pub async fn cancel_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &CancelOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "cancel-order", std::slice::from_ref(request))
            .await
    }

    /// Send `batch-cancel-orders` to cancel multiple orders over WebSocket.
    pub async fn cancel_orders_ws(
        &mut self,
        id: impl Into<String>,
        requests: &[CancelOrderRequest],
    ) -> Result<(), Error> {
        self.send_request(id, "batch-cancel-orders", requests).await
    }

    /// Send `amend-order` to amend one order over WebSocket.
    pub async fn amend_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &AmendOrderRequest,
    ) -> Result<(), Error> {
        self.send_request(id, "amend-order", std::slice::from_ref(request))
            .await
    }

    /// Send `batch-amend-orders` to amend multiple orders over WebSocket.
    pub async fn amend_orders_ws(
        &mut self,
        id: impl Into<String>,
        requests: &[AmendOrderRequest],
    ) -> Result<(), Error> {
        self.send_request(id, "batch-amend-orders", requests).await
    }

    /// Send `mass-cancel` to cancel market-maker-protection orders.
    pub async fn mass_cancel_ws(
        &mut self,
        id: impl Into<String>,
        request: &RequestParams,
    ) -> Result<(), Error> {
        self.send_request(id, "mass-cancel", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-order` to place a spread order over WebSocket.
    pub async fn place_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &RequestParams,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-order", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-amend-order` to amend a spread order over WebSocket.
    pub async fn amend_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &RequestParams,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-amend-order", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-cancel-order` to cancel a spread order over WebSocket.
    pub async fn cancel_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &RequestParams,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-cancel-order", std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-mass-cancel` to cancel spread orders over WebSocket.
    pub async fn mass_cancel_spread_ws(
        &mut self,
        id: impl Into<String>,
        request: &RequestParams,
    ) -> Result<(), Error> {
        self.send_request(id, "sprd-mass-cancel", std::slice::from_ref(request))
            .await
    }
}

#[derive(Serialize)]
struct OperationRequest<'a, A> {
    id: String,
    op: String,
    args: &'a [A],
}

pub(crate) fn operation_payload<A: Serialize>(
    id: impl Into<String>,
    op: impl Into<String>,
    args: &[A],
) -> Result<String, Error> {
    let payload = OperationRequest {
        id: id.into(),
        op: op.into(),
        args,
    };
    serde_json::to_string(&payload).map_err(Error::encode)
}
