use serde::Deserialize;

use crate::model::NumberString;

/// Spot borrow/repay mutation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpotBorrowRepayResult {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `side` field.
    #[serde(default)]
    pub side: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
}

/// Auto-repay setting result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetAutoRepayResult {
    /// Value returned by OKX in the `autoRepay` field.
    #[serde(default)]
    pub auto_repay: bool,
}

/// Spot borrow/repay history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpotBorrowRepayHistory {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `type` field.
    #[serde(default, rename = "type")]
    pub event_type: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `accBorrowed` field.
    #[serde(default)]
    pub acc_borrowed: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Auto-earn setting result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetAutoEarnResult {
    /// Auto earn type. `0`: auto earn (auto lend, auto staking); `1`: auto earn (USDG earn).
    #[serde(default)]
    pub earn_type: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Auto earn operation action, e.g. `turn_on`/`turn_off` (deprecated).
    #[serde(default)]
    pub action: String,
    /// Minimum lending APR (deprecated).
    #[serde(default)]
    pub apr: NumberString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_spot_borrow_repay_result() {
        let row: SpotBorrowRepayResult =
            serde_json::from_str(r#"{"ccy":"USDT","side":"borrow","amt":"5"}"#).unwrap();
        assert_eq!(row.ccy, "USDT");
        assert_eq!(row.side, "borrow");
    }
}
