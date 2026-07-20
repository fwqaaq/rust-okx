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

/// Request for [`set_collateral_assets`](crate::api::account::Account::set_collateral_assets).
#[derive(Debug, Clone, Serialize)]
pub struct SetCollateralAssetsRequest<'a> {
    #[serde(rename = "type")]
    collateral_type: Cow<'a, str>,
    #[serde(rename = "collateralEnabled")]
    collateral_enabled: bool,
    #[serde(rename = "ccyList", skip_serializing_if = "Vec::is_empty")]
    ccy_list: Vec<Cow<'a, str>>,
}

impl<'a> SetCollateralAssetsRequest<'a> {
    /// Create a request that updates the collateral setting for all assets.
    pub fn all(collateral_enabled: bool) -> Self {
        Self {
            collateral_type: Cow::Borrowed("all"),
            collateral_enabled,
            ccy_list: Vec::new(),
        }
    }

    /// Create a request that updates the collateral setting for specific currencies.
    pub fn custom<I, C>(ccy_list: I, collateral_enabled: bool) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Cow<'a, str>>,
    {
        Self {
            collateral_type: Cow::Borrowed("custom"),
            collateral_enabled,
            ccy_list: ccy_list.into_iter().map(Into::into).collect(),
        }
    }

    /// Create a collateral-assets request with an explicit OKX `type` value.
    pub fn new(collateral_type: impl Into<Cow<'a, str>>, collateral_enabled: bool) -> Self {
        Self {
            collateral_type: collateral_type.into(),
            collateral_enabled,
            ccy_list: Vec::new(),
        }
    }

    /// Set the currency list for a custom collateral-assets request.
    pub fn currencies<I, C>(mut self, ccy_list: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Cow<'a, str>>,
    {
        self.ccy_list = ccy_list.into_iter().map(Into::into).collect();
        self
    }
}

/// Query parameters for [`get_collateral_assets`](crate::api::account::Account::get_collateral_assets).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralAssetsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collateral_enabled: Option<bool>,
}

impl<'a> GetCollateralAssetsRequest<'a> {
    /// Create an unfiltered collateral-assets query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by one currency or a comma-separated list of up to 20 currencies.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by whether the asset is enabled as collateral.
    pub fn collateral_enabled(mut self, collateral_enabled: bool) -> Self {
        self.collateral_enabled = Some(collateral_enabled);
        self
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
