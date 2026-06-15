//! Authenticated trading endpoints (`/api/v5/trade/*`).

use serde::{Deserialize, Serialize};

use crate::client::OkxClient;
use crate::error::Error;
use crate::model::{NumberString, OrderSide, OrderState, OrderType, PositionSide, TradeMode};
use crate::transport::Transport;

const ORDER: &str = "/api/v5/trade/order";
const BATCH_ORDERS: &str = "/api/v5/trade/batch-orders";
const CANCEL_ORDER: &str = "/api/v5/trade/cancel-order";
const CANCEL_BATCH_ORDERS: &str = "/api/v5/trade/cancel-batch-orders";
const AMEND_ORDER: &str = "/api/v5/trade/amend-order";
const AMEND_BATCH_ORDERS: &str = "/api/v5/trade/amend-batch-orders";
const CLOSE_POSITION: &str = "/api/v5/trade/close-position";
const ORDERS_PENDING: &str = "/api/v5/trade/orders-pending";
const ORDERS_HISTORY: &str = "/api/v5/trade/orders-history";
const ORDERS_HISTORY_ARCHIVE: &str = "/api/v5/trade/orders-history-archive";
const FILLS: &str = "/api/v5/trade/fills";
const FILLS_HISTORY: &str = "/api/v5/trade/fills-history";

/// Accessor for the authenticated trading endpoints.
///
/// Obtain one via [`OkxClient::trade`](crate::OkxClient::trade). All methods
/// require credentials.
pub struct Trade<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Trade<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Place an order.
    ///
    /// `POST /api/v5/trade/order`. Authenticated. Build the request with
    /// [`PlaceOrderRequest::new`] plus optional setters. The returned vector
    /// contains one [`PlaceOrderResult`]; inspect its
    /// [`s_code`](PlaceOrderResult::s_code) to confirm acceptance (`"0"`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a
    /// non-zero top-level OKX code, or transport/decode errors.
    pub async fn place_order(
        &self,
        request: &PlaceOrderRequest,
    ) -> Result<Vec<PlaceOrderResult>, Error> {
        self.client.post(ORDER, request, true).await
    }

    /// Place multiple orders.
    ///
    /// `POST /api/v5/trade/batch-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn place_multiple_orders(
        &self,
        requests: &[PlaceOrderRequest],
    ) -> Result<Vec<PlaceOrderResult>, Error> {
        self.client.post(BATCH_ORDERS, &requests, true).await
    }

    /// Cancel an order by its OKX order ID.
    ///
    /// `POST /api/v5/trade/cancel-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn cancel_order(
        &self,
        inst_id: &str,
        ord_id: &str,
    ) -> Result<Vec<CancelOrderResult>, Error> {
        let body = CancelOrderBody { inst_id, ord_id };
        self.client.post(CANCEL_ORDER, &body, true).await
    }

    /// Cancel multiple orders.
    ///
    /// `POST /api/v5/trade/cancel-batch-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn cancel_multiple_orders(
        &self,
        requests: &[CancelOrderRequest],
    ) -> Result<Vec<CancelOrderResult>, Error> {
        self.client.post(CANCEL_BATCH_ORDERS, &requests, true).await
    }

    /// Amend an existing order.
    ///
    /// `POST /api/v5/trade/amend-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn amend_order(
        &self,
        request: &AmendOrderRequest,
    ) -> Result<Vec<AmendOrderResult>, Error> {
        self.client.post(AMEND_ORDER, request, true).await
    }

    /// Amend multiple existing orders.
    ///
    /// `POST /api/v5/trade/amend-batch-orders`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn amend_multiple_orders(
        &self,
        requests: &[AmendOrderRequest],
    ) -> Result<Vec<AmendOrderResult>, Error> {
        self.client.post(AMEND_BATCH_ORDERS, &requests, true).await
    }

    /// Close positions for an instrument.
    ///
    /// `POST /api/v5/trade/close-position`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn close_positions(
        &self,
        request: &ClosePositionRequest,
    ) -> Result<Vec<ClosePositionResult>, Error> {
        self.client.post(CLOSE_POSITION, request, true).await
    }

    /// Retrieve the details of a single order by its OKX order ID.
    ///
    /// `GET /api/v5/trade/order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_order(&self, inst_id: &str, ord_id: &str) -> Result<Vec<Order>, Error> {
        let query = GetOrderQuery { inst_id, ord_id };
        self.client.get(ORDER, &query, true).await
    }

    /// Retrieve pending orders.
    ///
    /// `GET /api/v5/trade/orders-pending`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_order_list(&self, request: &OrderListRequest) -> Result<Vec<Order>, Error> {
        self.client.get(ORDERS_PENDING, request, true).await
    }

    /// Retrieve order history for the recent window.
    ///
    /// `GET /api/v5/trade/orders-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_orders_history(
        &self,
        request: &OrderHistoryRequest,
    ) -> Result<Vec<Order>, Error> {
        self.client.get(ORDERS_HISTORY, request, true).await
    }

    /// Retrieve archived order history.
    ///
    /// `GET /api/v5/trade/orders-history-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_orders_history_archive(
        &self,
        request: &OrderHistoryRequest,
    ) -> Result<Vec<Order>, Error> {
        self.client.get(ORDERS_HISTORY_ARCHIVE, request, true).await
    }

    /// Retrieve recent fills.
    ///
    /// `GET /api/v5/trade/fills`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_fills(&self, request: &FillsRequest) -> Result<Vec<Fill>, Error> {
        self.client.get(FILLS, request, true).await
    }

    /// Retrieve historical fills.
    ///
    /// `GET /api/v5/trade/fills-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`place_order`](Self::place_order).
    pub async fn get_fills_history(&self, request: &FillsRequest) -> Result<Vec<Fill>, Error> {
        self.client.get(FILLS_HISTORY, request, true).await
    }
}

/// A request to place an order.
///
/// Construct with [`PlaceOrderRequest::new`] (required fields) and chain setters
/// for optional fields. Optional fields are omitted from the request body when
/// unset.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "tdMode")]
    td_mode: TradeMode,
    side: OrderSide,
    #[serde(rename = "ordType")]
    ord_type: OrderType,
    sz: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(rename = "px", skip_serializing_if = "Option::is_none")]
    px: Option<String>,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    tgt_ccy: Option<String>,
}

impl PlaceOrderRequest {
    /// Create a new order request with the required fields.
    pub fn new(
        inst_id: impl Into<String>,
        td_mode: TradeMode,
        side: OrderSide,
        ord_type: OrderType,
        sz: impl Into<String>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            side,
            ord_type,
            sz: sz.into(),
            ccy: None,
            tag: None,
            px: None,
            pos_side: None,
            cl_ord_id: None,
            reduce_only: None,
            tgt_ccy: None,
        }
    }

    /// Set the order price (required for `limit`-style orders).
    pub fn price(mut self, px: impl Into<String>) -> Self {
        self.px = Some(px.into());
        self
    }

    /// Set the position side (`long`/`short`/`net`).
    pub fn position_side(mut self, pos_side: PositionSide) -> Self {
        self.pos_side = Some(pos_side);
        self
    }

    /// Set a client-supplied order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<String>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Mark the order as reduce-only.
    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Set the quantity unit for spot market orders (`base_ccy`/`quote_ccy`).
    pub fn target_ccy(mut self, tgt_ccy: impl Into<String>) -> Self {
        self.tgt_ccy = Some(tgt_ccy.into());
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set an order tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

#[derive(Serialize)]
struct CancelOrderBody<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
    #[serde(rename = "ordId")]
    ord_id: &'a str,
}

#[derive(Serialize)]
struct GetOrderQuery<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
    #[serde(rename = "ordId")]
    ord_id: &'a str,
}

/// A request to cancel an order.
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
}

impl CancelOrderRequest {
    /// Cancel by OKX order ID.
    pub fn by_order_id(inst_id: impl Into<String>, ord_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: Some(ord_id.into()),
            cl_ord_id: None,
        }
    }

    /// Cancel by client order ID.
    pub fn by_client_order_id(inst_id: impl Into<String>, cl_ord_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: None,
            cl_ord_id: Some(cl_ord_id.into()),
        }
    }
}

/// A request to amend an order.
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
    #[serde(rename = "reqId", skip_serializing_if = "Option::is_none")]
    req_id: Option<String>,
    #[serde(rename = "cxlOnFail", skip_serializing_if = "Option::is_none")]
    cxl_on_fail: Option<bool>,
    #[serde(rename = "newSz", skip_serializing_if = "Option::is_none")]
    new_sz: Option<String>,
    #[serde(rename = "newPx", skip_serializing_if = "Option::is_none")]
    new_px: Option<String>,
}

impl AmendOrderRequest {
    /// Create an amend-order request for an instrument.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: None,
            cl_ord_id: None,
            req_id: None,
            cxl_on_fail: None,
            new_sz: None,
            new_px: None,
        }
    }

    /// Set the OKX order ID.
    pub fn order_id(mut self, ord_id: impl Into<String>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<String>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Set a request ID.
    pub fn request_id(mut self, req_id: impl Into<String>) -> Self {
        self.req_id = Some(req_id.into());
        self
    }

    /// Set whether OKX should cancel the order if amendment fails.
    pub fn cancel_on_fail(mut self, cxl_on_fail: bool) -> Self {
        self.cxl_on_fail = Some(cxl_on_fail);
        self
    }

    /// Set the new order size.
    pub fn new_size(mut self, new_sz: impl Into<String>) -> Self {
        self.new_sz = Some(new_sz.into());
        self
    }

    /// Set the new order price.
    pub fn new_price(mut self, new_px: impl Into<String>) -> Self {
        self.new_px = Some(new_px.into());
        self
    }
}

/// A request to close positions.
#[derive(Debug, Clone, Serialize)]
pub struct ClosePositionRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "autoCxl", skip_serializing_if = "Option::is_none")]
    auto_cancel: Option<bool>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

impl ClosePositionRequest {
    /// Create a close-position request.
    pub fn new(inst_id: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self {
            inst_id: inst_id.into(),
            mgn_mode,
            pos_side: None,
            ccy: None,
            auto_cancel: None,
            cl_ord_id: None,
            tag: None,
        }
    }

    /// Set the position side.
    pub fn position_side(mut self, pos_side: PositionSide) -> Self {
        self.pos_side = Some(pos_side);
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set whether pending orders should be canceled automatically.
    pub fn auto_cancel(mut self, auto_cancel: bool) -> Self {
        self.auto_cancel = Some(auto_cancel);
        self
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<String>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Set an order tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

/// Query parameters for pending order lists.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderListRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<crate::model::InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "ordType", skip_serializing_if = "Option::is_none")]
    ord_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<OrderState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl OrderListRequest {
    /// Create an empty order-list query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: crate::model::InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the order type filter.
    pub fn order_type(mut self, ord_type: OrderType) -> Self {
        self.ord_type = Some(ord_type);
        self
    }

    /// Set the order state filter.
    pub fn state(mut self, state: OrderState) -> Self {
        self.state = Some(state);
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for order history.
#[derive(Debug, Clone, Serialize)]
pub struct OrderHistoryRequest {
    #[serde(flatten)]
    base: OrderListRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
}

impl OrderHistoryRequest {
    /// Create an order-history query with the required instrument type.
    pub fn new(inst_type: crate::model::InstType) -> Self {
        Self {
            base: OrderListRequest::new().inst_type(inst_type),
            begin: None,
            end: None,
        }
    }

    /// Set the common order-list filters.
    pub fn filters(mut self, base: OrderListRequest) -> Self {
        self.base = base;
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, begin: impl Into<String>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, end: impl Into<String>) -> Self {
        self.end = Some(end.into());
        self
    }
}

/// Query parameters for fills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FillsRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<crate::model::InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl FillsRequest {
    /// Create an empty fills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: crate::model::InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the order ID filter.
    pub fn order_id(mut self, ord_id: impl Into<String>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, begin: impl Into<String>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, end: impl Into<String>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// The result of placing an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlaceOrderResult {
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Per-order status code (`"0"` on success).
    pub s_code: String,
    /// Per-order status message.
    #[serde(default)]
    pub s_msg: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// The result of cancelling an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelOrderResult {
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Per-order status code (`"0"` on success).
    pub s_code: String,
    /// Per-order status message.
    #[serde(default)]
    pub s_msg: String,
}

/// The result of amending an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AmendOrderResult {
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Request ID, if supplied.
    #[serde(default)]
    pub req_id: String,
    /// Per-order status code (`"0"` on success).
    pub s_code: String,
    /// Per-order status message.
    #[serde(default)]
    pub s_msg: String,
}

/// The result of closing a position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ClosePositionResult {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Client order ID, if supplied.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Tag, if supplied.
    #[serde(default)]
    pub tag: String,
}

/// Details of an existing order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Order {
    /// Instrument ID.
    pub inst_id: String,
    /// OKX order ID.
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Order type.
    pub ord_type: OrderType,
    /// Order side.
    pub side: OrderSide,
    /// Position side.
    pub pos_side: PositionSide,
    /// Trade mode.
    pub td_mode: TradeMode,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    pub state: OrderState,
    /// Creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
}

/// A trade fill.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Fill {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Fill side.
    pub side: OrderSide,
    /// Order type.
    pub ord_type: OrderType,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee amount.
    #[serde(default)]
    pub fee: NumberString,
    /// Fill timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

#[cfg(test)]
mod tests {
    use crate::model::{OrderSide, OrderType, TradeMode};
    use crate::test_util::MockTransport;
    use crate::{Credentials, OkxClient};

    use super::PlaceOrderRequest;

    fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
        OkxClient::with_transport(mock)
            .credentials(Credentials::new("key", "secret", "pass"))
            .build()
    }

    #[tokio::test]
    async fn place_order_posts_signed_json_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","tag":"","sCode":"0","sMsg":"","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let request = PlaceOrderRequest::new(
            "BTC-USDT",
            TradeMode::Cash,
            OrderSide::Buy,
            OrderType::Limit,
            "0.01",
        )
        .price("42000")
        .client_order_id("b1");

        let result = client.trade().place_order(&request).await.unwrap();
        assert_eq!(result[0].ord_id, "312");
        assert_eq!(result[0].s_code, "0");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/trade/order"));
        assert!(req.is_signed());

        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["instId"], "BTC-USDT");
        assert_eq!(sent["tdMode"], "cash");
        assert_eq!(sent["side"], "buy");
        assert_eq!(sent["ordType"], "limit");
        assert_eq!(sent["sz"], "0.01");
        assert_eq!(sent["px"], "42000");
        assert_eq!(sent["clOrdId"], "b1");
        // Unset optional fields are omitted.
        assert!(sent.get("reduceOnly").is_none());
    }

    #[tokio::test]
    async fn place_multiple_orders_posts_array_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","sCode":"0","sMsg":""},
            {"ordId":"313","clOrdId":"b2","sCode":"0","sMsg":""}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let requests = vec![
            PlaceOrderRequest::new(
                "BTC-USDT",
                TradeMode::Cash,
                OrderSide::Buy,
                OrderType::Limit,
                "0.01",
            )
            .price("42000")
            .client_order_id("b1"),
            PlaceOrderRequest::new(
                "BTC-USDT",
                TradeMode::Cash,
                OrderSide::Sell,
                OrderType::Limit,
                "0.02",
            )
            .price("43000")
            .client_order_id("b2"),
        ];

        let result = client
            .trade()
            .place_multiple_orders(&requests)
            .await
            .unwrap();
        assert_eq!(result[1].ord_id, "313");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/trade/batch-orders"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent[0]["clOrdId"], "b1");
        assert_eq!(sent[1]["side"], "sell");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn cancel_order_posts_ids() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","sCode":"0","sMsg":""}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client
            .trade()
            .cancel_order("BTC-USDT", "312")
            .await
            .unwrap();
        assert_eq!(result[0].ord_id, "312");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["instId"], "BTC-USDT");
        assert_eq!(sent["ordId"], "312");
    }

    #[tokio::test]
    async fn cancel_multiple_orders_posts_array_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","sCode":"0","sMsg":""}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let requests = vec![super::CancelOrderRequest::by_order_id("BTC-USDT", "312")];

        let result = client
            .trade()
            .cancel_multiple_orders(&requests)
            .await
            .unwrap();
        assert_eq!(result[0].s_code, "0");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/trade/cancel-batch-orders"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent[0]["instId"], "BTC-USDT");
        assert_eq!(sent[0]["ordId"], "312");
        assert!(sent[0].get("clOrdId").is_none());
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn amend_order_posts_builder_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","reqId":"r1","sCode":"0","sMsg":""}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::AmendOrderRequest::new("BTC-USDT")
            .order_id("312")
            .request_id("r1")
            .new_price("42100");

        let result = client.trade().amend_order(&request).await.unwrap();
        assert_eq!(result[0].req_id, "r1");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/trade/amend-order"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["instId"], "BTC-USDT");
        assert_eq!(sent["ordId"], "312");
        assert_eq!(sent["newPx"], "42100");
        assert!(sent.get("newSz").is_none());
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn amend_multiple_orders_posts_array_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","reqId":"r1","sCode":"0","sMsg":""}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let requests = vec![
            super::AmendOrderRequest::new("BTC-USDT")
                .client_order_id("b1")
                .new_size("0.03"),
        ];

        let result = client
            .trade()
            .amend_multiple_orders(&requests)
            .await
            .unwrap();
        assert_eq!(result[0].s_code, "0");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/trade/amend-batch-orders"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent[0]["clOrdId"], "b1");
        assert_eq!(sent[0]["newSz"], "0.03");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn close_positions_posts_builder_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","posSide":"long","clOrdId":"close1","tag":"t"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::ClosePositionRequest::new("BTC-USDT-SWAP", TradeMode::Cross)
            .position_side(crate::model::PositionSide::Long)
            .auto_cancel(true)
            .client_order_id("close1");

        let result = client.trade().close_positions(&request).await.unwrap();
        assert_eq!(result[0].cl_ord_id, "close1");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/trade/close-position"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["instId"], "BTC-USDT-SWAP");
        assert_eq!(sent["mgnMode"], "cross");
        assert_eq!(sent["posSide"], "long");
        assert_eq!(sent["autoCxl"], true);
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_order_queries_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"live","cTime":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let orders = client.trade().get_order("BTC-USDT", "312").await.unwrap();
        assert_eq!(orders[0].ord_id, "312");
        assert_eq!(orders[0].state, crate::model::OrderState::Live);
        assert_eq!(orders[0].side, OrderSide::Buy);

        let req = mock.captured();
        assert_eq!(req.method, http::Method::GET);
        assert_eq!(req.query(), Some("instId=BTC-USDT&ordId=312"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_order_list_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"live","cTime":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::OrderListRequest::new()
            .inst_type(crate::model::InstType::Spot)
            .inst_id("BTC-USDT")
            .limit(1);

        let orders = client.trade().get_order_list(&request).await.unwrap();
        assert_eq!(orders[0].ord_id, "312");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&instId=BTC-USDT&limit=1"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_orders_history_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"filled","cTime":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::OrderHistoryRequest::new(crate::model::InstType::Spot)
            .begin("100")
            .end("200");

        let orders = client.trade().get_orders_history(&request).await.unwrap();
        assert_eq!(orders[0].state, crate::model::OrderState::Filled);

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&begin=100&end=200"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_orders_history_archive_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"canceled","cTime":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::OrderHistoryRequest::new(crate::model::InstType::Spot).filters(
            super::OrderListRequest::new()
                .inst_type(crate::model::InstType::Spot)
                .limit(1),
        );

        let orders = client
            .trade()
            .get_orders_history_archive(&request)
            .await
            .unwrap();
        assert_eq!(orders[0].state, crate::model::OrderState::Canceled);

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&limit=1"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_fills_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","tradeId":"t1","ordId":"312",
             "fillPx":"42000","fillSz":"0.01","side":"buy","ordType":"limit",
             "feeCcy":"USDT","fee":"-1","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::FillsRequest::new()
            .inst_type(crate::model::InstType::Spot)
            .order_id("312");

        let fills = client.trade().get_fills(&request).await.unwrap();
        assert_eq!(fills[0].trade_id, "t1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&ordId=312"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_fills_history_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","tradeId":"t1","ordId":"312",
             "fillPx":"42000","fillSz":"0.01","side":"buy","ordType":"limit",
             "feeCcy":"USDT","fee":"-1","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::FillsRequest::new()
            .inst_type(crate::model::InstType::Spot)
            .begin("100")
            .end("200")
            .limit(1);

        let fills = client.trade().get_fills_history(&request).await.unwrap();
        assert_eq!(fills[0].fee.as_str(), "-1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&begin=100&end=200&limit=1"));
        assert!(req.is_signed());
    }
}
