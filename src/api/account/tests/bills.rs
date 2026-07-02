use crate::model::InstType;
use crate::test_util::MockTransport;

use super::super::{
    ApplyBillsHistoryArchiveRequest, BillSubtypesRequest, BillsArchiveRequest,
    BillsHistoryArchiveFileState, BillsHistoryArchiveQuarter, BillsHistoryArchiveRequest,
    BillsHistoryArchiveStatus, BillsRequest, PositionsHistoryRequest,
};
use super::signed_client;

#[tokio::test]
async fn get_account_bills_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "bal":"10000","balChg":"-10","billId":"12344","ccy":"USDT","execType":"T","fee":"-0.001468",
        "from":"","instId":"BTC-USDT","instType":"SPOT","mgnMode":"isolated","notes":"",
        "ordId":"12344","pnl":"0","posBal":"0","posBalChg":"0","qty":"0.02","subType":"1",
        "sz":"-100","to":"","tradeId":"1","type":"2","uTime":"1597026383085","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = BillsRequest::new()
        .inst_type(InstType::Spot)
        .currency("USDT")
        .bill_type("1")
        .limit(1);

    let bills = client.account().get_account_bills(&request).await.unwrap();
    assert_eq!(bills[0].bill_id, "12344");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&ccy=USDT&type=1&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_bills_archive_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "bal":"10000","balChg":"-10","billId":"12345","ccy":"USDT","execType":"T","fee":"-0.001468",
        "from":"","instId":"BTC-USDT","instType":"SPOT","mgnMode":"isolated","notes":"",
        "ordId":"12345","pnl":"0","posBal":"0","posBalChg":"0","qty":"0.02","subType":"1",
        "sz":"-100","to":"","tradeId":"2","type":"2","uTime":"1597026383085","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = BillsArchiveRequest::new()
        .filters(BillsRequest::new().currency("USDT"))
        .begin("100")
        .end("200");

    let bills = client
        .account()
        .get_account_bills_archive(&request)
        .await
        .unwrap();
    assert_eq!(bills[0].bill_id, "12345");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=USDT&begin=100&end=200"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn apply_bills_history_archive_posts_body_and_parses_available_status() {
    let body = r#"{"code":"0","msg":"","data":[{"result":"true","ts":"1646892328000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ApplyBillsHistoryArchiveRequest::new("2023", BillsHistoryArchiveQuarter::Q1)
        .bill_type("1,2,3");

    let result = client
        .account()
        .apply_bills_history_archive(&request)
        .await
        .unwrap();
    assert_eq!(result[0].status, BillsHistoryArchiveStatus::LinkAvailable);
    assert_eq!(result[0].ts.as_str(), "1646892328000");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/bills-history-archive"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["year"], "2023");
    assert_eq!(sent["quarter"], "Q1");
    assert_eq!(sent["type"], "1,2,3");
    assert!(req.is_signed());
}

#[tokio::test]
async fn apply_bills_history_archive_parses_generating_status() {
    let body = r#"{"code":"0","msg":"","data":[{"result":"false","ts":"1646892328000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ApplyBillsHistoryArchiveRequest::new("2023", BillsHistoryArchiveQuarter::Q2);

    let result = client
        .account()
        .apply_bills_history_archive(&request)
        .await
        .unwrap();

    assert_eq!(result[0].status, BillsHistoryArchiveStatus::Generating);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/bills-history-archive"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_bills_history_archive_uses_builder_query_and_parses_finished_file() {
    let body = r#"{"code":"0","msg":"","data":[{"fileHref":"http://example.test/bills.csv","state":"finished","ts":"1646892328000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request =
        BillsHistoryArchiveRequest::new("2023", BillsHistoryArchiveQuarter::Q4).bill_type("1,2,3");

    let files = client
        .account()
        .get_bills_history_archive(&request)
        .await
        .unwrap();
    assert_eq!(files[0].file_href, "http://example.test/bills.csv");
    assert_eq!(files[0].state, BillsHistoryArchiveFileState::Finished);
    assert_eq!(files[0].ts.as_str(), "1646892328000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("year=2023&quarter=Q4&type=1%2C2%2C3"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_bills_history_archive_parses_ongoing_and_failed_states() {
    let body = r#"{"code":"0","msg":"","data":[
        {"fileHref":"","state":"ongoing","ts":"1646892328000"},
        {"fileHref":"","state":"failed","ts":"1646892328000"}
    ]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = BillsHistoryArchiveRequest::new("2023", BillsHistoryArchiveQuarter::Q4);

    let files = client
        .account()
        .get_bills_history_archive(&request)
        .await
        .unwrap();

    assert_eq!(files[0].state, BillsHistoryArchiveFileState::Ongoing);
    assert_eq!(files[1].state, BillsHistoryArchiveFileState::Failed);

    let req = mock.captured();
    assert_eq!(req.query(), Some("year=2023&quarter=Q4"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_bill_subtypes_uses_builder_query_and_parses_mapping() {
    let body = r#"{"code":"0","msg":"","data":[{"type":"1","typeDesc":"Transfer","subTypeDetails":[{"subType":"11","subTypeDesc":"Transfer in"},{"subType":"12","subTypeDesc":"Transfer out"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = BillSubtypesRequest::new().bill_type("1,2");

    let mappings = client.account().get_bill_subtypes(&request).await.unwrap();
    assert_eq!(mappings[0].bill_type, "1");
    assert_eq!(mappings[0].type_desc, "Transfer");
    assert_eq!(mappings[0].sub_type_details[0].sub_type, "11");
    assert_eq!(mappings[0].sub_type_details[0].sub_type_desc, "Transfer in");

    let req = mock.captured();
    assert_eq!(req.query(), Some("type=1%2C2"));
    assert!(req.uri.contains("/api/v5/account/subtypes"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_positions_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"FUTURES","instId":"BTC-USD-220930","mgnMode":"cross","type":"2","posId":"307173036051017730",
        "lever":"10","direction":"long","triggerPx":"","liqPx":"","uly":"BTC-USD",
        "openAvgPx":"2565.55","closeAvgPx":"2566.31","openMaxPos":"2","closeTotalPos":"2",
        "realizedPnl":"0.000090081525489","fee":"-0.000246405","fundingFee":"0","liqPenalty":"0",
        "pnl":"0.000336486525489","pnlRatio":"0.00052","cTime":"1619507758793","uTime":"1619507761462"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = PositionsHistoryRequest::new()
        .inst_type(InstType::Futures)
        .inst_id("BTC-USD-220930")
        .limit(1);

    let result = client
        .account()
        .get_positions_history(&request)
        .await
        .unwrap();
    assert_eq!(result[0].realized_pnl.as_str(), "0.000090081525489");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instType=FUTURES&instId=BTC-USD-220930&limit=1")
    );
    assert!(req.is_signed());
}
