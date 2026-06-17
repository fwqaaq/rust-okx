use crate::model::NumberString;
use serde::Deserialize;

/// Savings balance row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SavingBalance {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `loanAmt` field.
    #[serde(default)]
    pub loan_amt: NumberString,
    /// Value returned by OKX in the `pendingAmt` field.
    #[serde(default)]
    pub pending_amt: NumberString,
    /// Value returned by OKX in the `earnings` field.
    #[serde(default)]
    pub earnings: NumberString,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
}

/// Savings purchase/redemption result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SavingsPurchaseRedemptionResult {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `side` field.
    #[serde(default)]
    pub side: String,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
}

/// Set-lending-rate result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetLendingRateResult {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
}

/// Lending history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LendingHistory {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `earnings` field.
    #[serde(default)]
    pub earnings: NumberString,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Public borrow-history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicBorrowHistory {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Public borrow-info row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicBorrowInfo {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `avgAmt` field.
    #[serde(default)]
    pub avg_amt: NumberString,
    /// Value returned by OKX in the `avgRate` field.
    #[serde(default)]
    pub avg_rate: NumberString,
    /// Value returned by OKX in the `preRate` field.
    #[serde(default)]
    pub pre_rate: NumberString,
}
