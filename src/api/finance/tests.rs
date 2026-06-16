use http::Method;

use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

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
async fn staking_defi_purchase_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = StakingDefiPurchaseRequest::new()
        .param("productId", "p1")
        .string_list("investData", ["USDT"]);

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
    assert!(req.body_str().contains(r#""productId":"p1""#));
    assert!(req.is_signed());
}

#[tokio::test]
async fn eth_and_sol_public_product_info_are_public() {
    let body = r#"{"code":"0","msg":"","data":[{"productId":"p1"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client.finance().eth_staking().product_info().await.unwrap();
    assert_eq!(rows[0].product_id, "p1");
    assert!(!mock.captured().is_signed());

    let body = r#"{"code":"0","msg":"","data":[{"productId":"p2"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client.finance().sol_staking().product_info().await.unwrap();
    assert_eq!(rows[0].product_id, "p2");
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn flexible_loan_max_loan_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FlexibleLoanMaxLoanRequest::new()
        .param("borrowCcy", "USDT")
        .string_list("supCollateral", ["BTC", "ETH"]);

    let rows = client
        .finance()
        .flexible_loan()
        .max_loan(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/finance/flexible-loan/max-loan"));
    assert!(req.body_str().contains(r#""supCollateral":["BTC","ETH"]"#));
    assert!(req.is_signed());
}
