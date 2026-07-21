use crate::api::account::{
    PrecheckSetDeltaNeutralRequest, SetFeeTypeRequest, SetSettleCurrencyRequest,
};
use crate::test_util::MockTransport;

use super::signed_client;

#[tokio::test]
async fn precheck_set_delta_neutral_uses_documented_query() {
    let body = r#"{"code":"0","msg":"","data":[{"unmatchedInfoCheck":[{"type":"delta_risk","deltaLever":"2","ordList":["123"],"posList":["456"]}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = PrecheckSetDeltaNeutralRequest::new("1");

    let result = client
        .account()
        .precheck_set_delta_neutral(&request)
        .await
        .unwrap();
    assert_eq!(result[0].unmatched_info_check[0].unmatched_type, "delta_risk");
    assert_eq!(
        result[0].unmatched_info_check[0].delta_lever.as_str(),
        "2"
    );
    assert_eq!(result[0].unmatched_info_check[0].ord_list, ["123"]);
    assert_eq!(result[0].unmatched_info_check[0].pos_list, ["456"]);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("stgyType=1"));
    assert!(req
        .uri
        .contains("/api/v5/account/precheck-set-delta-neutral?"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_settle_currency_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"settleCcy":"USDC"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_settle_currency(&SetSettleCurrencyRequest::new("USDC"))
        .await
        .unwrap();
    assert_eq!(result[0].settle_ccy, "USDC");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-settle-currency"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["settleCcy"], "USDC");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_fee_type_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"feeType":"1"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_fee_type(&SetFeeTypeRequest::new("1"))
        .await
        .unwrap();
    assert_eq!(result[0].fee_type, "1");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-fee-type"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["feeType"], "1");
    assert!(req.is_signed());
}
