use http::Method;

use super::{
    SpreadBooksRequest, SpreadCancelAllAfterRequest, SpreadOrderRequest, SpreadPublicTradesRequest,
    SpreadTradesRequest, SpreadsRequest,
};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn public_spreads_and_books_match_official_examples() {
    let spreads_body = r#"{"code":"0","msg":"","data":[{"sprdId":"BTC-USDT_BTC-USDT-SWAP","sprdType":"linear","state":"live","baseCcy":"BTC","szCcy":"BTC","quoteCcy":"USDT","tickSz":"0.1","minSz":"0.01","lotSz":"0.01","listTime":"1597026383085","expTime":"","uTime":"1597026383085","legs":[{"instId":"BTC-USDT","side":"buy"},{"instId":"BTC-USDT-SWAP","side":"sell"}]}]}"#;
    let mock = MockTransport::new(spreads_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .spread()
        .get_spreads(&SpreadsRequest::new().base_currency("BTC"))
        .await
        .unwrap();
    assert_eq!(rows[0].tick_sz.as_str(), "0.1");
    assert_eq!(mock.captured().query(), Some("baseCcy=BTC"));
    assert!(!mock.captured().is_signed());

    let books_body = r#"{"code":"0","msg":"","data":[{"asks":[["41006.8","0.60038921","1"]],"bids":[["41006.3","0.30178218","2"]],"ts":"1629966436396"}]}"#;
    let mock = MockTransport::new(books_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .spread()
        .get_order_book(&SpreadBooksRequest::new("BTC-USDT_BTC-USDT-SWAP"))
        .await
        .unwrap();
    assert_eq!(rows[0].asks[0][0].as_str(), "41006.8");
    assert_eq!(
        mock.captured().query(),
        Some("sprdId=BTC-USDT_BTC-USDT-SWAP")
    );
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn public_trades_match_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"sprdId":"BTC-USDT_BTC-USDC-SWAP","side":"sell","sz":"0.1","px":"964.1","tradeId":"242720719","ts":"1654161641568"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .spread()
        .get_public_trades(&SpreadPublicTradesRequest::new())
        .await
        .unwrap();

    assert_eq!(rows[0].px.as_str(), "964.1");
    assert_eq!(mock.captured().query(), None);
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn place_order_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"clOrdId":"b15","ordId":"312269865356374016","tag":"","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SpreadOrderRequest::new("BTC-USDT_BTC-USDT-SWAP", "buy", "limit", "2", "2.15")
        .client_order_id("b15");

    let rows = client.spread().place_order(&request).await.unwrap();

    assert_eq!(rows[0].ord_id, "312269865356374016");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"sprdId":"BTC-USDT_BTC-USDT-SWAP","side":"buy","ordType":"limit","sz":"2","px":"2.15","clOrdId":"b15"}"#
    );
    assert_eq!(mock.captured().method, Method::POST);
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn private_trades_match_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"sprdId":"BTC-USDT-SWAP_BTC-USDT-200329","tradeId":"123","ordId":"123445","clOrdId":"b16","tag":"","fillPx":"999","fillSz":"3","state":"filled","side":"buy","execType":"M","ts":"1597026383085","legs":[{"instId":"BTC-USDT-SWAP","px":"20000","sz":"3","szCont":"0.03","side":"buy","fillPnl":"","fee":"","feeCcy":"","tradeId":"1232342342"}],"code":"","msg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .spread()
        .get_trades(&SpreadTradesRequest::new().limit("100"))
        .await
        .unwrap();

    assert_eq!(rows[0].legs[0].sz_cont.as_str(), "0.03");
    assert_eq!(mock.captured().query(), Some("limit=100"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn cancel_all_after_matches_official_shape() {
    let body = r#"{"code":"0","msg":"","data":[{"triggerTime":"1587971460","ts":"1587971400"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .spread()
        .cancel_all_after(&SpreadCancelAllAfterRequest::new("60"))
        .await
        .unwrap();

    assert_eq!(rows[0].trigger_time.as_str(), "1587971460");
    assert_eq!(mock.captured().body_str(), r#"{"timeOut":"60"}"#);
    assert!(mock.captured().is_signed());
}
