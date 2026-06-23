use std::borrow::Cow;

use serde::Serialize;

/// Request for [`set_position_mode`](crate::api::account::Account::set_position_mode).
#[derive(Debug, Clone, Serialize)]
pub struct SetPositionModeRequest<'a> {
    #[serde(rename = "posMode")]
    pos_mode: Cow<'a, str>,
}

impl<'a> SetPositionModeRequest<'a> {
    /// Create a position-mode update.
    pub fn new(pos_mode: impl Into<Cow<'a, str>>) -> Self {
        Self {
            pos_mode: pos_mode.into(),
        }
    }
}

/// Request for [`set_greeks`](crate::api::account::Account::set_greeks).
#[derive(Debug, Clone, Serialize)]
pub struct SetGreeksRequest<'a> {
    #[serde(rename = "greeksType")]
    greeks_type: Cow<'a, str>,
}

impl<'a> SetGreeksRequest<'a> {
    /// Create a greeks-display update.
    pub fn new(greeks_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            greeks_type: greeks_type.into(),
        }
    }
}

/// Request for [`set_isolated_mode`](crate::api::account::Account::set_isolated_mode).
#[derive(Debug, Clone, Serialize)]
pub struct SetIsolatedModeRequest<'a> {
    #[serde(rename = "isoMode")]
    iso_mode: Cow<'a, str>,
    #[serde(rename = "type")]
    mode_type: Cow<'a, str>,
}

impl<'a> SetIsolatedModeRequest<'a> {
    /// Create an isolated-mode update.
    pub fn new(iso_mode: impl Into<Cow<'a, str>>, mode_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            iso_mode: iso_mode.into(),
            mode_type: mode_type.into(),
        }
    }
}

/// Request for [`set_auto_loan`](crate::api::account::Account::set_auto_loan).
#[derive(Debug, Clone, Serialize)]
pub struct SetAutoLoanRequest {
    #[serde(rename = "autoLoan")]
    auto_loan: bool,
}

impl SetAutoLoanRequest {
    /// Create an auto-loan update.
    pub fn new(auto_loan: bool) -> Self {
        Self { auto_loan }
    }
}

/// Request for [`set_account_level`](crate::api::account::Account::set_account_level).
#[derive(Debug, Clone, Serialize)]
pub struct SetAccountLevelRequest<'a> {
    #[serde(rename = "acctLv")]
    acct_lv: Cow<'a, str>,
}

impl<'a> SetAccountLevelRequest<'a> {
    /// Create an account-level update.
    pub fn new(acct_lv: impl Into<Cow<'a, str>>) -> Self {
        Self {
            acct_lv: acct_lv.into(),
        }
    }
}
