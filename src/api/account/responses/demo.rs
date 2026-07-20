use serde::Deserialize;

use crate::model::NumberString;

/// Result of adjusting balances in the demo trading environment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DemoAdjustBalanceResult {
    /// Remaining daily increase quota.
    #[serde(default)]
    pub remain_cnt: NumberString,
    /// Total daily increase quota.
    #[serde(default)]
    pub total_cnt: NumberString,
    /// Per-currency operation details.
    #[serde(default)]
    pub details: Vec<DemoBalanceAdjustmentDetail>,
}

/// Per-currency result of a demo balance adjustment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DemoBalanceAdjustmentDetail {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Adjustment amount applied.
    #[serde(default)]
    pub amt: NumberString,
    /// Balance after the operation.
    #[serde(default)]
    pub bal: NumberString,
}
