use crate::api::account::{ResetMmpStatusRequest, SetMmpConfigRequest};
use crate::model::InstType;
use crate::test_util::MockTransport;

use super::signed_client;

#[tokio::test]
async fn set_mmp_config_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"frozenInterval":"2000","instFamily":"BTC-USD","qtyLimit":"100","timeInterval":"5000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetMmpConfigRequest::new("BTC-USD", "5000", "2000", "100");

    let result = client.account().set_mmp_config(&request).await.unwrap();
    assert_eq!(result[0].inst_family, "BTC-USD");
    assert_eq!(result[0].time_interval.as_str(), "5000");
    assert_eq!(result[0].frozen_interval.as_str(), "2000");
    assert_eq!(result[0].qty_limit.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/mmp-config"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instFamily"], "BTC-USD");
    assert_eq!(sent["timeInterval"], "5000");
    assert_eq!(sent["frozenInterval"], "2000");
    assert_eq!(sent["qtyLimit"], "100");
    assert!(req.is_signed());
}

#[tokio::test]
async fn reset_mmp_status_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"result":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ResetMmpStatusRequest::new("BTC-USD").instrument_type(InstType::Option);

    let result = client
        .account()
        .reset_mmp_status(&request)
        .await
        .unwrap();
    assert!(result[0].result);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/mmp-reset"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instType"], "OPTION");
    assert_eq!(sent["instFamily"], "BTC-USD");
    assert!(req.is_signed());
}
