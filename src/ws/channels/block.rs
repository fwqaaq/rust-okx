//! `block` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to private block-trading `rfqs`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-rfqs-channel>
pub fn rfqs() -> Arg {
    Arg::new("rfqs")
}

/// Subscribe to private block-trading `quotes`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-quotes-channel>
pub fn quotes() -> Arg {
    Arg::new("quotes")
}

/// Subscribe to private `struc-block-trades`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
pub fn structure_block_trades() -> Arg {
    Arg::new("struc-block-trades")
}

/// Subscribe to public `public-struc-block-trades`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-structure-block-trades-channel>
pub fn public_structure_block_trades() -> Arg {
    Arg::new("public-struc-block-trades")
}

/// Subscribe to `public-block-trades` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-public-block-trades-channel>
pub fn public_block_trades(inst_id: impl Into<String>) -> Arg {
    Arg::new("public-block-trades").inst_id(inst_id)
}

/// Subscribe to `block-tickers` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#block-trading-websocket-block-tickers-channel>
pub fn block_tickers(inst_id: impl Into<String>) -> Arg {
    Arg::new("block-tickers").inst_id(inst_id)
}
