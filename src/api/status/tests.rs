use crate::OkxClient;
use crate::test_util::MockTransport;

use super::StatusRequest;

const DOCS_RESPONSE: &str = r#"{
    "code": "0",
    "msg": "",
    "data": [{
        "begin": "1672823400000",
        "end": "1672823520000",
        "href": "",
        "preOpenBegin": "",
        "scheDesc": "",
        "serviceType": "8",
        "state": "completed",
        "maintType": "1",
        "env": "1",
        "system": "unified",
        "title": "Trading account system upgrade (in batches of accounts)"
    }]
}"#;

#[tokio::test]
async fn get_status_no_filter_parses_response() {
    let mock = MockTransport::new(DOCS_RESPONSE);
    let client = OkxClient::with_transport(mock.clone()).build();

    let items = client
        .status()
        .get_status(&StatusRequest::default())
        .await
        .unwrap();

    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].title,
        "Trading account system upgrade (in batches of accounts)"
    );
    assert_eq!(items[0].state, "completed");
    assert_eq!(items[0].begin.as_str(), "1672823400000");
    assert_eq!(items[0].end.as_str(), "1672823520000");
    assert!(items[0].pre_open_begin.is_empty());
    assert_eq!(items[0].service_type, "8");
    assert_eq!(items[0].maint_type, "1");
    assert_eq!(items[0].env, "1");
    assert_eq!(items[0].system, "unified");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), None);
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_status_state_filter_builds_query() {
    let mock = MockTransport::new(DOCS_RESPONSE);
    let client = OkxClient::with_transport(mock.clone()).build();

    client
        .status()
        .get_status(&StatusRequest::new().state("canceled"))
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(req.query(), Some("state=canceled"));
    assert!(!req.is_signed());
}
