//! Trading bot channel models (`grid-orders-*`, `grid-positions`, `recurring-buy`, etc.).
//!
//! Private channels; login required.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

/// Spot/contract grid-order channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridOrderUpdate {
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// OKX-assigned algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type, e.g., `SPOT`, `SWAP`, `FUTURES`.
    #[serde(default)]
    pub inst_type: String,
    /// Algo order type.
    ///
    /// Documented values: `grid` (spot grid), `contract_grid` (contract grid),
    /// `moon_grid` (moon grid).
    #[serde(default)]
    pub algo_ord_type: String,
    /// Current state of the grid strategy.
    ///
    /// Documented values: `starting`, `running`, `stopping`, `stopped`.
    #[serde(default)]
    pub state: String,
    /// Grid direction: `long`, `short`, or `neutral`.
    #[serde(default)]
    pub direction: String,
    /// Grid spacing type: `arithmetic` or `geometric`.
    #[serde(default)]
    pub run_type: String,
    /// Total position size, in base currency for spot or contracts for derivatives.
    #[serde(default)]
    pub sz: NumberString,
    /// Initial investment amount.
    #[serde(default)]
    pub investment: NumberString,
    /// Total profit and loss since the strategy started.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Profit and loss ratio since the strategy started.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Type of stop condition that ended the strategy.
    ///
    /// Documented values: `0` (no stop), `1` (price stop), `2` (loss stop), `3` (profit stop).
    #[serde(default)]
    pub stop_type: String,
    /// Cancellation reason code when the strategy was stopped.
    #[serde(default)]
    pub cancel_type: String,
    /// Strategy creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Backward-compatible name for grid-order updates.
pub type TradingBotUpdate = GridOrderUpdate;

/// `grid-positions` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-positions-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridPositionUpdate {
    /// OKX-assigned algo order ID of the grid strategy.
    #[serde(default)]
    pub algo_id: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type, e.g., `SWAP`, `FUTURES`.
    #[serde(default)]
    pub inst_type: String,
    /// Margin mode: `cross` or `isolated`.
    #[serde(default)]
    pub mgn_mode: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity (number of contracts).
    #[serde(default)]
    pub pos: NumberString,
    /// Average entry price of the position.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Unrealized profit and loss ratio.
    #[serde(default)]
    pub upl_ratio: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `grid-sub-orders` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-sub-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GridSubOrderUpdate {
    /// Parent algo order ID of the grid strategy.
    #[serde(default)]
    pub algo_id: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// OKX-assigned order ID of this sub-order.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID of this sub-order.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Order state: `live`, `partially_filled`, `filled`, or `canceled`.
    #[serde(default)]
    pub state: String,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Fee amount (negative means deducted).
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Profit and loss for this sub-order.
    #[serde(default)]
    pub pnl: NumberString,
    /// Order creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Currency allocation nested in [`RecurringBuyOrderUpdate`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringBuyAllocation {
    /// Currency code, e.g., `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Allocation ratio for this currency (0–1).
    #[serde(default)]
    pub ratio: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `algo-recurring-buy` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-recurring-buy-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RecurringBuyOrderUpdate {
    /// OKX-assigned algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument type covered by this strategy (typically `SPOT`).
    #[serde(default)]
    pub inst_type: String,
    /// Algo order type: `recurring_buy`.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Strategy state: `running`, `stopping`, or `stopped`.
    #[serde(default)]
    pub state: String,
    /// User-defined strategy name.
    #[serde(default)]
    pub stgy_name: String,
    /// List of currencies and their allocation ratios.
    #[serde(default)]
    pub recurring_list: Vec<RecurringBuyAllocation>,
    /// Recurrence period: `monthly`, `weekly`, or `daily`.
    #[serde(default)]
    pub period: String,
    /// Day of the month or week on which to execute (applicable to monthly/weekly periods).
    #[serde(default)]
    pub recurring_day: String,
    /// Hour of day on which to execute (0–23).
    #[serde(default)]
    pub recurring_hour: String,
    /// Time (HH:MM) within the hour to execute.
    #[serde(default)]
    pub recurring_time: String,
    /// Time-zone identifier for `recurring_time`, e.g., `8` (UTC+8).
    #[serde(default)]
    pub time_zone: String,
    /// Quote-currency amount spent per execution cycle.
    #[serde(default)]
    pub amt: NumberString,
    /// Total quote-currency amount invested to date.
    #[serde(default)]
    pub investment_amt: NumberString,
    /// Quote currency used for investment, e.g., `USDT`.
    #[serde(default)]
    pub investment_ccy: String,
    /// Cumulative total investment amount.
    #[serde(default)]
    pub total_investment: NumberString,
    /// Total profit and loss since strategy start.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Annualized rate of return.
    #[serde(default)]
    pub total_ann_rate: NumberString,
    /// Profit and loss ratio since strategy start.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Total market cap of purchased assets in USD.
    #[serde(default)]
    pub mkt_cap: NumberString,
    /// Number of completed buy cycles.
    #[serde(default)]
    pub cycles: NumberString,
    /// Strategy tag.
    #[serde(default)]
    pub tag: String,
    /// Strategy creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `copytrading-notification` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#copy-trading-websocket-copy-trading-notification-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyTradingNotification {
    /// Type of copy-trading notification.
    ///
    /// Documented values: `open_position`, `close_position`, `adjust_margin`, etc.
    #[serde(default)]
    pub notification_type: String,
    /// Human-readable notification message.
    #[serde(default)]
    pub notification_msg: String,
    /// Instrument type, e.g., `SWAP`, `FUTURES`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Sub-position ID assigned by OKX.
    #[serde(default)]
    pub sub_pos_id: String,
    /// Unique code of the lead trader.
    #[serde(default)]
    pub unique_code: String,
    /// Notification push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
