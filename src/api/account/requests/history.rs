use std::borrow::Cow;

use serde::Serialize;

use crate::model::{InstType, TradeMode};

/// Query parameters for account bills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillsRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(rename = "ctType", skip_serializing_if = "Option::is_none")]
    ct_type: Option<Cow<'a, str>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<Cow<'a, str>>,
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    sub_type: Option<Cow<'a, str>>,
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

impl<'a> BillsRequest<'a> {
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
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Set the contract type filter.
    pub fn contract_type(mut self, ct_type: impl Into<Cow<'a, str>>) -> Self {
        self.ct_type = Some(ct_type.into());
        self
    }

    /// Set the bill type filter.
    pub fn bill_type(mut self, bill_type: impl Into<Cow<'a, str>>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }

    /// Set the bill subtype filter.
    pub fn sub_type(mut self, sub_type: impl Into<Cow<'a, str>>) -> Self {
        self.sub_type = Some(sub_type.into());
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

    /// Set the begin timestamp filter.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp filter.
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

/// Query parameters for archived account bills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillsArchiveRequest<'a> {
    #[serde(flatten)]
    base: BillsRequest<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
}

impl<'a> BillsArchiveRequest<'a> {
    /// Create an empty archived-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the common bills filters.
    pub fn filters(mut self, base: BillsRequest<'a>) -> Self {
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

/// Query parameters for account bill type/subtype mapping.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillSubtypesRequest<'a> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<Cow<'a, str>>,
}

impl<'a> BillSubtypesRequest<'a> {
    /// Create an empty bill-subtypes query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by bill type. Multiple values can be separated by commas.
    pub fn bill_type(mut self, bill_type: impl Into<Cow<'a, str>>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }
}

/// Quarter selector for historical account-bills archive generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum BillsHistoryArchiveQuarter {
    /// First quarter.
    #[serde(rename = "Q1")]
    Q1,
    /// Second quarter.
    #[serde(rename = "Q2")]
    Q2,
    /// Third quarter.
    #[serde(rename = "Q3")]
    Q3,
    /// Fourth quarter.
    #[serde(rename = "Q4")]
    Q4,
}

/// Parameters for historical account-bills archive generation and download-link lookup.
#[derive(Debug, Clone, Serialize)]
pub struct BillsHistoryArchiveRequest<'a> {
    year: Cow<'a, str>,
    quarter: BillsHistoryArchiveQuarter,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<Cow<'a, str>>,
}

impl<'a> BillsHistoryArchiveRequest<'a> {
    /// Create a request for a year and quarter.
    pub fn new(year: impl Into<Cow<'a, str>>, quarter: BillsHistoryArchiveQuarter) -> Self {
        Self {
            year: year.into(),
            quarter,
            bill_type: None,
        }
    }

    /// Filter by bill type. Multiple values can be separated by commas.
    pub fn bill_type(mut self, bill_type: impl Into<Cow<'a, str>>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }
}

/// Request body for applying historical account-bills archive generation.
pub type ApplyBillsHistoryArchiveRequest<'a> = BillsHistoryArchiveRequest<'a>;

/// Query parameters for position history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PositionsHistoryRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    close_type: Option<Cow<'a, str>>,
    #[serde(rename = "posId", skip_serializing_if = "Option::is_none")]
    pos_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> PositionsHistoryRequest<'a> {
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
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Set the OKX close type filter.
    pub fn close_type(mut self, close_type: impl Into<Cow<'a, str>>) -> Self {
        self.close_type = Some(close_type.into());
        self
    }

    /// Set the position ID filter.
    pub fn position_id(mut self, pos_id: impl Into<Cow<'a, str>>) -> Self {
        self.pos_id = Some(pos_id.into());
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
}
