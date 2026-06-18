//! `market` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `tickers` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-tickers-channel>
pub fn tickers(inst_id: impl Into<String>) -> Arg {
    Arg::new("tickers").inst_id(inst_id)
}

/// Subscribe to a `candle*` channel such as `candle1m`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-candlesticks-channel>
pub fn candlesticks(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
    Arg::new(channel).inst_id(inst_id)
}

/// Subscribe to aggregated `trades`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-trades-channel>
pub fn trades(inst_id: impl Into<String>) -> Arg {
    Arg::new("trades").inst_id(inst_id)
}

/// Subscribe to `trades-all`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-all-trades-channel>
pub fn all_trades(inst_id: impl Into<String>) -> Arg {
    Arg::new("trades-all").inst_id(inst_id)
}

/// Subscribe to an order-book channel such as `books`, `books5`, or
/// `books-l2-tbt`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-order-book-channel>
pub fn order_book(channel: impl Into<String>, inst_id: impl Into<String>) -> Arg {
    Arg::new(channel).inst_id(inst_id)
}

/// Subscribe to `option-trades` for one instrument family.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-option-trades-channel>
pub fn option_trades(inst_family: impl Into<String>) -> Arg {
    Arg::new("option-trades").inst_family(inst_family)
}

/// Subscribe to `call-auction-details` for one instrument.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-call-auction-details-channel>
pub fn call_auction_details(inst_id: impl Into<String>) -> Arg {
    Arg::new("call-auction-details").inst_id(inst_id)
}
