use serde::Serialize;

/// Request for [`get_balance`](crate::api::account::Account::get_balance),
/// [`get_interest_rate`](crate::api::account::Account::get_interest_rate),
/// [`get_max_withdrawal`](crate::api::account::Account::get_max_withdrawal), and
/// [`get_greeks`](crate::api::account::Account::get_greeks).
///
/// All fields are optional; omit to return data for all currencies.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BalanceRequest<'a> {
    /// Currency filter (comma-separated, e.g. `"BTC,USDT"`). `None` returns all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<&'a str>,
}
