use http::Method;

use crate::test_util::MockTransport;

use super::super::{FundsTransferRequest, TransferStateRequest};
use super::signed_client;

#[tokio::test]
async fn funds_transfer_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"transId":"754147","ccy":"USDT","amt":"1.5"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FundsTransferRequest::new("USDT", "1.5", "6", "18");

    let rows = client.funding().funds_transfer(&request).await.unwrap();
    assert_eq!(rows[0].trans_id, "754147");
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].amt.as_str(), "1.5");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/transfer"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "USDT");
    assert_eq!(sent["amt"], "1.5");
    assert_eq!(sent["from"], "6");
    assert_eq!(sent["to"], "18");
    assert!(sent.get("subAcct").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn transfer_state_queries_trans_id() {
    let body = r#"{"code":"0","msg":"","data":[{
        "transId":"754147","state":"success","ccy":"USDT","amt":"1.5",
        "from":"6","to":"18"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = TransferStateRequest::new("754147");
    let rows = client.funding().transfer_state(&request).await.unwrap();
    assert_eq!(rows[0].trans_id, "754147");
    assert_eq!(rows[0].state, "success");
    assert_eq!(rows[0].from_account, "6");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("transId=754147"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn transfer_state_includes_type_when_set() {
    let body = r#"{"code":"0","msg":"","data":[{"transId":"754147","state":"success","ccy":"USDT","amt":"1","from":"6","to":"18"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = TransferStateRequest::new("754147").transfer_type("1");
    client.funding().transfer_state(&request).await.unwrap();

    let req = mock.captured();
    assert_eq!(req.query(), Some("transId=754147&type=1"));
    assert!(req.is_signed());
}
