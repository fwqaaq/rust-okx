use serde::Deserialize;

use crate::model::{NumberString, deserialize_vec_or_empty_string};

/// Result of checking whether an account can switch delta-neutral strategy mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PrecheckSetDeltaNeutralResult {
    /// Checks that must be resolved before changing strategy mode.
    #[serde(default)]
    pub unmatched_info_check: Vec<DeltaNeutralUnmatchedInfo>,
}

/// An unmatched item returned by the delta-neutral strategy precheck.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeltaNeutralUnmatchedInfo {
    /// Unmatched information type.
    #[serde(rename = "type", default)]
    pub unmatched_type: String,
    /// Delta leverage, when applicable.
    #[serde(default)]
    pub delta_lever: NumberString,
    /// Related order IDs.
    #[serde(default)]
    pub ord_list: Vec<String>,
    /// Related position IDs.
    #[serde(default)]
    pub pos_list: Vec<String>,
}

/// Result of setting the settlement currency for USD-margined contracts.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetSettleCurrencyResult {
    /// Settlement currency selected by the account.
    #[serde(default)]
    pub settle_ccy: String,
}

/// Result of setting the spot fee type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetFeeTypeResult {
    /// Fee type selected by the account.
    #[serde(default)]
    pub fee_type: String,
}

/// Result of checking an account-mode switch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchPrecheckResult {
    /// Check result code (`0` means all checks passed).
    #[serde(default)]
    pub s_code: String,
    /// Current account level.
    #[serde(default)]
    pub cur_acct_lv: String,
    /// Target account level.
    #[serde(default)]
    pub acct_lv: String,
    /// Deprecated risk-offset type returned by OKX.
    #[serde(default)]
    pub risk_offset_type: String,
    /// Information that does not meet the target mode's requirements.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub unmatched_info_check: Vec<AccountSwitchUnmatchedInfo>,
    /// Cross-margin contract positions relevant to the switch.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub pos_list: Vec<AccountSwitchPosition>,
    /// Positions that do not pass the target mode's tier check.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub pos_tier_check: Vec<AccountSwitchPositionTier>,
    /// Margin information before switching account mode.
    #[serde(default)]
    pub mgn_bf: Option<AccountSwitchMargin>,
    /// Margin information after switching account mode.
    #[serde(default)]
    pub mgn_aft: Option<AccountSwitchMargin>,
}

/// An unmet requirement found by the account-mode switch precheck.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchUnmatchedInfo {
    /// Unmatched information type.
    #[serde(rename = "type", default)]
    pub unmatched_type: String,
    /// Total assets, when applicable.
    #[serde(default)]
    pub total_asset: NumberString,
    /// Related position IDs.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub pos_list: Vec<String>,
}

/// Cross-margin position involved in an account-mode switch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchPosition {
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Leverage after the switch.
    #[serde(default)]
    pub lever: NumberString,
}

/// Position that does not pass the target account mode's tier check.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchPositionTier {
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Position quantity.
    #[serde(default)]
    pub pos: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Maximum position size allowed under the target mode.
    #[serde(default)]
    pub max_sz: NumberString,
}

/// Margin snapshot before or after an account-mode switch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchMargin {
    /// Account available equity in USD.
    #[serde(default)]
    pub acct_avail_eq: NumberString,
    /// Maintenance margin ratio.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Per-currency margin information.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub details: Vec<AccountSwitchMarginDetail>,
}

/// Per-currency margin information for an account-mode switch.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchMarginDetail {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Available equity in this currency.
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Maintenance margin ratio for this currency.
    #[serde(default)]
    pub mgn_ratio: NumberString,
}

/// Result of presetting account-mode switch parameters.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountSwitchPresetResult {
    /// Current account level.
    #[serde(default)]
    pub cur_acct_lv: String,
    /// Target account level.
    #[serde(default)]
    pub acct_lv: String,
    /// Leverage preset for cross-margin positions.
    #[serde(default)]
    pub lever: NumberString,
    /// Deprecated risk-offset type returned by OKX.
    #[serde(default)]
    pub risk_offset_type: String,
}
