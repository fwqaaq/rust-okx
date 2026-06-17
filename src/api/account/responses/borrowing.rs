use serde::Deserialize;

use crate::model::NumberString;

/// Maximum loan amount available for an instrument or currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxLoan {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub mgn_ccy: String,
    /// Maximum loan amount.
    #[serde(default)]
    pub max_loan: NumberString,
}

/// Interest accrued by account borrowing.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestAccrued {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Accrued interest.
    #[serde(default)]
    pub interest: NumberString,
    /// Interest rate.
    #[serde(default)]
    pub interest_rate: NumberString,
    /// Liability.
    #[serde(default)]
    pub liab: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Account borrowing interest rate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestRate {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Interest rate.
    #[serde(default)]
    pub interest_rate: NumberString,
}

/// Result of a borrow/repay request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BorrowRepayResult {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Borrow or repay side.
    #[serde(default)]
    pub side: String,
    /// Requested amount.
    #[serde(default)]
    pub amt: NumberString,
    /// OKX borrow/repay order ID.
    #[serde(default)]
    pub ord_id: String,
}

/// Borrow/repay history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BorrowRepayHistory {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Borrow or repay side.
    #[serde(default)]
    pub side: String,
    /// Amount.
    #[serde(default)]
    pub amt: NumberString,
    /// OKX borrow/repay order ID.
    #[serde(default)]
    pub ord_id: String,
    /// OKX state value.
    #[serde(default)]
    pub state: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Borrowing interest limit information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestLimit {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Interest rate.
    #[serde(default)]
    pub rate: NumberString,
    /// Loan quota.
    #[serde(default)]
    pub loan_quota: NumberString,
    /// Used loan quota.
    #[serde(default)]
    pub used_loan: NumberString,
}
