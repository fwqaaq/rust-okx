//! WebSocket-specific response models.

use serde::Deserialize;

use crate::NumberString;

/// An order book push from `books`, `books5`, or related channels.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderBookUpdate {
    /// Ask levels.
    #[serde(default)]
    pub asks: Vec<BookLevel>,
    /// Bid levels.
    #[serde(default)]
    pub bids: Vec<BookLevel>,
    /// Checksum supplied by OKX, when present.
    #[serde(default)]
    pub checksum: i64,
    /// Push timestamp in Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
}

/// A single order book level in WebSocket book data.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "BookLevelRaw")]
#[non_exhaustive]
pub struct BookLevel {
    /// Price at this level.
    pub price: NumberString,
    /// Aggregated size.
    pub size: NumberString,
    /// Liquidated order count.
    pub liquidated_order_count: NumberString,
    /// Number of orders at this level.
    pub order_count: NumberString,
}

type BookLevelRaw = (NumberString, NumberString, NumberString, NumberString);

impl From<BookLevelRaw> for BookLevel {
    fn from(raw: BookLevelRaw) -> Self {
        Self {
            price: raw.0,
            size: raw.1,
            liquidated_order_count: raw.2,
            order_count: raw.3,
        }
    }
}

/// Account data pushed by the private `account` channel.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountUpdate {
    /// Total equity in USD.
    #[serde(default)]
    pub total_eq: NumberString,
    /// Adjusted / effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Per-currency balance rows.
    #[serde(default)]
    pub details: Vec<AccountBalanceUpdate>,
    /// Push timestamp in Unix milliseconds.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Per-currency account data in an account WebSocket push.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBalanceUpdate {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Equity.
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

/// Order data pushed by private order channels.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderUpdate {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
}
