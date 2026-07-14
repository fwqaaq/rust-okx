use crate::api::account::ResetMmpStatusRequest;
use crate::model::InstType;
use crate::test_util::MockTransport;

use super::signed_client;

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
