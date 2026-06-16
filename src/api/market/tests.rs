use crate::OkxClient;
use crate::model::InstType;
use crate::test_util::MockTransport;

#[tokio::test]
async fn get_ticker_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","last":"42000.1","lastSz":"0.1",
             "askPx":"42000.2","askSz":"1","bidPx":"42000.0","bidSz":"2",
             "open24h":"41000","high24h":"43000","low24h":"40000","vol24h":"1000",
             "volCcy24h":"42000000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let tickers = client.market().get_ticker("BTC-USDT").await.unwrap();
    assert_eq!(tickers[0].inst_id, "BTC-USDT");
    assert_eq!(tickers[0].last.as_str(), "42000.1");
    assert_eq!(tickers[0].bid_px.parse::<f64>().unwrap(), 42000.0);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("instId=BTC-USDT"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_tickers_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","last":"42000.1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let tickers = client
        .market()
        .get_tickers(crate::model::InstType::Swap, Some("BTC-USDT"))
        .await
        .unwrap();
    assert_eq!(tickers[0].inst_id, "BTC-USDT-SWAP");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_index_tickers_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USD","idxPx":"42000.1","open24h":"41000","high24h":"43000","low24h":"40000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let tickers = client
        .market()
        .get_index_tickers(Some("USD"), None)
        .await
        .unwrap();
    assert_eq!(tickers[0].inst_id, "BTC-USD");
    assert_eq!(tickers[0].idx_px.as_str(), "42000.1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("quoteCcy=USD"));
}

#[tokio::test]
async fn get_orderbook_parses_levels_and_passes_depth() {
    let body = r#"{"code":"0","msg":"","data":[
            {"asks":[["42000.2","1","0","3"]],"bids":[["42000.0","2","0","1"]],
             "ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let books = client
        .market()
        .get_orderbook("BTC-USDT", Some(5))
        .await
        .unwrap();
    let book = &books[0];
    assert_eq!(book.asks[0].price.as_str(), "42000.2");
    assert_eq!(book.asks[0].order_count.as_str(), "3");
    assert_eq!(book.bids[0].size.as_str(), "2");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&sz=5"));
}

#[tokio::test]
async fn get_order_lite_book_queries_instrument() {
    let body = r#"{"code":"0","msg":"","data":[
            {"asks":[["42000.2","1","0","3"]],"bids":[["42000.0","2","0","1"]],
             "ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let books = client
        .market()
        .get_order_lite_book("BTC-USDT")
        .await
        .unwrap();
    assert_eq!(books[0].asks[0].price.as_str(), "42000.2");

    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/market/books-lite?instId=BTC-USDT")
    );
}

#[tokio::test]
async fn get_candlesticks_parses_array_rows() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let candles = client
        .market()
        .get_candlesticks("BTC-USDT", Some("1H"), Some(1))
        .await
        .unwrap();
    assert_eq!(candles[0].open.as_str(), "42000");
    assert_eq!(candles[0].close.as_str(), "42500");
    assert_eq!(candles[0].confirm.as_str(), "1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&bar=1H&limit=1"));
}

#[tokio::test]
async fn get_history_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::CandlesticksRequest::new("BTC-USDT")
        .bar("1H")
        .limit(2);

    let candles = client
        .market()
        .get_history_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].high.as_str(), "43000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&bar=1H&limit=2"));
    assert!(!req.query().unwrap().contains("after"));
}

#[tokio::test]
async fn get_index_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::CandlesticksRequest::new("BTC-USD").after("10");

    let candles = client
        .market()
        .get_index_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].low.as_str(), "41000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USD&after=10"));
}

#[tokio::test]
async fn get_mark_price_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::CandlesticksRequest::new("BTC-USDT-SWAP").before("20");

    let candles = client
        .market()
        .get_mark_price_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].close.as_str(), "42500");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP&before=20"));
}

#[tokio::test]
async fn get_trades_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","tradeId":"1","px":"42000.1","sz":"0.02","side":"buy","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let trades = client
        .market()
        .get_trades("BTC-USDT", Some(1))
        .await
        .unwrap();
    assert_eq!(trades[0].trade_id, "1");
    assert_eq!(trades[0].px.as_str(), "42000.1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&limit=1"));
}

#[tokio::test]
async fn get_history_trades_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","tradeId":"1","px":"42000.1","sz":"0.02","side":"sell","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::HistoryTradesRequest::new("BTC-USDT")
        .trade_type("1")
        .before("100")
        .limit(1);

    let trades = client.market().get_history_trades(&request).await.unwrap();
    assert_eq!(trades[0].side, "sell");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instId=BTC-USDT&type=1&before=100&limit=1")
    );
    assert!(!req.query().unwrap().contains("after"));
}

#[tokio::test]
async fn get_platform_24_volume_parses_volume() {
    let body = r#"{"code":"0","msg":"","data":[{"volUsd":"1000","volCny":"7100"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let volume = client.market().get_platform_24_volume().await.unwrap();
    assert_eq!(volume[0].vol_usd.as_str(), "1000");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/market/platform-24-volume"));
    assert_eq!(req.query(), None);
}

#[tokio::test]
async fn get_index_components_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"index":"BTC-USD","ts":"1597026383085","components":[
                {"exch":"okx","symbol":"BTC-USDT","symPx":"42000","wgt":"1","cnvPx":"42000"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let components = client
        .market()
        .get_index_components("BTC-USD")
        .await
        .unwrap();
    assert_eq!(components[0].components[0].symbol, "BTC-USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("index=BTC-USD"));
}

#[tokio::test]
async fn get_exchange_rate_parses_rate() {
    let body = r#"{"code":"0","msg":"","data":[{"usdCny":"7.1"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rate = client.market().get_exchange_rate().await.unwrap();
    assert_eq!(rate[0].usd_cny.as_str(), "7.1");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/market/exchange-rate"));
    assert_eq!(req.query(), None);
}

#[tokio::test]
async fn get_block_ticker_builds_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","last":"42000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let ticker = client.market().get_block_ticker("BTC-USDT").await.unwrap();
    assert_eq!(ticker[0].inst_id, "BTC-USDT");

    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/market/block-ticker?instId=BTC-USDT")
    );
}

#[tokio::test]
async fn get_block_tickers_builds_filter_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let ticker = client
        .market()
        .get_block_tickers(InstType::Swap, Some("BTC-USDT"))
        .await
        .unwrap();
    assert_eq!(ticker[0].inst_id, "BTC-USDT-SWAP");
    assert_eq!(ticker[0].last.as_str(), "");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
}

#[tokio::test]
async fn get_block_trades_builds_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","tradeId":"1","px":"42000","sz":"0.1","side":"buy","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let trades = client
        .market()
        .get_block_trades("BTC-USDT", Some(1))
        .await
        .unwrap();
    assert_eq!(trades[0].px.as_str(), "42000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&limit=1"));
}

#[tokio::test]
async fn get_option_instrument_family_trades_builds_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USD-240628-50000-C","tradeId":"1","px":"10","sz":"1","side":"sell","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let trades = client
        .market()
        .get_option_instrument_family_trades("BTC-USD")
        .await
        .unwrap();
    assert_eq!(trades[0].side, "sell");

    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/market/option/instrument-family-trades?instFamily=BTC-USD")
    );
}
