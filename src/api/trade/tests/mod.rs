use crate::test_util::MockTransport;
use crate::{Credentials, Error, OkxClient};

mod advanced;
mod algo;
mod orders;
mod queries;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn invalid_advanced_trade_request_fails_before_transport() {
    let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
    let client = signed_client(mock.clone());
    let request = super::AlgoOrderRequest::new("BTC-USDT-SWAP", "cross", "buy", "trigger", "1");

    let error = client.trade().place_algo_order(&request).await.unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());
}

#[tokio::test]
async fn cancel_algo_orders_enforces_documented_batch_size_before_transport() {
    let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
    let client = signed_client(mock.clone());

    let error = client.trade().cancel_algo_orders(&[]).await.unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());

    let requests = (0..11)
        .map(|index| super::CancelAlgoOrderRequest::new(index.to_string(), "BTC-USDT"))
        .collect::<Vec<_>>();
    let error = client
        .trade()
        .cancel_algo_orders(&requests)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());
}
