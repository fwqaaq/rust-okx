use serde::Deserialize;

use crate::model::NumberString;

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
