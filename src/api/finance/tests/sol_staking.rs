use http::Method;

use crate::test_util::MockTransport;
use crate::OkxClient;

use super::super::FinanceHistoryRequest;
use super::signed_client;

#[tokio::test]
async fn sol_product_info_is_authenticated() {
    let body = r#"{"code":"0","msg":"","data":{
        "fastRedemptionAvail":"240",
        "fastRedemptionDailyLimit":"240",
        "rate":"5.57",
        "redemptDays":"2",
        "minAmt":"0.01"
    }}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let info = client.finance().sol_staking().product_info().await.unwrap();
    assert_eq!(info.rate.as_str(), "5.57");
    assert_eq!(info.fast_redemption_daily_limit.as_str(), "240");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/sol/product-info"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_purchase_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"789","ccy":"SOL","amt":"1","state":"3","type":"purchase","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().sol_staking().purchase("1").await.unwrap();
    assert_eq!(rows[0].ord_id, "789");
    assert_eq!(rows[0].ccy, "SOL");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/sol/purchase"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["amt"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_redeem_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"790","ccy":"SOL","amt":"1","state":"3","type":"redeem","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().sol_staking().redeem("1").await.unwrap();
    assert_eq!(rows[0].ord_id, "790");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/sol/redeem"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["amt"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_balance_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"SOL","amt":"10","earnings":"0.05"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().sol_staking().balance().await.unwrap();
    assert_eq!(rows[0].ccy, "SOL");
    assert_eq!(rows[0].amt.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/sol/balance"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_purchase_redeem_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"SOL","amt":"1","type":"redeem","state":"3","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FinanceHistoryRequest::new().limit(10);

    let rows = client
        .finance()
        .sol_staking()
        .purchase_redeem_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "SOL");
    assert_eq!(rows[0].event_type, "redeem");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("limit=10"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_apy_history_is_not_signed() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"SOL","apy":"0.0557","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .finance()
        .sol_staking()
        .apy_history("7")
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "SOL");
    assert_eq!(rows[0].apy.as_str(), "0.0557");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("days=7"));
    assert!(!req.is_signed(), "public endpoint must not be signed");
}
