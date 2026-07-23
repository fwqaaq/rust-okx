use crate::OkxClient;
use crate::model::InstType;
use crate::test_util::MockTransport;

use super::{
    CandlesRequest, CandlesticksRequest, FullOrderBookRequest, IndexRequest, IndexTickersRequest,
    InstFamilyRequest, InstIdRequest, OrderBookRequest, SbeOrderBookRequest,
    SpreadCandlesticksRequest, SpreadIdRequest, TickersRequest, TradesRequest,
};

#[tokio::test]
async fn get_ticker_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USD-SWAP","last":"9999.99","lastSz":"0.1",
             "askPx":"9999.99","askSz":"11","bidPx":"8888.88","bidSz":"5",
             "open24h":"9000","high24h":"10000","low24h":"8888.88","volCcy24h":"2222",
             "vol24h":"2222","sodUtc0":"2222","sodUtc8":"2222","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = InstIdRequest::new("BTC-USDT");
    let tickers = client.market().get_ticker(&request).await.unwrap();
    assert_eq!(tickers[0].inst_id, "BTC-USD-SWAP");
    assert_eq!(tickers[0].last.as_str(), "9999.99");
    assert_eq!(tickers[0].bid_px.parse::<f64>().unwrap(), 8888.88);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("instId=BTC-USDT"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_tickers_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SWAP","instId":"LTC-USD-SWAP","last":"9999.99","lastSz":"1",
        "askPx":"9999.99","askSz":"11","bidPx":"8888.88","bidSz":"5",
        "open24h":"9000","high24h":"10000","low24h":"8888.88",
        "volCcy24h":"2222","vol24h":"2222","sodUtc0":"0.1","sodUtc8":"0.1","ts":"1597026383085"},
        {"instType":"SWAP","instId":"BTC-USD-SWAP","last":"9999.99","lastSz":"1",
        "askPx":"9999.99","askSz":"11","bidPx":"8888.88","bidSz":"5",
        "open24h":"9000","high24h":"10000","low24h":"8888.88",
        "volCcy24h":"2222","vol24h":"2222","sodUtc0":"0.1","sodUtc8":"0.1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = TickersRequest::new(crate::model::InstType::Swap).inst_family("BTC-USDT");
    let tickers = client.market().get_tickers(&request).await.unwrap();
    assert_eq!(tickers[0].inst_id, "LTC-USD-SWAP");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_index_tickers_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT","idxPx":"43350","high24h":"43649.7","sodUtc0":"43444.1",
        "open24h":"43640.8","low24h":"43261.9","sodUtc8":"43328.7",
        "ts":"1649419644492"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = IndexTickersRequest::new().quote_currency("USD");
    let tickers = client.market().get_index_tickers(&request).await.unwrap();
    assert_eq!(tickers[0].inst_id, "BTC-USDT");
    assert_eq!(tickers[0].idx_px.as_str(), "43350");

    let req = mock.captured();
    assert_eq!(req.query(), Some("quoteCcy=USD"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_orderbook_parses_levels_and_passes_depth() {
    let body = r#"{"code":"0","msg":"","data":[
            {"asks":[["41006.8","0.60038921","0","1"]],"bids":[["41006.3","0.30178218","0","2"]],
             "ts":"1629966436396","seqId":3235851742}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = OrderBookRequest::new("BTC-USDT").size(5);
    let books = client.market().get_orderbook(&request).await.unwrap();
    let book = &books[0];
    assert_eq!(book.asks[0].price.as_str(), "41006.8");
    assert_eq!(book.asks[0].order_count.as_str(), "1");
    assert_eq!(book.bids[0].size.as_str(), "0.30178218");
    assert_eq!(book.seq_id, 3235851742);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&sz=5"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_full_orderbook_parses_three_field_levels() {
    let body = r#"{"code":"0","msg":"","data":[
            {"asks":[["41006.8","0.60038921","1"]],"bids":[["41006.3","0.30178218","2"]],
             "ts":"1629966436396"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = FullOrderBookRequest::new("BTC-USDT").size(1);
    let books = client
        .market()
        .get_full_orderbook(&request)
        .await
        .unwrap();
    assert_eq!(books[0].asks[0].price.as_str(), "41006.8");
    assert_eq!(books[0].asks[0].order_count.as_str(), "1");
    assert_eq!(books[0].bids[0].size.as_str(), "0.30178218");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&sz=1"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_sbe_orderbook_parses_documented_json_error() {
    let body = r#"{"code":"51000","msg":"Parameter instIdCode error","data":[]}"#;
    let mock = MockTransport::new(body).with_content_type("application/json");
    let client = OkxClient::with_transport(mock.clone()).build();

    let error = client
        .market()
        .get_sbe_orderbook(&SbeOrderBookRequest::new(12345))
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        crate::Error::Rest(crate::RestError::Okx { code, .. }) if code == "51000"
    ));

    let req = mock.captured();
    assert_eq!(req.query(), Some("instIdCode=12345&source=0"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_candlesticks_parses_array_rows() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","3.721","3.743","3.677","3.708","8422410","22698348.04828491","12698348.04828491","0"],
            ["1597026383085","3.731","3.799","3.494","3.72","24912403","67632347.24399722","37632347.24399722","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = CandlesRequest::new("BTC-USDT").bar("1H").limit(1);
    let candles = client.market().get_candlesticks(&request).await.unwrap();
    assert_eq!(candles[0].open.as_str(), "3.721");
    assert_eq!(candles[0].close.as_str(), "3.708");
    assert_eq!(candles[0].confirm.as_str(), "0");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&bar=1H&limit=1"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_history_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","3.721","3.743","3.677","3.708","8422410","22698348.04828491","12698348.04828491","1"],
            ["1597026383085","3.731","3.799","3.494","3.72","24912403","67632347.24399722","37632347.24399722","1"]]}"#;
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
    assert_eq!(candles[0].high.as_str(), "3.743");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&bar=1H&limit=2"));
    assert!(!req.query().unwrap().contains("after"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_index_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","3.721","3.743","3.677","3.708","0"],
            ["1597026383085","3.731","3.799","3.494","3.72","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::CandlesticksRequest::new("BTC-USD").after("10");

    let candles = client
        .market()
        .get_index_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].low.as_str(), "3.677");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USD&after=10"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_history_index_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","3.721","3.743","3.677","3.708","1"],
            ["1597026383085","3.731","3.799","3.494","3.72","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = CandlesticksRequest::new("BTC-USD").after("10");

    let candles = client
        .market()
        .get_history_index_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].low.as_str(), "3.677");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USD&after=10"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_mark_price_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","3.721","3.743","3.677","3.708","0"],
            ["1597026383085","3.731","3.799","3.494","3.72","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::CandlesticksRequest::new("BTC-USDT-SWAP").before("20");

    let candles = client
        .market()
        .get_mark_price_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].close.as_str(), "3.708");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP&before=20"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_history_mark_price_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","3.721","3.743","3.677","3.708","1"],
            ["1597026383085","3.731","3.799","3.494","3.72","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = CandlesticksRequest::new("BTC-USD-SWAP").before("20");

    let candles = client
        .market()
        .get_history_mark_price_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].close.as_str(), "3.708");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USD-SWAP&before=20"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_trades_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","side":"sell","sz":"0.00001","source":"0","px":"29963.2","tradeId":"242720720","ts":"1654161646974"},
            {"instId":"BTC-USDT","side":"sell","sz":"0.00001","source":"0","px":"29964.1","tradeId":"242720719","ts":"1654161641568"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = TradesRequest::new("BTC-USDT").limit(1);
    let trades = client.market().get_trades(&request).await.unwrap();
    assert_eq!(trades[0].trade_id, "242720720");
    assert_eq!(trades[0].px.as_str(), "29963.2");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&limit=1"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_history_trades_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","side":"sell","sz":"0.00001","source":"0","px":"29963.2","tradeId":"242720720","ts":"1654161646974"},
            {"instId":"BTC-USDT","side":"sell","sz":"0.00001","source":"0","px":"29964.1","tradeId":"242720719","ts":"1654161641568"}]}"#;
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
    assert!(!req.is_signed());
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
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_call_auction_details_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"ONDO-USDC","unmatchedSz":"9988764","eqPx":"0.6",
        "matchedSz":"44978","state":"continuous_trading",
        "auctionEndTime":"1726542000000","ts":"1726542000007"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let details = client
        .market()
        .get_call_auction_details(&InstIdRequest::new("ONDO-USDC"))
        .await
        .unwrap();
    assert_eq!(details[0].inst_id, "ONDO-USDC");
    assert_eq!(details[0].eq_px.as_str(), "0.6");
    assert_eq!(details[0].state, "continuous_trading");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=ONDO-USDC"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_spread_ticker_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "sprdId":"BTC-USDT_BTC-USDT-SWAP","last":"14.5","lastSz":"0.5",
        "askPx":"8.5","askSz":"12.0","bidPx":"0.5","bidSz":"12.0",
        "open24h":"4","high24h":"14.5","low24h":"-2.2","vol24h":"6.67",
        "ts":"1715331406485"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let ticker = client
        .market()
        .get_spread_ticker(&SpreadIdRequest::new("BTC-USDT_BTC-USDT-SWAP"))
        .await
        .unwrap();
    assert_eq!(ticker[0].sprd_id, "BTC-USDT_BTC-USDT-SWAP");
    assert_eq!(ticker[0].low24h.as_str(), "-2.2");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("sprdId=BTC-USDT_BTC-USDT-SWAP")
    );
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_spread_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        ["1597026383085","3.721","3.743","3.677","3.708","8422410","0"],
        ["1597026383085","3.731","3.799","3.494","3.72","24912403","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = SpreadCandlesticksRequest::new("BTC-USDT_BTC-USDT-SWAP")
        .bar("1H")
        .limit(2);

    let candles = client
        .market()
        .get_spread_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].vol.as_str(), "8422410");
    assert_eq!(candles[0].confirm.as_str(), "0");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("sprdId=BTC-USDT_BTC-USDT-SWAP&bar=1H&limit=2")
    );
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_spread_history_candlesticks_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
        ["1597026383085","3.721","3.743","3.677","3.708","8422410","1"],
        ["1597026383085","3.731","3.799","3.494","3.72","24912403","1"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request =
        SpreadCandlesticksRequest::new("BTC-USDT_BTC-USDT-SWAP").after("1597026383085");

    let candles = client
        .market()
        .get_spread_history_candlesticks(&request)
        .await
        .unwrap();
    assert_eq!(candles[0].open.as_str(), "3.721");
    assert_eq!(candles[0].confirm.as_str(), "1");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("sprdId=BTC-USDT_BTC-USDT-SWAP&after=1597026383085")
    );
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_index_components_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"index":"BTC-USD","ts":"1597026383085","components":[
                {"exch":"okx","symbol":"BTC-USDT","symPx":"42000","wgt":"1","cnvPx":"42000"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = IndexRequest::new("BTC-USD");
    let components = client
        .market()
        .get_index_components(&request)
        .await
        .unwrap();
    assert_eq!(components[0].components[0].symbol, "BTC-USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("index=BTC-USD"));
    assert!(!req.is_signed());
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
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_block_ticker_builds_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SWAP","instId":"BTC-USD-SWAP","last":"9999.99","lastSz":"0.1",
        "askPx":"9999.99","askSz":"11","bidPx":"8888.88","bidSz":"5",
        "open24h":"9000","high24h":"10000","low24h":"8888.88",
        "volCcy24h":"2222","vol24h":"2222","sodUtc0":"2222","sodUtc8":"2222","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = InstIdRequest::new("BTC-USDT");
    let ticker = client.market().get_block_ticker(&request).await.unwrap();
    assert_eq!(ticker[0].inst_id, "BTC-USD-SWAP");

    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/market/block-ticker?instId=BTC-USDT")
    );
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_block_tickers_builds_filter_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SWAP","instId":"LTC-USD-SWAP","last":"9999.99","lastSz":"1",
        "askPx":"9999.99","askSz":"11","bidPx":"8888.88","bidSz":"5",
        "open24h":"9000","high24h":"10000","low24h":"8888.88",
        "volCcy24h":"2222","vol24h":"2222","sodUtc0":"0.1","sodUtc8":"0.1","ts":"1597026383085"},
        {"instType":"SWAP","instId":"BTC-USD-SWAP","last":"9999.99","lastSz":"1",
        "askPx":"9999.99","askSz":"11","bidPx":"8888.88","bidSz":"5",
        "open24h":"9000","high24h":"10000","low24h":"8888.88",
        "volCcy24h":"2222","vol24h":"2222","sodUtc0":"0.1","sodUtc8":"0.1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = TickersRequest::new(InstType::Swap).inst_family("BTC-USDT");
    let ticker = client.market().get_block_tickers(&request).await.unwrap();
    assert_eq!(ticker[0].inst_id, "LTC-USD-SWAP");
    assert_eq!(ticker[0].last.as_str(), "9999.99");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_option_instrument_family_trades_builds_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"vol24h":"103381","tradeInfo":[
                {"instId":"BTC-USD-221111-17750-C","side":"sell","sz":"1","px":"0.0075","tradeId":"20","ts":"1668090715058"},
                {"instId":"BTC-USD-221111-17750-C","side":"sell","sz":"91","px":"0.01","tradeId":"19","ts":"1668090421062"}
            ],"optType":"C"},
            {"vol24h":"144499","tradeInfo":[
                {"instId":"BTC-USD-230127-10000-P","side":"sell","sz":"82","px":"0.019","tradeId":"23","ts":"1668090967057"},
                {"instId":"BTC-USD-221111-16250-P","side":"sell","sz":"102","px":"0.0045","tradeId":"24","ts":"1668090885050"}
            ],"optType":"P"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = InstFamilyRequest::new("BTC-USD");
    let trades = client
        .market()
        .get_option_instrument_family_trades(&request)
        .await
        .unwrap();
    assert_eq!(trades[0].opt_type, "C");

    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/market/option/instrument-family-trades?instFamily=BTC-USD")
    );
    assert!(!req.is_signed());
}
