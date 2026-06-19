use serde::Deserialize;

use crate::model::NumberString;

/// Per-currency loan record within an account risk-state response.
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

/// Account risk state (loan allocation summary).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RiskState {
    /// Total debt.
    #[serde(default)]
    pub debt: NumberString,
    /// Total accrued interest.
    #[serde(default)]
    pub interest: NumberString,
    /// Loan allocation string.
    #[serde(default)]
    pub loan_alloc: String,
    /// Timestamp of next discount event (Unix milliseconds).
    #[serde(default)]
    pub next_discount_time: NumberString,
    /// Timestamp of next interest accrual (Unix milliseconds).
    #[serde(default)]
    pub next_interest_time: NumberString,
    /// Per-currency loan records.
    #[serde(default)]
    pub records: Vec<LoanRecord>,
}

/// Simulated margin calculation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SimulatedMargin {
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Margin ratio.
    #[serde(default)]
    pub mr: NumberString,
    /// Notional value in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Per-instrument details returned by OKX.
    #[serde(default)]
    pub details: Vec<SimulatedMarginDetail>,
}

/// Per-instrument detail in a simulated margin response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SimulatedMarginDetail {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position size.
    #[serde(default)]
    pub pos: NumberString,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Unrealized PnL.
    #[serde(default)]
    pub upl: NumberString,
}

/// Account greeks row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Greek {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Black-Scholes delta.
    #[serde(rename = "deltaBS", default)]
    pub delta_bs: NumberString,
    /// Portfolio-adjusted delta.
    #[serde(rename = "deltaPA", default)]
    pub delta_pa: NumberString,
    /// Black-Scholes gamma.
    #[serde(rename = "gammaBS", default)]
    pub gamma_bs: NumberString,
    /// Black-Scholes theta.
    #[serde(rename = "thetaBS", default)]
    pub theta_bs: NumberString,
    /// Black-Scholes vega.
    #[serde(rename = "vegaBS", default)]
    pub vega_bs: NumberString,
}

/// Account position-tier row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountPositionTier {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Position type.
    #[serde(default)]
    pub pos_type: String,
    /// Minimum size in the tier.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Maximum size in the tier.
    #[serde(default)]
    pub max_sz: NumberString,
}

/// Position-builder result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderResult {
    /// Account level used for the calculation.
    #[serde(default)]
    pub acct_lv: String,
    /// Adjusted / effective equity.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Margin ratio.
    #[serde(default)]
    pub mr: NumberString,
    /// Simulated or real position data.
    #[serde(default)]
    pub pos_data: Vec<PositionBuilderPosition>,
    /// Simulated or real asset data.
    #[serde(default)]
    pub asset_data: Vec<PositionBuilderAsset>,
}

/// Position row returned by position builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderPosition {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position size.
    #[serde(default)]
    pub pos: NumberString,
    /// Average price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Unrealized PnL.
    #[serde(default)]
    pub upl: NumberString,
}

/// Asset row returned by position builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderAsset {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Equity.
    #[serde(default)]
    pub eq: NumberString,
}
