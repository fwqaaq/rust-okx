use serde::Deserialize;

use crate::model::NumberString;

/// Current or historical lead position.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadPosition {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Lead-position ID.
    #[serde(default)]
    pub sub_pos_id: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Opening order ID.
    #[serde(default)]
    pub open_ord_id: String,
    /// Average open price.
    #[serde(default)]
    pub open_avg_px: NumberString,
    /// Opening timestamp.
    #[serde(default)]
    pub open_time: NumberString,
    /// Position quantity.
    #[serde(default)]
    pub sub_pos: NumberString,
    /// Available position quantity.
    #[serde(default)]
    pub avail_sub_pos: NumberString,
    /// Take-profit trigger price.
    #[serde(default)]
    pub tp_trigger_px: NumberString,
    /// Stop-loss trigger price.
    #[serde(default)]
    pub sl_trigger_px: NumberString,
    /// Stop-order algo ID.
    #[serde(default)]
    pub algo_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Take-profit order price.
    #[serde(default)]
    pub tp_ord_px: NumberString,
    /// Stop-loss order price.
    #[serde(default)]
    pub sl_ord_px: NumberString,
    /// Margin.
    #[serde(default)]
    pub margin: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Unrealized profit-and-loss ratio.
    #[serde(default)]
    pub upl_ratio: NumberString,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Lead trader unique code.
    #[serde(default)]
    pub unique_code: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Closing timestamp.
    #[serde(default)]
    pub close_time: NumberString,
    /// Average close price.
    #[serde(default)]
    pub close_avg_px: NumberString,
    /// Realized profit and loss.
    #[serde(default)]
    pub pnl: NumberString,
    /// Realized profit-and-loss ratio.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Quantity already closed.
    #[serde(default)]
    pub close_sub_pos: NumberString,
    /// Partial or full close type.
    #[serde(default)]
    pub r#type: String,
}

/// Result of placing a lead stop order or closing a lead position.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadPositionAction {
    /// Lead-position ID.
    #[serde(default)]
    pub sub_pos_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
}

/// Lead instrument and whether it is enabled.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadInstrument {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Whether the instrument is enabled for leading.
    #[serde(default)]
    pub enabled: bool,
}

/// Realized profit-sharing detail.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ProfitSharingDetail {
    /// Profit-sharing currency.
    #[serde(default)]
    pub ccy: String,
    /// Profit-sharing amount.
    #[serde(default)]
    pub profit_sharing_amt: NumberString,
    /// Copy trader nickname.
    #[serde(default)]
    pub nick_name: String,
    /// Profit-sharing ID.
    #[serde(default)]
    pub profit_sharing_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Portrait link.
    #[serde(default)]
    pub port_link: String,
    /// Profit-sharing timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Total realized profit sharing.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TotalProfitSharing {
    /// Profit-sharing currency.
    #[serde(default)]
    pub ccy: String,
    /// Total profit-sharing amount.
    #[serde(default)]
    pub total_profit_sharing_amt: NumberString,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
}

/// Unrealized profit-sharing detail.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UnrealizedProfitSharingDetail {
    /// Profit-sharing currency.
    #[serde(default)]
    pub ccy: String,
    /// Unrealized profit-sharing amount.
    #[serde(default)]
    pub unrealized_profit_sharing_amt: NumberString,
    /// Copy trader nickname.
    #[serde(default)]
    pub nick_name: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Portrait link.
    #[serde(default)]
    pub port_link: String,
    /// Update timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Total unrealized profit sharing.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TotalUnrealizedProfitSharing {
    /// Settlement timestamp.
    #[serde(default)]
    pub profit_sharing_ts: NumberString,
    /// Total unrealized profit-sharing amount.
    #[serde(default)]
    pub total_unrealized_profit_sharing_amt: NumberString,
}

/// Boolean result returned by settings operations.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopySettingResult {
    /// Whether the setting succeeded.
    #[serde(default)]
    pub result: bool,
}

/// Per-instrument-type account configuration.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyAccountDetail {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Account role type.
    #[serde(default)]
    pub role_type: String,
    /// Profit-sharing ratio.
    #[serde(default)]
    pub profit_sharing_ratio: NumberString,
    /// Maximum copy trader count.
    #[serde(default)]
    pub max_copy_trader_num: NumberString,
    /// Current copy trader count.
    #[serde(default)]
    pub copy_trader_num: NumberString,
}

/// Copy Trading account configuration.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyAccountConfig {
    /// User unique code.
    #[serde(default)]
    pub unique_code: String,
    /// Nickname.
    #[serde(default)]
    pub nick_name: String,
    /// Portrait link.
    #[serde(default)]
    pub port_link: String,
    /// Configuration by instrument type.
    #[serde(default)]
    pub details: Vec<CopyAccountDetail>,
}

/// One instrument included in copy settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyInstrumentSetting {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Whether copying is enabled (`0` or `1`).
    #[serde(default)]
    pub enabled: String,
}

/// Current settings for copying a lead trader.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopySettings {
    /// Copy sizing mode.
    #[serde(default)]
    pub copy_mode: String,
    /// Fixed amount per copied order.
    #[serde(default)]
    pub copy_amt: NumberString,
    /// Copy ratio per order.
    #[serde(default)]
    pub copy_ratio: NumberString,
    /// Maximum total copy amount.
    #[serde(default)]
    pub copy_total_amt: NumberString,
    /// Take-profit ratio per order.
    #[serde(default)]
    pub tp_ratio: NumberString,
    /// Stop-loss ratio per order.
    #[serde(default)]
    pub sl_ratio: NumberString,
    /// Copy instrument selection mode.
    #[serde(default)]
    pub copy_inst_id_type: String,
    /// Lead instruments and their enabled state.
    #[serde(default)]
    pub inst_ids: Vec<CopyInstrumentSetting>,
    /// Total stop-loss amount.
    #[serde(default)]
    pub sl_total_amt: NumberString,
    /// Action for open positions when copying stops.
    #[serde(default)]
    pub sub_pos_close_type: String,
    /// Copy margin mode.
    #[serde(default)]
    pub copy_mgn_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Current copy state.
    #[serde(default)]
    pub copy_state: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
}

/// Lead trader currently copied by the account.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CurrentLeadTrader {
    /// Portrait link.
    #[serde(default)]
    pub port_link: String,
    /// Nickname.
    #[serde(default)]
    pub nick_name: String,
    /// Margin allocated to copying.
    #[serde(default)]
    pub margin: NumberString,
    /// Maximum total copy amount.
    #[serde(default)]
    pub copy_total_amt: NumberString,
    /// Total copy profit and loss.
    #[serde(default)]
    pub copy_total_pnl: NumberString,
    /// Lead trader unique code.
    #[serde(default)]
    pub unique_code: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Profit-sharing ratio.
    #[serde(default)]
    pub profit_sharing_ratio: NumberString,
    /// Timestamp when copying began.
    #[serde(default)]
    pub begin_copy_time: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Today's profit and loss.
    #[serde(default)]
    pub today_pnl: NumberString,
    /// Public or private lead mode.
    #[serde(default)]
    pub lead_mode: String,
}

/// Public limits for Copy Trading settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyTradingConfig {
    /// Maximum fixed amount per copied order.
    #[serde(default)]
    pub max_copy_amt: NumberString,
    /// Minimum fixed amount per copied order.
    #[serde(default)]
    pub min_copy_amt: NumberString,
    /// Maximum total copy amount.
    #[serde(default)]
    pub max_copy_total_amt: NumberString,
    /// Minimum copy ratio.
    #[serde(default)]
    pub min_copy_ratio: NumberString,
    /// Maximum copy ratio.
    #[serde(default)]
    pub max_copy_ratio: NumberString,
    /// Maximum take-profit ratio.
    #[serde(default)]
    pub max_tp_ratio: NumberString,
    /// Maximum stop-loss ratio.
    #[serde(default)]
    pub max_sl_ratio: NumberString,
}

/// One point in a lead trader's profit-and-loss ratio series.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PnlRatioPoint {
    /// Period start timestamp.
    #[serde(default)]
    pub begin_ts: NumberString,
    /// Profit-and-loss ratio.
    #[serde(default)]
    pub pnl_ratio: NumberString,
}

/// Public lead-trader rank entry.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadTraderRank {
    /// Assets under management.
    #[serde(default)]
    pub aum: NumberString,
    /// Current copy state.
    #[serde(default)]
    pub copy_state: String,
    /// Maximum copy trader count.
    #[serde(default)]
    pub max_copy_trader_num: NumberString,
    /// Current copy trader count.
    #[serde(default)]
    pub copy_trader_num: NumberString,
    /// Accumulated copy trader count.
    #[serde(default)]
    pub acc_copy_trader_num: NumberString,
    /// Portrait link.
    #[serde(default)]
    pub port_link: String,
    /// Nickname.
    #[serde(default)]
    pub nick_name: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Lead trader unique code.
    #[serde(default)]
    pub unique_code: String,
    /// Win ratio.
    #[serde(default)]
    pub win_ratio: NumberString,
    /// Number of lead days.
    #[serde(default)]
    pub lead_days: NumberString,
    /// Instruments currently led.
    #[serde(default)]
    pub trader_insts: Vec<String>,
    /// Profit and loss over the ranking window.
    #[serde(default)]
    pub pnl: NumberString,
    /// Profit-and-loss ratio over the ranking window.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Profit-and-loss ratio time series.
    #[serde(default)]
    pub pnl_ratios: Vec<PnlRatioPoint>,
}

/// Paginated public lead-trader ranks.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadTraderRanks {
    /// Data version.
    #[serde(default)]
    pub data_ver: String,
    /// Total page count.
    #[serde(default)]
    pub total_page: NumberString,
    /// Ranked lead traders.
    #[serde(default)]
    pub ranks: Vec<LeadTraderRank>,
}

/// Daily or weekly lead-trader performance point.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadTraderPnl {
    /// Period start timestamp.
    #[serde(default)]
    pub begin_ts: NumberString,
    /// Accumulated profit and loss.
    #[serde(default)]
    pub pnl: NumberString,
    /// Accumulated profit-and-loss ratio.
    #[serde(default)]
    pub pnl_ratio: NumberString,
}

/// Aggregate lead-trader statistics.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeadTraderStats {
    /// Win ratio.
    #[serde(default)]
    pub win_ratio: NumberString,
    /// Profitable day count.
    #[serde(default)]
    pub profit_days: NumberString,
    /// Loss day count.
    #[serde(default)]
    pub loss_days: NumberString,
    /// Current copy trader profit and loss.
    #[serde(default)]
    pub cur_copy_trader_pnl: NumberString,
    /// Average lead-position notional.
    #[serde(default)]
    pub avg_sub_pos_notional: NumberString,
    /// Investment amount.
    #[serde(default)]
    pub invest_amt: NumberString,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
}

/// Lead trader currency preference.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CurrencyPreference {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Preference ratio.
    #[serde(default)]
    pub ratio: NumberString,
}

/// One copy trader in the public copy-trader list.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyTrader {
    /// Timestamp when copying began.
    #[serde(default)]
    pub begin_copy_time: NumberString,
    /// Nickname.
    #[serde(default)]
    pub nick_name: String,
    /// Portrait link.
    #[serde(default)]
    pub port_link: String,
    /// Copy-trading profit and loss.
    #[serde(default)]
    pub pnl: NumberString,
}

/// Public summary of a lead trader's copy traders.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyTraders {
    /// Total copy trader profit and loss.
    #[serde(default)]
    pub copy_total_pnl: NumberString,
    /// Profit-and-loss currency.
    #[serde(default)]
    pub ccy: String,
    /// Copy trader count change over seven days.
    #[serde(default)]
    pub copy_trader_num_chg: NumberString,
    /// Copy trader count change ratio over seven days.
    #[serde(default)]
    pub copy_trader_num_chg_ratio: NumberString,
    /// Copy trader entries.
    #[serde(default)]
    pub copy_traders: Vec<CopyTrader>,
}
