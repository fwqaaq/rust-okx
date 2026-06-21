use serde::Serialize;

/// Request for [`get_saving_balance`](crate::api::finance::Savings::get_saving_balance).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CurrencyRequest<'a> {
    /// Currency filter, e.g. `Some("USDT")`. `None` returns all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<&'a str>,
}

/// Request for [`set_lending_rate`](crate::api::finance::Savings::set_lending_rate).
#[derive(Debug, Clone, Serialize)]
pub struct SetLendingRateRequest<'a> {
    /// Currency.
    pub ccy: &'a str,
    /// Lending rate.
    pub rate: &'a str,
}

/// Request for purchase/redeem operations on staking products.
#[derive(Debug, Clone, Serialize)]
pub struct AmountRequest<'a> {
    /// Amount.
    pub amt: &'a str,
}

/// Request for `cancel_redeem` on staking products.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRedeemRequest<'a> {
    /// Order ID to cancel.
    pub ord_id: &'a str,
}

/// Request for `apy_history` on staking products.
#[derive(Debug, Clone, Serialize)]
pub struct ApyHistoryRequest<'a> {
    /// Number of days of history, e.g. `"7"` or `"30"`.
    pub days: &'a str,
}

/// Common currency and cursor pagination query used by finance history endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FinanceHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FinanceHistoryRequest {
    /// Create an unfiltered history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the endpoint-specific `after` cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the endpoint-specific `before` cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the number of records to return, from 1 through 100.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
