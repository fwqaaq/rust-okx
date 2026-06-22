use crate::OkxClient;
use crate::test_util::MockTransport;

use super::AnnouncementsRequest;

const ANNOUNCEMENTS_RESPONSE: &str = r#"{
    "code": "0",
    "msg": "",
    "data": [{
        "details": [
            {
                "annType": "announcements-new-listings",
                "title": "OKX to list Virtuals Protocol (VIRTUAL) for spot trading",
                "url": "https://www.okx.com/help/okx-to-list-virtuals-protocol-virtual-for-spot-trading",
                "pTime": "1761620404821",
                "businessPTime": "1761620400000"
            },
            {
                "annType": "announcements-web3",
                "title": "Completion of X Layer Mainnet Upgrade",
                "url": "https://www.okx.com/help/completion-of-x-layer-mainnet-upgrade",
                "pTime": "1761582756071",
                "businessPTime": "1761580800000"
            }
        ],
        "totalPage": "123"
    }]
}"#;

const TYPES_RESPONSE: &str = r#"{
    "code": "0",
    "msg": "",
    "data": [
        {"annType": "announcements-new-listings", "annTypeDesc": "New listings"},
        {"annType": "announcements-delistings",   "annTypeDesc": "Delistings"}
    ]
}"#;

#[tokio::test]
async fn get_announcements_no_filter_parses_response() {
    let mock = MockTransport::new(ANNOUNCEMENTS_RESPONSE);
    let client = OkxClient::with_transport(mock.clone()).build();

    let pages = client
        .support()
        .get_announcements(&AnnouncementsRequest::default())
        .await
        .unwrap();

    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].total_page, "123");
    assert_eq!(pages[0].details.len(), 2);
    assert_eq!(pages[0].details[0].ann_type, "announcements-new-listings");
    assert_eq!(
        pages[0].details[0].title,
        "OKX to list Virtuals Protocol (VIRTUAL) for spot trading"
    );
    assert_eq!(pages[0].details[0].p_time.as_str(), "1761620404821");
    assert_eq!(
        pages[0].details[0].business_p_time.as_str(),
        "1761620400000"
    );

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), None);
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_announcements_with_filters_builds_query() {
    let mock = MockTransport::new(ANNOUNCEMENTS_RESPONSE);
    let client = OkxClient::with_transport(mock.clone()).build();

    client
        .support()
        .get_announcements(&AnnouncementsRequest {
            ann_type: Some("announcements-new-listings"),
            page: Some(2),
        })
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("annType=announcements-new-listings&page=2")
    );
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_announcement_types_parses_response() {
    let mock = MockTransport::new(TYPES_RESPONSE);
    let client = OkxClient::with_transport(mock.clone()).build();

    let types = client.support().get_announcement_types().await.unwrap();

    assert_eq!(types.len(), 2);
    assert_eq!(types[0].ann_type, "announcements-new-listings");
    assert_eq!(types[0].ann_type_desc, "New listings");
    assert_eq!(types[1].ann_type, "announcements-delistings");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), None);
    assert!(!req.is_signed());
}
