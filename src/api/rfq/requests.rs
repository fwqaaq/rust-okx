use serde::Serialize;

/// One leg of a new block-trading RFQ.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqLegRequest {
    inst_id: String,
    sz: String,
    side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    td_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lmt_px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tgt_ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl RfqLegRequest {
    /// Set the instrument, size, and direction.
    pub fn new(inst_id: impl Into<String>, sz: impl Into<String>, side: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            sz: sz.into(),
            side: side.into(),
            td_mode: None,
            ccy: None,
            lmt_px: None,
            pos_side: None,
            tgt_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Set the trade mode.
    pub fn trade_mode(mut self, td_mode: impl Into<String>) -> Self {
        self.td_mode = Some(td_mode.into());
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the taker's auto-execution limit price.
    pub fn limit_price(mut self, lmt_px: impl Into<String>) -> Self {
        self.lmt_px = Some(lmt_px.into());
        self
    }

    /// Set the position side.
    pub fn position_side(mut self, pos_side: impl Into<String>) -> Self {
        self.pos_side = Some(pos_side.into());
        self
    }

    /// Set the spot size currency unit.
    pub fn target_currency(mut self, tgt_ccy: impl Into<String>) -> Self {
        self.tgt_ccy = Some(tgt_ccy.into());
        self
    }

    /// Set the spot trading quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// One allocated RFQ leg for an account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqAllocationLegRequest {
    inst_id: String,
    sz: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    td_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos_side: Option<String>,
}

impl RfqAllocationLegRequest {
    /// Set the instrument and allocated size.
    pub fn new(inst_id: impl Into<String>, sz: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            sz: sz.into(),
            td_mode: None,
            ccy: None,
            pos_side: None,
        }
    }

    /// Set the trade mode.
    pub fn trade_mode(mut self, td_mode: impl Into<String>) -> Self {
        self.td_mode = Some(td_mode.into());
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the position side.
    pub fn position_side(mut self, pos_side: impl Into<String>) -> Self {
        self.pos_side = Some(pos_side.into());
        self
    }
}

/// RFQ allocation for one account.
#[derive(Debug, Clone, Serialize)]
pub struct RfqAccountAllocationRequest {
    acct: String,
    legs: Vec<RfqAllocationLegRequest>,
}

impl RfqAccountAllocationRequest {
    /// Set the account name and its allocated legs.
    pub fn new(acct: impl Into<String>, legs: Vec<RfqAllocationLegRequest>) -> Self {
        Self {
            acct: acct.into(),
            legs,
        }
    }
}

/// Request to create a block-trading RFQ.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRfqRequest {
    counterparties: Vec<String>,
    legs: Vec<RfqLegRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_partial_execution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acct_alloc: Option<Vec<RfqAccountAllocationRequest>>,
}

impl CreateRfqRequest {
    /// Set the counterparties and RFQ legs.
    pub fn new(counterparties: Vec<String>, legs: Vec<RfqLegRequest>) -> Self {
        Self {
            counterparties,
            legs,
            anonymous: None,
            cl_rfq_id: None,
            tag: None,
            allow_partial_execution: None,
            acct_alloc: None,
        }
    }

    /// Set whether the taker is anonymous.
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    /// Set the client RFQ ID.
    pub fn client_rfq_id(mut self, cl_rfq_id: impl Into<String>) -> Self {
        self.cl_rfq_id = Some(cl_rfq_id.into());
        self
    }

    /// Set the RFQ tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Set whether partial execution is allowed.
    pub fn allow_partial_execution(mut self, allow: bool) -> Self {
        self.allow_partial_execution = Some(allow);
        self
    }

    /// Set account-level allocations for a group RFQ.
    pub fn account_allocations(mut self, allocations: Vec<RfqAccountAllocationRequest>) -> Self {
        self.acct_alloc = Some(allocations);
        self
    }
}

/// Request to cancel one RFQ.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRfqRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_rfq_id: Option<String>,
}

impl CancelRfqRequest {
    /// Select an RFQ by system ID.
    pub fn by_rfq_id(rfq_id: impl Into<String>) -> Self {
        Self {
            rfq_id: Some(rfq_id.into()),
            cl_rfq_id: None,
        }
    }

    /// Select an RFQ by client ID.
    pub fn by_client_rfq_id(cl_rfq_id: impl Into<String>) -> Self {
        Self {
            rfq_id: None,
            cl_rfq_id: Some(cl_rfq_id.into()),
        }
    }
}

/// Request to cancel several RFQs.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchRfqsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    rfq_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_rfq_ids: Option<Vec<String>>,
}

impl CancelBatchRfqsRequest {
    /// Select RFQs by system IDs.
    pub fn by_rfq_ids(rfq_ids: Vec<String>) -> Self {
        Self {
            rfq_ids: Some(rfq_ids),
            cl_rfq_ids: None,
        }
    }

    /// Select RFQs by client IDs.
    pub fn by_client_rfq_ids(cl_rfq_ids: Vec<String>) -> Self {
        Self {
            rfq_ids: None,
            cl_rfq_ids: Some(cl_rfq_ids),
        }
    }
}

/// One leg size in a partial quote execution.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteLegRequest {
    inst_id: String,
    sz: String,
}

impl ExecuteQuoteLegRequest {
    /// Set the instrument and execution size.
    pub fn new(inst_id: impl Into<String>, sz: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            sz: sz.into(),
        }
    }
}

/// Request to execute a block-trading quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteQuoteRequest {
    rfq_id: String,
    quote_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    legs: Option<Vec<ExecuteQuoteLegRequest>>,
}

impl ExecuteQuoteRequest {
    /// Select an RFQ and quote.
    pub fn new(rfq_id: impl Into<String>, quote_id: impl Into<String>) -> Self {
        Self {
            rfq_id: rfq_id.into(),
            quote_id: quote_id.into(),
            legs: None,
        }
    }

    /// Set partial-execution leg sizes.
    pub fn legs(mut self, legs: Vec<ExecuteQuoteLegRequest>) -> Self {
        self.legs = Some(legs);
        self
    }
}

/// One leg of a maker quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteLegRequest {
    inst_id: String,
    sz: String,
    px: String,
    side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    td_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tgt_ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl QuoteLegRequest {
    /// Set the instrument, size, price, and leg direction.
    pub fn new(
        inst_id: impl Into<String>,
        sz: impl Into<String>,
        px: impl Into<String>,
        side: impl Into<String>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            sz: sz.into(),
            px: px.into(),
            side: side.into(),
            td_mode: None,
            ccy: None,
            pos_side: None,
            tgt_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Set the trade mode.
    pub fn trade_mode(mut self, td_mode: impl Into<String>) -> Self {
        self.td_mode = Some(td_mode.into());
        self
    }

    /// Set the margin currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the position side.
    pub fn position_side(mut self, pos_side: impl Into<String>) -> Self {
        self.pos_side = Some(pos_side.into());
        self
    }

    /// Set the spot size currency unit.
    pub fn target_currency(mut self, tgt_ccy: impl Into<String>) -> Self {
        self.tgt_ccy = Some(tgt_ccy.into());
        self
    }

    /// Set the spot trading quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// Request to create a maker quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuoteRequest {
    rfq_id: String,
    quote_side: String,
    legs: Vec<QuoteLegRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<String>,
}

impl CreateQuoteRequest {
    /// Set the RFQ, top-level quote direction, and quote legs.
    pub fn new(
        rfq_id: impl Into<String>,
        quote_side: impl Into<String>,
        legs: Vec<QuoteLegRequest>,
    ) -> Self {
        Self {
            rfq_id: rfq_id.into(),
            quote_side: quote_side.into(),
            legs,
            cl_quote_id: None,
            tag: None,
            anonymous: None,
            expires_in: None,
        }
    }

    /// Set the client quote ID.
    pub fn client_quote_id(mut self, cl_quote_id: impl Into<String>) -> Self {
        self.cl_quote_id = Some(cl_quote_id.into());
        self
    }

    /// Set the quote tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Set whether the maker is anonymous.
    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    /// Set the number of seconds before the quote expires.
    pub fn expires_in(mut self, expires_in: impl Into<String>) -> Self {
        self.expires_in = Some(expires_in.into());
        self
    }
}

/// Request to cancel one quote.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQuoteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rfq_id: Option<String>,
}

impl CancelQuoteRequest {
    /// Select a quote by system ID.
    pub fn by_quote_id(quote_id: impl Into<String>) -> Self {
        Self {
            quote_id: Some(quote_id.into()),
            cl_quote_id: None,
            rfq_id: None,
        }
    }

    /// Select a quote by client ID.
    pub fn by_client_quote_id(cl_quote_id: impl Into<String>) -> Self {
        Self {
            quote_id: None,
            cl_quote_id: Some(cl_quote_id.into()),
            rfq_id: None,
        }
    }

    /// Include the associated RFQ ID.
    pub fn rfq_id(mut self, rfq_id: impl Into<String>) -> Self {
        self.rfq_id = Some(rfq_id.into());
        self
    }
}

/// Request to cancel several quotes.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelBatchQuotesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_quote_ids: Option<Vec<String>>,
}

impl CancelBatchQuotesRequest {
    /// Select quotes by system IDs.
    pub fn by_quote_ids(quote_ids: Vec<String>) -> Self {
        Self {
            quote_ids: Some(quote_ids),
            cl_quote_ids: None,
        }
    }

    /// Select quotes by client IDs.
    pub fn by_client_quote_ids(cl_quote_ids: Vec<String>) -> Self {
        Self {
            quote_ids: None,
            cl_quote_ids: Some(cl_quote_ids),
        }
    }
}

/// Maker size and price-band settings for one product.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MakerInstrumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_block_sz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maker_px_band: Option<String>,
}

impl MakerInstrumentRequest {
    /// Select an instrument family.
    pub fn by_family(inst_family: impl Into<String>) -> Self {
        Self {
            inst_family: Some(inst_family.into()),
            ..Self::default()
        }
    }

    /// Select a spot instrument.
    pub fn by_instrument(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: Some(inst_id.into()),
            ..Self::default()
        }
    }

    /// Set the maximum block size.
    pub fn max_block_size(mut self, max_block_sz: impl Into<String>) -> Self {
        self.max_block_sz = Some(max_block_sz.into());
        self
    }

    /// Set the maker price band in ticks.
    pub fn maker_price_band(mut self, maker_px_band: impl Into<String>) -> Self {
        self.maker_px_band = Some(maker_px_band.into());
        self
    }
}

/// Maker settings for one instrument type.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MakerInstrumentSettingsRequest {
    inst_type: String,
    data: Vec<MakerInstrumentRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_all: Option<bool>,
}

impl MakerInstrumentSettingsRequest {
    /// Set the instrument type and product settings.
    pub fn new(inst_type: impl Into<String>, data: Vec<MakerInstrumentRequest>) -> Self {
        Self {
            inst_type: inst_type.into(),
            data,
            include_all: None,
        }
    }

    /// Set whether all products of this type should receive RFQs.
    pub fn include_all(mut self, include_all: bool) -> Self {
        self.include_all = Some(include_all);
        self
    }
}

/// Block-trading market maker protection configuration.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqMmpConfigRequest {
    time_interval: String,
    frozen_interval: String,
    count_limit: String,
}

impl RfqMmpConfigRequest {
    /// Set the monitoring window, frozen period, and execution-attempt limit.
    pub fn new(
        time_interval: impl Into<String>,
        frozen_interval: impl Into<String>,
        count_limit: impl Into<String>,
    ) -> Self {
        Self {
            time_interval: time_interval.into(),
            frozen_interval: frozen_interval.into(),
            count_limit: count_limit.into(),
        }
    }
}

/// Request configuring RFQ cancel-all-after protection.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqCancelAllAfterRequest {
    time_out: String,
}

impl RfqCancelAllAfterRequest {
    /// Set the timeout in seconds; zero disables the protection.
    pub fn new(time_out: impl Into<String>) -> Self {
        Self {
            time_out: time_out.into(),
        }
    }
}

/// Filters for retrieving RFQs.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl RfqsRequest {
    /// Create an unfiltered RFQ query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by system RFQ ID.
    pub fn rfq_id(mut self, value: impl Into<String>) -> Self {
        self.rfq_id = Some(value.into());
        self
    }

    /// Filter by client RFQ ID.
    pub fn client_rfq_id(mut self, value: impl Into<String>) -> Self {
        self.cl_rfq_id = Some(value.into());
        self
    }

    /// Filter by RFQ state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Return records newer than this RFQ ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this RFQ ID.
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

/// Filters for retrieving quotes.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl QuotesRequest {
    /// Create an unfiltered quote query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by system RFQ ID.
    pub fn rfq_id(mut self, value: impl Into<String>) -> Self {
        self.rfq_id = Some(value.into());
        self
    }

    /// Filter by client RFQ ID.
    pub fn client_rfq_id(mut self, value: impl Into<String>) -> Self {
        self.cl_rfq_id = Some(value.into());
        self
    }

    /// Filter by system quote ID.
    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
        self
    }

    /// Filter by client quote ID.
    pub fn client_quote_id(mut self, value: impl Into<String>) -> Self {
        self.cl_quote_id = Some(value.into());
        self
    }

    /// Filter by quote state.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Return records newer than this quote ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this quote ID.
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

/// Filters for retrieving private block trades.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RfqTradesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_rfq_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_td_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_successful: Option<bool>,
}

impl RfqTradesRequest {
    /// Create an unfiltered private-trade query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by system RFQ ID.
    pub fn rfq_id(mut self, value: impl Into<String>) -> Self {
        self.rfq_id = Some(value.into());
        self
    }

    /// Filter by client RFQ ID.
    pub fn client_rfq_id(mut self, value: impl Into<String>) -> Self {
        self.cl_rfq_id = Some(value.into());
        self
    }

    /// Filter by system quote ID.
    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
        self
    }

    /// Filter by block trade ID.
    pub fn block_trade_id(mut self, value: impl Into<String>) -> Self {
        self.block_td_id = Some(value.into());
        self
    }

    /// Filter by client quote ID.
    pub fn client_quote_id(mut self, value: impl Into<String>) -> Self {
        self.cl_quote_id = Some(value.into());
        self
    }

    /// Return records newer than this block trade ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this block trade ID.
    pub fn end_id(mut self, value: impl Into<String>) -> Self {
        self.end_id = Some(value.into());
        self
    }

    /// Filter from this execution timestamp.
    pub fn begin_timestamp(mut self, value: impl Into<String>) -> Self {
        self.begin_ts = Some(value.into());
        self
    }

    /// Filter through this execution timestamp.
    pub fn end_timestamp(mut self, value: impl Into<String>) -> Self {
        self.end_ts = Some(value.into());
        self
    }

    /// Set the page size.
    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }

    /// Filter by execution success.
    pub fn successful(mut self, value: bool) -> Self {
        self.is_successful = Some(value);
        self
    }
}

/// Pagination for public multi-leg block trades.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicRfqTradesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
}

impl PublicRfqTradesRequest {
    /// Create an unpaginated public-trade query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return records newer than this block trade ID.
    pub fn begin_id(mut self, value: impl Into<String>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records older than this block trade ID.
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
