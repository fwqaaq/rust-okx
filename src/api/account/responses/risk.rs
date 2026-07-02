use std::collections::HashMap;

use serde::Deserialize;

use crate::model::NumberString;

/// Account risk state, as returned by `GET /api/v5/account/risk-state`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RiskState {
    /// Account risk status in auto-borrow mode. `true` if the account is
    /// currently in a specific risk state.
    #[serde(default)]
    pub at_risk: bool,
    /// Derivatives risk unit list.
    #[serde(default)]
    pub at_risk_idx: Vec<String>,
    /// Margin risk unit list.
    #[serde(default)]
    pub at_risk_mgn: Vec<String>,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
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
    /// Underlying. Applicable to `FUTURES`/`SWAP`/`OPTION`.
    #[serde(default)]
    pub uly: String,
    /// Instrument family. Applicable to `FUTURES`/`SWAP`/`OPTION`.
    #[serde(default)]
    pub inst_family: String,
    /// Max number of positions.
    #[serde(default)]
    pub max_sz: NumberString,
    /// Limitation of position type. Only applicable to cross `OPTION` under
    /// portfolio margin mode.
    #[serde(default)]
    pub pos_type: String,
}

/// Result of setting the spot risk offset amount.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetRiskOffsetAmountResult {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// User-defined spot risk offset amount.
    #[serde(rename = "clSpotInUseAmt", default)]
    pub cl_spot_in_use_amt: NumberString,
}

/// Position-builder result, as returned by
/// `POST /api/v5/account/position-builder`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderResult {
    /// Adjusted equity (USD) for the account.
    #[serde(default)]
    pub eq: NumberString,
    /// Total MMR (USD) for the account.
    #[serde(default)]
    pub total_mmr: NumberString,
    /// Total IMR (USD) for the account.
    #[serde(default)]
    pub total_imr: NumberString,
    /// Borrow MMR (USD) for the account.
    #[serde(default)]
    pub borrow_mmr: NumberString,
    /// Derivatives MMR (USD) for the account.
    #[serde(default)]
    pub deriv_mmr: NumberString,
    /// Cross maintenance margin ratio for the account.
    #[serde(default)]
    pub margin_ratio: NumberString,
    /// UPL for the account.
    #[serde(default)]
    pub upl: NumberString,
    /// Leverage of the account.
    #[serde(default)]
    pub acct_lever: NumberString,
    /// Update time for the account, Unix timestamp format in milliseconds,
    /// e.g. `1597026383085`.
    #[serde(default)]
    pub ts: NumberString,
    /// Asset info.
    #[serde(default)]
    pub assets: Vec<PositionBuilderAsset>,
    /// Risk unit info.
    #[serde(default)]
    pub risk_unit_data: Vec<PositionBuilderRiskUnit>,
}

/// Asset info returned by position builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderAsset {
    /// Currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Currency equity.
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Spot in use.
    #[serde(default)]
    pub spot_in_use: NumberString,
    /// Borrowing MMR (USD). (Deprecated)
    #[serde(default)]
    pub borrow_mmr: NumberString,
    /// Borrowing IMR (USD).
    #[serde(default)]
    pub borrow_imr: NumberString,
}

/// Risk unit info returned by position builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderRiskUnit {
    /// Risk unit, e.g. `BTC`.
    #[serde(default)]
    pub risk_unit: String,
    /// Risk unit MMR before volatility (USD). Returns `""` if users don't
    /// pass in `idxVol`.
    #[serde(default)]
    pub mmr_bf: NumberString,
    /// Risk unit MMR (USD).
    #[serde(default)]
    pub mmr: NumberString,
    /// Risk unit IMR before volatility (USD). Returns `""` if users don't
    /// pass in `idxVol`.
    #[serde(default)]
    pub imr_bf: NumberString,
    /// Risk unit IMR (USD).
    #[serde(default)]
    pub imr: NumberString,
    /// Risk unit UPL (USD).
    #[serde(default)]
    pub upl: NumberString,
    /// Stress testing value of spot and volatility (all derivatives, and
    /// spot trading in spot-derivatives risk offset mode).
    #[serde(default)]
    pub mr1: NumberString,
    /// Stress testing value of time value of money (TVM) (for options).
    #[serde(default)]
    pub mr2: NumberString,
    /// Stress testing value of volatility span (for options).
    #[serde(default)]
    pub mr3: NumberString,
    /// Stress testing value of basis (for all derivatives).
    #[serde(default)]
    pub mr4: NumberString,
    /// Stress testing value of interest rate risk (for options).
    #[serde(default)]
    pub mr5: NumberString,
    /// Stress testing value of extremely volatile markets (for all
    /// derivatives, and spot trading in spot-derivatives risk offset mode).
    #[serde(default)]
    pub mr6: NumberString,
    /// Stress testing value of position reduction cost (for all derivatives).
    #[serde(default)]
    pub mr7: NumberString,
    /// Borrowing MMR/IMR.
    #[serde(default)]
    pub mr8: NumberString,
    /// USDT-USDC-USD hedge risk.
    #[serde(default)]
    pub mr9: NumberString,
    /// MR1 scenario analysis.
    #[serde(default)]
    pub mr1_scenarios: Option<PositionBuilderMr1Scenarios>,
}

/// MR1 scenario analysis for a risk unit.
///
/// Each scenario maps a price volatility ratio (in percentage, e.g. `0.01`
/// representing 1%) to the P&L under stress tests, measured in USD.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderMr1Scenarios {
    /// P&L of stress tests under different price volatility ratios when
    /// volatility shocks down.
    #[serde(default)]
    pub vol_shock_down: HashMap<String, NumberString>,
    /// P&L of stress tests under different price volatility ratios when
    /// volatility keeps the same.
    #[serde(default)]
    pub vol_same: HashMap<String, NumberString>,
    /// P&L of stress tests under different price volatility ratios when
    /// volatility shocks up.
    #[serde(default)]
    pub vol_shock_up: HashMap<String, NumberString>,
}
