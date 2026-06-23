use http::Method;

use crate::test_util::MockTransport;

use super::super::{CurrencyRequest, DepositAddressRequest};
use super::signed_client;

#[tokio::test]
async fn get_currencies_sends_signed_get() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","name":"Bitcoin","chain":"BTC-Bitcoin",
        "minWd":"0.001","minDep":"0.001","minFee":"0.0005",
        "canDep":true,"canWd":true,"canInternal":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = CurrencyRequest::new().currency("BTC");
    let rows = client.funding().get_currencies(&request).await.unwrap();
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].name, "Bitcoin");
    assert!(rows[0].can_dep);

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_currencies_omits_ccy_when_absent() {
    let body = r#"{"code":"0","msg":"","data":[]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    client
        .funding()
        .get_currencies(&CurrencyRequest::default())
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_balances_sends_signed_get() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"USDT","bal":"1000","frozenBal":"0","availBal":"1000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = CurrencyRequest::new().currency("USDT");
    let rows = client.funding().get_balances(&request).await.unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].bal.as_str(), "1000");
    assert_eq!(rows[0].avail_bal.as_str(), "1000");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_non_tradable_assets_sends_signed_get() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"OKB","amt":"10","type":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .funding()
        .get_non_tradable_assets(&CurrencyRequest::default())
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "OKB");
    assert_eq!(rows[0].amt.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(req.uri.ends_with("/api/v5/asset/non-tradable-assets"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_deposit_address_queries_required_currency() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"USDT","chain":"USDT-TRC20",
        "addr":"TN4E3PsU3YfEYvJW7i5YQKQ5P5y3YPAXB","tag":"","selected":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = DepositAddressRequest::new("USDT");
    let rows = client
        .funding()
        .get_deposit_address(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert_eq!(rows[0].chain, "USDT-TRC20");
    assert!(rows[0].selected);

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("ccy=USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_asset_valuation_omits_currency_when_absent() {
    let body = r#"{"code":"0","msg":"","data":[{
        "details":{"funding":"500","trading":"1000","earn":"200","classic":"0"},
        "totalBal":"1700"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .funding()
        .get_asset_valuation(&CurrencyRequest::default())
        .await
        .unwrap();
    assert_eq!(rows[0].total_bal.as_str(), "1700");
    assert_eq!(rows[0].details.funding.as_str(), "500");

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(req.uri.ends_with("/api/v5/asset/asset-valuation"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}
