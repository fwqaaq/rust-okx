use crate::test_util::MockTransport;

use super::super::{
    EasyConvertHistoryRequest, EasyConvertRequest, OneClickRepayCurrencyListRequest,
    OneClickRepayHistoryRequest, OneClickRepayRequest,
};
use super::signed_client;

#[tokio::test]
async fn get_easy_convert_currency_list_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{
        "fromData":[{"fromCcy":"BTC","fromAmt":"0.00100168","usdAmt":"51.34"}],
        "toCcy":["USDT","OKB","OKS","ETH","DOT","OKT"]
    }]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .trade()
        .get_easy_convert_currency_list()
        .await
        .unwrap();
    assert_eq!(rows[0].to_ccy[0], "USDT");
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn easy_convert_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "fillFromSz":"0.001","fillToSz":"51.36","fromCcy":"BTC","toCcy":"USDT",
        "status":"filled","uTime":"1654090400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = EasyConvertRequest::new(["BTC"], "USDT");

    client.trade().easy_convert(&request).await.unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["fromCcy"][0], "BTC");
    assert_eq!(sent["toCcy"], "USDT");
}

#[tokio::test]
async fn get_easy_convert_history_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "fillFromSz":"0.001","fillToSz":"51.36","fromCcy":"BTC","toCcy":"USDT",
        "acct":"1","status":"filled","uTime":"1654090400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = EasyConvertHistoryRequest::new().after("1654090400000");

    client
        .trade()
        .get_easy_convert_history(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert_eq!(req.query(), Some("after=1654090400000"));
}

#[tokio::test]
async fn get_one_click_repay_currency_list_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{
        "debtData":[{"debtCcy":"USDT","debtAmt":"100","debtUsdAmt":"100"}],
        "debtType":"cross",
        "repayData":[{"repayCcy":"BTC","repayAmt":"0.001"}]
    }]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let list_request = OneClickRepayCurrencyListRequest::new().debt_type("cross");

    client
        .trade()
        .get_one_click_repay_currency_list(&list_request)
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(req.query(), Some("debtType=cross"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn one_click_repay_v1_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "debtCcy":"USDT","repayCcy":"BTC","repayAllDebt":true,
        "fillDebtSz":"100","fillRepaySz":"0.001","status":"filled","uTime":"1654090400000","ts":"1654090400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OneClickRepayRequest::new(["USDT"], "BTC");

    client.trade().one_click_repay(&request).await.unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.ends_with("/api/v5/trade/one-click-repay"));
}

#[tokio::test]
async fn one_click_repay_v2_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "debtCcy":"USDT","repayCcy":"BTC","repayAllDebt":true,
        "fillDebtSz":"100","fillRepaySz":"0.001","status":"filled","uTime":"1654090400000","ts":"1654090400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OneClickRepayRequest::v2("USDT", ["BTC"]);

    client.trade().one_click_repay_v2(&request).await.unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.ends_with("/api/v5/trade/one-click-repay-v2"));
}

#[tokio::test]
async fn get_one_click_repay_history_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "debtCcy":"USDT","repayCcy":"BTC","fillDebtSz":"100","fillRepaySz":"0.001",
        "status":"filled","uTime":"1654090400000","ts":"1654090400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OneClickRepayHistoryRequest::new().after("1654090400000");

    client
        .trade()
        .get_one_click_repay_history(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert_eq!(req.query(), Some("after=1654090400000"));
}

#[tokio::test]
async fn get_one_click_repay_history_v2_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "debtCcy":"USDT","repayCcy":"BTC","fillDebtSz":"100","fillRepaySz":"0.001",
        "status":"filled","uTime":"1654090400000","ts":"1654090400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OneClickRepayHistoryRequest::new().after("1654090400000");

    client
        .trade()
        .get_one_click_repay_history_v2(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.contains("one-click-repay-history-v2"));
}
