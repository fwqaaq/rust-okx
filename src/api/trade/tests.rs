use crate::model::{OrderSide, OrderType, TradeMode};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

use super::PlaceOrderRequest;

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn place_order_posts_signed_json_body() {
    let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","tag":"","sCode":"0","sMsg":"","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let request = PlaceOrderRequest::new(
        "BTC-USDT",
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Limit,
        "0.01",
    )
    .price("42000")
    .client_order_id("b1");

    let result = client.trade().place_order(&request).await.unwrap();
    assert_eq!(result[0].ord_id, "312");
    assert_eq!(result[0].s_code, "0");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/trade/order"));
    assert!(req.is_signed());

    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["tdMode"], "cash");
    assert_eq!(sent["side"], "buy");
    assert_eq!(sent["ordType"], "limit");
    assert_eq!(sent["sz"], "0.01");
    assert_eq!(sent["px"], "42000");
    assert_eq!(sent["clOrdId"], "b1");
    // Unset optional fields are omitted.
    assert!(sent.get("reduceOnly").is_none());
}

#[tokio::test]
async fn place_multiple_orders_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","sCode":"0","sMsg":""},
            {"ordId":"313","clOrdId":"b2","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let requests = vec![
        PlaceOrderRequest::new(
            "BTC-USDT",
            TradeMode::Cash,
            OrderSide::Buy,
            OrderType::Limit,
            "0.01",
        )
        .price("42000")
        .client_order_id("b1"),
        PlaceOrderRequest::new(
            "BTC-USDT",
            TradeMode::Cash,
            OrderSide::Sell,
            OrderType::Limit,
            "0.02",
        )
        .price("43000")
        .client_order_id("b2"),
    ];

    let result = client
        .trade()
        .place_multiple_orders(&requests)
        .await
        .unwrap();
    assert_eq!(result[1].ord_id, "313");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/trade/batch-orders"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent[0]["clOrdId"], "b1");
    assert_eq!(sent[1]["side"], "sell");
    assert!(req.is_signed());
}

#[tokio::test]
async fn cancel_order_posts_ids() {
    let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .trade()
        .cancel_order("BTC-USDT", "312")
        .await
        .unwrap();
    assert_eq!(result[0].ord_id, "312");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["ordId"], "312");
}

#[tokio::test]
async fn cancel_multiple_orders_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let requests = vec![super::CancelOrderRequest::by_order_id("BTC-USDT", "312")];

    let result = client
        .trade()
        .cancel_multiple_orders(&requests)
        .await
        .unwrap();
    assert_eq!(result[0].s_code, "0");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/trade/cancel-batch-orders"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent[0]["instId"], "BTC-USDT");
    assert_eq!(sent[0]["ordId"], "312");
    assert!(sent[0].get("clOrdId").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn amend_order_posts_builder_body() {
    let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","reqId":"r1","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AmendOrderRequest::new("BTC-USDT")
        .order_id("312")
        .request_id("r1")
        .new_price("42100");

    let result = client.trade().amend_order(&request).await.unwrap();
    assert_eq!(result[0].req_id, "r1");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/trade/amend-order"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["ordId"], "312");
    assert_eq!(sent["newPx"], "42100");
    assert!(sent.get("newSz").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn amend_multiple_orders_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[
            {"ordId":"312","clOrdId":"b1","reqId":"r1","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let requests = vec![
        super::AmendOrderRequest::new("BTC-USDT")
            .client_order_id("b1")
            .new_size("0.03"),
    ];

    let result = client
        .trade()
        .amend_multiple_orders(&requests)
        .await
        .unwrap();
    assert_eq!(result[0].s_code, "0");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/trade/amend-batch-orders"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent[0]["clOrdId"], "b1");
    assert_eq!(sent[0]["newSz"], "0.03");
    assert!(req.is_signed());
}

#[tokio::test]
async fn close_positions_posts_builder_body() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","posSide":"long","clOrdId":"close1","tag":"t"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::ClosePositionRequest::new("BTC-USDT-SWAP", TradeMode::Cross)
        .position_side(crate::model::PositionSide::Long)
        .auto_cancel(true)
        .client_order_id("close1");

    let result = client.trade().close_positions(&request).await.unwrap();
    assert_eq!(result[0].cl_ord_id, "close1");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/trade/close-position"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["mgnMode"], "cross");
    assert_eq!(sent["posSide"], "long");
    assert_eq!(sent["autoCxl"], true);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_order_queries_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"live","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let orders = client.trade().get_order("BTC-USDT", "312").await.unwrap();
    assert_eq!(orders[0].ord_id, "312");
    assert_eq!(orders[0].state, crate::model::OrderState::Live);
    assert_eq!(orders[0].side, OrderSide::Buy);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(req.query(), Some("instId=BTC-USDT&ordId=312"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_order_list_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"live","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::OrderListRequest::new()
        .inst_type(crate::model::InstType::Spot)
        .inst_id("BTC-USDT")
        .limit(1);

    let orders = client.trade().get_order_list(&request).await.unwrap();
    assert_eq!(orders[0].ord_id, "312");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&instId=BTC-USDT&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_orders_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"filled","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::OrderHistoryRequest::new(crate::model::InstType::Spot)
        .begin("100")
        .end("200");

    let orders = client.trade().get_orders_history(&request).await.unwrap();
    assert_eq!(orders[0].state, crate::model::OrderState::Filled);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&begin=100&end=200"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_orders_history_archive_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ordId":"312","clOrdId":"b1","px":"42000","sz":"0.01",
             "ordType":"limit","side":"buy","posSide":"net","tdMode":"cash",
             "accFillSz":"0","avgPx":"","state":"canceled","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::OrderHistoryRequest::new(crate::model::InstType::Spot).filters(
        super::OrderListRequest::new()
            .inst_type(crate::model::InstType::Spot)
            .limit(1),
    );

    let orders = client
        .trade()
        .get_orders_history_archive(&request)
        .await
        .unwrap();
    assert_eq!(orders[0].state, crate::model::OrderState::Canceled);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fills_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","tradeId":"t1","ordId":"312",
             "fillPx":"42000","fillSz":"0.01","side":"buy","ordType":"limit",
             "feeCcy":"USDT","fee":"-1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FillsRequest::new()
        .inst_type(crate::model::InstType::Spot)
        .order_id("312");

    let fills = client.trade().get_fills(&request).await.unwrap();
    assert_eq!(fills[0].trade_id, "t1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&ordId=312"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fills_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","tradeId":"t1","ordId":"312",
             "fillPx":"42000","fillSz":"0.01","side":"buy","ordType":"limit",
             "feeCcy":"USDT","fee":"-1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::FillsRequest::new()
        .inst_type(crate::model::InstType::Spot)
        .begin("100")
        .end("200")
        .limit(1);

    let fills = client.trade().get_fills_history(&request).await.unwrap();
    assert_eq!(fills[0].fee.as_str(), "-1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&begin=100&end=200&limit=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn algo_order_endpoints_are_signed_and_flexible() {
    let body = r#"{"code":"0","msg":"","data":[
            {"algoId":"a1","algoClOrdId":"c1","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AlgoOrderRequest::new()
        .param("instId", "BTC-USDT")
        .param("tdMode", "cash")
        .param("side", "buy")
        .param("ordType", "conditional");

    let rows = client.trade().place_algo_order(&request).await.unwrap();
    assert_eq!(rows[0].algo_id, "a1");
    let req = mock.captured();
    assert!(req.is_signed());
    assert!(req.uri.ends_with("/api/v5/trade/order-algo"));
    assert!(req.body_str().contains(r#""ordType":"conditional""#));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let cancel = super::CancelAlgoOrderRequest::new()
        .param("instId", "BTC-USDT")
        .param("algoId", "a1");
    client.trade().cancel_algo_orders(&[cancel]).await.unwrap();
    assert!(mock.captured().uri.ends_with("/api/v5/trade/cancel-algos"));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let amend = super::AmendAlgoOrderRequest::new()
        .param("instId", "BTC-USDT")
        .param("algoId", "a1")
        .param("newSz", "0.2");
    client.trade().amend_algo_order(&amend).await.unwrap();
    assert!(mock.captured().uri.ends_with("/api/v5/trade/amend-algos"));
}

#[tokio::test]
async fn algo_order_query_endpoints_are_signed() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","algoId":"a1","state":"live","side":"buy","sz":"1"}]}"#;

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AlgoOrderListRequest::new().param("ordType", "conditional");
    let rows = client.trade().get_algo_order_list(&request).await.unwrap();
    assert_eq!(rows[0].state, "live");
    assert_eq!(mock.captured().query(), Some("ordType=conditional"));
    assert!(mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AlgoOrderHistoryRequest::new()
        .param("ordType", "conditional")
        .param("state", "canceled");
    client
        .trade()
        .get_algo_orders_history(&request)
        .await
        .unwrap();
    assert_eq!(
        mock.captured().query(),
        Some("ordType=conditional&state=canceled")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::AlgoOrderDetailsRequest::new().param("algoId", "a1");
    client
        .trade()
        .get_algo_order_details(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("algoId=a1"));
}

#[tokio::test]
async fn easy_convert_endpoints_are_signed() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"1","ts":"1597026383085"}]}"#;

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let rows = client
        .trade()
        .get_easy_convert_currency_list()
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/trade/easy-convert-currency-list")
    );
    assert!(mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::EasyConvertRequest::new()
        .param("fromCcy", "BTC")
        .param("toCcy", "USDT");
    client.trade().easy_convert(&request).await.unwrap();
    assert!(mock.captured().body_str().contains(r#""fromCcy":"BTC""#));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = super::EasyConvertHistoryRequest::new().param("after", "1");
    client
        .trade()
        .get_easy_convert_history(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("after=1"));
}

#[tokio::test]
async fn one_click_repay_v1_and_v2_endpoints_are_signed() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","ordId":"r1","state":"filled"}]}"#;
    let request = super::OneClickRepayCurrencyListRequest::new().param("debtCcy", "USDT");

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    client
        .trade()
        .get_one_click_repay_currency_list(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("debtCcy=USDT"));
    assert!(mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    client
        .trade()
        .get_one_click_repay_currency_list_v2(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/trade/one-click-repay-currency-list-v2?debtCcy=USDT")
    );

    let body_request = super::OneClickRepayRequest::new()
        .param("debtCcy", "USDT")
        .param("repayCcy", "BTC");
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    client.trade().one_click_repay(&body_request).await.unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/trade/one-click-repay")
    );

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    client
        .trade()
        .one_click_repay_v2(&body_request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/trade/one-click-repay-v2")
    );

    let history_request = super::OneClickRepayHistoryRequest::new().param("after", "1");
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    client
        .trade()
        .get_one_click_repay_history(&history_request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("after=1"));

    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    client
        .trade()
        .get_one_click_repay_history_v2(&history_request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .ends_with("/api/v5/trade/one-click-repay-history-v2?after=1")
    );
}
