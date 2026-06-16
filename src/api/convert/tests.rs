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
async fn get_currency_pair_sends_signed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"fromCcy":"BTC","toCcy":"USDT"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .convert()
        .get_currency_pair(Some("BTC"), Some("USDT"))
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/asset/convert/currency-pair?fromCcy=BTC&toCcy=USDT")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn estimate_quote_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"quoteId":"q1","sCode":"0"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertQuoteRequest::new()
        .param("baseCcy", "BTC")
        .param("quoteCcy", "USDT")
        .param("side", "buy")
        .param("rfqSz", "1")
        .param("rfqSzCcy", "USDT");

    let rows = client.convert().estimate_quote(&request).await.unwrap();
    assert_eq!(rows[0].quote_id, "q1");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/convert/estimate-quote"));
    assert!(req.body_str().contains(r#""baseCcy":"BTC""#));
    assert!(req.is_signed());
}

#[tokio::test]
async fn convert_trade_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1","sCode":"0"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertTradeRequest::new()
        .param("quoteId", "q1")
        .param("baseCcy", "BTC")
        .param("quoteCcy", "USDT")
        .param("side", "buy")
        .param("sz", "1")
        .param("szCcy", "USDT");

    let rows = client.convert().convert_trade(&request).await.unwrap();
    assert_eq!(rows[0].ord_id, "1");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/convert/trade"));
    assert!(req.body_str().contains(r#""quoteId":"q1""#));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_convert_history_omits_unset_fields() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertHistoryRequest::new().param("limit", "1");

    client
        .convert()
        .get_convert_history(&request)
        .await
        .unwrap();
    let req = mock.captured();
    assert_eq!(req.query(), Some("limit=1"));
    assert!(req.is_signed());
}
