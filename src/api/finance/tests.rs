use http::Method;

use crate::test_util::MockTransport;
use crate::{Credentials, Error, OkxClient};

use super::*;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn savings_balance_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .finance()
        .savings()
        .get_saving_balance(Some("USDT"))
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/savings/balance?ccy=USDT")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn staking_defi_purchase_posts_typed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        StakingDefiPurchaseRequest::new("p1", vec![StakingDefiInvestment::new("USDT", "1")]);

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
    assert_eq!(sent["productId"], "p1");
    assert_eq!(sent["investData"][0]["ccy"], "USDT");
    assert_eq!(sent["investData"][0]["amt"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_and_sol_public_product_info_are_public() {
    let body = r#"{"code":"0","msg":"","data":{
        "fastRedemptionAvail": "240",
        "fastRedemptionDailyLimit": "240",
        "rate": "5.57",
        "redemptDays": "2",
        "minAmt": "0.01"
    }}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().sol_staking().product_info().await.unwrap();
    assert_eq!(rows.rate.as_str(), "5.57");

    let body = r#"{"code":"0","msg":"","data":[{
        "fastRedemptionDailyLimit": "100",
        "rate": "2.23",
        "redemptDays": "8",
        "minAmt": "0.001"
    }]}"#;

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.finance().eth_staking().product_info().await.unwrap();
    assert_eq!(rows[0].fast_redemption_daily_limit.as_str(), "100");
}

#[tokio::test]
async fn eth_cancel_redeem() {
    let mock = MockTransport::new(r#"{"code":"0","data":[{ "ordId":"1234567890"}],"msg":""}"#);
    let client = signed_client(mock.clone());
    let rows = client
        .finance()
        .eth_staking()
        .cancel_redeem("1234567890")
        .await
        .unwrap();

    assert_eq!(rows[0].ord_id, "1234567890");
}

#[tokio::test]
async fn flexible_loan_max_loan_posts_typed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"borrowCcy":"USDT","maxLoan":"100"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanMaxLoanRequest::new("USDT").collateral("BTC", "1");

    let rows = client
        .finance()
        .flexible_loan()
        .max_loan(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].borrow_ccy, "USDT");
    assert_eq!(rows[0].max_loan.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/flexible-loan/max-loan"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["borrowCcy"], "USDT");
    assert_eq!(sent["collateralCcy"], "BTC");
    assert_eq!(sent["collateralAmt"], "1");
    assert!(req.is_signed());
}

#[tokio::test]
async fn invalid_finance_request_fails_before_transport() {
    let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
    let client = signed_client(mock.clone());
    let request = SavingsPurchaseRedemptionRequest::new("USDT", "1", "invalid");

    let error = client
        .finance()
        .savings()
        .purchase_redemption(&request)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());
}
