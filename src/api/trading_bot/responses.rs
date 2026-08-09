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

/// Result of placing a recurring-buy algo order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringCreateResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
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

/// Result of amending or stopping a recurring-buy algo order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringActionResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Per-order result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-order result message.
    #[serde(default)]
    pub s_msg: String,
}

/// Result of a recurring-buy maintenance operation.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringOperationResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Event result code.
    #[serde(default)]
    pub s_code: String,
    /// Event result message.
    #[serde(default)]
    pub s_msg: String,
}

/// One currency allocation and its recurring-buy performance.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringCurrency {
    /// Recurring-buy currency.
    #[serde(default)]
    pub ccy: String,
    /// Allocation ratio.
    #[serde(default)]
    pub ratio: NumberString,
    /// Minimum purchase price.
    #[serde(default)]
    pub min_px: NumberString,
    /// Maximum purchase price.
    #[serde(default)]
    pub max_px: NumberString,
    /// Accumulated amount in the recurring-buy currency.
    #[serde(default)]
    pub total_amt: NumberString,
    /// Profit in the investment currency.
    #[serde(default)]
    pub profit: NumberString,
    /// Average purchase price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Current market price.
    #[serde(default)]
    pub px: NumberString,
}

/// Recurring-buy algo order details.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringOrder {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Algo order state.
    #[serde(default)]
    pub state: String,
    /// Custom strategy name.
    #[serde(default)]
    pub stgy_name: String,
    /// Currency allocations and performance.
    #[serde(default)]
    pub recurring_list: Vec<RecurringCurrency>,
    /// Investment period.
    #[serde(default)]
    pub period: String,
    /// Monthly date or weekly day.
    #[serde(default)]
    pub recurring_day: NumberString,
    /// Hourly recurrence interval.
    #[serde(default)]
    pub recurring_hour: NumberString,
    /// Hour of day for the investment.
    #[serde(default)]
    pub recurring_time: NumberString,
    /// UTC time zone.
    #[serde(default)]
    pub time_zone: NumberString,
    /// Amount invested per cycle.
    #[serde(default)]
    pub amt: NumberString,
    /// Accumulated invested amount.
    #[serde(default)]
    pub investment_amt: NumberString,
    /// Investment currency.
    #[serde(default)]
    pub investment_ccy: String,
    /// Next investment timestamp.
    #[serde(default)]
    pub next_invest_time: NumberString,
    /// Total profit and loss.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Total annualized rate.
    #[serde(default)]
    pub total_ann_rate: NumberString,
    /// Profit and loss ratio.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Current market value.
    #[serde(default)]
    pub mkt_cap: NumberString,
    /// Number of completed cycles.
    #[serde(default)]
    pub cycles: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Quote currency used for trading.
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Funding sources.
    #[serde(default)]
    pub source: Vec<String>,
    /// Recurring time type.
    #[serde(default)]
    pub recurring_time_type: String,
    /// Custom recurring-buy minute.
    #[serde(default)]
    pub recurring_time_minutes: NumberString,
}

/// A recurring-buy sub-order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringSubOrder {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
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
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
}

/// Result of creating a signal channel.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalChannelCreated {
    /// Signal channel ID.
    #[serde(default)]
    pub signal_chan_id: String,
    /// Token that identifies the user when signals place orders.
    #[serde(default)]
    pub signal_chan_token: String,
}

/// Signal channel metadata.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalChannel {
    /// Signal channel ID.
    #[serde(default)]
    pub signal_chan_id: String,
    /// Signal channel name.
    #[serde(default)]
    pub signal_chan_name: String,
    /// Signal channel description.
    #[serde(default)]
    pub signal_chan_desc: String,
    /// Token that identifies the user when signals place orders.
    #[serde(default)]
    pub signal_chan_token: String,
    /// Signal source type.
    #[serde(default)]
    pub signal_source_type: String,
}

/// Result of creating or stopping a signal bot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalActionResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Per-order result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-order result message.
    #[serde(default)]
    pub s_msg: String,
}

/// Result of a signal-bot operation that returns only an algo ID.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalAlgoResult {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
}

/// Entry settings returned for a signal bot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalEntrySetting {
    /// Whether repeated entries in the same direction are allowed.
    #[serde(default)]
    pub allow_multiple_entry: bool,
    /// Entry sizing type.
    #[serde(default)]
    pub entry_type: String,
    /// Fixed margin or contract amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Percentage amount per order.
    #[serde(default)]
    pub ratio: NumberString,
}

/// Exit settings returned for a signal bot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalExitSetting {
    /// Take-profit/stop-loss calculation type.
    #[serde(default)]
    pub tp_sl_type: String,
    /// Take-profit percentage.
    #[serde(default)]
    pub tp_pct: NumberString,
    /// Stop-loss percentage.
    #[serde(default)]
    pub sl_pct: NumberString,
}

/// Signal-bot algo order details.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalAlgoOrder {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument IDs.
    #[serde(default)]
    pub inst_ids: Vec<String>,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Algo order state.
    #[serde(default)]
    pub state: String,
    /// Stop reason.
    #[serde(default)]
    pub cancel_type: String,
    /// Total profit and loss.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Total profit and loss ratio.
    #[serde(default)]
    pub total_pnl_ratio: NumberString,
    /// Total strategy-account equity.
    #[serde(default)]
    pub total_eq: NumberString,
    /// Floating profit and loss.
    #[serde(default)]
    pub float_pnl: NumberString,
    /// Realized profit and loss.
    #[serde(default)]
    pub realized_pnl: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Investment amount.
    #[serde(default)]
    pub invest_amt: NumberString,
    /// Sub-order type.
    #[serde(default)]
    pub sub_ord_type: String,
    /// Limit-price offset ratio.
    #[serde(default)]
    pub ratio: NumberString,
    /// Entry settings.
    #[serde(default)]
    pub entry_setting_param: SignalEntrySetting,
    /// Exit settings.
    #[serde(default)]
    pub exit_setting_param: SignalExitSetting,
    /// Signal channel ID.
    #[serde(default)]
    pub signal_chan_id: String,
    /// Signal channel name.
    #[serde(default)]
    pub signal_chan_name: String,
    /// Signal source type.
    #[serde(default)]
    pub signal_source_type: String,
}

/// Open position held by a signal bot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalPosition {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
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
    /// Notional value in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Automatic-deleveraging signal level.
    #[serde(default)]
    pub adl: NumberString,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
}

/// Historical position closed by a signal bot.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalPositionHistory {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Position creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Position update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Average open price.
    #[serde(default)]
    pub open_avg_px: NumberString,
    /// Average close price.
    #[serde(default)]
    pub close_avg_px: NumberString,
    /// Profit and loss.
    #[serde(default)]
    pub pnl: NumberString,
    /// Profit and loss ratio.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Position direction.
    #[serde(default)]
    pub direction: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
}

/// Opaque success element for placing a signal sub-order.
///
/// The current OKX parameter table documents no fields inside `data` and its
/// official success example returns an empty array.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalSubOrderPlacement {}

/// Result of canceling a signal sub-order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalCancelSubOrderResult {
    /// Signal sub-order ID.
    #[serde(default)]
    pub signal_ord_id: String,
    /// Per-order result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-order result message.
    #[serde(default)]
    pub s_msg: String,
}

/// Signal-bot sub-order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalSubOrder {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Algo order type.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Sub-order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied sub-order ID.
    #[serde(default)]
    pub cl_ord_id: String,
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
    /// Fee amount.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Average filled price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Accumulated fill quantity.
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

/// Sub-order metadata attached to a signal event.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalTriggeredOrder {
    /// Client-supplied sub-order ID.
    #[serde(default)]
    pub cl_ord_id: String,
}

/// Signal-bot event-history row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SignalEvent {
    /// Alert message.
    #[serde(default)]
    pub alert_msg: String,
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Event type.
    #[serde(default)]
    pub event_type: String,
    /// Event creation timestamp.
    #[serde(default)]
    pub event_ctime: NumberString,
    /// Event update timestamp.
    #[serde(default)]
    pub event_utime: NumberString,
    /// Event processing message.
    #[serde(default)]
    pub event_process_msg: String,
    /// Event status.
    #[serde(default)]
    pub event_status: String,
    /// Triggered sub-order data.
    #[serde(default)]
    pub triggered_ord_data: Vec<SignalTriggeredOrder>,
}
