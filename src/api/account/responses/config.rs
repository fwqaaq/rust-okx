use serde::Deserialize;

use crate::model::{InstType, NumberString, PositionSide, TradeMode};

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
    /// API key permissions, comma-separated (e.g. `"read_only,trade,withdraw"`).
    #[serde(default)]
    pub perm: String,
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

/// Estimated impact and limits for a leverage adjustment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AdjustLeverageInfo {
    /// Estimated margin in quote currency that can be transferred out.
    #[serde(default)]
    pub est_avail_quote_trans: NumberString,
    /// Estimated margin that can be transferred out.
    #[serde(default)]
    pub est_avail_trans: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub est_liq_px: NumberString,
    /// Estimated maximum amount / contract quantity.
    #[serde(default)]
    pub est_max_amt: NumberString,
    /// Estimated margin needed by the position.
    #[serde(default)]
    pub est_mgn: NumberString,
    /// Estimated margin in quote currency needed by the position.
    #[serde(default)]
    pub est_quote_mgn: NumberString,
    /// Estimated maximum quote-currency loan amount for margin.
    #[serde(default)]
    pub est_quote_max_amt: NumberString,
    /// Whether pending orders exist.
    #[serde(default)]
    pub exist_ord: bool,
    /// Maximum leverage.
    #[serde(default)]
    pub max_lever: NumberString,
    /// Minimum leverage.
    #[serde(default)]
    pub min_lever: NumberString,
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

/// Result of updating collateral assets.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetCollateralAssetsResult {
    /// OKX collateral-assets update type.
    #[serde(rename = "type", default)]
    pub collateral_type: String,
    /// Currency list included in the update.
    #[serde(default)]
    pub ccy_list: Vec<String>,
    /// Whether assets are enabled as collateral.
    #[serde(default)]
    pub collateral_enabled: bool,
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
