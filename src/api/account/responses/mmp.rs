use serde::Deserialize;

/// Result of resetting Market Maker Protection status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ResetMmpStatusResult {
    /// Whether the reset request succeeded.
    #[serde(default)]
    pub result: bool,
}
