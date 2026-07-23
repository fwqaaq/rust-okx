use serde::Deserialize;

use crate::model::NumberString;

/// Result row returned by `POST /api/v5/trade/mass-cancel`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MassCancelResult {
    /// Whether OKX accepted the mass-cancel request.
    #[serde(default)]
    pub result: bool,
}

/// Result row returned by `POST /api/v5/trade/cancel-all-after`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelAllAfterResult {
    /// Trigger time returned by OKX.
    #[serde(default)]
    pub trigger_time: NumberString,
    /// Order tag covered by the request.
    #[serde(default)]
    pub tag: String,
    /// Request processing timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Account-level sub-account rate-limit information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountRateLimit {
    /// Current sub-account fill ratio.
    #[serde(default)]
    pub fill_ratio: NumberString,
    /// Current master-account fill ratio.
    #[serde(default)]
    pub main_fill_ratio: NumberString,
    /// Current account rate limit.
    #[serde(default)]
    pub acc_rate_limit: NumberString,
    /// Next-period account rate limit.
    #[serde(default)]
    pub next_acc_rate_limit: NumberString,
    /// Data timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Margin impact returned by `POST /api/v5/trade/order-precheck`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderPrecheckResult {
    /// Adjusted equity after the hypothetical order.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Change in adjusted equity.
    #[serde(default)]
    pub adj_eq_chg: NumberString,
    /// Initial margin requirement after the hypothetical order.
    #[serde(default)]
    pub imr: NumberString,
    /// Change in initial margin requirement.
    #[serde(default)]
    pub imr_chg: NumberString,
    /// Maintenance margin requirement after the hypothetical order.
    #[serde(default)]
    pub mmr: NumberString,
    /// Change in maintenance margin requirement.
    #[serde(default)]
    pub mmr_chg: NumberString,
    /// Margin ratio after the hypothetical order.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Change in margin ratio.
    #[serde(default)]
    pub mgn_ratio_chg: NumberString,
    /// Available balance after the hypothetical order.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Change in available balance.
    #[serde(default)]
    pub avail_bal_chg: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
    /// Difference between the liquidation and mark prices.
    #[serde(default)]
    pub liq_px_diff: NumberString,
    /// Liquidation-price difference ratio.
    #[serde(default)]
    pub liq_px_diff_ratio: NumberString,
    /// Position balance after the hypothetical order.
    #[serde(default)]
    pub pos_bal: NumberString,
    /// Change in position balance.
    #[serde(default)]
    pub pos_bal_chg: NumberString,
    /// Liability after the hypothetical order.
    #[serde(default)]
    pub liab: NumberString,
    /// Change in liability.
    #[serde(default)]
    pub liab_chg: NumberString,
    /// Currency of the liability change.
    #[serde(default)]
    pub liab_chg_ccy: String,
    /// Result type returned by OKX.
    #[serde(rename = "type", default)]
    pub type_: String,
}
