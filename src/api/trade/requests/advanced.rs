use std::borrow::Cow;

use serde::Serialize;

/// Request body for `POST /api/v5/trade/easy-convert`.
#[derive(Debug, Clone, Serialize)]
pub struct EasyConvertRequest<'a> {
    #[serde(rename = "fromCcy")]
    from_ccy: Vec<Cow<'a, str>>,
    #[serde(rename = "toCcy")]
    to_ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Cow<'a, str>>,
}

impl<'a> EasyConvertRequest<'a> {
    /// Create an easy-convert request with one to five source currencies.
    pub fn new<I, S>(from_ccy: I, to_ccy: impl Into<Cow<'a, str>>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self {
            from_ccy: from_ccy.into_iter().map(Into::into).collect(),
            to_ccy: to_ccy.into(),
            source: None,
        }
    }

    /// Set the funding source: `1` for trading or `2` for funding.
    pub fn source(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.source = Some(value.into());
        self
    }
}

/// Query parameters for easy-convert history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EasyConvertHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> EasyConvertHistoryRequest<'a> {
    /// Create an empty history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return records earlier than this millisecond timestamp.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records newer than this millisecond timestamp.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the number of results, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// Query parameters for one-click-repay currency-list endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OneClickRepayCurrencyListRequest<'a> {
    #[serde(rename = "debtType", skip_serializing_if = "Option::is_none")]
    debt_type: Option<Cow<'a, str>>,
}

impl<'a> OneClickRepayCurrencyListRequest<'a> {
    /// Create an unfiltered currency-list query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter debt by `cross` or `isolated` type.
    pub fn debt_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.debt_type = Some(value.into());
        self
    }
}

/// One or more debt currencies accepted by the legacy and v2 repay APIs.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum DebtCurrencySelection<'a> {
    One(Cow<'a, str>),
    Many(Vec<Cow<'a, str>>),
}

/// Request body shared by one-click-repay v1 and v2 endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct OneClickRepayRequest<'a> {
    #[serde(rename = "debtCcy")]
    debt_ccy: DebtCurrencySelection<'a>,
    #[serde(rename = "repayCcy", skip_serializing_if = "Option::is_none")]
    repay_ccy: Option<Cow<'a, str>>,
    #[serde(rename = "repayCcyList", skip_serializing_if = "Option::is_none")]
    repay_ccy_list: Option<Vec<Cow<'a, str>>>,
}

impl<'a> OneClickRepayRequest<'a> {
    /// Build the legacy v1 request (`debtCcy` array + one `repayCcy`).
    pub fn new<I, S>(debt_ccy: I, repay_ccy: impl Into<Cow<'a, str>>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self {
            debt_ccy: DebtCurrencySelection::Many(debt_ccy.into_iter().map(Into::into).collect()),
            repay_ccy: Some(repay_ccy.into()),
            repay_ccy_list: None,
        }
    }

    /// Build the v2 request (`debtCcy` string + prioritized `repayCcyList`).
    pub fn v2<I, S>(debt_ccy: impl Into<Cow<'a, str>>, repay_ccy_list: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        Self {
            debt_ccy: DebtCurrencySelection::One(debt_ccy.into()),
            repay_ccy: None,
            repay_ccy_list: Some(repay_ccy_list.into_iter().map(Into::into).collect()),
        }
    }
}

/// Query parameters for one-click-repay history endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OneClickRepayHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> OneClickRepayHistoryRequest<'a> {
    /// Create an empty history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return records earlier than this millisecond timestamp.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records newer than this millisecond timestamp.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the number of results, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}
