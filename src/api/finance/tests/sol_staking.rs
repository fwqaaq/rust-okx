use http::Method;

use crate::OkxClient;
use crate::test_util::MockTransport;

use super::super::{AmountRequest, ApyHistoryRequest, FinanceHistoryRequest};
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
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/sol/product-info")
    );
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_purchase_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"789","ccy":"SOL","amt":"1","state":"3","type":"purchase","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = AmountRequest::new("1");
    let rows = client
        .finance()
        .sol_staking()
        .purchase(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "789");
    assert_eq!(rows[0].ccy, "SOL");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/sol/purchase")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["amt"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_redeem_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"790","ccy":"SOL","amt":"1","state":"3","type":"redeem","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = AmountRequest::new("1");
    let rows = client
        .finance()
        .sol_staking()
        .redeem(&request)
        .await
        .unwrap();
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
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"OKSOL","amt":"10","latestInterestAccrual":"0.01","totalInterestAccrual":"0.05"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().sol_staking().balance().await.unwrap();
    assert_eq!(rows[0].ccy, "OKSOL");
    assert_eq!(rows[0].amt.as_str(), "10");
    assert_eq!(rows[0].latest_interest_accrual.as_str(), "0.01");
    assert_eq!(rows[0].total_interest_accrual.as_str(), "0.05");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/sol/balance")
    );
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_purchase_redeem_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{"amt":"1","redeemingAmt":"","status":"success","requestTime":"1683413171000","completedTime":"1683413171000","estCompletedTime":"","type":"redeem"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FinanceHistoryRequest::new().limit(10);

    let rows = client
        .finance()
        .sol_staking()
        .purchase_redeem_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].event_type, "redeem");
    assert_eq!(rows[0].amt.as_str(), "1");
    assert_eq!(rows[0].status, "success");
    assert_eq!(rows[0].request_time.as_str(), "1683413171000");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("limit=10"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn sol_apy_history_is_not_signed() {
    let body = r#"{"code":"0","msg":"","data":[{"rate":"0.0557","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = ApyHistoryRequest::new("7");
    let rows = client
        .finance()
        .sol_staking()
        .apy_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].rate.as_str(), "0.0557");
    assert_eq!(rows[0].ts.as_str(), "1597026383085");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("days=7"));
    assert!(!req.is_signed(), "public endpoint must not be signed");
}
