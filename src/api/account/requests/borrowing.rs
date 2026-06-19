use serde::Serialize;

use crate::model::TradeMode;

/// Query parameters for maximum loan.
#[derive(Debug, Clone, Serialize)]
pub struct MaxLoanRequest {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "mgnCcy", skip_serializing_if = "Option::is_none")]
    mgn_ccy: Option<String>,
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl MaxLoanRequest {
    /// Create an instrument-based maximum-loan query.
    ///
    /// `inst_id` may contain one to five comma-separated instrument IDs, as
    /// documented by OKX. Use [`Self::by_currency`] for Spot-mode manual-borrow
    /// quota queries.
    pub fn new(inst_id: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self::by_instrument(inst_id, mgn_mode)
    }

    /// Create an instrument-based maximum-loan query.
    pub fn by_instrument(inst_id: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self {
            mgn_mode,
            inst_id: Some(inst_id.into()),
            ccy: None,
            mgn_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Create a currency-based Spot-mode manual-borrow quota query.
    pub fn by_currency(ccy: impl Into<String>) -> Self {
        Self {
            mgn_mode: TradeMode::Cross,
            inst_id: None,
            ccy: Some(ccy.into()),
            mgn_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Replace the selector with a currency-based Spot-mode query.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.inst_id = None;
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin currency.
    pub fn margin_currency(mut self, mgn_ccy: impl Into<String>) -> Self {
        self.mgn_ccy = Some(mgn_ccy.into());
        self
    }

    /// Set the trade quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// Query parameters for interest-accrued records.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestAccruedRequest {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl InterestAccruedRequest {
    /// Create an empty interest-accrued query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
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
}

/// Request body for borrow/repay.
#[derive(Debug, Clone, Serialize)]
pub struct BorrowRepayRequest {
    ccy: String,
    side: String,
    amt: String,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl BorrowRepayRequest {
    /// Create a borrow/repay request.
    pub fn new(ccy: impl Into<String>, side: impl Into<String>, amt: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            side: side.into(),
            amt: amt.into(),
            ord_id: None,
        }
    }

    /// Set the related order ID.
    pub fn order_id(mut self, ord_id: impl Into<String>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }
}

/// Query parameters for borrow/repay history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BorrowRepayHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl BorrowRepayHistoryRequest {
    /// Create an empty borrow/repay-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
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
}

/// Query parameters for interest limits.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestLimitsRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    limit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl InterestLimitsRequest {
    /// Create an empty interest-limits query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the OKX interest-limit type.
    pub fn limit_type(mut self, limit_type: impl Into<String>) -> Self {
        self.limit_type = Some(limit_type.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}
