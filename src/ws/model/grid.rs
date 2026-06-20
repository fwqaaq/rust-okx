//! Trading bot channel models (`grid-orders-*`, `grid-positions`, `recurring-buy`, etc.).
//!
//! Private channels; login required.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

/// Rebate transfer entry nested in [`GridOrderUpdate`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RebateTrans {
    /// Rebate amount.
    #[serde(default)]
    pub rebate: NumberString,
    /// Rebate currency.
    #[serde(default)]
    pub rebate_ccy: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Trigger parameter entry nested in [`GridOrderUpdate`].
///
/// Fields marked "rsi only" are populated when `trigger_strategy` is `rsi`.
/// Fields marked "price only" are populated when `trigger_strategy` is `price`.
/// `stop_type` is populated only when `trigger_action` is `stop`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TriggerParam {
    /// Trigger action: `start` or `stop`.
    #[serde(default)]
    pub trigger_action: String,
    /// Trigger strategy: `instant`, `price`, or `rsi`.
    #[serde(default)]
    pub trigger_strategy: String,
    /// Delay in seconds after the trigger condition is met.
    #[serde(default)]
    pub delay_seconds: String,
    /// Actual time the action was triggered (Unix milliseconds).
    #[serde(default)]
    pub trigger_time: String,
    /// How the action was triggered: `manual` or `auto`.
    #[serde(default)]
    pub trigger_type: String,
    /// K-line type (rsi only): e.g. `3m`, `1H`, `1D`.
    #[serde(default)]
    pub timeframe: String,
    /// RSI threshold integer 1–100 (rsi only).
    #[serde(default)]
    pub thold: String,
    /// RSI trigger condition (rsi only): `cross_up`, `cross_down`, `above`, `below`, `cross`.
    #[serde(default)]
    pub trigger_cond: String,
    /// RSI time period (rsi only): `14`.
    #[serde(default)]
    pub time_period: String,
    /// Trigger price (price only).
    #[serde(default)]
    pub trigger_px: String,
    /// Stop type when `trigger_action` is `stop`.
    ///
    /// Spot grid: `1` = sell base currency, `2` = keep base currency.
    /// Contract grid: `1` = market close all positions, `2` = keep positions.
    #[serde(default)]
    pub stop_type: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Spot/contract grid-order channel row.
///
/// Returned by both `grid-orders-spot` and `grid-orders-contract` channels.
/// Spot-only and contract-only fields default to zero/empty when not present.
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
    /// Algo order type: `grid` (spot grid) or `contract_grid` (contract grid).
    #[serde(default)]
    pub algo_ord_type: String,
    /// Current state: `starting`, `running`, `stopping`, `stopped`.
    ///
    /// Contract grid also has `no_close_position`.
    #[serde(default)]
    pub state: String,
    /// Rebate transfer info.
    #[serde(default)]
    pub rebate_trans: Vec<RebateTrans>,
    /// Trigger parameters for start/stop conditions.
    #[serde(default)]
    pub trigger_params: Vec<TriggerParam>,
    /// Upper price of the grid range.
    #[serde(default)]
    pub max_px: NumberString,
    /// Lower price of the grid range.
    #[serde(default)]
    pub min_px: NumberString,
    /// Number of grid levels.
    #[serde(default)]
    pub grid_num: NumberString,
    /// Grid spacing type: `1` = arithmetic, `2` = geometric.
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
    /// Number of arbitrage cycles completed.
    #[serde(default)]
    pub arbitrage_num: NumberString,
    /// Investment amount per grid.
    #[serde(default)]
    pub single_amt: NumberString,
    /// Estimated minimum profit margin per grid.
    #[serde(default)]
    pub per_min_profit_rate: NumberString,
    /// Estimated maximum profit margin per grid.
    #[serde(default)]
    pub per_max_profit_rate: NumberString,
    /// Price at strategy launch.
    #[serde(default)]
    pub run_px: NumberString,
    /// Total profit and loss since strategy start.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// P&L ratio since strategy start.
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Accumulated investment amount.
    #[serde(default)]
    pub investment: NumberString,
    /// Profit from completed grid cycles.
    #[serde(default)]
    pub grid_profit: NumberString,
    /// Unrealized (floating) P&L.
    #[serde(default)]
    pub float_profit: NumberString,
    /// Total annualized rate of return.
    #[serde(default)]
    pub total_annualized_rate: NumberString,
    /// Grid annualized rate of return.
    #[serde(default)]
    pub annualized_rate: NumberString,
    /// Algo order stop reason code.
    ///
    /// `0` none, `1` manual, `2` take-profit, `3` stop-loss, `4` risk control,
    /// `5` delivery, `6` signal.
    #[serde(default)]
    pub cancel_type: String,
    /// Stop type when the strategy ended.
    ///
    /// Spot: `1` = sell base, `2` = keep base.
    /// Contract: `1` = market close all, `2` = keep positions.
    #[serde(default)]
    pub stop_type: String,
    /// Total count of pending sub-orders.
    #[serde(default)]
    pub active_ord_num: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Profit sharing ratio (empty for non-copy orders).
    #[serde(default)]
    pub profit_sharing_ratio: String,
    /// Copy order type: `0` normal, `1` copy without sharing, `2` copy with sharing, `3` lead.
    #[serde(default)]
    pub copy_type: String,
    /// Quote currency used for trading.
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Strategy creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,

    // ── Spot-grid-only fields ────────────────────────────────────────────────
    /// Quote currency investment amount (spot only).
    #[serde(default)]
    pub quote_sz: NumberString,
    /// Base currency investment amount (spot only).
    #[serde(default)]
    pub base_sz: NumberString,
    /// Current quote currency holdings (spot only).
    #[serde(default)]
    pub cur_quote_sz: NumberString,
    /// Current base currency holdings (spot only).
    #[serde(default)]
    pub cur_base_sz: NumberString,
    /// Available profit in quote currency (spot only).
    #[serde(default)]
    pub profit: NumberString,
    /// Stop result (spot only): `0` default, `1` sold at market, `-1` failed to sell.
    #[serde(default)]
    pub stop_result: String,

    // ── Contract-grid-only fields ────────────────────────────────────────────
    /// Contract grid type: `long`, `short`, or `neutral` (contract only).
    #[serde(default)]
    pub direction: String,
    /// Whether a position was opened at strategy activation (contract only).
    #[serde(default)]
    pub base_pos: bool,
    /// Used margin in USDT (contract only).
    #[serde(default)]
    pub sz: NumberString,
    /// Leverage (contract only).
    #[serde(default)]
    pub lever: NumberString,
    /// Actual leverage (contract only).
    #[serde(default)]
    pub actual_lever: NumberString,
    /// Estimated liquidation price (contract only).
    #[serde(default)]
    pub liq_px: NumberString,
    /// Margin frozen by pending orders (contract only).
    #[serde(default)]
    pub ord_frozen: NumberString,
    /// Available margin (contract only).
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Total equity of the strategy account (contract only).
    #[serde(default)]
    pub eq: NumberString,
    /// Take-profit ratio, e.g. `0.1` = 10% (contract only).
    #[serde(default)]
    pub tp_ratio: NumberString,
    /// Stop-loss ratio, e.g. `0.1` = 10% (contract only).
    #[serde(default)]
    pub sl_ratio: NumberString,
    /// Accumulated trading fee (contract only).
    #[serde(default)]
    pub fee: NumberString,
    /// Accumulated funding fee (contract only).
    #[serde(default)]
    pub funding_fee: NumberString,

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
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type, e.g., `SWAP`, `FUTURES`.
    #[serde(default)]
    pub inst_type: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
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
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Latest traded price.
    #[serde(default)]
    pub last: NumberString,
    /// Notional value of the position in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// ADL signal area (1–5; lower = weaker deleveraging risk).
    #[serde(default)]
    pub adl: String,
    /// Maintenance margin ratio.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Algo order created time (Unix milliseconds).
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
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type, e.g., `SWAP`, `SPOT`.
    #[serde(default)]
    pub inst_type: String,
    /// Algo order type: `grid` or `contract_grid`.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Group ID.
    #[serde(default)]
    pub group_id: String,
    /// OKX-assigned order ID of this sub-order.
    #[serde(default)]
    pub ord_id: String,
    /// Sub-order trade mode: `cross`, `isolated`, or `cash`.
    #[serde(default)]
    pub td_mode: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Order type: `market`, `limit`, or `ioc`.
    #[serde(default)]
    pub ord_type: String,
    /// Order quantity to buy or sell.
    #[serde(default)]
    pub sz: NumberString,
    /// Sub-order state: `live`, `partially_filled`, `filled`, `canceled`, `cancelling`.
    #[serde(default)]
    pub state: String,
    /// Order side: `buy` or `sell`.
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
    /// Rebate amount.
    #[serde(default)]
    pub rebate: NumberString,
    /// Rebate currency.
    #[serde(default)]
    pub rebate_ccy: String,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Accumulated filled quantity.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Position side: `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Profit and loss for this sub-order.
    #[serde(default)]
    pub pnl: NumberString,
    /// Contract value (futures/swap/option only).
    #[serde(default)]
    pub ct_val: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Sub-order created time (Unix milliseconds).
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

/// Currency allocation entry nested in [`RecurringBuyOrderUpdate`].
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
    /// Minimum price of price range (`""` = no limit).
    #[serde(default)]
    pub min_px: String,
    /// Maximum price of price range (`""` = no limit).
    #[serde(default)]
    pub max_px: String,
    /// Accumulated quantity in units of this currency.
    #[serde(default)]
    pub total_amt: NumberString,
    /// Profit in units of `investment_ccy`.
    #[serde(default)]
    pub profit: NumberString,
    /// Average recurring buy price, quoted in `investment_ccy`.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Current market price, quoted in `investment_ccy`.
    #[serde(default)]
    pub px: NumberString,
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
    /// Algo order type: `recurring`.
    #[serde(default)]
    pub algo_ord_type: String,
    /// Strategy state: `running`, `stopping`, `stopped`, or `pause`.
    #[serde(default)]
    pub state: String,
    /// User-defined strategy name (up to 40 characters).
    #[serde(default)]
    pub stgy_name: String,
    /// List of currencies and their allocation ratios.
    #[serde(default)]
    pub recurring_list: Vec<RecurringBuyAllocation>,
    /// Recurrence period: `monthly`, `weekly`, `daily`, or `hourly`.
    #[serde(default)]
    pub period: String,
    /// Day of month (1–28) or week (1–7) for monthly/weekly periods.
    #[serde(default)]
    pub recurring_day: String,
    /// Hour interval for hourly period: `1`, `4`, `8`, or `12`.
    #[serde(default)]
    pub recurring_hour: String,
    /// Clock hour within the day to execute (0–23).
    #[serde(default)]
    pub recurring_time: String,
    /// UTC time zone offset, e.g., `8` for UTC+8.
    #[serde(default)]
    pub time_zone: String,
    /// Quote-currency amount spent per execution cycle.
    #[serde(default)]
    pub amt: NumberString,
    /// Accumulated quote-currency amount invested to date.
    #[serde(default)]
    pub investment_amt: NumberString,
    /// Investment currency: `USDT` or `USDC`.
    #[serde(default)]
    pub investment_ccy: String,
    /// Next scheduled investment time (Unix milliseconds).
    #[serde(default)]
    pub next_invest_time: NumberString,
    /// Total profit and loss since strategy start.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Total annualized rate of yield.
    #[serde(default)]
    pub total_ann_rate: NumberString,
    /// Rate of yield (P&L ratio).
    #[serde(default)]
    pub pnl_ratio: NumberString,
    /// Market value of purchased assets in USDT.
    #[serde(default)]
    pub mkt_cap: NumberString,
    /// Number of completed buy cycles.
    #[serde(default)]
    pub cycles: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Quote currency used for trading.
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Recurring buy time type.
    #[serde(default)]
    pub recurring_time_type: String,
    /// Custom recurring buy minutes.
    #[serde(default)]
    pub recurring_time_minutes: String,
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

/// `copytrading-lead-notification` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#copy-trading-websocket-copy-trading-notification-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CopyTradingNotification {
    /// Notification type.
    ///
    /// `1` lead trading failed: max position limit reached.
    /// `2` lead trading failed: max daily lead count reached.
    /// `3` lead trading failed: USDT equity below minimum.
    #[serde(default)]
    pub info_type: String,
    /// Instrument type, e.g., `SWAP`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Order side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Lead position ID assigned by OKX.
    #[serde(default)]
    pub sub_pos_id: String,
    /// Unique code of the lead trader.
    #[serde(default)]
    pub unique_code: String,
    /// Maximum daily number of lead trades.
    #[serde(default)]
    pub max_lead_trader_num: NumberString,
    /// Minimum USDT equity required for lead trading.
    #[serde(default)]
    pub min_lead_eq: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
