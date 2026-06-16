use serde::Serialize;

use crate::model::{OrderSide, RequestValidationError, ValidateRequest};

const CLIENT_REQUEST_ID_MAX_LEN: usize = 32;

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

impl ValidateRequest for ConvertCurrenciesRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        Ok(())
    }
}

/// Query parameters for [`Convert::get_currency_pair`](super::Convert::get_currency_pair).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertCurrencyPairRequest {
    from_ccy: String,
    to_ccy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_mode: Option<ConvertMode>,
}

impl ConvertCurrencyPairRequest {
    /// Create a currency-pair request with the required source and destination currencies.
    pub fn new(from_ccy: impl Into<String>, to_ccy: impl Into<String>) -> Self {
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

impl ValidateRequest for ConvertCurrencyPairRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        require_non_empty("fromCcy", &self.from_ccy)?;
        require_non_empty("toCcy", &self.to_ccy)
    }
}

/// Request body for [`Convert::estimate_quote`](super::Convert::estimate_quote).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertQuoteRequest {
    base_ccy: String,
    quote_ccy: String,
    side: OrderSide,
    rfq_sz: String,
    rfq_sz_ccy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_q_req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_mode: Option<ConvertMode>,
}

impl ConvertQuoteRequest {
    /// Create an estimate-quote request with all fields required by OKX.
    pub fn new(
        base_ccy: impl Into<String>,
        quote_ccy: impl Into<String>,
        side: OrderSide,
        rfq_sz: impl Into<String>,
        rfq_sz_ccy: impl Into<String>,
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
    pub fn client_quote_request_id(mut self, id: impl Into<String>) -> Self {
        self.cl_q_req_id = Some(id.into());
        self
    }

    /// Set the broker order tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Select the standard or VIP large-order Convert mode.
    pub fn convert_mode(mut self, convert_mode: ConvertMode) -> Self {
        self.convert_mode = Some(convert_mode);
        self
    }
}

impl ValidateRequest for ConvertQuoteRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        require_non_empty("baseCcy", &self.base_ccy)?;
        require_non_empty("quoteCcy", &self.quote_ccy)?;
        validate_side(&self.side)?;
        require_non_empty("rfqSz", &self.rfq_sz)?;
        require_non_empty("rfqSzCcy", &self.rfq_sz_ccy)?;
        validate_client_request_id("clQReqId", self.cl_q_req_id.as_deref())
    }
}

/// Request body for [`Convert::convert_trade`](super::Convert::convert_trade).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertTradeRequest {
    quote_id: String,
    base_ccy: String,
    quote_ccy: String,
    side: OrderSide,
    sz: String,
    sz_ccy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_t_req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convert_mode: Option<ConvertMode>,
}

impl ConvertTradeRequest {
    /// Create a Convert trade request with all fields required by OKX.
    pub fn new(
        quote_id: impl Into<String>,
        base_ccy: impl Into<String>,
        quote_ccy: impl Into<String>,
        side: OrderSide,
        sz: impl Into<String>,
        sz_ccy: impl Into<String>,
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
    pub fn client_trade_request_id(mut self, id: impl Into<String>) -> Self {
        self.cl_t_req_id = Some(id.into());
        self
    }

    /// Set the broker order tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Select the standard or VIP large-order Convert mode.
    pub fn convert_mode(mut self, convert_mode: ConvertMode) -> Self {
        self.convert_mode = Some(convert_mode);
        self
    }
}

impl ValidateRequest for ConvertTradeRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        require_non_empty("quoteId", &self.quote_id)?;
        require_non_empty("baseCcy", &self.base_ccy)?;
        require_non_empty("quoteCcy", &self.quote_ccy)?;
        validate_side(&self.side)?;
        require_non_empty("sz", &self.sz)?;
        require_non_empty("szCcy", &self.sz_ccy)?;
        validate_client_request_id("clTReqId", self.cl_t_req_id.as_deref())
    }
}

/// Query parameters for [`Convert::get_convert_history`](super::Convert::get_convert_history).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    cl_t_req_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

impl ConvertHistoryRequest {
    /// Create an empty history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by the client trade request ID (`clTReqId`).
    pub fn client_trade_request_id(mut self, id: impl Into<String>) -> Self {
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
        self.limit = Some(limit.to_string());
        self
    }

    /// Filter by the broker order tag used by the original Convert trade.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}

impl ValidateRequest for ConvertHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        validate_client_request_id("clTReqId", self.cl_t_req_id.as_deref())?;

        if let Some(limit) = &self.limit {
            let parsed = limit
                .parse::<u64>()
                .expect("ConvertHistoryRequest::limit stores a u8 as decimal text");
            if !(1..=100).contains(&parsed) {
                return Err(RequestValidationError::OutOfRange {
                    field: "limit",
                    min: 1,
                    max: 100,
                });
            }
        }

        Ok(())
    }
}

fn require_non_empty(field: &'static str, value: &str) -> Result<(), RequestValidationError> {
    if value.is_empty() {
        Err(RequestValidationError::EmptyField { field })
    } else {
        Ok(())
    }
}

fn validate_side(side: &OrderSide) -> Result<(), RequestValidationError> {
    match side {
        OrderSide::Buy | OrderSide::Sell => Ok(()),
        _ => Err(RequestValidationError::InvalidFormat {
            field: "side",
            expected: "buy or sell",
        }),
    }
}

fn validate_client_request_id(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), RequestValidationError> {
    let Some(value) = value else {
        return Ok(());
    };

    require_non_empty(field, value)?;
    if value.chars().count() > CLIENT_REQUEST_ID_MAX_LEN {
        return Err(RequestValidationError::TooLong {
            field,
            max: CLIENT_REQUEST_ID_MAX_LEN,
        });
    }
    if !value.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(RequestValidationError::InvalidFormat {
            field,
            expected: "1-32 ASCII alphanumeric characters",
        });
    }

    Ok(())
}
