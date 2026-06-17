//! Block trading channel models (`rfqs`, `quotes`, `block-trades`, `block-tickers`).
//!
//! Mixed public and private channels.

use serde::Deserialize;
use serde_json::Value;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// A leg embedded in block RFQ/quote/trade messages.
    BlockLeg {
        inst_id: String,
        sz: NumberString,
        side: String,
        px: NumberString,
        trade_id: String,
        tgt_ccy: String,
        fee: NumberString,
        fee_ccy: String,
        sz_cont: NumberString,
        trade_quote_ccy: String
    }
}

ws_object! {
    /// Block-trading `rfqs` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-rfqs-channel>
    BlockRfqUpdate {
        rfq_id: String,
        cl_rfq_id: String,
        tag: String,
        trader_code: String,
        valid_until: NumberString,
        counterparties: Vec<String>,
        legs: Vec<BlockLeg>,
        anonymous: bool,
        allow_partial_execution: bool,
        state: String,
        group_id: String,
        acct_alloc: Vec<Value>,
        c_time: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// Block-trading `quotes` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-quotes-channel>
    BlockQuoteUpdate {
        quote_id: String,
        cl_quote_id: String,
        rfq_id: String,
        cl_rfq_id: String,
        trader_code: String,
        quote_side: String,
        valid_until: NumberString,
        legs: Vec<BlockLeg>,
        state: String,
        c_time: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// Structure-block-trade channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
    StructureBlockTradeUpdate {
        trade_id: String,
        block_td_id: String,
        rfq_id: String,
        cl_rfq_id: String,
        quote_id: String,
        cl_quote_id: String,
        tag: String,
        strategy: String,
        is_successful: bool,
        error_code: String,
        t_trader_code: String,
        m_trader_code: String,
        group_id: String,
        legs: Vec<BlockLeg>,
        acct_alloc: Vec<Value>,
        c_time: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// Public block-trade channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-public-block-trades-channel>
    PublicBlockTradeUpdate {
        block_td_id: String,
        group_id: String,
        strategy: String,
        c_time: NumberString,
        legs: Vec<BlockLeg>,
        inst_id: String,
        trade_id: String,
        px: NumberString,
        sz: NumberString,
        side: String,
        ts: NumberString
    }
}

/// Public structure-block-trade channel row.
///
/// The public channel emits the same top-level shape as
/// [`PublicBlockTradeUpdate`].
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
pub type PublicStructureBlockTradeUpdate = PublicBlockTradeUpdate;

ws_object! {
    /// `block-tickers` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-block-tickers-channel>
    BlockTickerUpdate {
        inst_id: String,
        last: NumberString,
        last_sz: NumberString,
        bid_px: NumberString,
        bid_sz: NumberString,
        ask_px: NumberString,
        ask_sz: NumberString,
        vol24h: NumberString,
        vol_ccy24h: NumberString,
        ts: NumberString
    }
}
