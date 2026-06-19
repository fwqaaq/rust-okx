use http::Method;

use crate::OkxClient;
use crate::test_util::MockTransport;

use super::super::{
    FlexibleLoanAdjustCollateralRequest, FlexibleLoanCollateralAssetsRequest,
    FlexibleLoanHistoryRequest, FlexibleLoanInfoRequest, FlexibleLoanInterestAccruedRequest,
    FlexibleLoanMaxLoanRequest, FlexibleLoanMaxRedeemRequest,
};
use super::signed_client;

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
async fn borrow_currencies_sends_signed_request() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","minLoan":"10","maxLoan":"100000","rate":"0.0001"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .finance()
        .flexible_loan()
        .borrow_currencies()
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].min_loan.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/flexible-loan/borrow-currencies")
    );
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn collateral_assets_is_not_signed() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"BTC","minCollateral":"0.001","maxCollateral":"100","discountRate":"0.85"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = FlexibleLoanCollateralAssetsRequest::new().currency("BTC");

    let rows = client
        .finance()
        .flexible_loan()
        .collateral_assets(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].discount_rate.as_str(), "0.85");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(!req.is_signed(), "public endpoint must not be signed");
}

#[tokio::test]
async fn max_collateral_redeem_amount_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"collateralCcy":"BTC","maxRedeem":"0.5"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanMaxRedeemRequest::new().currency("BTC");

    let rows = client
        .finance()
        .flexible_loan()
        .max_collateral_redeem_amount(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].collateral_ccy, "BTC");
    assert_eq!(rows[0].max_redeem.as_str(), "0.5");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn adjust_collateral_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1001","borrowCcy":"USDT","borrowAmt":"500","collateralCcy":"BTC","collateralAmt":"0.01","state":"normal","cTime":"1597026383085","uTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanAdjustCollateralRequest::new("1001", "BTC", "0.005", "add");

    let rows = client
        .finance()
        .flexible_loan()
        .adjust_collateral(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1001");
    assert_eq!(rows[0].collateral_ccy, "BTC");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/flexible-loan/adjust-collateral")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ordId"], "1001");
    assert_eq!(sent["collateralCcy"], "BTC");
    assert_eq!(sent["amt"], "0.005");
    assert_eq!(sent["type"], "add");
    assert!(req.is_signed());
}

#[tokio::test]
async fn loan_info_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1001","loanToValue":"0.3","liquidationLtv":"0.8","marginCallLtv":"0.7","accruedInterest":"0.001"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanInfoRequest::new().order_id("1001");

    let rows = client
        .finance()
        .flexible_loan()
        .loan_info(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1001");
    assert_eq!(rows[0].loan_to_value.as_str(), "0.3");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ordId=1001"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn loan_history_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1001","type":"borrow","ccy":"USDT","amt":"500","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanHistoryRequest::new().order_id("1001").limit(5);

    let rows = client
        .finance()
        .flexible_loan()
        .loan_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "1001");
    assert_eq!(rows[0].event_type, "borrow");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ordId=1001&limit=5"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn interest_accrued_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1001","ccy":"USDT","interest":"0.0001","rate":"0.0001","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanInterestAccruedRequest::new()
        .currency("USDT")
        .limit(10);

    let rows = client
        .finance()
        .flexible_loan()
        .interest_accrued(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].interest.as_str(), "0.0001");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT&limit=10"));
    assert!(req.is_signed());
}
