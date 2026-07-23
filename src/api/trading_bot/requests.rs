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
