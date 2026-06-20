//! Block trading channel models (`rfqs`, `quotes`, `struc-block-trades`, `block-tickers`).
//!
//! Mixed public and private channels.

use serde::Deserialize;
use serde_json::Value;

use super::ExtraFields;
use crate::model::NumberString;

/// A leg embedded in block RFQ, quote, and structure-block-trade messages.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockLeg {
    /// Instrument ID of this leg, e.g., `BTC-USD-240329-40000-C`.
    #[serde(default)]
    pub inst_id: String,
    /// Trade mode: `cross`, `isolated`, or `cash`.
    #[serde(default)]
    pub td_mode: String,
    /// Margin currency (cross MARGIN orders in Futures mode only).
    #[serde(default)]
    pub ccy: String,
    /// Leg size in base currency or contracts.
    #[serde(default)]
    pub sz: NumberString,
    /// Leg side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Leg price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade ID for this leg (only present after execution).
    #[serde(default)]
    pub trade_id: String,
    /// Target currency for the leg quantity (spot only): `base_ccy` or `quote_ccy`.
    #[serde(default)]
    pub tgt_ccy: String,
    /// Fee charged for this leg (negative means deducted; positive means rebate).
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency for this leg.
    #[serde(default)]
    pub fee_ccy: String,
    /// The quote currency used for trading (spot only).
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
    /// Client-supplied RFQ ID (empty for Maker).
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
    /// Whether partial execution is allowed on this RFQ.
    #[serde(default)]
    pub allow_partial_execution: bool,
    /// RFQ state.
    ///
    /// Documented values: `active`, `canceled`, `filled`, `expired`, `traded_away`, `failed`.
    #[serde(default)]
    pub state: String,
    /// Flow type; only applicable to Makers (empty for Takers).
    #[serde(default)]
    pub flow_type: String,
    /// Group ID when this RFQ belongs to an RFQ group.
    #[serde(default)]
    pub group_id: String,
    /// Account allocation details (taker only).
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
    /// Client-supplied quote ID (empty for Taker).
    #[serde(default)]
    pub cl_quote_id: String,
    /// RFQ ID being responded to.
    #[serde(default)]
    pub rfq_id: String,
    /// Client-supplied RFQ ID (empty for Maker).
    #[serde(default)]
    pub cl_rfq_id: String,
    /// Quote tag.
    #[serde(default)]
    pub tag: String,
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
    /// Documented values: `active`, `canceled`, `filled`, `expired`, `failed`.
    #[serde(default)]
    pub state: String,
    /// Reason for the current state (e.g., `mmp_canceled`).
    #[serde(default)]
    pub reason: String,
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

/// Structure-block-trade channel row (private `struc-block-trades`).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StructureBlockTradeUpdate {
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
    /// Legs that make up this block trade.
    #[serde(default)]
    pub legs: Vec<BlockLeg>,
    /// Account allocation details for this trade.
    #[serde(default)]
    pub acct_alloc: Vec<Value>,
    /// Trade creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Public block-trade channel row (`public-block-trades`).
///
/// Each push represents a single filled instrument trade.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-public-block-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicBlockTradeUpdate {
    /// Instrument ID, e.g., `BTC-USD-231020-5000-P`.
    #[serde(default)]
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Fill price.
    #[serde(default)]
    pub px: NumberString,
    /// Fill size.
    #[serde(default)]
    pub sz: NumberString,
    /// Fill side: `buy` or `sell` (taker perspective).
    #[serde(default)]
    pub side: String,
    /// Implied volatility (options only).
    #[serde(default)]
    pub fill_vol: NumberString,
    /// Forward price (options only).
    #[serde(default)]
    pub fwd_px: NumberString,
    /// Index price (futures / swap / option).
    #[serde(default)]
    pub idx_px: NumberString,
    /// Mark price (futures / swap / option).
    #[serde(default)]
    pub mark_px: NumberString,
    /// Group RFQ ID; empty for normal trades.
    #[serde(default)]
    pub group_id: String,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Public structure-block-trade channel row (`public-struc-block-trades`).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-public-structure-block-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicStructureBlockTradeUpdate {
    /// Block trade ID.
    #[serde(default)]
    pub block_td_id: String,
    /// Group RFQ ID; empty for normal trades.
    #[serde(default)]
    pub group_id: String,
    /// Legs of the structure block trade.
    ///
    /// Each leg carries `instId`, `px`, `sz`, `side`, and `tradeId`.
    #[serde(default)]
    pub legs: Vec<BlockLeg>,
    /// Trade creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `block-tickers` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-block-tickers-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockTickerUpdate {
    /// Instrument type, e.g., `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `LTC-USD-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// 24h block-trading volume in currency units.
    #[serde(default)]
    pub vol_ccy24h: NumberString,
    /// 24h block-trading volume in contracts.
    #[serde(default)]
    pub vol24h: NumberString,
    /// Ticker data generation time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
