use serde::Deserialize;

use crate::model::NumberString;

/// The trading-account balance summary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBalance {
    /// Total equity in USD.
    #[serde(default)]
    pub total_eq: NumberString,
    /// Isolated margin equity in USD.
    #[serde(default)]
    pub iso_eq: NumberString,
    /// Adjusted / effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Account level available equity.
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Cross margin frozen for pending orders in USD.
    #[serde(default)]
    pub ord_froz: NumberString,
    /// Initial margin requirement in USD.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement in USD.
    #[serde(default)]
    pub mmr: NumberString,
    /// Potential borrowing IMR in USD.
    #[serde(default)]
    pub borrow_froz: NumberString,
    /// Account-level margin ratio.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Gross notional value of all open derivative positions in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Notional value for Borrow in USD.
    #[serde(default)]
    pub notional_usd_for_borrow: NumberString,
    /// Notional value of perpetual futures positions in USD.
    #[serde(default)]
    pub notional_usd_for_swap: NumberString,
    /// Notional value of expiry futures positions in USD.
    #[serde(default)]
    pub notional_usd_for_futures: NumberString,
    /// Notional value of option positions in USD.
    #[serde(default)]
    pub notional_usd_for_option: NumberString,
    /// Account-level unrealized PnL in USD.
    #[serde(default)]
    pub upl: NumberString,
    /// Account delta denominated in USD.
    #[serde(default)]
    pub delta: NumberString,
    /// Account-level delta leverage.
    #[serde(default)]
    pub delta_lever: NumberString,
    /// Delta-neutral status.
    #[serde(default)]
    pub delta_neutral_status: String,
    /// Per-currency balance details.
    #[serde(default)]
    pub details: Vec<BalanceDetail>,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Balance details for a single currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceDetail {
    /// Currency, e.g. `USDT`.
    pub ccy: String,
    /// Equity of the currency.
    #[serde(default)]
    pub eq: NumberString,
    /// Cash balance.
    #[serde(default)]
    pub cash_bal: NumberString,
    /// Update time of currency balance information.
    #[serde(default)]
    pub u_time: NumberString,
    /// Isolated margin equity of currency.
    #[serde(default)]
    pub iso_eq: NumberString,
    /// Available equity of currency.
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Discount equity of currency in USD.
    #[serde(default)]
    pub dis_eq: NumberString,
    /// Frozen balance for Dip Sniper and Peak Sniper.
    #[serde(default)]
    pub fixed_bal: NumberString,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
    /// Margin frozen for open orders.
    #[serde(default)]
    pub ord_frozen: NumberString,
    /// Liabilities of currency.
    #[serde(default)]
    pub liab: NumberString,
    /// Unrealized PnL of currency.
    #[serde(default)]
    pub upl: NumberString,
    /// Liabilities due to unrealized loss.
    #[serde(default)]
    pub upl_liab: NumberString,
    /// Cross liabilities of currency.
    #[serde(default)]
    pub cross_liab: NumberString,
    /// Trial fund balance.
    #[serde(default)]
    pub reward_bal: NumberString,
    /// Isolated liabilities of currency.
    #[serde(default)]
    pub iso_liab: NumberString,
    /// Cross maintenance margin ratio of currency.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Cross initial margin requirement at currency level.
    #[serde(default)]
    pub imr: NumberString,
    /// Cross maintenance margin requirement at currency level.
    #[serde(default)]
    pub mmr: NumberString,
    /// Accrued interest of currency.
    #[serde(default)]
    pub interest: NumberString,
    /// Risk indicator of forced repayment.
    #[serde(default)]
    pub twap: NumberString,
    /// Forced repayment type.
    #[serde(default)]
    pub frp_type: String,
    /// Maximum borrowable amount.
    #[serde(default)]
    pub max_loan: NumberString,
    /// Equity in USD of currency.
    #[serde(default)]
    pub eq_usd: NumberString,
    /// Potential borrowing IMR of currency in USD.
    #[serde(default)]
    pub borrow_froz: NumberString,
    /// Leverage of currency.
    #[serde(default)]
    pub notional_lever: NumberString,
    /// Total equity allocated to trading bots.
    #[serde(default)]
    pub stgy_eq: NumberString,
    /// Isolated unrealized PnL of currency.
    #[serde(default)]
    pub iso_upl: NumberString,
    /// Actual spot hedging amount in use.
    #[serde(default)]
    pub spot_in_use_amt: NumberString,
    /// User-defined spot hedging amount.
    #[serde(default)]
    pub cl_spot_in_use_amt: NumberString,
    /// Maximum possible spot hedging amount.
    #[serde(default)]
    pub max_spot_in_use: NumberString,
    /// Spot copy trading balance.
    #[serde(default)]
    pub spot_iso_bal: NumberString,
    /// Smart sync equity.
    #[serde(default)]
    pub smt_sync_eq: NumberString,
    /// Spot smart sync equity.
    #[serde(default)]
    pub spot_copy_trading_eq: NumberString,
    /// Spot balance.
    #[serde(default)]
    pub spot_bal: NumberString,
    /// Spot average cost price.
    #[serde(default)]
    pub open_avg_px: NumberString,
    /// Spot accumulated cost price.
    #[serde(default)]
    pub acc_avg_px: NumberString,
    /// Spot unrealized PnL.
    #[serde(default)]
    pub spot_upl: NumberString,
    /// Spot unrealized PnL ratio.
    #[serde(default)]
    pub spot_upl_ratio: NumberString,
    /// Spot accumulated PnL.
    #[serde(default)]
    pub total_pnl: NumberString,
    /// Spot accumulated PnL ratio.
    #[serde(default)]
    pub total_pnl_ratio: NumberString,
    /// Platform collateral restriction status.
    #[serde(default)]
    pub col_res: String,
    /// Whether collateral is enabled for this currency.
    #[serde(default)]
    pub collateral_enabled: bool,
    /// Whether collateral is restricted for this currency.
    #[serde(default)]
    pub collateral_restrict: bool,
    /// Auto-conversion status for collateral borrowing.
    #[serde(default)]
    pub col_borr_auto_conversion: String,
    /// Auto lend status.
    #[serde(default)]
    pub auto_lend_status: String,
    /// Auto lend matched amount.
    #[serde(default)]
    pub auto_lend_mt_amt: NumberString,
}
