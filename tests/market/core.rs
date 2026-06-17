use crate::common::public_client;

#[tokio::test]
async fn market_ticker_parses() {
    let client = public_client();

    // API: GET /api/v5/market/ticker
    // STATUS: LIVE — public, read-only.
    let tickers = client
        .market()
        .get_ticker("BTC-USDT")
        .await
        .expect("market/ticker");

    assert_eq!(tickers.len(), 1);
    assert_eq!(tickers[0].inst_id, "BTC-USDT");
    let last: f64 = tickers[0]
        .last
        .parse()
        .expect("last price should be numeric");
    assert!(last > 0.0);
}

#[tokio::test]
async fn market_order_books_parse() {
    let client = public_client();

    // API: GET /api/v5/market/books
    // STATUS: LIVE — public, read-only.
    let books = client
        .market()
        .get_orderbook("BTC-USDT", Some(5))
        .await
        .expect("market/books");
    let book = &books[0];
    assert!(!book.asks.is_empty());
    assert!(!book.bids.is_empty());
    let best_ask: f64 = book.asks[0].price.parse().expect("numeric ask");
    let best_bid: f64 = book.bids[0].price.parse().expect("numeric bid");
    assert!(best_ask >= best_bid);

    // API: GET /api/v5/market/books-lite
    // STATUS: LIVE — public, read-only.
    client
        .market()
        .get_order_lite_book("BTC-USDT")
        .await
        .expect("market/books-lite");
}

#[tokio::test]
async fn market_candles_and_trades_parse() {
    let client = public_client();

    // API: GET /api/v5/market/candles
    // STATUS: LIVE — public, read-only.
    let candles = client
        .market()
        .get_candlesticks("BTC-USDT", Some("1H"), Some(10))
        .await
        .expect("market/candles");
    assert!(!candles.is_empty());
    let high: f64 = candles[0].high.parse().expect("numeric high");
    let low: f64 = candles[0].low.parse().expect("numeric low");
    assert!(high >= low);

    // API: GET /api/v5/market/trades
    // STATUS: LIVE — public, read-only.
    client
        .market()
        .get_trades("BTC-USDT", Some(10))
        .await
        .expect("market/trades");
}
