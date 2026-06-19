use crate::test_util::MockTransport;

use super::super::{
    AlgoOrderDetailsRequest, AlgoOrderHistoryRequest, AlgoOrderListRequest, AlgoOrderRequest,
    AmendAlgoOrderRequest, CancelAlgoOrderRequest,
};
use super::signed_client;

#[tokio::test]
async fn place_algo_order_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "algoId":"2510789768709120","algoClOrdId":"","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        AlgoOrderRequest::new("BTC-USDT", "cash", "buy", "conditional", "0.1")
            .take_profit("65000", "68000");

    let rows = client.trade().place_algo_order(&request).await.unwrap();
    assert_eq!(rows[0].algo_id, "2510789768709120");
    assert_eq!(rows[0].s_code, "0");

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.ends_with("/api/v5/trade/order-algo"));
    assert!(req.body_str().contains(r#""ordType":"conditional""#));
}

#[tokio::test]
async fn cancel_algo_orders_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "algoId":"2510789768709120","algoClOrdId":"","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let cancel = CancelAlgoOrderRequest::new("2510789768709120", "BTC-USDT");

    client
        .trade()
        .cancel_algo_orders(&[cancel])
        .await
        .unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.ends_with("/api/v5/trade/cancel-algos"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent[0]["algoId"], "2510789768709120");
    assert_eq!(sent[0]["instId"], "BTC-USDT");
}

#[tokio::test]
async fn amend_algo_order_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "algoId":"2510789768709120","algoClOrdId":"","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let amend = AmendAlgoOrderRequest::new("BTC-USDT")
        .algo_id("2510789768709120")
        .new_size("0.2");

    client.trade().amend_algo_order(&amend).await.unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.ends_with("/api/v5/trade/amend-algos"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["algoId"], "2510789768709120");
    assert_eq!(sent["newSz"], "0.2");
}

#[tokio::test]
async fn get_algo_order_list_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "algoId":"2510789768709120","clOrdId":"","instType":"SPOT","instId":"BTC-USDT",
        "ordId":"","ccy":"","algoClOrdId":"","sz":"0.1","closeFraction":"","ordType":"conditional",
        "side":"buy","posSide":"net","tdMode":"cash","state":"live","lever":"",
        "tpTriggerPx":"68000","tpTriggerPxType":"last","tpOrdPx":"-1",
        "slTriggerPx":"","slTriggerPxType":"last","slOrdPx":"-1",
        "triggerPx":"","triggerPxType":"","ordPx":"",
        "tag":"","actualSz":"","actualPx":"","actualSide":"","pTime":"1597026383085",
        "cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AlgoOrderListRequest::new("conditional");

    let rows = client.trade().get_algo_order_list(&request).await.unwrap();
    assert_eq!(rows[0].state, "live");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ordType=conditional"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_algo_orders_history_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "algoId":"2510789768709120","clOrdId":"","instType":"SPOT","instId":"BTC-USDT",
        "ordId":"","ccy":"","algoClOrdId":"","sz":"0.1","closeFraction":"","ordType":"conditional",
        "side":"buy","posSide":"net","tdMode":"cash","state":"canceled","lever":"",
        "tpTriggerPx":"68000","tpTriggerPxType":"last","tpOrdPx":"-1",
        "slTriggerPx":"","slTriggerPxType":"last","slOrdPx":"-1",
        "cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AlgoOrderHistoryRequest::new("conditional").state("canceled");

    client
        .trade()
        .get_algo_orders_history(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("ordType=conditional&state=canceled")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_algo_order_details_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "algoId":"2510789768709120","clOrdId":"","instType":"SPOT","instId":"BTC-USDT",
        "ordId":"","ccy":"","algoClOrdId":"","sz":"0.1","closeFraction":"","ordType":"conditional",
        "side":"buy","posSide":"net","tdMode":"cash","state":"live","lever":"",
        "cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AlgoOrderDetailsRequest::by_algo_id("2510789768709120");

    client
        .trade()
        .get_algo_order_details(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(req.query(), Some("algoId=2510789768709120"));
    assert!(req.is_signed());
}
