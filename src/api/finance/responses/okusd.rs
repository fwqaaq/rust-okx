use serde::Deserialize;

use crate::model::NumberString;

/// OKUSD subscription and redemption quotas.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OkusdLimits {
    /// Subscription quota information.
    #[serde(default)]
    pub sub_limit: OkusdSubscriptionLimit,
    /// Fast-redemption quota information.
    #[serde(default)]
    pub fast_redeem_limit: OkusdRedemptionLimit,
    /// Standard-redemption quota information.
    #[serde(default)]
    pub std_redeem_limit: OkusdRedemptionLimit,
    /// Server timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// OKUSD subscription quota usage.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OkusdSubscriptionLimit {
    /// Maximum amount that can still be subscribed today.
    #[serde(default)]
    pub max_sub_amt: NumberString,
    /// Account-level daily subscription limit.
    #[serde(default)]
    pub personal_daily_limit: NumberString,
    /// Amount already subscribed by the account today.
    #[serde(default)]
    pub personal_used_amt: NumberString,
    /// Platform-wide daily subscription limit.
    #[serde(default)]
    pub platform_daily_limit: NumberString,
    /// Amount already subscribed across the platform today.
    #[serde(default)]
    pub platform_used_amt: NumberString,
}

/// OKUSD redemption quota usage and fee rate.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OkusdRedemptionLimit {
    /// Account-level daily redemption limit.
    #[serde(default)]
    pub personal_daily_limit: NumberString,
    /// Redemption amount already used by the account today.
    #[serde(default)]
    pub personal_used_amt: NumberString,
    /// Platform-wide daily redemption limit.
    #[serde(default)]
    pub platform_daily_limit: NumberString,
    /// Redemption amount already used across the platform today.
    #[serde(default)]
    pub platform_used_amt: NumberString,
    /// Redemption fee rate.
    #[serde(default)]
    pub fee_rate: NumberString,
}

/// Result of an OKUSD subscription.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OkusdSubscription {
    /// System order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Subscription currency.
    #[serde(default)]
    pub ccy: String,
    /// Actual USDT amount subscribed.
    #[serde(default)]
    pub amt: NumberString,
    /// OKUSD amount credited.
    #[serde(default)]
    pub okusd_amt: NumberString,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Order creation timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of an OKUSD redemption.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OkusdRedemption {
    /// System order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Redemption currency.
    #[serde(default)]
    pub ccy: String,
    /// OKUSD amount redeemed.
    #[serde(default)]
    pub amt: NumberString,
    /// Fee charged in USDT.
    #[serde(default)]
    pub fee: NumberString,
    /// Net USDT amount credited.
    #[serde(default)]
    pub usdt_amt: NumberString,
    /// Redemption type.
    #[serde(default)]
    pub redeem_type: String,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Estimated settlement timestamp.
    #[serde(default)]
    pub est_settlement_time: NumberString,
    /// Order creation timestamp.
    #[serde(default)]
    pub ts: NumberString,
}
