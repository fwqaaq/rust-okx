use std::borrow::Cow;

use serde::Serialize;

/// Request for [`get_saving_balance`](crate::api::finance::Savings::get_saving_balance).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CurrencyRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> CurrencyRequest<'a> {
    /// Create an unfiltered currency query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Request for [`set_lending_rate`](crate::api::finance::Savings::set_lending_rate).
#[derive(Debug, Clone, Serialize)]
pub struct SetLendingRateRequest<'a> {
    ccy: Cow<'a, str>,
    rate: Cow<'a, str>,
}

impl<'a> SetLendingRateRequest<'a> {
    /// Create a lending-rate update.
    pub fn new(ccy: impl Into<Cow<'a, str>>, rate: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            rate: rate.into(),
        }
    }
}

/// Request for purchase/redeem operations on staking products.
#[derive(Debug, Clone, Serialize)]
pub struct AmountRequest<'a> {
    amt: Cow<'a, str>,
}

impl<'a> AmountRequest<'a> {
    /// Create an amount request.
    pub fn new(amt: impl Into<Cow<'a, str>>) -> Self {
        Self { amt: amt.into() }
    }
}

/// Request for `cancel_redeem` on staking products.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRedeemRequest<'a> {
    ord_id: Cow<'a, str>,
}

impl<'a> CancelRedeemRequest<'a> {
    /// Create a cancel-redeem request.
    pub fn new(ord_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ord_id: ord_id.into(),
        }
    }
}

/// Request for `apy_history` on staking products.
#[derive(Debug, Clone, Serialize)]
pub struct ApyHistoryRequest<'a> {
    days: Cow<'a, str>,
}

impl<'a> ApyHistoryRequest<'a> {
    /// Create an APY history query for the given number of days.
    pub fn new(days: impl Into<Cow<'a, str>>) -> Self {
        Self { days: days.into() }
    }
}

/// Common currency and cursor pagination query used by finance history endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FinanceHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> FinanceHistoryRequest<'a> {
    /// Create an unfiltered history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the endpoint-specific `after` cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the endpoint-specific `before` cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the number of records to return, from 1 through 100.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
