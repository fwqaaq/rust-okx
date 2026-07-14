use serde::Deserialize;

use crate::model::NumberString;

/// Current Market Maker Protection configuration and trigger status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MmpConfig {
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Whether MMP is currently triggered.
    #[serde(default)]
    pub mmp_frozen: bool,
    /// Remaining time in milliseconds until MMP is no longer triggered.
    #[serde(default)]
    pub mmp_frozen_until: NumberString,
    /// MMP monitoring window in milliseconds.
    #[serde(default)]
    pub time_interval: NumberString,
    /// Frozen period in milliseconds.
    #[serde(default)]
    pub frozen_interval: NumberString,
    /// Trade quantity limit in contracts.
    #[serde(default)]
    pub qty_limit: NumberString,
}

/// Market Maker Protection configuration returned after an update.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetMmpConfigResult {
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// MMP monitoring window in milliseconds.
    #[serde(default)]
    pub time_interval: NumberString,
    /// Frozen period in milliseconds.
    #[serde(default)]
    pub frozen_interval: NumberString,
    /// Trade quantity limit in contracts.
    #[serde(default)]
    pub qty_limit: NumberString,
}

/// Result of resetting Market Maker Protection status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResetMmpStatusResult {
    /// Whether the reset request succeeded.
    #[serde(default)]
    pub result: bool,
}
