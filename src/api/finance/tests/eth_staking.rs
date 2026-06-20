use http::Method;

use crate::OkxClient;
use crate::test_util::MockTransport;

use super::super::{AmountRequest, ApyHistoryRequest, CancelRedeemRequest, FinanceHistoryRequest};
use super::signed_client;

#[tokio::test]
async fn eth_product_info_is_authenticated() {
    let body = r#"{"code":"0","msg":"","data":[{
        "fastRedemptionDailyLimit":"100",
        "rate":"2.23",
        "redemptDays":"8",
        "minAmt":"0.001"
    }]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().eth_staking().product_info().await.unwrap();
    assert_eq!(rows[0].fast_redemption_daily_limit.as_str(), "100");
    assert_eq!(rows[0].rate.as_str(), "2.23");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/eth/product-info")
    );
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_purchase_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"123","ccy":"ETH","amt":"0.1","state":"3","type":"purchase","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = AmountRequest { amt: "0.1" };
    let rows = client
        .finance()
        .eth_staking()
        .purchase(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "123");
    assert_eq!(rows[0].ccy, "ETH");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/eth/purchase")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["amt"], "0.1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_redeem_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"456","ccy":"ETH","amt":"0.1","state":"3","type":"redeem","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = AmountRequest { amt: "0.1" };
    let rows = client
        .finance()
        .eth_staking()
        .redeem(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "456");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/eth/redeem"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["amt"], "0.1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_cancel_redeem() {
    let mock = MockTransport::new(r#"{"code":"0","data":[{"ordId":"1234567890"}],"msg":""}"#);
    let client = signed_client(mock.clone());

    let request = CancelRedeemRequest {
        ord_id: "1234567890",
    };
    let rows = client
        .finance()
        .eth_staking()
        .cancel_redeem(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1234567890");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/eth/cancel-redeem")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ordId"], "1234567890");
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_balance_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"ETH","amt":"1.5","earnings":"0.01"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().eth_staking().balance().await.unwrap();
    assert_eq!(rows[0].ccy, "ETH");
    assert_eq!(rows[0].amt.as_str(), "1.5");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/staking-defi/eth/balance")
    );
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_purchase_redeem_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"ETH","amt":"0.1","type":"purchase","state":"3","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FinanceHistoryRequest::new().limit(5);

    let rows = client
        .finance()
        .eth_staking()
        .purchase_redeem_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "ETH");
    assert_eq!(rows[0].event_type, "purchase");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("limit=5"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_apy_history_is_not_signed() {
    let body =
        r#"{"code":"0","msg":"","data":[{"ccy":"ETH","apy":"0.0223","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let request = ApyHistoryRequest { days: "30" };
    let rows = client
        .finance()
        .eth_staking()
        .apy_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "ETH");
    assert_eq!(rows[0].apy.as_str(), "0.0223");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("days=30"));
    assert!(!req.is_signed(), "public endpoint must not be signed");
}
