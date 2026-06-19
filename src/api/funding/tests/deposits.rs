use http::Method;

use crate::test_util::MockTransport;

use super::super::{
    DepositHistoryRequest, DepositLightningRequest, DepositWithdrawStatusRequest,
    FundingBillsRequest,
};
use super::signed_client;

#[tokio::test]
async fn get_deposit_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "depId":"DEP123","txId":"0xabc","ccy":"USDT","chain":"USDT-ERC20",
        "amt":"100","state":"2","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = DepositHistoryRequest::new().currency("USDT").limit(5);

    let rows = client
        .funding()
        .get_deposit_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].dep_id, "DEP123");
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].state, "2");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT&limit=5"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_bills_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "billId":"BILL123","ccy":"USDT","balChg":"-100","bal":"900","type":"1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FundingBillsRequest::new().currency("USDT").limit(10);

    let rows = client.funding().get_bills(&request).await.unwrap();
    assert_eq!(rows[0].bill_id, "BILL123");
    assert_eq!(rows[0].ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT&limit=10"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_deposit_lightning_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","amt":"0.001",
        "invoice":"lnbc1m1p3xnhl2pp5jptserfk3zk4qy42tlucycrfwxhydvlemu9pqr93tsuenrngzqa",
        "to":"funding"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = DepositLightningRequest::new("BTC", "0.001");

    let rows = client
        .funding()
        .get_deposit_lightning(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].amt.as_str(), "0.001");
    assert!(!rows[0].invoice.is_empty());

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=BTC&amt=0.001"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_deposit_withdraw_status_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "wdId":"WD123","txId":"0xabc","ccy":"USDT","chain":"USDT-ERC20",
        "to":"0x1234","state":"success"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = DepositWithdrawStatusRequest::new()
        .withdrawal_id("WD123")
        .currency("USDT");

    let rows = client
        .funding()
        .get_deposit_withdraw_status(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].wd_id, "WD123");
    assert_eq!(rows[0].state, "success");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("wdId=WD123&ccy=USDT"));
    assert!(req.is_signed());
}
