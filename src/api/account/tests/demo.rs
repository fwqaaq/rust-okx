use crate::api::account::{
    DemoAdjustBalanceRequest, DemoBalanceAdjustment, DemoBalanceAdjustmentType,
};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

#[tokio::test]
async fn adjust_demo_account_balance_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"remainCnt":"2","totalCnt":"3","details":[{"ccy":"BTC","amt":"0.5","bal":"1.5"},{"ccy":"USDT","amt":"3000","bal":"13000"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone())
        .credentials(Credentials::new("key", "secret", "pass"))
        .demo_trading(true)
        .build();
    let request = DemoAdjustBalanceRequest::new(
        DemoBalanceAdjustmentType::Increase,
        [
            DemoBalanceAdjustment::new("BTC", "0.5"),
            DemoBalanceAdjustment::new("USDT", "3000"),
        ],
    );

    let result = client
        .account()
        .adjust_demo_account_balance(&request)
        .await
        .unwrap();
    assert_eq!(result[0].remain_cnt.as_str(), "2");
    assert_eq!(result[0].total_cnt.as_str(), "3");
    assert_eq!(result[0].details[0].ccy, "BTC");
    assert_eq!(result[0].details[0].amt.as_str(), "0.5");
    assert_eq!(result[0].details[0].bal.as_str(), "1.5");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/demo-adjust-balance"));
    assert_eq!(
        req.headers
            .get("x-simulated-trading")
            .and_then(|value| value.to_str().ok()),
        Some("1")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["type"], "increase");
    assert_eq!(sent["adjustments"][0]["ccy"], "BTC");
    assert_eq!(sent["adjustments"][0]["amt"], "0.5");
    assert_eq!(sent["adjustments"][1]["ccy"], "USDT");
    assert_eq!(sent["adjustments"][1]["amt"], "3000");
    assert!(req.is_signed());
}
