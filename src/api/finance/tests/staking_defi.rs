use http::Method;

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
        "ccy":"DOT","productId":"101","protocol":"Polkadot","protocolType":"defi",
        "term":"0","apy":"0.1767","earlyRedeem":false,"state":"purchasable",
        "investData":[{"bal":"0","ccy":"DOT","maxAmt":"0","minAmt":"2"}],
        "earningData":[{"ccy":"DOT","earningType":"0"}],
        "redeemPeriod":["1H","24H"],"fastRedemptionDailyLimit":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiOffersRequest::new().protocol_type("staking");

    let rows = client
        .finance()
        .staking_defi()
        .get_offers(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].product_id, "101");
    assert_eq!(rows[0].protocol_type, "defi");
    assert_eq!(rows[0].protocol, "Polkadot");
    assert!(!rows[0].early_redeem);
    assert_eq!(rows[0].invest_data[0].min_amt.as_str(), "2");
    assert_eq!(rows[0].earning_data[0].earning_type, "0");
    assert_eq!(rows[0].redeem_period, ["1H", "24H"]);

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
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"ETH","ordId":"1","productId":"1234","state":"1","protocol":"eth-staking","protocolType":"staking","term":"0","apy":"0.05","investData":[{"ccy":"ETH","amt":"0.5"}],"earningData":[{"ccy":"ETH","earningType":"0","earnings":"0.01"}],"fastRedemptionData":[{"ccy":"ETH","redeemingAmt":"0.2"}],"purchasedTime":"1597026383085","estSettlementTime":"1597026383086","cancelRedemptionDeadline":"1597026383087","tag":"my-tag"}]}"#;
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
    assert_eq!(rows[0].state, "1");
    assert_eq!(rows[0].invest_data[0].ccy, "ETH");
    assert_eq!(rows[0].invest_data[0].amt.as_str(), "0.5");
    assert_eq!(rows[0].earning_data[0].earning_type, "0");
    assert_eq!(rows[0].earning_data[0].earnings.as_str(), "0.01");
    assert_eq!(
        rows[0].fast_redemption_data[0].redeeming_amt.as_str(),
        "0.2"
    );
    assert_eq!(rows[0].purchased_time.as_str(), "1597026383085");
    assert_eq!(rows[0].tag, "my-tag");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("protocolType=staking"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_orders_history_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","ordId":"1","productId":"1234","state":"3","protocol":"defi-protocol","protocolType":"defi","term":"0","apy":"0.08","investData":[{"ccy":"USDT","amt":"100"}],"earningData":[{"ccy":"USDT","earningType":"1","realizedEarnings":"1.23"}],"purchasedTime":"1712908001000","redeemedTime":"1712914294000","tag":""}]}"#;
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
    assert_eq!(rows[0].state, "3");
    assert_eq!(rows[0].invest_data[0].amt.as_str(), "100");
    assert_eq!(rows[0].earning_data[0].earning_type, "1");
    assert_eq!(rows[0].earning_data[0].realized_earnings.as_str(), "1.23");
    assert_eq!(rows[0].redeemed_time.as_str(), "1712914294000");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("protocolType=defi&limit=5"));
    assert!(req.is_signed());
}
