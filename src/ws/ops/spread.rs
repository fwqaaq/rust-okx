//! Spread Trading WebSocket trade operations.

use crate::Error;
use crate::ws::client::{OkxWs, WsChannelGroup};
use crate::ws::conn::WsConnector;
use crate::ws::request::{
    AmendSpreadOrderRequest, CancelSpreadOrderRequest, MassCancelSpreadOrdersRequest,
    PlaceSpreadOrderRequest,
};

impl<C: WsConnector> OkxWs<C> {
    /// Send `sprd-order` to place a spread order over the business WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-place-order>
    pub async fn place_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &PlaceSpreadOrderRequest,
    ) -> Result<(), Error> {
        self.require_channel_group(WsChannelGroup::Business, "sprd-order")?;
        self.require_credentials("sprd-order")?;
        self.send_request_with_expiry(id, "sprd-order", None, std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-amend-order` to amend a spread order over the business WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-amend-order>
    pub async fn amend_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &AmendSpreadOrderRequest,
    ) -> Result<(), Error> {
        self.require_channel_group(WsChannelGroup::Business, "sprd-amend-order")?;
        self.require_credentials("sprd-amend-order")?;
        self.send_request_with_expiry(id, "sprd-amend-order", None, std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-cancel-order` to cancel a spread order over the business WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-order>
    pub async fn cancel_spread_order_ws(
        &mut self,
        id: impl Into<String>,
        request: &CancelSpreadOrderRequest,
    ) -> Result<(), Error> {
        self.require_channel_group(WsChannelGroup::Business, "sprd-cancel-order")?;
        self.require_credentials("sprd-cancel-order")?;
        self.send_request_with_expiry(id, "sprd-cancel-order", None, std::slice::from_ref(request))
            .await
    }

    /// Send `sprd-mass-cancel` to cancel spread orders over the business WebSocket.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-all-orders>
    pub async fn mass_cancel_spread_ws(
        &mut self,
        id: impl Into<String>,
        request: &MassCancelSpreadOrdersRequest,
    ) -> Result<(), Error> {
        self.require_channel_group(WsChannelGroup::Business, "sprd-mass-cancel")?;
        self.require_credentials("sprd-mass-cancel")?;
        self.send_request_with_expiry(id, "sprd-mass-cancel", None, std::slice::from_ref(request))
            .await
    }
}
