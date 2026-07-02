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

/// Borrowing interest and limit information, as returned by
/// `GET /api/v5/account/interest-limits`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestLimit {
    /// Current debt in USD.
    #[serde(default)]
    pub debt: NumberString,
    /// Current interest in USD. Only applicable to Market loans.
    #[serde(default)]
    pub interest: NumberString,
    /// Next deduct time (Unix milliseconds).
    #[serde(default)]
    pub next_discount_time: NumberString,
    /// Next accrual time (Unix milliseconds).
    #[serde(default)]
    pub next_interest_time: NumberString,
    /// VIP loan allocation for the current trading account (percent).
    #[serde(default)]
    pub loan_alloc: String,
    /// Per-currency loan records.
    #[serde(default)]
    pub records: Vec<LoanRecord>,
}

/// Per-currency loan record nested in [`InterestLimit`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoanRecord {
    /// Available loan amount.
    #[serde(default)]
    pub avail_loan: String,
    /// Average borrow rate.
    #[serde(default)]
    pub avg_rate: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Accrued interest.
    #[serde(default)]
    pub interest: NumberString,
    /// Total loan quota.
    #[serde(default)]
    pub loan_quota: NumberString,
    /// Position loan.
    #[serde(default)]
    pub pos_loan: String,
    /// Current borrow rate.
    #[serde(default)]
    pub rate: NumberString,
    /// Remaining loan limit.
    #[serde(default)]
    pub surplus_lmt: NumberString,
    /// Used loan limit.
    #[serde(default)]
    pub used_lmt: NumberString,
    /// Used loan amount.
    #[serde(default)]
    pub used_loan: String,
    /// Interest-free liability.
    #[serde(default)]
    pub interest_free_liab: String,
    /// Potential borrowing amount.
    #[serde(default)]
    pub potential_borrowing_amt: String,
}
