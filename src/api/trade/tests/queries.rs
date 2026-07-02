use crate::model::{InstType, OrderSide, OrderState};
use crate::test_util::MockTransport;

use super::super::{
    FillHistoryRequest, FillsRequest, GetOrderRequest, OrderHistoryRequest, OrderListRequest,
};
use super::signed_client;

#[tokio::test]
async fn get_order_queries_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SPOT","instId":"BTC-USDT","tgtCcy":"","ccy":"","ordId":"312269865356374016",
        "clOrdId":"b15","tag":"","px":"59200","pxUsd":"","pxVol":"","pxType":"","sz":"0.1",
        "pnl":"0","ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
        "accFillSz":"0.1","fillPx":"59200","tradeId":"123","fillSz":"0.1",
        "fillTime":"1597026383085","avgPx":"59200","state":"filled","stpId":"","stpMode":"",
        "lever":"","attachAlgoClOrdId":"","tpTriggerPx":"","tpTriggerPxType":"","tpOrdPx":"",
        "slTriggerPx":"","slTriggerPxType":"","slOrdPx":"",
        "attachAlgoOrds":[{"attachAlgoId":"a1","attachAlgoClOrdId":"c1","tpOrdKind":"condition",
        "tpTriggerPx":"60000","tpTriggerRatio":"","tpTriggerPxType":"last","tpOrdPx":"-1",
        "slTriggerPx":"58000","slTriggerRatio":"","slTriggerPxType":"last","slOrdPx":"-1",
        "sz":"0.1","amendPxOnTriggerType":"0","callbackRatio":"","callbackSpread":"",
        "activePx":"","failCode":"","failReason":""}],
        "linkedAlgoOrd":{"algoId":"777"},
        "feeCcy":"USDT","fee":"-0.01","rebateCcy":"USDT","rebate":"0","source":"","category":"normal",
        "reduceOnly":"false","isTpLimit":"false","cancelSource":"","cancelSourceReason":"",
        "quickMgnType":"","algoClOrdId":"","algoId":"","uTime":"1597026383085",
        "cTime":"1597026383085","tradeQuoteCcy":"USDT","outcome":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = GetOrderRequest::new("BTC-USDT", "312269865356374016");
    let orders = client.trade().get_order(&request).await.unwrap();
    assert_eq!(orders[0].ord_id, "312269865356374016");
    assert_eq!(orders[0].state, OrderState::Filled);
    assert_eq!(orders[0].side, OrderSide::Buy);
    assert_eq!(orders[0].inst_type, "SPOT");
    assert_eq!(orders[0].trade_quote_ccy, "USDT");
    assert_eq!(orders[0].category, "normal");
    assert_eq!(orders[0].attach_algo_ords.len(), 1);
    assert_eq!(orders[0].attach_algo_ords[0].attach_algo_id, "a1");
    assert_eq!(
        orders[0].attach_algo_ords[0].tp_trigger_px.as_str(),
        "60000"
    );
    assert_eq!(orders[0].linked_algo_ord.as_ref().unwrap().algo_id, "777");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(
        req.query(),
        Some("instId=BTC-USDT&ordId=312269865356374016")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_order_list_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SPOT","instId":"BTC-USDT","ccy":"","ordId":"312269865356374016",
        "clOrdId":"b15","tag":"","px":"59200","sz":"0.1","pnl":"0","ordType":"limit",
        "side":"buy","posSide":"net","tdMode":"cash","accFillSz":"0","fillPx":"",
        "tradeId":"","fillSz":"0","fillTime":"","avgPx":"","state":"live","lever":"",
        "feeCcy":"USDT","fee":"0","rebateCcy":"USDT","rebate":"0","category":"normal",
        "uTime":"1597026383085","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OrderListRequest::new()
        .inst_type(InstType::Spot)
        .inst_id("BTC-USDT")
        .limit(1);

    let orders = client.trade().get_order_list(&request).await.unwrap();
    assert_eq!(orders[0].ord_id, "312269865356374016");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&instId=BTC-USDT&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_orders_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SPOT","instId":"BTC-USDT","ccy":"","ordId":"312269865356374016",
        "clOrdId":"b15","tag":"","px":"59200","sz":"0.1","pnl":"0","ordType":"limit",
        "side":"buy","posSide":"net","tdMode":"cash","accFillSz":"0.1","fillPx":"59200",
        "tradeId":"123","fillSz":"0.1","fillTime":"1597026383085","avgPx":"59200",
        "state":"filled","lever":"","feeCcy":"USDT","fee":"-0.01","rebateCcy":"USDT",
        "rebate":"0","category":"normal","uTime":"1597026383085","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OrderHistoryRequest::new(InstType::Spot)
        .begin("100")
        .end("200");

    let orders = client.trade().get_orders_history(&request).await.unwrap();
    assert_eq!(orders[0].state, OrderState::Filled);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&begin=100&end=200"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_orders_history_archive_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SPOT","instId":"BTC-USDT","ccy":"","ordId":"312269865356374016",
        "clOrdId":"b15","tag":"","px":"59200","sz":"0.1","pnl":"0","ordType":"limit",
        "side":"buy","posSide":"net","tdMode":"cash","accFillSz":"0","fillPx":"","tradeId":"",
        "fillSz":"0","fillTime":"","avgPx":"","state":"canceled","lever":"","feeCcy":"USDT",
        "fee":"0","rebateCcy":"USDT","rebate":"0","category":"normal",
        "uTime":"1597026383085","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = OrderHistoryRequest::new(InstType::Spot)
        .filters(OrderListRequest::new().inst_type(InstType::Spot).limit(1));

    let orders = client
        .trade()
        .get_orders_history_archive(&request)
        .await
        .unwrap();
    assert_eq!(orders[0].state, OrderState::Canceled);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fills_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SPOT","instId":"BTC-USDT","tradeId":"123","ordId":"312269865356374016",
        "clOrdId":"b15","billId":"12344","subType":"1","tag":"","fillPx":"59200","fillSz":"0.1",
        "fillIdxPx":"","fillPnl":"0","fillPxVol":"","fillPxUsd":"","fillMarkVol":"",
        "fillFwdPx":"","fillMarkPx":"","side":"buy","posSide":"net","execType":"T",
        "feeCcy":"USDT","fee":"-0.059","feeRate":"-0.001","ts":"1597026383085",
        "fillTime":"1597026383085","tradeQuoteCcy":"USDT"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FillsRequest::new()
        .inst_type(InstType::Spot)
        .order_id("312269865356374016");

    let fills = client.trade().get_fills(&request).await.unwrap();
    assert_eq!(fills[0].trade_id, "123");
    assert_eq!(fills[0].bill_id, "12344");
    assert_eq!(fills[0].sub_type, "1");
    assert_eq!(fills[0].exec_type, "T");
    assert_eq!(fills[0].fee_rate.as_str(), "-0.001");
    assert_eq!(fills[0].trade_quote_ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&ordId=312269865356374016"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fills_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "side":"buy","fillSz":"0.00192834","fillPx":"51858","fillPxVol":"","fillFwdPx":"",
        "fee":"-0.00000192834","fillPnl":"0","ordId":"680800019749904384","feeRate":"-0.001",
        "instType":"SPOT","fillPxUsd":"","instId":"BTC-USDT","clOrdId":"","posSide":"net",
        "billId":"680800019754098688","subType":"1","fillMarkVol":"","tag":"",
        "fillTime":"1708587373361","execType":"T","fillIdxPx":"","tradeId":"744876980",
        "fillMarkPx":"","feeCcy":"BTC","ts":"1708587373362","tradeQuoteCcy":"USDT"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FillHistoryRequest::new(InstType::Spot)
        .begin("100")
        .end("200")
        .limit(1);

    let fills = client.trade().get_fills_history(&request).await.unwrap();
    assert_eq!(fills[0].fee.as_str(), "-0.00000192834");
    assert_eq!(fills[0].sub_type, "1");
    assert_eq!(fills[0].bill_id, "680800019754098688");
    assert_eq!(fills[0].trade_quote_ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&begin=100&end=200&limit=1"));
    assert!(req.is_signed());
}
