use http::Method;

use crate::OkxClient;
use crate::test_util::MockTransport;

use super::super::{
    CurrencyRequest, FinanceHistoryRequest, SavingsPurchaseRedemptionRequest, SetLendingRateRequest,
};
use super::signed_client;

#[tokio::test]
async fn savings_balance_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1","loanAmt":"0","pendingAmt":"0","earnings":"0.001","rate":"0.01"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = CurrencyRequest::new().currency("USDT");
    let rows = client
        .finance()
        .savings()
        .get_saving_balance(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].amt.as_str(), "1");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/savings/balance?ccy=USDT")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn purchase_redemption_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1","side":"purchase","rate":"0.01"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SavingsPurchaseRedemptionRequest::new("USDT", "1", "purchase").rate("0.01");

    let rows = client
        .finance()
        .savings()
        .purchase_redemption(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].side, "purchase");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/savings/purchase-redempt")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "USDT");
    assert_eq!(sent["amt"], "1");
    assert_eq!(sent["side"], "purchase");
    assert_eq!(sent["rate"], "0.01");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_lending_rate_posts_signed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","rate":"0.02"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = SetLendingRateRequest::new("USDT", "0.02");
    let rows = client
        .finance()
        .savings()
        .set_lending_rate(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].rate.as_str(), "0.02");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/savings/set-lending-rate")
    );
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "USDT");
    assert_eq!(sent["rate"], "0.02");
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_lending_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1","earnings":"0.001","rate":"0.01","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FinanceHistoryRequest::new().currency("USDT").limit(5);

    let rows = client
        .finance()
        .savings()
        .get_lending_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].ts.as_str(), "1597026383085");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT&limit=5"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_public_borrow_history_is_not_signed() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1000","rate":"0.0001","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = FinanceHistoryRequest::new().currency("USDT").limit(1);

    let rows = client
        .finance()
        .savings()
        .get_public_borrow_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].rate.as_str(), "0.0001");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT&limit=1"));
    assert!(!req.is_signed(), "public endpoint must not be signed");
}

#[tokio::test]
async fn get_public_borrow_info_omits_currency_when_absent() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","avgAmt":"5000","avgRate":"0.0002","preRate":"0.0001"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .finance()
        .savings()
        .get_public_borrow_info(&CurrencyRequest::default())
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].avg_amt.as_str(), "5000");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/finance/savings/lending-rate-summary")
    );
    assert_eq!(req.query(), None);
    assert!(!req.is_signed(), "public endpoint must not be signed");
}
