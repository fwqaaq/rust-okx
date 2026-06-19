//! Generic WebSocket operation transport.

use serde::Serialize;

use crate::Error;
use crate::model::ValidateRequest;
use crate::ws::WsError;
use crate::ws::client::OkxWs;
use crate::ws::conn::WsConnector;
use crate::ws::request::OperationRequest;

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

    pub(crate) async fn send_validated_request_with_expiry<A: Serialize + ValidateRequest>(
        &mut self,
        id: impl Into<String>,
        op: &'static str,
        exp_time: Option<String>,
        args: &[A],
    ) -> Result<(), Error> {
        for arg in args {
            arg.validate()?;
        }
        self.send_request_with_expiry(id, op, exp_time, args).await
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
    serde_json::to_string(&payload)
        .map_err(|e| WsError::Encode { source: e })
        .map_err(Error::from)
}
