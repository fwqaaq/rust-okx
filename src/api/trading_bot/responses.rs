use serde::Deserialize;

use crate::model::NumberString;

/// Result of creating, amending, stopping, or copying a grid algo.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridActionResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Per-order result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-order result message.
    #[serde(default)]
    pub s_msg: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
}

/// Result of amending core grid parameters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridAmendBasicResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Required top-up amount.
    #[serde(default)]
    pub required_topup_amount: NumberString,
    /// Maximum allowed top-up amount for contract grid.
    #[serde(default)]
    pub max_topup_amount: NumberString,
}

/// Result of closing or canceling a contract-grid close order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridCloseResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Close-position order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
}

/// Result of instantly triggering a grid algo.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridInstantTriggerResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
}

/// Rebate transfer attached to a grid algo.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridRebateTransfer {
    /// Rebate amount.
    #[serde(default)]
    pub rebate: NumberString,
    /// Rebate currency.
    #[serde(default)]
    pub rebate_ccy: String,
}

/// Trigger settings and observed trigger state.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridTrigger {
    /// Trigger action.
    #[serde(default)]
    pub trigger_action: String,
    /// Trigger strategy.
    #[serde(default)]
    pub trigger_strategy: String,
    /// Delay after the action is triggered.
    #[serde(default)]
    pub delay_seconds: NumberString,
    /// Actual trigger timestamp.
    #[serde(default)]
    pub trigger_time: NumberString,
    /// Trigger type.
    #[serde(default)]
    pub trigger_type: String,
    /// Candlestick timeframe.
    #[serde(default)]
    pub timeframe: String,
    /// RSI threshold.
    #[serde(default)]
    pub thold: NumberString,
    /// RSI trigger condition.
    #[serde(default)]
    pub trigger_cond: String,
    /// RSI time period.
    #[serde(default)]
    pub time_period: NumberString,
    /// Trigger price.
    #[serde(default)]
    pub trigger_px: NumberString,
    /// Stop type.
    #[serde(default)]
    pub stop_type: String,
}

/// Grid algo order details.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridAlgoOrder {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Grid algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Algo state.
    #[serde(default)]
    pub state: String,
    /// Rebate transfers.
    #[serde(default)]
    pub rebate_trans: Vec<GridRebateTransfer>,
    /// Trigger configurations.
    #[serde(default)]
    pub trigger_params: Vec<GridTrigger>,
    /// Upper grid price.
    #[serde(default)]
    pub max_px: NumberString,
    /// Lower grid price.
    #[serde(default)]
    pub min_px: NumberString,
    /// Number of grid levels.
    #[serde(default)]
    pub grid_num: NumberString,
    /// Grid running mode.
    #[serde(default)]
    pub run_type: String,
    /// Take-profit trigger price.
    #[serde(default)]
    pub tp_trigger_px: NumberString,
    /// Stop-loss trigger price.
    #[serde(default)]
    pub sl_trigger_px: NumberString,
    /// Number of trades executed.
    #[serde(default)]
    pub trade_num: NumberString,
    /// Number of arbitrages executed.
    #[serde(default)]
    pub arbitrage_num: NumberString,
    /// Amount per grid.
    #[serde(default)]
    pub single_amt: NumberString,
    /// Estimated minimum profit rate per grid.
    #[serde(default)]
    pub per_min_profit_rate: NumberString,
    /// Estimated maximum profit rate per grid.
    #[serde(default)]
    pub per_max_profit_rate: NumberString,
    /// Launch price.
    #[serde(default)]
    pub run_px: NumberString,
    /// Total profit and loss.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Profit and loss ratio.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Total investment.
    #[serde(default)]
    pub investment: NumberString,
    /// Grid profit.
    #[serde(default)]
    pub grid_profit: NumberString,
    /// Floating profit.
    #[serde(default)]
    pub float_profit: NumberString,
    /// Total annualized rate.
    #[serde(default)]
    pub total_annualized_rate: NumberString,
    /// Grid annualized rate.
    #[serde(default)]
    pub annualized_rate: NumberString,
    /// Cancellation type.
    #[serde(default)]
    pub cancel_type: String,
    /// Stop type.
    #[serde(default)]
    pub stop_type: String,
    /// Number of active sub-orders.
    #[serde(default)]
    pub active_ord_num: NumberString,
    /// Initial spot quote-currency investment.
    #[serde(default)]
    pub quote_sz: NumberString,
    /// Initial spot base-currency investment.
    #[serde(default)]
    pub base_sz: NumberString,
    /// Current spot quote-currency amount.
    #[serde(default)]
    pub cur_quote_sz: NumberString,
    /// Current spot base-currency amount.
    #[serde(default)]
    pub cur_base_sz: NumberString,
    /// Withdrawable profit.
    #[serde(default)]
    pub profit: NumberString,
    /// Stop result.
    #[serde(default)]
    pub stop_result: String,
    /// Contract-grid direction.
    #[serde(default)]
    pub direction: String,
    /// Whether a base position was opened.
    #[serde(default)]
    pub base_pos: bool,
    /// Contract-grid investment.
    #[serde(default)]
    pub sz: NumberString,
    /// Configured leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Actual leverage.
    #[serde(default)]
    pub actual_lever: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Frozen order margin.
    #[serde(default)]
    pub ord_frozen: NumberString,
    /// Available equity.
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Equity.
    #[serde(default)]
    pub eq: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Profit sharing ratio.
    #[serde(default)]
    pub profit_sharing_ratio: NumberString,
    /// Copy order type.
    #[serde(default)]
    pub copy_type: String,
    /// Take-profit ratio.
    #[serde(default)]
    pub tp_ratio: NumberString,
    /// Stop-loss ratio.
    #[serde(default)]
    pub sl_ratio: NumberString,
    /// Accumulated fee.
    #[serde(default)]
    pub fee: NumberString,
    /// Accumulated fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Accumulated funding fee.
    #[serde(default)]
    pub funding_fee: NumberString,
    /// Spot trading quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
}

/// Grid sub-order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridSubOrder {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Grid algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Sub-order group ID.
    #[serde(default)]
    pub group_id: String,
    /// Sub-order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Trade mode.
    #[serde(default)]
    pub td_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Order type.
    #[serde(default)]
    pub ord_type: String,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Order side.
    #[serde(default)]
    pub side: String,
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Fee.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Rebate.
    #[serde(default)]
    pub rebate: NumberString,
    /// Rebate currency.
    #[serde(default)]
    pub rebate_ccy: String,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Profit and loss.
    #[serde(default)]
    pub pnl: NumberString,
    /// Contract value.
    #[serde(default)]
    pub ct_val: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
}

/// Contract-grid position.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridPosition {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Average open price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity.
    #[serde(default)]
    pub pos: NumberString,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Maintenance margin ratio.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Unrealized profit and loss ratio.
    #[serde(default)]
    pub upl_ratio: NumberString,
    /// Latest traded price.
    #[serde(default)]
    pub last: NumberString,
    /// Position notional in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Auto-deleveraging rank.
    #[serde(default)]
    pub adl: NumberString,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
}

/// Result of withdrawing spot-grid income.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridWithdrawIncome {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Withdrawn profit.
    #[serde(default)]
    pub profit: NumberString,
}

/// Computed contract-grid margin adjustment.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridMarginComputation {
    /// Maximum adjustable amount.
    #[serde(default)]
    pub max_amt: NumberString,
    /// Leverage after adjustment.
    #[serde(default)]
    pub lever: NumberString,
}

/// Minimum investment amount in one currency.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridInvestmentAmount {
    /// Investment amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Investment currency.
    #[serde(default)]
    pub ccy: String,
}

/// Public Grid AI parameters.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridAiParameter {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Grid algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Backtest duration.
    #[serde(default)]
    pub duration: String,
    /// Number of grid levels.
    #[serde(default)]
    pub grid_num: NumberString,
    /// Upper grid price.
    #[serde(default)]
    pub max_px: NumberString,
    /// Lower grid price.
    #[serde(default)]
    pub min_px: NumberString,
    /// Estimated maximum profit rate per grid.
    #[serde(default)]
    pub per_max_profit_rate: NumberString,
    /// Estimated minimum profit rate per grid.
    #[serde(default)]
    pub per_min_profit_rate: NumberString,
    /// Per-grid profit ratio.
    #[serde(default)]
    pub per_grid_profit_ratio: NumberString,
    /// Grid annualized rate.
    #[serde(default)]
    pub annualized_rate: NumberString,
    /// Minimum investment.
    #[serde(default)]
    pub min_investment: NumberString,
    /// Investment currency.
    #[serde(default)]
    pub ccy: String,
    /// Grid running mode.
    #[serde(default)]
    pub run_type: String,
    /// Contract-grid direction.
    #[serde(default)]
    pub direction: String,
    /// Contract-grid leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Source currency.
    #[serde(default)]
    pub source_ccy: String,
}

/// Minimum grid investment computation.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridMinInvestment {
    /// Per-currency minimum investments.
    #[serde(default)]
    pub min_investment_data: Vec<GridInvestmentAmount>,
    /// Amount allocated to one grid level.
    #[serde(default)]
    pub single_amt: NumberString,
}

/// Public RSI grid backtest result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridRsiBackTesting {
    /// Number of triggers in the backtest.
    #[serde(default)]
    pub trigger_num: NumberString,
}

/// Maximum supported grid quantity.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridQuantity {
    /// Maximum number of grid levels.
    #[serde(default)]
    pub max_grid_qty: NumberString,
}

/// Result of adding grid investment or adjusting margin.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridAlgoResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
}
