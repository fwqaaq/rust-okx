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
    /// Adjusted / effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
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
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
}
