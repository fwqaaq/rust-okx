use serde::Serialize;

/// Grid-bot trigger configuration.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridTriggerRequest {
    /// Trigger action.
    pub trigger_action: String,
    /// Trigger strategy.
    pub trigger_strategy: String,
    /// Delay after the action is triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_seconds: Option<String>,
    /// Candlestick timeframe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// RSI threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thold: Option<String>,
    /// RSI trigger condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_cond: Option<String>,
    /// RSI time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    /// Trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<String>,
    /// Stop type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_type: Option<String>,
}

/// Request to place a grid algo order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridOrderRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Upper grid price.
    pub max_px: String,
    /// Lower grid price.
    pub min_px: String,
    /// Number of grid levels.
    pub grid_num: String,
    /// Grid running mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
    /// Take-profit trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Stop-loss trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Client algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Profit sharing ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_sharing_ratio: Option<String>,
    /// Trigger configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_params: Option<Vec<GridTriggerRequest>>,
    /// Quote-currency investment for spot grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_sz: Option<String>,
    /// Base-currency investment for spot grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_sz: Option<String>,
    /// Quote currency used for spot trading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_quote_ccy: Option<String>,
    /// Margin used by a contract grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    /// Contract-grid direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Contract-grid leverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    /// Whether an existing base position is opened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_pos: Option<bool>,
    /// Take-profit ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ratio: Option<String>,
    /// Stop-loss ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ratio: Option<String>,
}

/// Request to amend core grid price parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridAmendBasicRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// New minimum grid price.
    pub min_px: String,
    /// New maximum grid price.
    pub max_px: String,
    /// New number of grid levels.
    pub grid_num: String,
    /// Optional contract-grid top-up amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topup_amount: Option<String>,
}

/// Request to amend grid stop settings or add spot investment.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridAmendRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Stop-loss trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Take-profit trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take-profit ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ratio: Option<String>,
    /// Stop-loss ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ratio: Option<String>,
    /// Spot-grid top-up amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_up_amt: Option<String>,
    /// Trigger configurations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_params: Option<Vec<GridTriggerRequest>>,
}

/// Request to stop a grid algo order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridStopRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Stop handling type.
    pub stop_type: String,
}

/// Request to close a contract-grid position.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridClosePositionRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Whether to close at market.
    pub mkt_close: bool,
    /// Close size for a limit close.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    /// Close price for a limit close.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
}

/// Request to cancel a contract-grid close order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCancelCloseRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Close-position order ID.
    pub ord_id: String,
}

/// Request to trigger a grid algo immediately.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridInstantTriggerRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Optional spot-grid top-up amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_up_amt: Option<String>,
}

/// Query for active or historical grid algo orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridOrdersRequest {
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Instrument ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Return records earlier than this algo ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this algo ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query for one grid algo order or its positions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridAlgoRequest {
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Algo order ID.
    pub algo_id: String,
}

/// Query for grid sub-orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridSubOrdersRequest {
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Algo order ID.
    pub algo_id: String,
    /// Sub-order type.
    pub r#type: String,
    /// Group ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// Return records earlier than this order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request selecting one grid algo order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridAlgoIdRequest {
    /// Algo order ID.
    pub algo_id: String,
}

/// Request to compute a contract-grid margin adjustment.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridComputeMarginRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Adjustment type.
    pub r#type: String,
    /// Adjustment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
}

/// Request to adjust a contract-grid margin balance.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridMarginBalanceRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Adjustment type.
    pub r#type: String,
    /// Adjustment amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
    /// Adjustment percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<String>,
}

/// Request to add grid investment.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridAdjustInvestmentRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Amount to add.
    pub amt: String,
    /// Whether grid profit may be reinvested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reinvest_profit: Option<String>,
}

/// Query for public Grid AI parameters.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridAiParamRequest {
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Contract-grid direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Backtest duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

/// One currency amount used for investment.
#[derive(Debug, Clone, Serialize)]
pub struct GridInvestmentDataRequest {
    /// Investment amount.
    pub amt: String,
    /// Investment currency.
    pub ccy: String,
}

/// Request to compute minimum grid investment.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridMinInvestmentRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Upper grid price.
    pub max_px: String,
    /// Lower grid price.
    pub min_px: String,
    /// Number of grid levels.
    pub grid_num: String,
    /// Grid running mode.
    pub run_type: String,
    /// Contract-grid direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Contract-grid leverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    /// Whether an existing base position is opened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_pos: Option<bool>,
    /// Investment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_type: Option<String>,
    /// Trigger strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_strategy: Option<String>,
    /// Spot-grid top-up amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_up_amt: Option<String>,
    /// Per-currency investments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investment_data: Option<Vec<GridInvestmentDataRequest>>,
}

/// Query for the public RSI grid backtest.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRsiBackTestingRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// Candlestick timeframe.
    pub timeframe: String,
    /// RSI threshold.
    pub thold: String,
    /// RSI time period.
    pub time_period: String,
    /// RSI trigger condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_cond: Option<String>,
    /// Backtest duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

/// Query for the maximum grid quantity.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridQuantityRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// Grid running mode.
    pub run_type: String,
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Upper grid price.
    pub max_px: String,
    /// Lower grid price.
    pub min_px: String,
    /// Contract-grid leverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
}

/// Request to copy a lead grid algo order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridCopyOrderRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// Grid algo order type.
    pub algo_ord_type: String,
    /// Lead algo order ID.
    pub source_algo_id: String,
    /// Spot quote-currency investment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_sz: Option<String>,
    /// Contract-grid leverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    /// Whether to reserve profit automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reserve: Option<bool>,
    /// Contract-grid investment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    /// Actual margin investment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_margin_sz: Option<String>,
    /// Extra margin investment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_margin_sz: Option<String>,
    /// Client algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// One currency allocation in a recurring-buy strategy.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringCurrencyRequest {
    /// Currency to buy.
    pub ccy: String,
    /// Allocation ratio.
    pub ratio: String,
    /// Optional minimum purchase price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_px: Option<String>,
    /// Optional maximum purchase price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_px: Option<String>,
}

/// Request to place a recurring-buy algo order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringOrderRequest {
    /// Custom strategy name.
    pub stgy_name: String,
    /// Currency allocations.
    pub recurring_list: Vec<RecurringCurrencyRequest>,
    /// Investment period.
    pub period: String,
    /// Hour of day for the investment.
    pub recurring_time: String,
    /// UTC time zone.
    pub time_zone: String,
    /// Amount invested per cycle.
    pub amt: String,
    /// Investment currency.
    pub investment_ccy: String,
    /// Trading mode.
    pub td_mode: String,
    /// Monthly date or weekly day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_day: Option<String>,
    /// Hourly recurrence interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_hour: Option<String>,
    /// Client-supplied algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Quote currency used for trading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_quote_ccy: Option<String>,
    /// Funding sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
    /// Recurring time type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_time_type: Option<String>,
}

/// Request to rename a recurring-buy algo order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringAmendRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// New strategy name.
    pub stgy_name: String,
}

/// Request selecting one recurring-buy algo order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringAlgoIdRequest {
    /// Algo order ID.
    pub algo_id: String,
}

/// Query for active or historical recurring-buy algo orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringOrdersRequest {
    /// Algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Return records earlier than this algo ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this algo ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query for recurring-buy sub-orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringSubOrdersRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Sub-order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Return records earlier than this order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to add investment or amend a recurring-buy amount.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringAmountRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Investment amount.
    pub amount: String,
}

/// One amended price range in a recurring-buy strategy.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringPriceRangeRequest {
    /// Recurring-buy currency.
    pub ccy: String,
    /// Minimum price, or an empty string for no limit.
    pub min_px: String,
    /// Maximum price, or an empty string for no limit.
    pub max_px: String,
}

/// Request to amend recurring-buy price ranges.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringAmendPriceRangeRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// New price ranges.
    pub recurring_list: Vec<RecurringPriceRangeRequest>,
}

/// Request to amend a recurring-buy schedule.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringAmendTimeRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Recurring time type.
    pub recurring_time_type: String,
    /// UTC time zone.
    pub time_zone: String,
    /// Investment period.
    pub period: String,
    /// Hourly recurrence interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_hour: Option<String>,
    /// Monthly date or weekly day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_day: Option<String>,
    /// Hour of day for a custom schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_time: Option<String>,
}

/// Request to create a signal channel.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalCreateRequest {
    /// Signal channel name.
    pub signal_chan_name: String,
    /// Optional signal channel description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_chan_desc: Option<String>,
}

/// Query for signal channels.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalsRequest {
    /// Signal source type.
    pub signal_source_type: String,
    /// Signal channel ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_chan_id: Option<String>,
    /// Return records earlier than this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Entry settings for a signal bot.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalEntrySettingRequest {
    /// Whether repeated entries in the same direction are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_entry: Option<bool>,
    /// Entry sizing type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<String>,
    /// Fixed margin or contract amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<String>,
    /// Percentage amount per order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
}

/// Exit settings for a signal bot.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalExitSettingRequest {
    /// Take-profit/stop-loss calculation type.
    pub tp_sl_type: String,
    /// Take-profit percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_pct: Option<String>,
    /// Stop-loss percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_pct: Option<String>,
}

/// Request to create a signal bot.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalOrderRequest {
    /// Signal channel ID.
    pub signal_chan_id: String,
    /// Leverage for a contract signal.
    pub lever: String,
    /// Investment amount.
    pub invest_amt: String,
    /// Sub-order type.
    pub sub_ord_type: String,
    /// Whether every USDT-margined contract is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// Instrument IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_ids: Option<Vec<String>>,
    /// Limit-price offset ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    /// Entry settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_setting_param: Option<SignalEntrySettingRequest>,
    /// Exit settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_setting_param: Option<SignalExitSettingRequest>,
}

/// Request selecting one signal bot by algo ID.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAlgoIdRequest {
    /// Algo order ID.
    pub algo_id: String,
}

/// Request selecting one signal bot and its algo type.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAlgoRequest {
    /// Signal algo order type.
    pub algo_ord_type: String,
    /// Algo order ID.
    pub algo_id: String,
}

/// Query for active or historical signal bots.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalOrdersRequest {
    /// Signal algo order type.
    pub algo_ord_type: String,
    /// Algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Return records earlier than this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to adjust signal-bot margin.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalMarginBalanceRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Adjustment type (`add` or `reduce`).
    pub r#type: String,
    /// Adjustment amount.
    pub amt: String,
    /// Whether newly added margin is reinvested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reinvest: Option<bool>,
}

/// Request to amend signal-bot take-profit and stop-loss settings.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalAmendTpSlRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Replacement exit settings.
    pub exit_setting_param: SignalExitSettingRequest,
}

/// Request to set signal-bot instruments.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalSetInstrumentsRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Instrument IDs.
    pub inst_ids: Vec<String>,
    /// Whether every USDT-margined contract is included.
    pub include_all: bool,
}

/// Query for historical signal-bot positions.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalPositionHistoryRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Instrument ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Return records with earlier update times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records with newer update times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to close a signal-bot position.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalClosePositionRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Instrument ID.
    pub inst_id: String,
}

/// Request to place a signal-bot sub-order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalSubOrderRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// Algo order ID.
    pub algo_id: String,
    /// Order side.
    pub side: String,
    /// Order type.
    pub ord_type: String,
    /// Order size.
    pub sz: String,
    /// Limit order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Whether the order may only reduce a position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

/// Request to cancel a signal-bot sub-order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalCancelSubOrderRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Signal sub-order ID.
    pub signal_ord_id: String,
}

/// Query for signal-bot sub-orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalSubOrdersRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Signal algo order type.
    pub algo_ord_type: String,
    /// Sub-order state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Signal sub-order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_ord_id: Option<String>,
    /// Return records earlier than this order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Inclusive creation-time lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// Inclusive creation-time upper bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Sub-order category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Query for signal-bot event history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalEventHistoryRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Return records created earlier than this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records created newer than this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Trigger configuration for a DCA bot.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaTriggerRequest {
    /// Trigger action.
    pub trigger_action: String,
    /// Trigger strategy.
    pub trigger_strategy: String,
    /// Candlestick timeframe for an RSI trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// RSI threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thold: Option<String>,
    /// RSI trigger condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_cond: Option<String>,
    /// RSI time period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_period: Option<String>,
    /// Contract-DCA price trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<String>,
}

/// Request to create a DCA bot.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaCreateRequest {
    /// Instrument ID.
    pub inst_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Initial order amount.
    pub init_ord_amt: String,
    /// Whether contract-DCA profit is reinvested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reinvest: Option<String>,
    /// Safety order amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_ord_amt: Option<String>,
    /// Maximum number of safety orders.
    pub max_safety_ords: String,
    /// Safety-order price step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px_steps: Option<String>,
    /// Price-step multiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px_steps_mult: Option<String>,
    /// Safety-order amount multiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vol_mult: Option<String>,
    /// Take-profit target per cycle.
    pub tp_pct: String,
    /// Stop-loss target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_pct: Option<String>,
    /// Stop-loss order mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_mode: Option<String>,
    /// Contract-DCA direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Contract-DCA leverage.
    pub lever: String,
    /// Trigger settings.
    pub trigger_params: Vec<DcaTriggerRequest>,
    /// Lead-trader profit-sharing ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_sharing_ratio: Option<String>,
    /// Contract-DCA tracking mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_mode: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Client-supplied algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
    /// Spot-DCA quote currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_quote_ccy: Option<String>,
}

/// Request to amend Spot DCA parameters.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaAmendRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Safety-order price step.
    pub px_steps: String,
    /// Price-step multiplier.
    pub px_steps_mult: String,
    /// Safety-order amount multiplier.
    pub vol_mult: String,
    /// Take-profit target.
    pub tp_pct: String,
    /// Stop-loss target.
    pub sl_pct: String,
    /// Initial order amount.
    pub init_ord_amt: String,
    /// Safety order amount.
    pub safety_ord_amt: String,
    /// Maximum number of safety orders.
    pub max_safety_ords: String,
    /// Whether all funds are reserved.
    pub reserve_funds: bool,
    /// Signal trigger settings.
    pub trigger_params: Vec<DcaTriggerRequest>,
}

/// Request to stop a DCA bot.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaStopRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Stop type.
    pub stop_type: String,
}

/// Query for ongoing or historical DCA bots.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaOrdersRequest {
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Return records earlier than this algo ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this algo ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Query for DCA sub-orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaSubOrdersRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Cycle ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_id: Option<String>,
    /// Return records earlier than this sub-order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this sub-order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to add investment through a DCA sub-order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaManualBuyRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Limit price.
    pub price: String,
    /// Amount to invest.
    pub amt: String,
    /// Spot-DCA order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    /// Spot-DCA quote currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_quote_ccy: Option<String>,
}

/// Request to change Contract DCA profit reinvestment.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaReinvestmentRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Whether profit is reinvested.
    pub allow_reinvest: bool,
}

/// Request to change Contract DCA take-profit price.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaTakeProfitRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Take-profit price.
    pub tp_price: String,
}

/// Request selecting a DCA bot.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaAlgoRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
}

/// Query for DCA cycles.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaCyclesRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// DCA algo order type.
    pub algo_ord_type: String,
    /// Instrument ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Return records earlier than this cycle ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this cycle ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to add or reduce Contract DCA margin.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DcaMarginRequest {
    /// Algo order ID.
    pub algo_id: String,
    /// Margin adjustment amount.
    pub amt: String,
}
