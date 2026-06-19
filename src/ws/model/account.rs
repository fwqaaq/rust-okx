//! Trading account channel models (`account`, `positions`, `balance_and_position`, etc.).
//!
//! Private channels; login required.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

/// A close-order algo attached to a position.
///
/// Populated in [`PositionUpdate::close_order_algo`] when a close-position
/// algo order (placed with `closeFraction=1`) is associated with the position.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-positions-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloseOrderAlgo {
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Stop-loss trigger price.
    #[serde(default)]
    pub sl_trigger_px: String,
    /// Stop-loss trigger price type: `last`, `index`, or `mark`.
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Take-profit trigger price.
    #[serde(default)]
    pub tp_trigger_px: String,
    /// Take-profit trigger price type: `last`, `index`, or `mark`.
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Fraction of the position to close when the algo is triggered (e.g. `"0.6"`).
    #[serde(default)]
    pub close_fraction: String,
}

/// Private `positions` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-positions-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionUpdate {
    /// Instrument type, e.g., `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Margin mode: `cross` or `isolated`.
    #[serde(default)]
    pub mgn_mode: String,
    /// OKX-assigned position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity (contracts for derivatives; base currency for margin).
    #[serde(default)]
    pub pos: NumberString,
    /// Position currency (base currency for MARGIN positions only).
    #[serde(default)]
    pub pos_ccy: String,
    /// Available position (not frozen by closing orders).
    #[serde(default)]
    pub avail_pos: NumberString,
    /// Average entry price of the position.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Non-settlement average entry price (cross FUTURES only).
    #[serde(default)]
    pub non_settle_avg_px: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Unrealized profit and loss ratio.
    #[serde(default)]
    pub upl_ratio: NumberString,
    /// Unrealized PnL calculated using the last traded price.
    #[serde(default)]
    pub upl_last_px: NumberString,
    /// Unrealized PnL ratio calculated using the last traded price.
    #[serde(default)]
    pub upl_ratio_last_px: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Initial margin requirement in USD.
    #[serde(default)]
    pub imr: NumberString,
    /// Margin balance (isolated mode only).
    #[serde(default)]
    pub margin: NumberString,
    /// Margin ratio.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Maintenance margin requirement in USD.
    #[serde(default)]
    pub mmr: NumberString,
    /// Liabilities of the position (for cross-margin positions).
    #[serde(default)]
    pub liab: NumberString,
    /// Liability currency.
    #[serde(default)]
    pub liab_ccy: String,
    /// Accrued interest.
    #[serde(default)]
    pub interest: NumberString,
    /// USD price of the instrument's settlement currency.
    #[serde(default)]
    pub usd_px: NumberString,
    /// Quantity of base currency hedged via spot (Portfolio margin mode only).
    #[serde(default)]
    pub hedged_pos: NumberString,
    /// Trade ID of the most recent fill for this position.
    #[serde(default)]
    pub trade_id: String,
    /// Options value in USD (options positions only).
    #[serde(default)]
    pub opt_val: NumberString,
    /// Pending closing-order liability value.
    #[serde(default)]
    pub pending_close_ord_liab_val: NumberString,
    /// Notional value of the position in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Auto-deleveraging (ADL) indicator level (1–5); higher means higher ADL risk.
    #[serde(default)]
    pub adl: String,
    /// Settlement or margin currency of the position.
    #[serde(default)]
    pub ccy: String,
    /// Last traded price.
    #[serde(default)]
    pub last: NumberString,
    /// Index price.
    #[serde(default)]
    pub idx_px: NumberString,
    /// Breakeven price.
    #[serde(default)]
    pub be_px: NumberString,
    /// Black-Scholes delta (options only).
    #[serde(default)]
    pub delta_bs: NumberString,
    /// PA delta (options only).
    #[serde(default)]
    pub delta_pa: NumberString,
    /// Black-Scholes gamma (options only).
    #[serde(default)]
    pub gamma_bs: NumberString,
    /// PA gamma (options only).
    #[serde(default)]
    pub gamma_pa: NumberString,
    /// Black-Scholes theta (options only).
    #[serde(default)]
    pub theta_bs: NumberString,
    /// PA theta (options only).
    #[serde(default)]
    pub theta_pa: NumberString,
    /// Black-Scholes vega (options only).
    #[serde(default)]
    pub vega_bs: NumberString,
    /// PA vega (options only).
    #[serde(default)]
    pub vega_pa: NumberString,
    /// Spot quantity used for hedging (Portfolio margin mode only).
    #[serde(default)]
    pub spot_in_use_amt: NumberString,
    /// Currency of the spot hedge quantity.
    #[serde(default)]
    pub spot_in_use_ccy: String,
    /// User-defined spot hedge amount.
    #[serde(default)]
    pub cl_spot_in_use_amt: NumberString,
    /// Maximum spot hedge amount calculated by OKX.
    #[serde(default)]
    pub max_spot_in_use_amt: NumberString,
    /// External business reference ID (e.g., copy-trading).
    #[serde(default)]
    pub biz_ref_id: String,
    /// External business reference type.
    #[serde(default)]
    pub biz_ref_type: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Position creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,
    /// Realized profit and loss.
    ///
    /// `realizedPnl = pnl + fee + fundingFee + liqPenalty + settledPnl`
    ///
    /// Only applicable to FUTURES/SWAP/OPTION.
    #[serde(default)]
    pub realized_pnl: NumberString,
    /// Accumulated PnL from closing orders, excluding fees.
    #[serde(default)]
    pub pnl: NumberString,
    /// Accumulated transaction fee. Negative = fee charged; positive = rebate.
    #[serde(default)]
    pub fee: NumberString,
    /// Accumulated funding fee.
    #[serde(default)]
    pub funding_fee: NumberString,
    /// Accumulated liquidation penalty (negative when non-zero).
    #[serde(default)]
    pub liq_penalty: NumberString,
    /// Accumulated settled P&L calculated by settlement price.
    ///
    /// Only applicable to cross FUTURES.
    #[serde(default)]
    pub settled_pnl: NumberString,
    /// Close-position algo orders attached to this position.
    ///
    /// Non-empty only after placing an algo order with `closeFraction=1`.
    #[serde(default)]
    pub close_order_algo: Vec<CloseOrderAlgo>,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// A data row pushed by the private `balance_and_position` WebSocket channel.
///
/// This channel provides near-real-time updates for account cash balances and
/// positions. Updates may be triggered by events such as:
///
/// - order fills;
/// - funding transfers;
/// - delivery or exercise;
/// - liquidation or ADL;
/// - funding-fee deductions;
/// - margin or leverage adjustments.
///
/// A push may contain only [`BalanceAndPositionUpdate::bal_data`] when only a
/// balance changes, or only [`BalanceAndPositionUpdate::pos_data`] when only a
/// position changes.
///
/// During the initial snapshot, OKX only pushes:
///
/// - currencies whose cash balance is non-zero;
/// - positions whose quantity is non-zero.
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionUpdate {
    /// Push time of the balance and position information.
    ///
    /// Unix timestamp in milliseconds, for example `1597026383085`.
    #[serde(default)]
    pub p_time: NumberString,

    /// Event that caused this update.
    ///
    /// - Typical values include:
    ///   `delivered`, `exercised`, `transferred`, `filled`, `liquidation`
    ///   `claw_back`, `adl`, `funding_fee`, `adjust_margin`,`set_leverage`
    ///   `interest_deduction`, `settlement`
    pub event_type: String,

    /// Updated account cash-balance records.
    ///
    /// This array may be empty when the push only contains position changes.
    #[serde(default)]
    pub bal_data: Vec<BalData>,

    /// Updated position records.
    ///
    /// This array may be empty when the push only contains balance changes.
    #[serde(default)]
    pub pos_data: Vec<PosData>,

    /// Trades associated with the balance or position update.
    #[serde(default)]
    pub trades: Vec<Trade>,

    /// Additional fields added by OKX that are not yet represented explicitly.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// A currency-balance record from the `balData` array.
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalData {
    /// Currency code, for example `BTC` or `USDT`.
    #[serde(default)]
    pub ccy: String,

    /// Cash balance of the currency.
    ///
    /// OKX returns numeric values as JSON strings.
    #[serde(default)]
    pub cash_bal: NumberString,

    /// Time when this currency balance was last updated.
    ///
    /// Unix timestamp in milliseconds, for example `1597026383085`.
    #[serde(default)]
    pub u_time: NumberString,

    /// Additional fields added by OKX that are not yet represented explicitly.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// A position record from the `posData` array.
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PosData {
    /// Position ID assigned by OKX.
    #[serde(default)]
    pub pos_id: String,

    /// ID of the latest trade associated with the position.
    ///
    /// Trade IDs should be treated as opaque identifiers rather than numeric
    /// values.
    #[serde(default)]
    pub trade_id: String,

    /// Instrument ID, for example `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,

    /// Instrument type.
    ///
    /// Typical values include:
    ///
    /// - `MARGIN`;
    /// - `SWAP`;
    /// - `FUTURES`;
    /// - `OPTION`.
    #[serde(default)]
    pub inst_type: String,

    /// Margin mode.
    ///
    /// Possible values:
    ///
    /// - `cross`;
    /// - `isolated`.
    #[serde(default)]
    pub mgn_mode: String,

    /// Position side.
    ///
    /// Possible values:
    ///
    /// - `long`: long side in long/short mode;
    /// - `short`: short side in long/short mode;
    /// - `net`: net position mode.
    #[serde(default)]
    pub pos_side: String,

    /// Position quantity.
    ///
    /// For derivatives, the unit is normally the number of contracts.
    ///
    /// In isolated margin mode, OKX may push a position whose quantity is `0`
    /// after a manual margin transfer.
    #[serde(default)]
    pub pos: NumberString,

    /// Currency used for margin.
    #[serde(default)]
    pub ccy: String,

    /// Position currency.
    ///
    /// Only applicable to `MARGIN` positions.
    #[serde(default)]
    pub pos_ccy: String,

    /// Average entry price of the position.
    #[serde(default)]
    pub avg_px: NumberString,

    /// Non-settlement entry price.
    ///
    /// This value only reflects the average price at which the position was
    /// opened or increased.
    ///
    /// Only applicable to cross-margin `FUTURES` positions.
    #[serde(default)]
    pub non_settle_avg_px: String,

    /// Accumulated settled profit and loss calculated using settlement prices.
    ///
    /// Only applicable to cross-margin `FUTURES` positions.
    #[serde(default)]
    pub settled_pnl: String,

    /// Time when the position was last updated.
    ///
    /// Unix timestamp in milliseconds, for example `1597026383085`.
    #[serde(default)]
    pub u_time: NumberString,

    /// Additional fields added by OKX that are not yet represented explicitly.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// A trade associated with a balance or position update.
///
/// The `trades` array identifies fills related to the pushed balance or
/// position change.
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Trade {
    /// Instrument ID, for example `BTC-USDT` or `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,

    /// Trade ID assigned by OKX.
    ///
    /// This value should be treated as an opaque identifier.
    #[serde(default)]
    pub trade_id: String,

    /// Additional fields added by OKX that are not yet represented explicitly.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Balance row in a `balance_and_position` push.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionBalance {
    /// Currency code, e.g., `BTC` or `USDT`.
    #[serde(default)]
    pub ccy: String,
    /// Cash balance of the currency.
    #[serde(default)]
    pub cash_bal: NumberString,
    /// Time when this currency balance was last updated (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Position row in a `balance_and_position` push.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionPosition {
    /// OKX-assigned position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Trade ID of the most recent fill for this position.
    #[serde(default)]
    pub trade_id: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type, e.g., `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Margin mode: `cross` or `isolated`.
    #[serde(default)]
    pub mgn_mode: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity (contracts for derivatives; base currency for margin).
    #[serde(default)]
    pub pos: NumberString,
    /// Settlement or margin currency of the position.
    #[serde(default)]
    pub ccy: String,
    /// Position currency (base currency for MARGIN positions only).
    #[serde(default)]
    pub pos_ccy: String,
    /// Average entry price of the position.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Non-settlement average entry price (cross FUTURES only).
    #[serde(default)]
    pub non_settle_avg_px: NumberString,
    /// Accumulated settled PnL using settlement prices (cross FUTURES only).
    #[serde(default)]
    pub settled_pnl: NumberString,
    /// Time when this position was last updated (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Trade row in a `balance_and_position` push.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionTrade {
    /// Instrument ID of the fill, e.g., `BTC-USDT` or `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Trade ID assigned by OKX.
    #[serde(default)]
    pub trade_id: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Private `liquidation-warning` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-liquidation-warning-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LiquidationWarningUpdate {
    /// Instrument type, e.g., `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity (contracts for derivatives).
    #[serde(default)]
    pub pos: NumberString,
    /// Margin mode: `cross` or `isolated`.
    #[serde(default)]
    pub mgn_mode: String,
    /// Current margin ratio; liquidation is triggered when this reaches 1.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Current mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
    /// Settlement or margin currency.
    #[serde(default)]
    pub ccy: String,
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

/// Private `account-greeks` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-greeks-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountGreeksUpdate {
    /// Currency denominating the greeks, e.g., `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Account-level Black-Scholes delta in this currency.
    #[serde(default)]
    pub delta_bs: NumberString,
    /// Account-level PA delta in this currency.
    #[serde(default)]
    pub delta_pa: NumberString,
    /// Account-level Black-Scholes gamma in this currency.
    #[serde(default)]
    pub gamma_bs: NumberString,
    /// Account-level PA gamma in this currency.
    #[serde(default)]
    pub gamma_pa: NumberString,
    /// Account-level Black-Scholes theta in this currency.
    #[serde(default)]
    pub theta_bs: NumberString,
    /// Account-level PA theta in this currency.
    #[serde(default)]
    pub theta_pa: NumberString,
    /// Account-level Black-Scholes vega in this currency.
    #[serde(default)]
    pub vega_bs: NumberString,
    /// Account-level PA vega in this currency.
    #[serde(default)]
    pub vega_pa: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Account data row pushed by the private `account` WebSocket channel.
///
/// The account channel provides account-level equity, margin requirements,
/// liabilities, notional values, risk indicators, and detailed per-currency
/// balances.
///
/// OKX may push account information:
///
/// - after account-related events, such as order placement, cancellation or
///   execution;
/// - at regular intervals;
/// - as an initial paginated snapshot.
///
/// Only currencies with a non-zero `eq`, `availEq`, or `availBal` are included
/// in the initial and regular snapshots.
///
/// The outer WebSocket message fields `curPage` and `lastPage`
/// are not part of this structure. They belong to the WebSocket push envelope.
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountUpdate {
    /// Adjusted or effective account equity denominated in USD.
    ///
    /// This is the net value of assets that can provide margin after applying
    /// collateral discount rates.
    ///
    /// Applicable to Spot mode, Multi-currency margin mode and Portfolio
    /// margin mode.
    #[serde(default)]
    pub adj_eq: NumberString,

    /// Account-level available equity.
    ///
    /// Currencies restricted by the collateralized borrowing limit are
    /// excluded.
    ///
    /// Applicable to Multi-currency margin mode and Portfolio margin mode.
    #[serde(default)]
    pub avail_eq: NumberString,

    /// Potential borrowing initial margin requirement of the account in USD.
    ///
    /// An empty string may be returned when the field is not applicable to the
    /// current account mode.
    #[serde(default)]
    pub borrow_froz: NumberString,

    /// Account delta denominated in USD.
    #[serde(default)]
    pub delta: NumberString,

    /// Account-level delta leverage for a delta-neutral strategy.
    ///
    /// Calculated by OKX as `delta / totalEq`.
    #[serde(default)]
    pub delta_lever: NumberString,

    /// Delta-neutral risk status.
    ///
    /// Documented values:
    ///
    /// - `"0"`: normal;
    /// - `"1"`: transfers are restricted;
    /// - `"2"`: delta-reducing restrictions are active.
    ///
    /// This remains a string so newly introduced OKX status values can still
    /// be decoded.
    #[serde(default)]
    pub delta_neutral_status: String,

    /// Detailed account information grouped by currency.
    #[serde(default)]
    pub details: Vec<AccountBalanceUpdate>,

    /// Account-level initial margin requirement in USD.
    ///
    /// This is the sum of initial margin requirements for cross-margin
    /// positions and pending orders.
    #[serde(default)]
    pub imr: NumberString,

    /// Isolated-margin equity denominated in USD.
    ///
    /// Applicable to Futures mode, Multi-currency margin mode and Portfolio
    /// margin mode.
    #[serde(default)]
    pub iso_eq: NumberString,

    /// Account maintenance margin ratio.
    ///
    /// Applicable to Spot mode, Multi-currency margin mode and Portfolio
    /// margin mode.
    #[serde(default)]
    pub mgn_ratio: NumberString,

    /// Account-level maintenance margin requirement in USD.
    ///
    /// This is the sum of maintenance margin requirements for cross-margin
    /// positions and pending orders.
    #[serde(default)]
    pub mmr: NumberString,

    /// Total notional value of account positions in USD.
    #[serde(default)]
    pub notional_usd: NumberString,

    /// Notional value attributed to borrowing in USD.
    #[serde(default)]
    pub notional_usd_for_borrow: NumberString,

    /// Notional value of expiry-futures positions in USD.
    #[serde(default)]
    pub notional_usd_for_futures: NumberString,

    /// Notional value of option positions in USD.
    #[serde(default)]
    pub notional_usd_for_option: NumberString,

    /// Notional value of perpetual-futures positions in USD.
    #[serde(default)]
    pub notional_usd_for_swap: NumberString,

    /// Cross-margin amount frozen by pending orders, denominated in USD.
    #[serde(default)]
    pub ord_froz: NumberString,

    /// Total account equity denominated in USD.
    #[serde(default)]
    pub total_eq: NumberString,

    /// Time when the account information was last updated.
    ///
    /// Unix timestamp in milliseconds, for example `1705564223311`.
    #[serde(default)]
    pub u_time: NumberString,

    /// Account-level cross-margin unrealized profit and loss in USD.
    ///
    /// Applicable to Multi-currency margin mode and Portfolio margin mode.
    #[serde(default)]
    pub upl: NumberString,

    /// Additional account fields introduced by OKX that are not yet
    /// represented explicitly.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Per-currency account data contained in [`AccountUpdate::details`].
///
/// Fields that do not apply to the current account mode are commonly returned
/// by OKX as empty strings.
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBalanceUpdate {
    /// Auto-lending status for this currency.
    ///
    /// Documented values:
    ///
    /// - `unsupported`: auto lending is unsupported;
    /// - `off`: supported but disabled;
    /// - `pending`: enabled and waiting for matching;
    /// - `active`: enabled and matched.
    #[serde(default)]
    pub auto_lend_status: String,

    /// Amount of the currency currently matched for auto lending.
    ///
    /// Returns zero when [`Self::auto_lend_status`] is `unsupported`, `off`,
    /// or `pending`.
    #[serde(default)]
    pub auto_lend_mt_amt: NumberString,

    /// Available balance of the currency.
    #[serde(default)]
    pub avail_bal: NumberString,

    /// Available equity of the currency.
    ///
    /// Applicable to Futures mode, Multi-currency margin mode and Portfolio
    /// margin mode.
    #[serde(default)]
    pub avail_eq: NumberString,

    /// Potential borrowing initial margin requirement for this currency in
    /// USD.
    #[serde(default)]
    pub borrow_froz: NumberString,

    /// Cash balance of the currency.
    #[serde(default)]
    pub cash_bal: NumberString,

    /// Currency code, for example `BTC` or `USDT`.
    #[serde(default)]
    pub ccy: String,

    /// USD price index of the currency.
    #[serde(default)]
    pub coin_usd_price: NumberString,

    /// Cross-margin liabilities of the currency.
    #[serde(default)]
    pub cross_liab: NumberString,

    /// Platform-level collateral restriction status.
    ///
    /// Documented values:
    ///
    /// - `"0"`: no restriction;
    /// - `"1"`: close to the platform collateral limit;
    /// - `"2"`: restriction enabled; the currency cannot provide margin for
    ///   new orders.
    #[serde(default)]
    pub col_res: String,

    /// Whether this currency is enabled as collateral.
    ///
    /// Primarily applicable to Multi-currency margin mode.
    #[serde(default)]
    pub collateral_enabled: bool,

    /// Auto-conversion risk indicator.
    ///
    /// Documented levels range from `"0"` through `"5"`:
    ///
    /// - `"0"`: no current auto-conversion risk;
    /// - `"1"` to `"3"`: increasing risk;
    /// - `"4"`: auto conversion may occur soon;
    /// - `"5"`: auto conversion is in progress.
    #[serde(default)]
    pub col_borr_auto_conversion: String,

    /// Discounted equity of the currency in USD.
    #[serde(default)]
    pub dis_eq: NumberString,

    /// Total equity of the currency.
    #[serde(default)]
    pub eq: NumberString,

    /// Equity of the currency denominated in USD.
    #[serde(default)]
    pub eq_usd: NumberString,

    /// Smart-sync equity.
    ///
    /// The default is zero and the field is only applicable to copy traders.
    #[serde(default)]
    pub smt_sync_eq: NumberString,

    /// Spot smart-sync copy-trading equity.
    ///
    /// The default is zero and the field is only applicable to copy traders.
    #[serde(default)]
    pub spot_copy_trading_eq: NumberString,

    /// Balance frozen for Dip Sniper and Peak Sniper products.
    #[serde(default)]
    pub fixed_bal: NumberString,

    /// Frozen balance of the currency.
    #[serde(default)]
    pub frozen_bal: NumberString,

    /// Forced-repayment type.
    ///
    /// Documented values:
    ///
    /// - `"0"`: no forced repayment;
    /// - `"1"`: user-based forced repayment;
    /// - `"2"`: platform-based forced repayment.
    #[serde(default)]
    pub frp_type: String,

    /// Currency-level cross initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,

    /// Accrued interest for this currency.
    #[serde(default)]
    pub interest: NumberString,

    /// Isolated-margin equity of the currency.
    #[serde(default)]
    pub iso_eq: NumberString,

    /// Isolated liabilities of the currency.
    #[serde(default)]
    pub iso_liab: NumberString,

    /// Isolated unrealized profit and loss of the currency.
    #[serde(default)]
    pub iso_upl: NumberString,

    /// Total liabilities of the currency.
    ///
    /// OKX represents liabilities as a positive value.
    #[serde(default)]
    pub liab: NumberString,

    /// Maximum amount of this currency that can currently be borrowed.
    #[serde(default)]
    pub max_loan: NumberString,

    /// Cross-maintenance-margin ratio of the currency.
    #[serde(default)]
    pub mgn_ratio: NumberString,

    /// Currency-level cross maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,

    /// Leverage calculated at the currency level.
    ///
    /// Applicable to Futures mode.
    #[serde(default)]
    pub notional_lever: NumberString,

    /// Margin frozen by open orders for this currency.
    #[serde(default)]
    pub ord_frozen: NumberString,

    /// Trial-fund balance.
    #[serde(default)]
    pub reward_bal: NumberString,

    /// Actual spot-hedging amount currently in use.
    ///
    /// Applicable to Portfolio margin mode.
    #[serde(default)]
    pub spot_in_use_amt: NumberString,

    /// User-defined spot-hedging amount.
    ///
    /// Applicable to Portfolio margin mode.
    #[serde(default)]
    pub cl_spot_in_use_amt: NumberString,

    /// Maximum spot-hedging amount calculated by OKX.
    ///
    /// Applicable to Portfolio margin mode.
    ///
    /// The exact JSON field is `maxSpotInUseAmt`.
    #[serde(default)]
    pub max_spot_in_use_amt: NumberString,

    /// Spot balance obtained through copy trading.
    ///
    /// This includes amounts currently frozen by copy-trading open orders.
    #[serde(default)]
    pub spot_iso_bal: NumberString,

    /// Total equity assigned to trading-bot strategies for this currency.
    #[serde(default)]
    pub stgy_eq: NumberString,

    /// Forced-repayment risk indicator.
    ///
    /// Values range from zero through five. A larger value indicates a higher
    /// probability that forced repayment will be triggered.
    #[serde(default)]
    pub twap: NumberString,

    /// Time when this currency entry was last updated.
    ///
    /// Unix timestamp in milliseconds.
    #[serde(default)]
    pub u_time: NumberString,

    /// Unrealized profit and loss for margin and derivative positions of this
    /// currency.
    #[serde(default)]
    pub upl: NumberString,

    /// Liabilities caused by unrealized losses of this currency.
    #[serde(default)]
    pub upl_liab: NumberString,

    /// Spot balance.
    ///
    /// The unit is the currency itself, for example BTC.
    #[serde(default)]
    pub spot_bal: NumberString,

    /// Average acquisition price of the current spot balance in USD.
    ///
    /// This is a single JSON string, not an array.
    #[serde(default)]
    pub open_avg_px: NumberString,

    /// Accumulated average acquisition price of the spot balance in USD.
    ///
    /// This is a single JSON string, not an array.
    #[serde(default)]
    pub acc_avg_px: NumberString,

    /// Spot unrealized profit and loss in USD.
    #[serde(default)]
    pub spot_upl: NumberString,

    /// Spot unrealized profit and loss ratio.
    #[serde(default)]
    pub spot_upl_ratio: NumberString,

    /// Accumulated spot profit and loss in USD.
    #[serde(default)]
    pub total_pnl: NumberString,

    /// Accumulated spot profit and loss ratio.
    #[serde(default)]
    pub total_pnl_ratio: NumberString,

    /// Additional per-currency fields introduced by OKX that are not yet
    /// represented explicitly.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
