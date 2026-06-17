use serde::Deserialize;

use super::balances::BalanceDetail;

use crate::model::{InstType, NumberString, PositionSide, TradeMode};

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
