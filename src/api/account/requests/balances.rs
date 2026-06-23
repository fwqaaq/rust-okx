use std::borrow::Cow;

use serde::Serialize;

/// Request for [`get_balance`](crate::api::account::Account::get_balance),
/// [`get_interest_rate`](crate::api::account::Account::get_interest_rate),
/// [`get_max_withdrawal`](crate::api::account::Account::get_max_withdrawal), and
/// [`get_greeks`](crate::api::account::Account::get_greeks).
///
/// All fields are optional; omit to return data for all currencies.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BalanceRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> BalanceRequest<'a> {
    /// Create an unfiltered balance query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}
