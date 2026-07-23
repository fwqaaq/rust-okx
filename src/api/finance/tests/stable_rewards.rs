use crate::OkxClient;
use crate::test_util::MockTransport;

use super::super::{
    StableRewardsApyHistoryRequest, StableRewardsBalanceRequest, StableRewardsCurrencyRequest,
};
use super::signed_client;

#[tokio::test]
async fn product_info_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"details":[{"ccy":"USDG","settleCcy":"USDC","subFeeRate":"0.0003","redemptFeeRate":"0","minSubAmt":"1","minRedeemAmt":"0.0000001","remainingSubQuota":"1000000","remainingRedemptQuota":"500000","canRedeem":true},{"ccy":"USDG","settleCcy":"USDT","subFeeRate":"0.0003","redemptFeeRate":"","minSubAmt":"1","minRedeemAmt":"","remainingSubQuota":"1000000","remainingRedemptQuota":"","canRedeem":false}],"ts":"1718035200000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .finance()
        .stable_rewards()
        .get_product_info(&StableRewardsCurrencyRequest::new("USDG"))
        .await
        .unwrap();

    assert_eq!(rows[0].details[0].settle_ccy, "USDC");
    assert!(rows[0].details[0].can_redeem);
    assert_eq!(rows[0].details[1].redempt_fee_rate.as_str(), "");
    assert_eq!(mock.captured().query(), Some("ccy=USDG"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn balance_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"details":[{"ccy":"USDG","amt":"100","totalEarnAccrual":"0.0003","state":"earning"}],"ts":"1718035200000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .finance()
        .stable_rewards()
        .get_balance(&StableRewardsBalanceRequest::new().currency("USDG"))
        .await
        .unwrap();

    assert_eq!(rows[0].details[0].amt.as_str(), "100");
    assert_eq!(rows[0].details[0].state, "earning");
    assert_eq!(mock.captured().query(), Some("ccy=USDG"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn apy_history_is_public_and_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"rate":"0.004","ts":"1718035200000"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .finance()
        .stable_rewards()
        .get_apy_history(&StableRewardsApyHistoryRequest::new("USDG").days(100))
        .await
        .unwrap();

    assert_eq!(rows[0].rate.as_str(), "0.004");
    assert_eq!(mock.captured().query(), Some("ccy=USDG&days=100"));
    assert!(!mock.captured().is_signed());
}
