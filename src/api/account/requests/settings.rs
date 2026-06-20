use serde::Serialize;

/// Request for [`set_position_mode`](crate::api::account::Account::set_position_mode).
#[derive(Debug, Clone, Serialize)]
pub struct SetPositionModeRequest<'a> {
    /// Position mode: `"long_short_mode"` or `"net_mode"`.
    #[serde(rename = "posMode")]
    pub pos_mode: &'a str,
}

/// Request for [`set_greeks`](crate::api::account::Account::set_greeks).
#[derive(Debug, Clone, Serialize)]
pub struct SetGreeksRequest<'a> {
    /// Greeks display type: `"PA"` (coin) or `"BS"` (Black-Scholes).
    #[serde(rename = "greeksType")]
    pub greeks_type: &'a str,
}

/// Request for [`set_isolated_mode`](crate::api::account::Account::set_isolated_mode).
#[derive(Debug, Clone, Serialize)]
pub struct SetIsolatedModeRequest<'a> {
    /// Isolated margin mode: `"automatic"` or `"autonomy"`.
    #[serde(rename = "isoMode")]
    pub iso_mode: &'a str,
    /// Instrument type scope: `"MARGIN"` or `"CONTRACTS"`.
    #[serde(rename = "type")]
    pub mode_type: &'a str,
}

/// Request for [`set_auto_loan`](crate::api::account::Account::set_auto_loan).
#[derive(Debug, Clone, Serialize)]
pub struct SetAutoLoanRequest {
    /// `true` to enable automatic borrowing; `false` to disable.
    #[serde(rename = "autoLoan")]
    pub auto_loan: bool,
}

/// Request for [`set_account_level`](crate::api::account::Account::set_account_level).
#[derive(Debug, Clone, Serialize)]
pub struct SetAccountLevelRequest<'a> {
    /// Target account level, e.g. `"1"` (simple), `"2"` (single-currency margin), etc.
    #[serde(rename = "acctLv")]
    pub acct_lv: &'a str,
}
