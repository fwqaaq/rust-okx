//! Integration tests against OKX **public** endpoints (no credentials).
//!
//! These hit the live OKX REST API and validate that real responses parse into
//! our models. They require network access; they are not gated on credentials
//! because the endpoints are public.

mod common;

use common::public_client;
use rust_okx::model::InstType;

/// `GET /api/v5/market/ticker` — latest ticker for one instrument.
/// Verifies the response parses and the `last` price is a positive number.
#[tokio::test]
async fn market_ticker_parses() {
    let client = public_client();
    let tickers = client
        .market()
        .get_ticker("BTC-USDT")
        .await
        .expect("ticker request failed");

    assert_eq!(tickers.len(), 1);
    assert_eq!(tickers[0].inst_id, "BTC-USDT");
    let last: f64 = tickers[0]
        .last
        .parse()
        .expect("last price should be numeric");
    assert!(last > 0.0, "BTC-USDT last price should be positive");
}

/// `GET /api/v5/market/books` — order book snapshot.
/// Verifies asks/bids levels parse and the best ask is above the best bid.
#[tokio::test]
async fn market_orderbook_parses() {
    let client = public_client();
    let books = client
        .market()
        .get_orderbook("BTC-USDT", Some(5))
        .await
        .expect("orderbook request failed");

    let book = &books[0];
    assert!(!book.asks.is_empty(), "asks should not be empty");
    assert!(!book.bids.is_empty(), "bids should not be empty");
    let best_ask: f64 = book.asks[0].price.parse().unwrap();
    let best_bid: f64 = book.bids[0].price.parse().unwrap();
    assert!(best_ask >= best_bid, "best ask should be >= best bid");
}

/// `GET /api/v5/market/candles` — candlestick (OHLCV) data.
/// Verifies the array-encoded rows parse and high >= low.
#[tokio::test]
async fn market_candles_parse() {
    let client = public_client();
    let candles = client
        .market()
        .get_candlesticks("BTC-USDT", Some("1H"), Some(10))
        .await
        .expect("candles request failed");

    assert!(!candles.is_empty(), "should return at least one candle");
    let c = &candles[0];
    let high: f64 = c.high.parse().unwrap();
    let low: f64 = c.low.parse().unwrap();
    assert!(high >= low, "candle high should be >= low");
}

/// `GET /api/v5/public/instruments` — tradable instruments for a type.
/// Verifies SPOT instruments parse and BTC-USDT is present.
#[tokio::test]
async fn public_instruments_parse() {
    let client = public_client();
    let instruments = client
        .public_data()
        .get_instruments(InstType::Spot, None)
        .await
        .expect("instruments request failed");

    assert!(!instruments.is_empty(), "should return SPOT instruments");
    assert!(
        instruments.iter().any(|i| i.inst_id == "BTC-USDT"),
        "BTC-USDT should be a listed SPOT instrument"
    );
}
