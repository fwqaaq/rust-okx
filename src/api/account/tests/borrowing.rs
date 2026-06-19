use crate::model::TradeMode;
use crate::test_util::MockTransport;

use super::super::{
    InterestAccruedRequest, InterestLimitsRequest, MaxLoanRequest, SetAutoEarnRequest,
    SetAutoRepayRequest, SpotBorrowRepayHistoryRequest, SpotManualBorrowRepayRequest,
};
use super::signed_client;

#[tokio::test]
async fn get_max_loan_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT","mgnMode":"cross","mgnCcy":"USDT","maxLoan":"0.59662225","ccy":"","side":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MaxLoanRequest::new("BTC-USDT", TradeMode::Cross).margin_currency("USDT");

    let result = client.account().get_max_loan(&request).await.unwrap();
    assert_eq!(result[0].max_loan.as_str(), "0.59662225");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instId=BTC-USDT&mgnMode=cross&mgnCcy=USDT")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_interest_accrued_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT","ccy":"BTC","mgnMode":"cross","type":"1",
        "interest":"0.0001","interestRate":"0.00001667","liab":"0.006","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = InterestAccruedRequest::new()
        .inst_id("BTC-USDT")
        .currency("BTC")
        .limit(1);

    let result = client
        .account()
        .get_interest_accrued(&request)
        .await
        .unwrap();
    assert_eq!(result[0].interest_rate.as_str(), "0.00001667");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&ccy=BTC&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_interest_rate_queries_currency() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"BTC","interestRate":"0.00007"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .get_interest_rate(Some("BTC"))
        .await
        .unwrap();
    assert_eq!(result[0].ccy, "BTC");
    assert_eq!(result[0].interest_rate.as_str(), "0.00007");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_interest_limits_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","rate":"0.00007","loanQuota":"10","usedLoan":"0.01",
        "interest":"0.001","surplusLmt":"9.99","surplusLmtDetails":{"allAcctRemainingQuota":"9.99","curAcctRemainingQuota":"9.99","platRemainingQuota":"9.99"}}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = InterestLimitsRequest::new().limit_type("1").currency("BTC");

    let result = client
        .account()
        .get_interest_limits(&request)
        .await
        .unwrap();
    assert_eq!(result[0].loan_quota.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.query(), Some("type=1&ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn spot_manual_borrow_repay_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","side":"borrow","amt":"0.01","ordId":"","accBorrowed":"","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SpotManualBorrowRepayRequest::new("BTC", "borrow", "0.01");

    let rows = client
        .account()
        .spot_manual_borrow_repay(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].side, "borrow");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "BTC");
    assert_eq!(sent["side"], "borrow");
    assert_eq!(sent["amt"], "0.01");
}

#[tokio::test]
async fn set_auto_repay_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"autoRepay":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetAutoRepayRequest::new(true);

    client.account().set_auto_repay(&request).await.unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.body_str().contains(r#""autoRepay":true"#));
}

#[tokio::test]
async fn get_spot_borrow_repay_history_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","type":"borrow","amt":"0.01","accBorrowed":"0.02",
        "refundAmt":"0","penaltyAmt":"0","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SpotBorrowRepayHistoryRequest::new().currency("BTC");

    client
        .account()
        .get_spot_borrow_repay_history(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    assert_eq!(req.query(), Some("ccy=BTC"));
}

#[tokio::test]
async fn set_auto_loan_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"autoLoan":"true"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().set_auto_loan(true).await.unwrap();
    assert_eq!(result[0].auto_loan, "true");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-auto-loan"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["autoLoan"], true);
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_auto_earn_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetAutoEarnRequest::new("0", "BTC", "turn_on");

    client.account().set_auto_earn(&request).await.unwrap();

    let req = mock.captured();
    assert!(req.is_signed());
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["earnType"], "0");
    assert_eq!(sent["ccy"], "BTC");
    assert_eq!(sent["action"], "turn_on");
}
