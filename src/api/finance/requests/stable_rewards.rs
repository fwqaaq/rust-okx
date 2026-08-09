use std::borrow::Cow;

use serde::Serialize;

/// Query selecting one Stable Rewards currency.
#[derive(Debug, Clone, Serialize)]
pub struct StableRewardsCurrencyRequest<'a> {
    ccy: Cow<'a, str>,
}

impl<'a> StableRewardsCurrencyRequest<'a> {
    /// Select a stablecoin.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self { ccy: ccy.into() }
    }
}

/// Optional currency filter for Stable Rewards balances.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StableRewardsBalanceRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> StableRewardsBalanceRequest<'a> {
    /// Create an unfiltered balance query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by stablecoin.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Query for Stable Rewards APY history.
#[derive(Debug, Clone, Serialize)]
pub struct StableRewardsApyHistoryRequest<'a> {
    ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days: Option<u32>,
}

impl<'a> StableRewardsApyHistoryRequest<'a> {
    /// Select a stablecoin.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            days: None,
        }
    }

    /// Set the number of days, up to 100.
    pub fn days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }
}
