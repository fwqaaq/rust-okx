use http::Method;

use crate::OkxClient;
use crate::test_util::MockTransport;

use super::super::{
    ApplyMonthlyStatementRequest, FundingBillsHistoryRequest, MonthlyStatementRequest,
    StatementMonth,
};
use super::signed_client;

#[tokio::test]
async fn bills_history_matches_official_response_example() {
    let body = r#"{"code":"0","msg":"","data":[{"billId":"12344","ccy":"BTC","clientId":"","balChg":"2","bal":"12","type":"1","ts":"1597026383085","notes":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FundingBillsHistoryRequest::new()
        .currency("BTC")
        .bill_type("1")
        .third_party_type("2")
        .client_id("client-1")
        .after("1597026383085")
        .before("1597026384000")
        .limit(100)
        .paging_type("1");

    let rows = client.funding().get_bills_history(&request).await.unwrap();

    assert_eq!(rows[0].bill_id, "12344");
    assert_eq!(rows[0].bal_chg.as_str(), "2");
    assert_eq!(rows[0].notes, "");
    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(
        req.query(),
        Some(
            "ccy=BTC&type=1&thirdPartyType=2&clientId=client-1&after=1597026383085&before=1597026384000&limit=100&pagingType=1"
        )
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn exchange_list_is_public_and_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"exchId":"did:ethr:0xfeb4f99829a9acdf52979abee87e83addf22a7e1","exchName":"1xbet"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client.funding().get_exchange_list().await.unwrap();

    assert_eq!(rows[0].exchange_name, "1xbet");
    assert_eq!(
        rows[0].exchange_id,
        "did:ethr:0xfeb4f99829a9acdf52979abee87e83addf22a7e1"
    );
    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert!(req.uri.ends_with("/api/v5/asset/exchange-list"));
    assert_eq!(req.query(), None);
    assert!(!req.is_signed());
}

#[tokio::test]
async fn apply_monthly_statement_matches_official_example() {
    let body = r#"{"code":"0","data":[{"ts":"1646892328000"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ApplyMonthlyStatementRequest::new().month(StatementMonth::January);

    let rows = client
        .funding()
        .apply_monthly_statement(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].ts.as_str(), "1646892328000");
    let req = mock.captured();
    assert_eq!(req.method, Method::POST);
    assert_eq!(req.body_str(), r#"{"month":"Jan"}"#);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_monthly_statement_matches_official_example() {
    let body = r#"{"code":"0","data":[{"fileHref":"http://xxx","state":"finished","ts":1646892328000}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .funding()
        .get_monthly_statement(&MonthlyStatementRequest::new(StatementMonth::January))
        .await
        .unwrap();

    assert_eq!(rows[0].file_href, "http://xxx");
    assert_eq!(rows[0].state, "finished");
    assert_eq!(rows[0].ts, 1_646_892_328_000);
    let req = mock.captured();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.query(), Some("month=Jan"));
    assert!(req.is_signed());
}
