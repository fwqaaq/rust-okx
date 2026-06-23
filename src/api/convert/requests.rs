use std::borrow::Cow;

use serde::Serialize;

use crate::model::OrderSide;

/// Convert mode used by currency-pair, quote, and trade requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum ConvertMode {
    /// Standard Convert flow.
    #[serde(rename = "0")]
    Standard,
    /// Large-order Convert flow for eligible VIP users.
    #[serde(rename = "1")]
    LargeOrderVip,
}

/// Empty query for [`Convert::get_currencies`](super::Convert::get_currencies).
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct ConvertCurrenciesRequest {}

impl ConvertCurrenciesRequest {
    /// Create the empty currencies request.
    pub const fn new() -> Self {
        Self {}
    }
}

/// Query parameters for [`Convert::get_currency_pair`](super::Convert::get_currency_pair).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertCurrencyPairRequest<'a> {
    from_ccy: Cow<'a, str>,
    to_ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_mode: Option<ConvertMode>,
}

impl<'a> ConvertCurrencyPairRequest<'a> {
    /// Create a currency-pair request with the required source and destination currencies.
    pub fn new(from_ccy: impl Into<Cow<'a, str>>, to_ccy: impl Into<Cow<'a, str>>) -> Self {
        Self {
            from_ccy: from_ccy.into(),
            to_ccy: to_ccy.into(),
            convert_mode: None,
        }
    }

    /// Select the standard or VIP large-order Convert mode.
    pub fn convert_mode(mut self, convert_mode: ConvertMode) -> Self {
        self.convert_mode = Some(convert_mode);
        self
    }
}

/// Request body for [`Convert::estimate_quote`](super::Convert::estimate_quote).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertQuoteRequest<'a> {
    base_ccy: Cow<'a, str>,
    quote_ccy: Cow<'a, str>,
    side: OrderSide,
    rfq_sz: Cow<'a, str>,
    rfq_sz_ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_q_req_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_mode: Option<ConvertMode>,
}

impl<'a> ConvertQuoteRequest<'a> {
    /// Create an estimate-quote request with all fields required by OKX.
    pub fn new(
        base_ccy: impl Into<Cow<'a, str>>,
        quote_ccy: impl Into<Cow<'a, str>>,
        side: OrderSide,
        rfq_sz: impl Into<Cow<'a, str>>,
        rfq_sz_ccy: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            base_ccy: base_ccy.into(),
            quote_ccy: quote_ccy.into(),
            side,
            rfq_sz: rfq_sz.into(),
            rfq_sz_ccy: rfq_sz_ccy.into(),
            cl_q_req_id: None,
            tag: None,
            convert_mode: None,
        }
    }

    /// Set the client quote request ID (`clQReqId`).
    pub fn client_quote_request_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_q_req_id = Some(id.into());
        self
    }

    /// Set the broker order tag.
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Select the standard or VIP large-order Convert mode.
    pub fn convert_mode(mut self, convert_mode: ConvertMode) -> Self {
        self.convert_mode = Some(convert_mode);
        self
    }
}

/// Request body for [`Convert::convert_trade`](super::Convert::convert_trade).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertTradeRequest<'a> {
    quote_id: Cow<'a, str>,
    base_ccy: Cow<'a, str>,
    quote_ccy: Cow<'a, str>,
    side: OrderSide,
    sz: Cow<'a, str>,
    sz_ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_t_req_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_mode: Option<ConvertMode>,
}

impl<'a> ConvertTradeRequest<'a> {
    /// Create a Convert trade request with all fields required by OKX.
    pub fn new(
        quote_id: impl Into<Cow<'a, str>>,
        base_ccy: impl Into<Cow<'a, str>>,
        quote_ccy: impl Into<Cow<'a, str>>,
        side: OrderSide,
        sz: impl Into<Cow<'a, str>>,
        sz_ccy: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            quote_id: quote_id.into(),
            base_ccy: base_ccy.into(),
            quote_ccy: quote_ccy.into(),
            side,
            sz: sz.into(),
            sz_ccy: sz_ccy.into(),
            cl_t_req_id: None,
            tag: None,
            convert_mode: None,
        }
    }

    /// Set the client trade request ID (`clTReqId`).
    pub fn client_trade_request_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_t_req_id = Some(id.into());
        self
    }

    /// Set the broker order tag.
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Select the standard or VIP large-order Convert mode.
    pub fn convert_mode(mut self, convert_mode: ConvertMode) -> Self {
        self.convert_mode = Some(convert_mode);
        self
    }
}

/// Query parameters for [`Convert::get_convert_history`](super::Convert::get_convert_history).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_t_req_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
}

impl<'a> ConvertHistoryRequest<'a> {
    /// Create an empty history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by the client trade request ID (`clTReqId`).
    pub fn client_trade_request_id(mut self, id: impl Into<Cow<'a, str>>) -> Self {
        self.cl_t_req_id = Some(id.into());
        self
    }

    /// Return records earlier than this millisecond timestamp.
    pub fn after(mut self, timestamp_ms: u64) -> Self {
        self.after = Some(timestamp_ms.to_string());
        self
    }

    /// Return records newer than this millisecond timestamp.
    pub fn before(mut self, timestamp_ms: u64) -> Self {
        self.before = Some(timestamp_ms.to_string());
        self
    }

    /// Set the maximum number of returned rows. OKX accepts values from 1 to 100.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by the broker order tag used by the original Convert trade.
    pub fn tag(mut self, tag: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}
