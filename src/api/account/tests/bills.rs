use crate::model::InstType;
use crate::test_util::MockTransport;

use super::super::{BillsArchiveRequest, BillsRequest, PositionsHistoryRequest};
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
