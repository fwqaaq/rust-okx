use http::Method;

use crate::Error;
use crate::test_util::MockTransport;

use super::super::{
    StakingDefiActiveOrdersRequest, StakingDefiCancelRequest, StakingDefiInvestment,
    StakingDefiOffersRequest, StakingDefiOrderHistoryRequest, StakingDefiPurchaseRequest,
    StakingDefiRedeemRequest,
};
use super::signed_client;

#[tokio::test]
async fn get_offers_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "productId":"1234","protocolType":"staking","name":"ETH Staking",
        "ccy":"ETH","term":"0","apy":"0.05","state":"purchasable",
        "investData":[{"ccy":"ETH","amt":"0","earnings":"0"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiOffersRequest::new().protocol_type("staking");

    let rows = client
        .finance()
        .staking_defi()
        .get_offers(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].product_id, "1234");
    assert_eq!(rows[0].protocol_type, "staking");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("protocolType=staking"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn staking_defi_purchase_posts_typed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1","productId":"1234","protocolType":"staking","ccy":"ETH","amt":"0.5","state":"1","term":"0","apy":"0.05","cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        StakingDefiPurchaseRequest::new("1234", vec![StakingDefiInvestment::new("ETH", "0.5")]);

    let rows = client
        .finance()
        .staking_defi()
        .purchase(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/purchase"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["productId"], "1234");
    assert_eq!(sent["investData"][0]["ccy"], "ETH");
    assert_eq!(sent["investData"][0]["amt"], "0.5");
    assert!(req.is_signed());
}

#[tokio::test]
async fn redeem_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1","productId":"1234","protocolType":"staking","ccy":"ETH","amt":"0.5","state":"3","term":"0","apy":"0.05","cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiRedeemRequest::new("1", "staking");

    let rows = client
        .finance()
        .staking_defi()
        .redeem(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1");
    assert_eq!(rows[0].state, "3");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/redeem"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ordId"], "1");
    assert_eq!(sent["protocolType"], "staking");
    assert!(req.is_signed());
}

#[tokio::test]
async fn cancel_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1","productId":"1234","protocolType":"staking","ccy":"ETH","amt":"0.5","state":"4","term":"0","apy":"0.05","cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiCancelRequest::new("1", "staking");

    let rows = client
        .finance()
        .staking_defi()
        .cancel(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/staking-defi/cancel"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ordId"], "1");
    assert_eq!(sent["protocolType"], "staking");
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_active_orders_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1","productId":"1234","protocolType":"staking","ccy":"ETH","amt":"0.5","state":"1","term":"0","apy":"0.05","cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiActiveOrdersRequest::new().protocol_type("staking");

    let rows = client
        .finance()
        .staking_defi()
        .get_active_orders(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("protocolType=staking"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_orders_history_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1","productId":"1234","protocolType":"defi","ccy":"USDT","amt":"100","state":"5","term":"0","apy":"0.08","cTime":"1597026383085","uTime":"1597026383086"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiOrderHistoryRequest::new()
        .protocol_type("defi")
        .limit(5);

    let rows = client
        .finance()
        .staking_defi()
        .get_orders_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1");
    assert_eq!(rows[0].protocol_type, "defi");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("protocolType=defi&limit=5"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn invalid_staking_defi_request_fails_before_transport() {
    let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
    let client = signed_client(mock.clone());
    let request = StakingDefiRedeemRequest::new("1", "unknown_type");

    let error = client
        .finance()
        .staking_defi()
        .redeem(&request)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());
}
