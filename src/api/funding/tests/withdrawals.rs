use http::Method;

use crate::test_util::MockTransport;

use super::super::{
    CancelWithdrawalRequest, WithdrawalHistoryRequest, WithdrawalLightningRequest,
    WithdrawalRequest,
};
use super::signed_client;

#[tokio::test]
async fn withdrawal_posts_signed_body() {
    let body =
        r#"{"code":"0","msg":"","data":[{"wdId":"58700","clientId":"","ccy":"USDT","amt":"10"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = WithdrawalRequest::new("USDT", "10", "4", "TN4E3PsU3YfEYvJW7i5YQKQ5P5y3YPAXB")
        .chain("USDT-TRC20");

    let rows = client.funding().withdrawal(&request).await.unwrap();
    assert_eq!(rows[0].wd_id, "58700");
    assert_eq!(rows[0].ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/withdrawal"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "USDT");
    assert_eq!(sent["amt"], "10");
    assert_eq!(sent["dest"], "4");
    assert_eq!(sent["toAddr"], "TN4E3PsU3YfEYvJW7i5YQKQ5P5y3YPAXB");
    assert_eq!(sent["chain"], "USDT-TRC20");
    assert!(req.is_signed());
}

#[tokio::test]
async fn withdrawal_lightning_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"BTC","amt":"0.001","wdId":"LN123"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = WithdrawalLightningRequest::new(
        "BTC",
        "lnbc1m1p3xnhl2pp5jptserfk3zk4qy42tlucycrfwxhydvlemu9pqr93tsuenrngzqa",
    );

    let rows = client
        .funding()
        .withdrawal_lightning(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].wd_id, "LN123");
    assert_eq!(rows[0].ccy, "BTC");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/withdrawal-lightning"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "BTC");
    assert!(!sent["invoice"].as_str().unwrap().is_empty());
    assert!(req.is_signed());
}

#[tokio::test]
async fn cancel_withdrawal_posts_wd_id() {
    let body =
        r#"{"code":"0","msg":"","data":[{"wdId":"58700","clientId":"","ccy":"USDT","amt":"10"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = CancelWithdrawalRequest { wd_id: "58700" };
    let rows = client.funding().cancel_withdrawal(&request).await.unwrap();
    assert_eq!(rows[0].wd_id, "58700");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/cancel-withdrawal"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["wdId"], "58700");
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_withdrawal_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "wdId":"58700","clientId":"","txId":"0xabc","ccy":"USDT","chain":"USDT-ERC20",
        "amt":"10","fee":"0.5","state":"2","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = WithdrawalHistoryRequest::new().currency("USDT").limit(5);

    let rows = client
        .funding()
        .get_withdrawal_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].wd_id, "58700");
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].state, "2");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT&limit=5"));
    assert!(req.is_signed());
}
