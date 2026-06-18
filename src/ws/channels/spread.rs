//! `spread` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to all private `sprd-orders` updates.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-orders-channel>
pub fn orders() -> Arg {
    Arg::new("sprd-orders")
}

/// Subscribe to private `sprd-orders` filtered by spread ID.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-orders-channel>
pub fn orders_by_spread(sprd_id: impl Into<String>) -> Arg {
    Arg::new("sprd-orders").sprd_id(sprd_id)
}

/// Subscribe to private `sprd-trades` for one spread.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-trades-channel>
pub fn trades(sprd_id: impl Into<String>) -> Arg {
    Arg::new("sprd-trades").sprd_id(sprd_id)
}

/// Subscribe to a spread order-book channel such as `sprd-books5`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-order-book-channel>
pub fn order_book(channel: impl Into<String>, sprd_id: impl Into<String>) -> Arg {
    Arg::new(channel).sprd_id(sprd_id)
}

/// Subscribe to `sprd-public-trades` for one spread.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-public-trades-channel>
pub fn public_trades(sprd_id: impl Into<String>) -> Arg {
    Arg::new("sprd-public-trades").sprd_id(sprd_id)
}

/// Subscribe to `sprd-tickers` for one spread.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-tickers-channel>
pub fn tickers(sprd_id: impl Into<String>) -> Arg {
    Arg::new("sprd-tickers").sprd_id(sprd_id)
}

/// Subscribe to a `sprd-candle*` channel.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-candlesticks-channel>
pub fn candlesticks(channel: impl Into<String>, sprd_id: impl Into<String>) -> Arg {
    Arg::new(channel).sprd_id(sprd_id)
}
