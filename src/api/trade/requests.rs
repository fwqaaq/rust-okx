use std::borrow::Cow;

use serde::Serialize;

use crate::model::{OrderSide, OrderState, OrderType, PositionSide, TradeMode};

/// Request for [`get_order`](crate::api::trade::Trade::get_order).
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "ordId")]
    ord_id: Cow<'a, str>,
}

impl<'a> GetOrderRequest<'a> {
    /// Create an order query by OKX order ID.
    pub fn new(inst_id: impl Into<Cow<'a, str>>, ord_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: ord_id.into(),
        }
    }
}

mod advanced;
mod algo;

pub use advanced::*;
pub use algo::*;

/// A request to place an order.
///
/// Construct with [`PlaceOrderRequest::new`] (required fields) and chain setters
/// for optional fields. Optional fields are omitted from the request body when
/// unset.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "tdMode")]
    td_mode: TradeMode,
    side: OrderSide,
    #[serde(rename = "ordType")]
    ord_type: OrderType,
    sz: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
    #[serde(rename = "px", skip_serializing_if = "Option::is_none")]
    px: Option<Cow<'a, str>>,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy", skip_serializing_if = "Option::is_none")]
    tgt_ccy: Option<Cow<'a, str>>,
}

impl<'a> PlaceOrderRequest<'a> {
    /// Create a new order request with the required fields.
    pub fn new(
        inst_id: impl Into<Cow<'a, str>>,
        td_mode: TradeMode,
        side: OrderSide,
        ord_type: OrderType,
        sz: impl Into<Cow<'a, str>>,
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
    pub fn price(mut self, px: impl Into<Cow<'a, str>>) -> Self {
        self.px = Some(px.into());
        self
    }

    /// Set the position side (`long`/`short`/`net`).
    pub fn position_side(mut self, pos_side: PositionSide) -> Self {
        self.pos_side = Some(pos_side);
        self
    }

    /// Set a client-supplied order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Mark the order as reduce-only.
    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Set the quantity unit for spot market orders (`base_ccy`/`quote_ccy`).
    pub fn target_ccy(mut self, tgt_ccy: impl Into<Cow<'a, str>>) -> Self {
        self.tgt_ccy = Some(tgt_ccy.into());
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set an order tag.
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

/// A request to cancel an order.
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<Cow<'a, str>>,
}

impl<'a> CancelOrderRequest<'a> {
    /// Cancel by OKX order ID.
    pub fn by_order_id(inst_id: impl Into<Cow<'a, str>>, ord_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: Some(ord_id.into()),
            cl_ord_id: None,
        }
    }

    /// Cancel by client order ID.
    pub fn by_client_order_id(
        inst_id: impl Into<Cow<'a, str>>,
        cl_ord_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            ord_id: None,
            cl_ord_id: Some(cl_ord_id.into()),
        }
    }
}

/// A request to amend an order.
#[derive(Debug, Clone, Serialize)]
pub struct AmendOrderRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<Cow<'a, str>>,
    #[serde(rename = "reqId", skip_serializing_if = "Option::is_none")]
    req_id: Option<Cow<'a, str>>,
    #[serde(rename = "cxlOnFail", skip_serializing_if = "Option::is_none")]
    cxl_on_fail: Option<bool>,
    #[serde(rename = "newSz", skip_serializing_if = "Option::is_none")]
    new_sz: Option<Cow<'a, str>>,
    #[serde(rename = "newPx", skip_serializing_if = "Option::is_none")]
    new_px: Option<Cow<'a, str>>,
}

impl<'a> AmendOrderRequest<'a> {
    /// Create an amend-order request for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
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
    pub fn order_id(mut self, ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Set a request ID.
    pub fn request_id(mut self, req_id: impl Into<Cow<'a, str>>) -> Self {
        self.req_id = Some(req_id.into());
        self
    }

    /// Set whether OKX should cancel the order if amendment fails.
    pub fn cancel_on_fail(mut self, cxl_on_fail: bool) -> Self {
        self.cxl_on_fail = Some(cxl_on_fail);
        self
    }

    /// Set the new order size.
    pub fn new_size(mut self, new_sz: impl Into<Cow<'a, str>>) -> Self {
        self.new_sz = Some(new_sz.into());
        self
    }

    /// Set the new order price.
    pub fn new_price(mut self, new_px: impl Into<Cow<'a, str>>) -> Self {
        self.new_px = Some(new_px.into());
        self
    }
}

/// A request to close positions.
#[derive(Debug, Clone, Serialize)]
pub struct ClosePositionRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(rename = "autoCxl", skip_serializing_if = "Option::is_none")]
    auto_cancel: Option<bool>,
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
}

impl<'a> ClosePositionRequest<'a> {
    /// Create a close-position request.
    pub fn new(inst_id: impl Into<Cow<'a, str>>, mgn_mode: TradeMode) -> Self {
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
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set whether pending orders should be canceled automatically.
    pub fn auto_cancel(mut self, auto_cancel: bool) -> Self {
        self.auto_cancel = Some(auto_cancel);
        self
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Set an order tag.
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

/// Query parameters for pending order lists.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OrderListRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<crate::model::InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "ordType", skip_serializing_if = "Option::is_none")]
    ord_type: Option<OrderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<OrderState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> OrderListRequest<'a> {
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
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
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
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for order history.
#[derive(Debug, Clone, Serialize)]
pub struct OrderHistoryRequest<'a> {
    #[serde(flatten)]
    base: OrderListRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
}

impl<'a> OrderHistoryRequest<'a> {
    /// Create an order-history query with the required instrument type.
    pub fn new(inst_type: crate::model::InstType) -> Self {
        Self {
            base: OrderListRequest::new().inst_type(inst_type),
            begin: None,
            end: None,
        }
    }

    /// Set the common order-list filters.
    pub fn filters(mut self, base: OrderListRequest<'a>) -> Self {
        self.base = base;
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }
}

/// Query parameters for fills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FillsRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<crate::model::InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> FillsRequest<'a> {
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
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the order ID filter.
    pub fn order_id(mut self, ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for historical fills.
///
/// This is intentionally separate from [`FillsRequest`] because OKX documents
/// `instType` as required for `GET /api/v5/trade/fills-history`, while it is
/// optional for `GET /api/v5/trade/fills`.
#[derive(Debug, Clone, Serialize)]
pub struct FillHistoryRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: crate::model::InstType,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> FillHistoryRequest<'a> {
    /// Create a historical fills query with the required instrument type.
    pub fn new(inst_type: crate::model::InstType) -> Self {
        Self {
            inst_type,
            inst_id: None,
            ord_id: None,
            after: None,
            before: None,
            begin: None,
            end: None,
            limit: None,
        }
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the order ID filter.
    pub fn order_id(mut self, ord_id: impl Into<Cow<'a, str>>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
