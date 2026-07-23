use http::Method;

use super::signed_client;
use crate::api::finance::{OkusdRedeemRequest, OkusdRedeemType, OkusdSubscribeRequest};
use crate::test_util::MockTransport;

#[tokio::test]
async fn limits_match_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"subLimit":{"maxSubAmt":"45000000","personalDailyLimit":"5000000","personalUsedAmt":"500000","platformDailyLimit":"50000000","platformUsedAmt":"5000000"},"fastRedeemLimit":{"personalDailyLimit":"10000","personalUsedAmt":"0","platformDailyLimit":"5000000","platformUsedAmt":"1000000","feeRate":"0.001"},"stdRedeemLimit":{"personalDailyLimit":"1000000","personalUsedAmt":"0","platformDailyLimit":"40000000","platformUsedAmt":"0","feeRate":"0.00025"},"ts":"1718500000000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().okusd().get_limits().await.unwrap();

    assert_eq!(rows[0].sub_limit.max_sub_amt.as_str(), "45000000");
    assert_eq!(rows[0].fast_redeem_limit.fee_rate.as_str(), "0.001");
    assert_eq!(rows[0].std_redeem_limit.fee_rate.as_str(), "0.00025");
    assert_eq!(mock.captured().method, Method::GET);
    assert_eq!(mock.captured().query(), None);
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn subscribe_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"680012345678901234","clOrdId":"my-sub-001","ccy":"USDT","amt":"1000.00000000","okusdAmt":"1000.00000000","state":"success","ts":"1718500000000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OkusdSubscribeRequest::new("1000.00000000", "my-sub-001");

    let rows = client.finance().okusd().subscribe(&request).await.unwrap();

    assert_eq!(rows[0].okusd_amt.as_str(), "1000.00000000");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"amt":"1000.00000000","clOrdId":"my-sub-001"}"#
    );
    assert_eq!(mock.captured().method, Method::POST);
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn redeem_matches_official_fast_example() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"680012345678905678","clOrdId":"my-redeem-001","ccy":"OKUSD","amt":"1000.00000000","fee":"1.00000000","usdtAmt":"999.00000000","redeemType":"1","state":"success","estSettlementTime":"1718500010000","ts":"1718500000000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OkusdRedeemRequest::new("1000.00000000", OkusdRedeemType::Fast, "my-redeem-001");

    let rows = client.finance().okusd().redeem(&request).await.unwrap();

    assert_eq!(rows[0].usdt_amt.as_str(), "999.00000000");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"amt":"1000.00000000","redeemType":"1","clOrdId":"my-redeem-001"}"#
    );
    assert_eq!(mock.captured().method, Method::POST);
    assert!(mock.captured().is_signed());
}
