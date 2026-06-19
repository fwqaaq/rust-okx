//! Block trading channel models (`rfqs`, `quotes`, `block-trades`, `block-tickers`).
//!
//! Mixed public and private channels.

use serde::Deserialize;
use serde_json::Value;

use super::ExtraFields;
use crate::model::NumberString;

/// A leg embedded in block RFQ/quote/trade messages.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockLeg {
    /// Instrument ID of this leg, e.g., `BTC-USD-240329-40000-C`.
    #[serde(default)]
    pub inst_id: String,
    /// Leg size in base currency or contracts.
    #[serde(default)]
    pub sz: NumberString,
    /// Leg side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Leg price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade ID for this leg (only present after execution).
    #[serde(default)]
    pub trade_id: String,
    /// Target currency for the leg quantity (spot currency-trade only): `base_ccy` or `quote_ccy`.
    #[serde(default)]
    pub tgt_ccy: String,
    /// Fee charged for this leg (negative means deducted).
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency for this leg.
    #[serde(default)]
    pub fee_ccy: String,
    /// Leg size in contracts.
    #[serde(default)]
    pub sz_cont: NumberString,
    /// Quote currency used for the trade (event contracts only).
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Block-trading `rfqs` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-rfqs-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockRfqUpdate {
    /// OKX-assigned RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Client-supplied RFQ ID, if any.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// RFQ tag.
    #[serde(default)]
    pub tag: String,
    /// Taker's anonymized trader code.
    #[serde(default)]
    pub trader_code: String,
    /// Time until which this RFQ is valid (Unix milliseconds).
    #[serde(default)]
    pub valid_until: NumberString,
    /// List of counterparty trader codes invited to quote.
    #[serde(default)]
    pub counterparties: Vec<String>,
    /// Legs that make up this RFQ.
    #[serde(default)]
    pub legs: Vec<BlockLeg>,
    /// Whether the RFQ was submitted anonymously.
    #[serde(default)]
    pub anonymous: bool,
    /// Whether partial execution is allowed on this RFQ.
    #[serde(default)]
    pub allow_partial_execution: bool,
    /// RFQ state.
    ///
    /// Documented values: `active`, `canceled`, `pending_fill`, `filled`, `expired`, `failed`.
    #[serde(default)]
    pub state: String,
    /// Group ID when this RFQ belongs to an RFQ group.
    #[serde(default)]
    pub group_id: String,
    /// Account allocation details (array of allocation objects).
    #[serde(default)]
    pub acct_alloc: Vec<Value>,
    /// RFQ creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Block-trading `quotes` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-quotes-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockQuoteUpdate {
    /// OKX-assigned quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Client-supplied quote ID, if any.
    #[serde(default)]
    pub cl_quote_id: String,
    /// RFQ ID being responded to.
    #[serde(default)]
    pub rfq_id: String,
    /// Client-supplied RFQ ID being responded to.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// Maker's anonymized trader code.
    #[serde(default)]
    pub trader_code: String,
    /// Quote side from the maker's perspective: `buy` or `sell`.
    #[serde(default)]
    pub quote_side: String,
    /// Time until which this quote is valid (Unix milliseconds).
    #[serde(default)]
    pub valid_until: NumberString,
    /// Legs that make up this quote.
    #[serde(default)]
    pub legs: Vec<BlockLeg>,
    /// Quote state.
    ///
    /// Documented values: `active`, `canceled`, `pending_fill`, `filled`, `expired`, `failed`.
    #[serde(default)]
    pub state: String,
    /// Quote creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Structure-block-trade channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StructureBlockTradeUpdate {
    /// OKX-assigned trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Block trade ID that groups one or more legs.
    #[serde(default)]
    pub block_td_id: String,
    /// OKX-assigned RFQ ID.
    #[serde(default)]
    pub rfq_id: String,
    /// Client-supplied RFQ ID.
    #[serde(default)]
    pub cl_rfq_id: String,
    /// OKX-assigned quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Client-supplied quote ID.
    #[serde(default)]
    pub cl_quote_id: String,
    /// Tag associated with this trade.
    #[serde(default)]
    pub tag: String,
    /// Strategy name (e.g., `Straddle`, `Strangle`, `Butterfly`).
    #[serde(default)]
    pub strategy: String,
    /// Whether the trade execution was successful.
    #[serde(default)]
    pub is_successful: bool,
    /// Error code when `is_successful` is `false`; empty otherwise.
    #[serde(default)]
    pub error_code: String,
    /// Taker's anonymized trader code.
    #[serde(default)]
    pub t_trader_code: String,
    /// Maker's anonymized trader code.
    #[serde(default)]
    pub m_trader_code: String,
    /// Group ID (set when the trade belongs to an RFQ group).
    #[serde(default)]
    pub group_id: String,
    /// Legs that make up this block trade.
    #[serde(default)]
    pub legs: Vec<BlockLeg>,
    /// Account allocation details for this trade.
    #[serde(default)]
    pub acct_alloc: Vec<Value>,
    /// Trade creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Public block-trade channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-public-block-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicBlockTradeUpdate {
    /// Block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Group ID when this trade belongs to a multi-leg block.
    #[serde(default)]
    pub group_id: String,
    /// Strategy name for multi-leg trades (e.g., `Straddle`); empty for single-leg.
    #[serde(default)]
    pub strategy: String,
    /// Trade creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Legs of this block trade (multi-leg trades only; empty for single-leg).
    #[serde(default)]
    pub legs: Vec<BlockLeg>,
    /// Instrument ID (single-leg trades only).
    #[serde(default)]
    pub inst_id: String,
    /// Trade ID (single-leg trades only).
    #[serde(default)]
    pub trade_id: String,
    /// Fill price (single-leg trades only).
    #[serde(default)]
    pub px: NumberString,
    /// Fill size (single-leg trades only).
    #[serde(default)]
    pub sz: NumberString,
    /// Fill side: `buy` or `sell` (single-leg trades only).
    #[serde(default)]
    pub side: String,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Public structure-block-trade channel row.
///
/// The public channel emits the same top-level shape as
/// [`PublicBlockTradeUpdate`].
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
pub type PublicStructureBlockTradeUpdate = PublicBlockTradeUpdate;

/// `block-tickers` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-block-tickers-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockTickerUpdate {
    /// Instrument ID, e.g., `BTC-USD-240329-40000-C`.
    #[serde(default)]
    pub inst_id: String,
    /// Last block-traded price.
    #[serde(default)]
    pub last: NumberString,
    /// Last block-traded size.
    #[serde(default)]
    pub last_sz: NumberString,
    /// Best bid price.
    #[serde(default)]
    pub bid_px: NumberString,
    /// Best bid size.
    #[serde(default)]
    pub bid_sz: NumberString,
    /// Best ask price.
    #[serde(default)]
    pub ask_px: NumberString,
    /// Best ask size.
    #[serde(default)]
    pub ask_sz: NumberString,
    /// Block-trading volume in contracts over the last 24 hours.
    #[serde(default)]
    pub vol24h: NumberString,
    /// Block-trading volume in currency units over the last 24 hours.
    #[serde(default)]
    pub vol_ccy24h: NumberString,
    /// Ticker push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
