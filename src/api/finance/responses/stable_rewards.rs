use serde::Deserialize;

use crate::model::NumberString;

/// Stable Rewards product information.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StableRewardsProductInfo {
    /// Settlement-currency details.
    #[serde(default)]
    pub details: Vec<StableRewardsProductDetail>,
    /// Query time.
    #[serde(default)]
    pub ts: NumberString,
}

/// Subscription and redemption settings for one settlement currency.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StableRewardsProductDetail {
    /// Subscribable stablecoin.
    #[serde(default)]
    pub ccy: String,
    /// Settlement currency.
    #[serde(default)]
    pub settle_ccy: String,
    /// Subscription fee rate.
    #[serde(default)]
    pub sub_fee_rate: NumberString,
    /// Redemption fee rate.
    #[serde(default)]
    pub redempt_fee_rate: NumberString,
    /// Minimum subscription amount.
    #[serde(default)]
    pub min_sub_amt: NumberString,
    /// Minimum redemption amount.
    #[serde(default)]
    pub min_redeem_amt: NumberString,
    /// Remaining subscription quota.
    #[serde(default)]
    pub remaining_sub_quota: NumberString,
    /// Remaining redemption quota.
    #[serde(default)]
    pub remaining_redempt_quota: NumberString,
    /// Whether redemption is currently available.
    #[serde(default)]
    pub can_redeem: bool,
}

/// Stable Rewards account balance snapshot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StableRewardsBalance {
    /// Per-currency balances.
    #[serde(default)]
    pub details: Vec<StableRewardsBalanceDetail>,
    /// Query time.
    #[serde(default)]
    pub ts: NumberString,
}

/// Stable Rewards balance for one stablecoin.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StableRewardsBalanceDetail {
    /// Stablecoin.
    #[serde(default)]
    pub ccy: String,
    /// Amount held across the account.
    #[serde(default)]
    pub amt: NumberString,
    /// Lifetime interest accrued.
    #[serde(default)]
    pub total_earn_accrual: NumberString,
    /// Earning state.
    #[serde(default)]
    pub state: String,
}

/// Stable Rewards daily APY snapshot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StableRewardsApy {
    /// Daily APY.
    #[serde(default)]
    pub rate: NumberString,
    /// Snapshot time.
    #[serde(default)]
    pub ts: NumberString,
}
