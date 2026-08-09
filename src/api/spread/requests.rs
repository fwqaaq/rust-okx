use serde::Serialize;

/// Filters for the public spread catalog.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl SpreadsRequest {
    /// Create an unfiltered spread query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by base currency.
    pub fn base_currency(mut self, value: impl Into<String>) -> Self {
        self.base_ccy = Some(value.into());
        self
    }

    /// Filter by an included instrument ID.
    pub fn instrument_id(mut self, value: impl Into<String>) -> Self {
        self.inst_id = Some(value.into());
        self
    }

    /// Filter by spread ID.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }

    /// Filter by spread state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
}

/// Query for one public spread order book.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadBooksRequest {
    sprd_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sz: Option<String>,
}

impl SpreadBooksRequest {
    /// Select a spread.
    pub fn new(sprd_id: impl Into<String>) -> Self {
        Self {
            sprd_id: sprd_id.into(),
            sz: None,
        }
    }

    /// Set the depth per side.
    pub fn depth(mut self, value: impl Into<String>) -> Self {
        self.sz = Some(value.into());
        self
    }
}

/// Optional spread filter for public spread trades.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadPublicTradesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
}

impl SpreadPublicTradesRequest {
    /// Create an unfiltered public-trade query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by spread ID.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }
}

/// Request to place a Nitro Spread order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrderRequest {
    sprd_id: String,
    side: String,
    ord_type: String,
    sz: String,
    px: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

impl SpreadOrderRequest {
    /// Set the spread, side, order type, size, and price.
    pub fn new(
        sprd_id: impl Into<String>,
        side: impl Into<String>,
        ord_type: impl Into<String>,
        sz: impl Into<String>,
        px: impl Into<String>,
    ) -> Self {
        Self {
            sprd_id: sprd_id.into(),
            side: side.into(),
            ord_type: ord_type.into(),
            sz: sz.into(),
            px: px.into(),
            cl_ord_id: None,
            tag: None,
        }
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, value: impl Into<String>) -> Self {
        self.cl_ord_id = Some(value.into());
        self
    }

    /// Set the order tag.
    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }
}

/// Identifier for a spread order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrderIdRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
}

impl SpreadOrderIdRequest {
    /// Select an order by system ID.
    pub fn by_order_id(value: impl Into<String>) -> Self {
        Self {
            ord_id: Some(value.into()),
            cl_ord_id: None,
        }
    }

    /// Select an order by client ID.
    pub fn by_client_order_id(value: impl Into<String>) -> Self {
        Self {
            ord_id: None,
            cl_ord_id: Some(value.into()),
        }
    }
}

/// Optional spread filter for mass cancellation.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadMassCancelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
}

impl SpreadMassCancelRequest {
    /// Select all pending spread orders.
    pub fn all() -> Self {
        Self::default()
    }

    /// Limit cancellation to one spread.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }
}

/// Request to amend one spread order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadAmendOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_sz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_px: Option<String>,
}

impl SpreadAmendOrderRequest {
    /// Select an order by system ID and set a new size.
    pub fn new_size_by_order_id(ord_id: impl Into<String>, new_sz: impl Into<String>) -> Self {
        Self {
            ord_id: Some(ord_id.into()),
            new_sz: Some(new_sz.into()),
            ..Self::default()
        }
    }

    /// Select an order by system ID and set a new price.
    pub fn new_price_by_order_id(ord_id: impl Into<String>, new_px: impl Into<String>) -> Self {
        Self {
            ord_id: Some(ord_id.into()),
            new_px: Some(new_px.into()),
            ..Self::default()
        }
    }

    /// Select an order by client ID and set a new size.
    pub fn new_size_by_client_order_id(
        cl_ord_id: impl Into<String>,
        new_sz: impl Into<String>,
    ) -> Self {
        Self {
            cl_ord_id: Some(cl_ord_id.into()),
            new_sz: Some(new_sz.into()),
            ..Self::default()
        }
    }

    /// Select an order by client ID and set a new price.
    pub fn new_price_by_client_order_id(
        cl_ord_id: impl Into<String>,
        new_px: impl Into<String>,
    ) -> Self {
        Self {
            cl_ord_id: Some(cl_ord_id.into()),
            new_px: Some(new_px.into()),
            ..Self::default()
        }
    }

    /// Set the client amendment request ID.
    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.req_id = Some(value.into());
        self
    }

    /// Also set a new size.
    pub fn new_size(mut self, value: impl Into<String>) -> Self {
        self.new_sz = Some(value.into());
        self
    }

    /// Also set a new price.
    pub fn new_price(mut self, value: impl Into<String>) -> Self {
        self.new_px = Some(value.into());
        self
    }
}

/// Filters for active spread orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadPendingOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl SpreadPendingOrdersRequest {
    /// Create an unfiltered active-order query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by spread ID.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }

    /// Filter by order type.
    pub fn order_type(mut self, value: impl Into<String>) -> Self {
        self.ord_type = Some(value.into());
        self
    }

    /// Filter by order state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Return records newer than this order ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this order ID.
    pub fn end_id(mut self, value: impl Into<String>) -> Self {
        self.end_id = Some(value.into());
        self
    }

    /// Set the page size.
    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }
}

/// Filters for spread orders from the last 21 days.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrdersHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl SpreadOrdersHistoryRequest {
    /// Create an unfiltered recent-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by spread ID.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }

    /// Filter by order type.
    pub fn order_type(mut self, value: impl Into<String>) -> Self {
        self.ord_type = Some(value.into());
        self
    }

    /// Filter by order state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Return records newer than this order ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this order ID.
    pub fn end_id(mut self, value: impl Into<String>) -> Self {
        self.end_id = Some(value.into());
        self
    }

    /// Filter from this timestamp.
    pub fn begin_timestamp(mut self, value: impl Into<String>) -> Self {
        self.begin = Some(value.into());
        self
    }

    /// Filter through this timestamp.
    pub fn end_timestamp(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    /// Set the page size.
    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }
}

/// Filters for archived spread orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrdersArchiveRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl SpreadOrdersArchiveRequest {
    /// Create an unfiltered archive query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by spread ID.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }

    /// Filter by order type.
    pub fn order_type(mut self, value: impl Into<String>) -> Self {
        self.ord_type = Some(value.into());
        self
    }

    /// Filter by order state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Filter by instrument type.
    pub fn instrument_type(mut self, value: impl Into<String>) -> Self {
        self.inst_type = Some(value.into());
        self
    }

    /// Filter by instrument family.
    pub fn instrument_family(mut self, value: impl Into<String>) -> Self {
        self.inst_family = Some(value.into());
        self
    }

    /// Return records newer than this order ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this order ID.
    pub fn end_id(mut self, value: impl Into<String>) -> Self {
        self.end_id = Some(value.into());
        self
    }

    /// Filter from this timestamp.
    pub fn begin_timestamp(mut self, value: impl Into<String>) -> Self {
        self.begin = Some(value.into());
        self
    }

    /// Filter through this timestamp.
    pub fn end_timestamp(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    /// Set the page size.
    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }
}

/// Filters for private spread trades from the last seven days.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadTradesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sprd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trade_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl SpreadTradesRequest {
    /// Create an unfiltered private-trade query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by spread ID.
    pub fn spread_id(mut self, value: impl Into<String>) -> Self {
        self.sprd_id = Some(value.into());
        self
    }

    /// Filter by trade ID.
    pub fn trade_id(mut self, value: impl Into<String>) -> Self {
        self.trade_id = Some(value.into());
        self
    }

    /// Filter by order ID.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }

    /// Return records newer than this trade ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this trade ID.
    pub fn end_id(mut self, value: impl Into<String>) -> Self {
        self.end_id = Some(value.into());
        self
    }

    /// Filter from this timestamp.
    pub fn begin_timestamp(mut self, value: impl Into<String>) -> Self {
        self.begin = Some(value.into());
        self
    }

    /// Filter through this timestamp.
    pub fn end_timestamp(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    /// Set the page size.
    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }
}

/// Request configuring Nitro Spread cancel-all-after protection.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadCancelAllAfterRequest {
    time_out: String,
}

impl SpreadCancelAllAfterRequest {
    /// Set the timeout in seconds; zero disables the protection.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            time_out: value.into(),
        }
    }
}
