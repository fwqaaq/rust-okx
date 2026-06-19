use serde::Serialize;

use crate::model::{InstType, TradeMode};

/// Query parameters for account bills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillsRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(rename = "ctType", skip_serializing_if = "Option::is_none")]
    ct_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<String>,
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl BillsRequest {
    /// Create an empty account-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
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

    /// Set the contract type filter.
    pub fn contract_type(mut self, ct_type: impl Into<String>) -> Self {
        self.ct_type = Some(ct_type.into());
        self
    }

    /// Set the bill type filter.
    pub fn bill_type(mut self, bill_type: impl Into<String>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }

    /// Set the bill subtype filter.
    pub fn sub_type(mut self, sub_type: impl Into<String>) -> Self {
        self.sub_type = Some(sub_type.into());
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

/// Query parameters for archived account bills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillsArchiveRequest {
    #[serde(flatten)]
    base: BillsRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
}

impl BillsArchiveRequest {
    /// Create an empty archived-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the common bills filters.
    pub fn filters(mut self, base: BillsRequest) -> Self {
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

/// Query parameters for position history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PositionsHistoryRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    close_type: Option<String>,
    #[serde(rename = "posId", skip_serializing_if = "Option::is_none")]
    pos_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl PositionsHistoryRequest {
    /// Create an empty position-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Set the OKX close type filter.
    pub fn close_type(mut self, close_type: impl Into<String>) -> Self {
        self.close_type = Some(close_type.into());
        self
    }

    /// Set the position ID filter.
    pub fn position_id(mut self, pos_id: impl Into<String>) -> Self {
        self.pos_id = Some(pos_id.into());
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
