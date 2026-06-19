use http::Method;
use serde_json::json;

use crate::model::{OrderSide, RequestValidationError, ValidateRequest};
use crate::test_util::MockTransport;
use crate::{Credentials, Error, OkxClient};

use super::*;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[test]
fn currency_pair_serializes_official_fields() {
    let request =
        ConvertCurrencyPairRequest::new("USDT", "BTC").convert_mode(ConvertMode::LargeOrderVip);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "fromCcy": "USDT",
            "toCcy": "BTC",
            "convertMode": "1"
        })
    );
}

#[test]
fn estimate_quote_serializes_official_example() {
    let request = ConvertQuoteRequest::new("ETH", "USDT", OrderSide::Buy, "30", "USDT")
        .client_quote_request_id("quoteRequest1")
        .tag("broker-tag")
        .convert_mode(ConvertMode::Standard);

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "baseCcy": "ETH",
            "quoteCcy": "USDT",
            "side": "buy",
            "rfqSz": "30",
            "rfqSzCcy": "USDT",
            "clQReqId": "quoteRequest1",
            "tag": "broker-tag",
            "convertMode": "0"
        })
    );
}

#[test]
fn convert_trade_serializes_official_example() {
    let request = ConvertTradeRequest::new(
        "quoterETH-USDT16461885104612381",
        "ETH",
        "USDT",
        OrderSide::Buy,
        "30",
        "USDT",
    )
    .client_trade_request_id("tradeRequest1");

    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({
            "quoteId": "quoterETH-USDT16461885104612381",
            "baseCcy": "ETH",
            "quoteCcy": "USDT",
            "side": "buy",
            "sz": "30",
            "szCcy": "USDT",
            "clTReqId": "tradeRequest1"
        })
    );
}

#[test]
fn history_query_uses_string_wire_values_and_omits_unset_fields() {
    let request = ConvertHistoryRequest::new()
        .after(1_596_386_767_954)
        .before(1_596_386_768_000)
        .limit(100);

    assert_eq!(
        serde_urlencoded::to_string(request).unwrap(),
        "after=1596386767954&before=1596386768000&limit=100"
    );
}

#[test]
fn typed_requests_validate_documented_constraints() {
    let empty_currency = ConvertCurrencyPairRequest::new("", "BTC")
        .validate()
        .unwrap_err();
    assert_eq!(
        empty_currency,
        RequestValidationError::EmptyField { field: "fromCcy" }
    );

    let invalid_id = ConvertQuoteRequest::new("ETH", "USDT", OrderSide::Buy, "30", "USDT")
        .client_quote_request_id("not-valid!")
        .validate()
        .unwrap_err();
    assert_eq!(
        invalid_id,
        RequestValidationError::InvalidFormat {
            field: "clQReqId",
            expected: "1-32 ASCII alphanumeric characters",
        }
    );

    let invalid_side = ConvertTradeRequest::new(
        "q1",
        "ETH",
        "USDT",
        OrderSide::Unknown("hold".to_owned()),
        "30",
        "USDT",
    )
    .validate()
    .unwrap_err();
    assert_eq!(
        invalid_side,
        RequestValidationError::InvalidFormat {
            field: "side",
            expected: "buy or sell",
        }
    );

    let invalid_limit = ConvertHistoryRequest::new()
        .limit(0)
        .validate()
        .unwrap_err();
    assert_eq!(
        invalid_limit,
        RequestValidationError::OutOfRange {
            field: "limit",
            min: 1,
            max: 100,
        }
    );
}

#[tokio::test]
async fn get_currencies_sends_no_query_string() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"BTC","name":"Bitcoin","logoLink":"https://static.okx.com/cdn/wallet/logo/BTC.png","tag":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.convert().get_currencies().await.unwrap();
    assert_eq!(rows.len(), 1);

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(req.uri.ends_with("/api/v5/asset/convert/currencies"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_currency_pair_sends_signed_typed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"instId":"BTC-USDT","baseCcy":"BTC","baseCcyMax":"0.5","baseCcyMin":"0.0001","quoteCcy":"USDT","quoteCcyMax":"10000","quoteCcyMin":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertCurrencyPairRequest::new("USDT", "BTC");

    let rows = client.convert().get_currency_pair(&request).await.unwrap();
    assert_eq!(rows.len(), 1);

    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/asset/convert/currency-pair?fromCcy=USDT&toCcy=BTC")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn estimate_quote_posts_exact_typed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"baseCcy":"BTC","baseSz":"0.00003","clQReqId":"","cnvtPx":"30000","origRfqSz":"1","quoteCcy":"USDT","quoteId":"q1","quoteSz":"1","quoteTime":"1646188510461","rfqSz":"1","rfqSzCcy":"USDT","side":"buy","ttlMs":"10000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertQuoteRequest::new("BTC", "USDT", OrderSide::Buy, "1", "USDT");

    let rows = client.convert().estimate_quote(&request).await.unwrap();
    assert_eq!(rows[0].quote_id, "q1");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/convert/estimate-quote"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(req.body_str()).unwrap(),
        json!({
            "baseCcy": "BTC",
            "quoteCcy": "USDT",
            "side": "buy",
            "rfqSz": "1",
            "rfqSzCcy": "USDT"
        })
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn convert_trade_posts_exact_typed_body() {
    let body = r#"{"code":"0","msg":"","data":[{"baseCcy":"BTC","clTReqId":"","fillBaseSz":"0.00003","fillPx":"30000","fillQuoteSz":"1","instId":"BTC-USDT","quoteCcy":"USDT","quoteId":"q1","side":"buy","state":"fullyFilled","tradeId":"1","ts":"1646188520338"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertTradeRequest::new("q1", "BTC", "USDT", OrderSide::Buy, "1", "USDT");

    let rows = client.convert().convert_trade(&request).await.unwrap();
    assert_eq!(rows[0].trade_id, "1");

    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert!(req.uri.ends_with("/api/v5/asset/convert/trade"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(req.body_str()).unwrap(),
        json!({
            "quoteId": "q1",
            "baseCcy": "BTC",
            "quoteCcy": "USDT",
            "side": "buy",
            "sz": "1",
            "szCcy": "USDT"
        })
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn invalid_request_fails_before_transport() {
    let body = r#"{"code":"0","msg":"","data":[]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock);
    let request = ConvertQuoteRequest::new("", "USDT", OrderSide::Buy, "1", "USDT");

    let error = client.convert().estimate_quote(&request).await.unwrap_err();
    assert!(matches!(
        error,
        Error::InvalidRequest(RequestValidationError::EmptyField { field: "baseCcy" })
    ));
}

#[tokio::test]
async fn get_convert_history_sends_typed_query() {
    let body = r#"{"code":"0","msg":"","data":[{"clTReqId":"","instId":"BTC-USDT","side":"buy","fillPx":"30000","baseCcy":"BTC","quoteCcy":"USDT","fillBaseSz":"0.00003","state":"fullyFilled","tradeId":"1","fillQuoteSz":"1","ts":"1646188520000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ConvertHistoryRequest::new().limit(1);

    let rows = client
        .convert()
        .get_convert_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].trade_id, "1");
    assert_eq!(rows[0].state, ConvertTradeState::FullyFilled);

    let req = mock.captured();
    assert_eq!(req.query(), Some("limit=1"));
    assert!(req.is_signed());
}

#[test]
fn quote_deserializes_official_example() {
    let row: ConvertQuote = serde_json::from_str(
        r#"{
                "baseCcy":"ETH",
                "baseSz":"0.01023052",
                "clQReqId":"",
                "cnvtPx":"2932.40104429",
                "origRfqSz":"30",
                "quoteCcy":"USDT",
                "quoteId":"quoterETH-USDT16461885104612381",
                "quoteSz":"30",
                "quoteTime":"1646188510461",
                "rfqSz":"30",
                "rfqSzCcy":"USDT",
                "side":"buy",
                "ttlMs":"10000"
            }"#,
    )
    .unwrap();

    assert_eq!(row.quote_id, "quoterETH-USDT16461885104612381");
    assert_eq!(row.side, OrderSide::Buy);
    assert_eq!(row.cnvt_px.as_str(), "2932.40104429");
    assert_eq!(row.ttl_ms.as_str(), "10000");
}

#[test]
fn trade_result_deserializes_official_example() {
    let row: ConvertTradeResult = serde_json::from_str(
        r#"{
                "baseCcy":"ETH",
                "clTReqId":"",
                "fillBaseSz":"0.01023052",
                "fillPx":"2932.40104429",
                "fillQuoteSz":"30",
                "instId":"ETH-USDT",
                "quoteCcy":"USDT",
                "quoteId":"quoterETH-USDT16461885104612381",
                "side":"buy",
                "state":"fullyFilled",
                "tradeId":"trader16461885203381437",
                "ts":"1646188520338"
            }"#,
    )
    .unwrap();

    assert_eq!(row.trade_id, "trader16461885203381437");
    assert_eq!(row.state, ConvertTradeState::FullyFilled);
    assert_eq!(row.fill_quote_sz.as_str(), "30");
}

#[test]
fn unknown_trade_state_is_preserved() {
    let state: ConvertTradeState = serde_json::from_str(r#""pendingReview""#).unwrap();
    assert_eq!(
        state,
        ConvertTradeState::Unknown("pendingReview".to_owned())
    );
    assert_eq!(state.as_str(), "pendingReview");
}
