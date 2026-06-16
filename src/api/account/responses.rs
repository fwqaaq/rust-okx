use serde::Deserialize;

use crate::model::{InstType, NumberString, PositionSide, RestRow, TradeMode};

/// VIP interest-accrued row.
pub type VipInterestAccrued = RestRow;

/// VIP interest-deducted row.
pub type VipInterestDeducted = RestRow;

/// VIP-loan order row.
pub type VipLoanOrder = RestRow;

/// Fixed-loan borrowing limit row.
pub type FixedLoanBorrowingLimit = RestRow;

/// Fixed-loan borrowing quote row.
pub type FixedLoanBorrowingQuote = RestRow;

/// Fixed-loan borrowing order row.
pub type FixedLoanBorrowingOrder = RestRow;

/// Spot borrow/repay mutation result.
pub type SpotBorrowRepayResult = RestRow;

/// Auto-repay setting result.
pub type SetAutoRepayResult = RestRow;

/// Spot borrow/repay history row.
pub type SpotBorrowRepayHistory = RestRow;

/// Auto-earn setting result.
pub type SetAutoEarnResult = RestRow;

/// The trading-account balance summary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBalance {
    /// Total equity in USD.
    #[serde(default)]
    pub total_eq: NumberString,
    /// Adjusted / effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Per-currency balance details.
    #[serde(default)]
    pub details: Vec<BalanceDetail>,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Balance details for a single currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceDetail {
    /// Currency, e.g. `USDT`.
    pub ccy: String,
    /// Equity of the currency.
    #[serde(default)]
    pub eq: NumberString,
    /// Cash balance.
    #[serde(default)]
    pub cash_bal: NumberString,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
}

/// An open position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Position {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Position side.
    pub pos_side: PositionSide,
    /// Margin mode.
    pub mgn_mode: TradeMode,
    /// Quantity of positions.
    #[serde(default)]
    pub pos: NumberString,
    /// Average open price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
}

/// Account position-risk snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionRisk {
    /// Adjusted/effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Balance data included in the risk snapshot.
    #[serde(default)]
    pub bal_data: Vec<BalanceDetail>,
    /// Position data included in the risk snapshot.
    #[serde(default)]
    pub pos_data: Vec<Position>,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Account configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountConfig {
    /// Account ID.
    #[serde(default)]
    pub uid: String,
    /// Account level.
    #[serde(default)]
    pub acct_lv: String,
    /// Position mode.
    #[serde(default)]
    pub pos_mode: String,
    /// Greeks display type.
    #[serde(default)]
    pub greeks_type: String,
    /// Whether auto-borrow is enabled. OKX returns this as a JSON boolean.
    #[serde(default)]
    pub auto_loan: bool,
}

/// Account bill row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBill {
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Bill type.
    #[serde(rename = "type", default)]
    pub bill_type: String,
    /// Bill subtype.
    #[serde(default)]
    pub sub_type: String,
    /// Balance change.
    #[serde(default)]
    pub sz: NumberString,
    /// Balance after the change.
    #[serde(default)]
    pub bal: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of setting position mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetPositionModeResult {
    /// Position mode.
    #[serde(default)]
    pub pos_mode: String,
}

/// Leverage information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeverageInfo {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode.
    pub mgn_mode: TradeMode,
    /// Position side.
    pub pos_side: PositionSide,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
}

/// Maximum order size information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxOrderSize {
    /// Instrument ID.
    pub inst_id: String,
    /// Maximum buy size.
    #[serde(default)]
    pub max_buy: NumberString,
    /// Maximum sell size.
    #[serde(default)]
    pub max_sell: NumberString,
}

/// Maximum available size information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxAvailableSize {
    /// Instrument ID.
    pub inst_id: String,
    /// Available buy size.
    #[serde(default)]
    pub avail_buy: NumberString,
    /// Available sell size.
    #[serde(default)]
    pub avail_sell: NumberString,
}

/// Trade fee-rate information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FeeRate {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Fee category.
    #[serde(default)]
    pub category: String,
    /// Maker fee rate.
    #[serde(default)]
    pub maker: NumberString,
    /// Taker fee rate.
    #[serde(default)]
    pub taker: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Maximum withdrawal amount for a currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxWithdrawal {
    /// Currency.
    pub ccy: String,
    /// Maximum withdrawal amount.
    #[serde(default)]
    pub max_wd: NumberString,
    /// Maximum withdrawal amount excluding borrowed amount.
    #[serde(default)]
    pub max_wd_ex: NumberString,
}

/// Historical position row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionHistory {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Margin mode.
    pub mgn_mode: TradeMode,
    /// Close type.
    #[serde(rename = "type", default)]
    pub close_type: String,
    /// Realized PnL.
    #[serde(default)]
    pub realized_pnl: NumberString,
    /// Created time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Updated time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Account risk state.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RiskState {
    /// Whether the account is currently at risk, as represented by OKX.
    #[serde(default)]
    pub at_risk: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of adding or reducing margin on a position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AdjustMarginResult {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Adjustment amount.
    #[serde(default)]
    pub amt: NumberString,
    /// OKX adjustment type.
    #[serde(rename = "type", default)]
    pub adjustment_type: String,
}

/// Account-level instrument configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountInstrument {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Base currency.
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency.
    #[serde(default)]
    pub quote_ccy: String,
    /// Settlement currency.
    #[serde(default)]
    pub settle_ccy: String,
}

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

/// Result of updating the greeks display type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetGreeksResult {
    /// Greeks display type.
    #[serde(default)]
    pub greeks_type: String,
}

/// Result of updating isolated margin mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetIsolatedModeResult {
    /// Isolated margin mode.
    #[serde(default)]
    pub iso_mode: String,
    /// OKX isolated-mode scope type.
    #[serde(rename = "type", default)]
    pub mode_type: String,
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

/// Result of updating risk offset type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetRiskOffsetTypeResult {
    /// OKX risk offset type.
    #[serde(rename = "type", default)]
    pub risk_offset_type: String,
}

/// Result of updating auto loan.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetAutoLoanResult {
    /// Auto-loan setting as returned by OKX.
    #[serde(default)]
    pub auto_loan: String,
}

/// Result of updating account level.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetAccountLevelResult {
    /// Account level.
    #[serde(default)]
    pub acct_lv: String,
}

/// Result of activating option trading.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ActivateOptionResult {
    /// OKX result marker, when returned.
    #[serde(default)]
    pub result: String,
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
