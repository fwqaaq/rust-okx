//! WebSocket-specific response models.

use serde::Deserialize;

use crate::model::{NumberString, RestRow};

/// Generic row for forward-compatible WebSocket channels whose payloads are
/// sparse or feature-dependent.
pub type WsRow = RestRow;

/// Public-data `instruments` channel row.
pub type InstrumentUpdate = RestRow;

/// Public-data `event-contract-markets` channel row.
pub type EventContractMarketUpdate = RestRow;

/// Public-data `open-interest` channel row.
pub type OpenInterestUpdate = RestRow;

/// Public-data `funding-rate` channel row.
pub type FundingRateUpdate = RestRow;

/// Public-data `price-limit` channel row.
pub type PriceLimitUpdate = RestRow;

/// Public-data `opt-summary` channel row.
pub type OptionSummaryUpdate = RestRow;

/// Public-data `estimated-price` channel row.
pub type EstimatedPriceUpdate = RestRow;

/// Public-data `mark-price` channel row.
pub type MarkPriceUpdate = RestRow;

/// Public-data `index-tickers` channel row.
pub type IndexTickerUpdate = RestRow;

/// Public-data mark/index candlestick channel row.
pub type ReferenceCandleUpdate = RestRow;

/// Public-data `liquidation-orders` channel row.
pub type LiquidationOrderUpdate = RestRow;

/// Public-data `adl-warning` channel row.
pub type AdlWarningUpdate = RestRow;

/// Public-data `economic-calendar` channel row.
pub type EconomicCalendarUpdate = RestRow;

/// Private `positions` channel row.
pub type PositionUpdate = RestRow;

/// Private `balance_and_position` channel row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionUpdate {
    /// Push time in Unix milliseconds.
    #[serde(default)]
    pub p_time: NumberString,
    /// Event that triggered the push, such as `snapshot`, `filled`, or `transferred`.
    #[serde(default)]
    pub event_type: String,
    /// Changed balance rows. This can be empty when only positions changed.
    #[serde(default)]
    pub bal_data: Vec<BalanceAndPositionBalance>,
    /// Changed position rows. This can be empty when only balances changed.
    #[serde(default)]
    pub pos_data: Vec<BalanceAndPositionPosition>,
    /// Trades associated with this update.
    #[serde(default)]
    pub trades: Vec<BalanceAndPositionTrade>,
}

/// Balance row in a `balance_and_position` push.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionBalance {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Cash balance.
    #[serde(default)]
    pub cash_bal: NumberString,
    /// Update time in Unix milliseconds.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Position row in a `balance_and_position` push.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionPosition {
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Last trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Position quantity.
    #[serde(default)]
    pub pos: NumberString,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// Position currency for margin positions.
    #[serde(default)]
    pub pos_ccy: String,
    /// Average open price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Non-settlement entry price.
    #[serde(default)]
    pub non_settle_avg_px: NumberString,
    /// Accumulated settled P&L.
    #[serde(default)]
    pub settled_pnl: NumberString,
    /// Update time in Unix milliseconds.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Trade row in a `balance_and_position` push.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceAndPositionTrade {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
}

/// Private `liquidation-warning` channel row.
pub type LiquidationWarningUpdate = RestRow;

/// Private `account-greeks` channel row.
pub type AccountGreeksUpdate = RestRow;

/// Private `fills` channel row.
pub type FillUpdate = RestRow;

/// Private `orders-algo` channel row.
pub type AlgoOrderUpdate = RestRow;

/// Private `algo-advance` channel row.
pub type AdvancedAlgoOrderUpdate = RestRow;

/// Trading-bot channel row.
pub type TradingBotUpdate = RestRow;

/// Copy-trading notification row.
pub type CopyTradingNotification = RestRow;

/// Block-trading RFQ channel row.
pub type BlockRfqUpdate = RestRow;

/// Block-trading quote channel row.
pub type BlockQuoteUpdate = RestRow;

/// Block-trading structure-block-trades channel row.
pub type StructureBlockTradeUpdate = RestRow;

/// Public block-trade channel row.
pub type PublicBlockTradeUpdate = RestRow;

/// Block ticker channel row.
pub type BlockTickerUpdate = RestRow;

/// Spread order channel row.
pub type SpreadOrderUpdate = RestRow;

/// Spread trade channel row.
pub type SpreadTradeUpdate = RestRow;

/// Spread order-book channel row.
pub type SpreadOrderBookUpdate = RestRow;

/// Spread ticker channel row.
pub type SpreadTickerUpdate = RestRow;

/// Spread candlestick channel row.
pub type SpreadCandleUpdate = RestRow;

/// Funding `deposit-info` channel row.
pub type DepositInfoUpdate = RestRow;

/// Funding `withdrawal-info` channel row.
pub type WithdrawalInfoUpdate = RestRow;

/// Status channel row.
pub type StatusUpdate = RestRow;

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
