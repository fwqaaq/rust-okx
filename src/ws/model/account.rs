//! Trading account channel models (`account`, `positions`, `balance_and_position`, etc.).
//!
//! Private channels; login required.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

ws_object! {
    /// Private `positions` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-positions-channel>
    PositionUpdate {
        inst_type: String,
        mgn_mode: String,
        pos_id: String,
        pos_side: String,
        pos: NumberString,
        pos_ccy: String,
        avail_pos: NumberString,
        avg_px: NumberString,
        non_settle_avg_px: NumberString,
        upl: NumberString,
        upl_ratio: NumberString,
        upl_last_px: NumberString,
        upl_ratio_last_px: NumberString,
        lever: NumberString,
        liq_px: NumberString,
        mark_px: NumberString,
        imr: NumberString,
        margin: NumberString,
        mgn_ratio: NumberString,
        mmr: NumberString,
        liab: NumberString,
        liab_ccy: String,
        interest: NumberString,
        usd_px: NumberString,
        hedged_pos: NumberString,
        trade_id: String,
        opt_val: NumberString,
        pending_close_ord_liab_val: NumberString,
        notional_usd: NumberString,
        adl: String,
        ccy: String,
        last: NumberString,
        idx_px: NumberString,
        be_px: NumberString,
        delta_bs: NumberString,
        delta_pa: NumberString,
        gamma_bs: NumberString,
        gamma_pa: NumberString,
        theta_bs: NumberString,
        theta_pa: NumberString,
        vega_bs: NumberString,
        vega_pa: NumberString,
        spot_in_use_amt: NumberString,
        spot_in_use_ccy: String,
        cl_spot_in_use_amt: NumberString,
        max_spot_in_use_amt: NumberString,
        biz_ref_id: String,
        biz_ref_type: String,
        inst_id: String,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
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
    /// See [`EventType`] for the documented OKX string values.
    #[serde(default)]
    pub event_type: Option<EventType>,

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

/// Event type for [`BalanceAndPositionUpdate`].
///
/// OKX docs:
/// <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
///
/// OKX serializes these values as lowercase `snake_case` strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventType {
    /// OKX value: `snapshot`.
    Snapshot,
    /// OKX value: `delivered`.
    Delivered,
    /// OKX value: `exercised`.
    Exercised,
    /// OKX value: `transferred`.
    Transferred,
    /// OKX value: `filled`.
    Filled,
    /// OKX value: `liquidation`.
    Liquidation,
    /// OKX value: `claw_back`.
    ClawBack,
    /// OKX value: `adl`.
    Adl,
    /// OKX value: `funding_fee`.
    FundingFee,
    /// OKX value: `adjust_margin`.
    AdjustMargin,
    /// OKX value: `set_leverage`.
    SetLeverage,
    /// OKX value: `interest_deduction`.
    InterestDeduction,
    /// OKX value: `settlement`.
    Settlement,
    /// Missing or currently unknown OKX event type.
    Unknown(String),
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(match value.as_str() {
            "snapshot" => Self::Snapshot,
            "delivered" => Self::Delivered,
            "exercised" => Self::Exercised,
            "transferred" => Self::Transferred,
            "filled" => Self::Filled,
            "liquidation" => Self::Liquidation,
            "claw_back" => Self::ClawBack,
            "adl" => Self::Adl,
            "funding_fee" => Self::FundingFee,
            "adjust_margin" => Self::AdjustMargin,
            "set_leverage" => Self::SetLeverage,
            "interest_deduction" => Self::InterestDeduction,
            "settlement" => Self::Settlement,
            _ => Self::Unknown(value),
        })
    }
}

ws_object! {
    /// Balance row in a `balance_and_position` push.
    BalanceAndPositionBalance {
        ccy: String,
        cash_bal: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// Position row in a `balance_and_position` push.
    BalanceAndPositionPosition {
        pos_id: String,
        trade_id: String,
        inst_id: String,
        inst_type: String,
        mgn_mode: String,
        pos_side: String,
        pos: NumberString,
        ccy: String,
        pos_ccy: String,
        avg_px: NumberString,
        non_settle_avg_px: NumberString,
        settled_pnl: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// Trade row in a `balance_and_position` push.
    BalanceAndPositionTrade {
        inst_id: String,
        trade_id: String
    }
}

ws_object! {
    /// Private `liquidation-warning` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-liquidation-warning-channel>
    LiquidationWarningUpdate {
        inst_type: String,
        inst_id: String,
        pos_side: String,
        pos: NumberString,
        mgn_mode: String,
        mgn_ratio: NumberString,
        mark_px: NumberString,
        liq_px: NumberString,
        ccy: String,
        u_time: NumberString,
        p_time: NumberString
    }
}

ws_object! {
    /// Private `account-greeks` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-greeks-channel>
    AccountGreeksUpdate {
        ccy: String,
        delta_bs: NumberString,
        delta_pa: NumberString,
        gamma_bs: NumberString,
        gamma_pa: NumberString,
        theta_bs: NumberString,
        theta_pa: NumberString,
        vega_bs: NumberString,
        vega_pa: NumberString,
        ts: NumberString
    }
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
/// The outer WebSocket message fields `eventType`, `curPage`, and `lastPage`
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

    /// Deprecated platform collateral-restriction indicator.
    ///
    /// Use [`Self::col_res`] instead.
    #[deprecated(note = "OKX deprecated collateralRestrict; use col_res instead")]
    #[serde(default)]
    pub collateral_restrict: bool,

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
