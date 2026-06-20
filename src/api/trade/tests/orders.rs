use crate::model::{OrderSide, OrderType, PositionSide, TradeMode};
use crate::test_util::MockTransport;

use super::super::{
    AmendOrderRequest, CancelOrderRequest, ClosePositionRequest, PlaceOrderRequest,
};
use super::signed_client;

#[tokio::test]
async fn place_order_posts_signed_json_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "clOrdId":"oktswap6","ordId":"312269865356374016","tag":"","ts":"1695190491421",
        "sCode":"0","sMsg":"","subCode":""}],"inTime":"1695190491421339","outTime":"1695190491423240"}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = PlaceOrderRequest::new(
        "BTC-USDT",
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Limit,
        "0.1",
    )
    .price("59200")
    .client_order_id("b15");

    let result = client.trade().place_order(&request).await.unwrap();
    assert_eq!(result[0].ord_id, "312269865356374016");
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
    assert_eq!(sent["sz"], "0.1");
    assert_eq!(sent["px"], "59200");
    assert_eq!(sent["clOrdId"], "b15");
    assert!(sent.get("reduceOnly").is_none());
}

#[tokio::test]
async fn place_multiple_orders_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[
        {"clOrdId":"oktswap6","ordId":"12345689","tag":"","ts":"1695190491421",
         "sCode":"0","sMsg":"","subCode":""},
        {"clOrdId":"oktswap7","ordId":"12344","tag":"","ts":"1695190491421",
         "sCode":"0","sMsg":"","subCode":""}],
        "inTime":"1695190491421339","outTime":"1695190491423240"}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let requests = vec![
        PlaceOrderRequest::new(
            "BTC-USDT",
            TradeMode::Cash,
            OrderSide::Buy,
            OrderType::Limit,
            "0.1",
        )
        .price("59200")
        .client_order_id("b15"),
        PlaceOrderRequest::new(
            "BTC-USDT",
            TradeMode::Cash,
            OrderSide::Sell,
            OrderType::Limit,
            "0.2",
        )
        .price("60000")
        .client_order_id("b16"),
    ];

    let result = client
        .trade()
        .place_multiple_orders(&requests)
        .await
        .unwrap();
    assert_eq!(result[1].ord_id, "12344");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/trade/batch-orders"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent[0]["clOrdId"], "b15");
    assert_eq!(sent[1]["side"], "sell");
    assert!(req.is_signed());
}

#[tokio::test]
async fn cancel_order_posts_ids() {
    let body = r#"{"code":"0","msg":"","data":[{
        "clOrdId":"oktswap6","ordId":"12345689","ts":"1695190491421","sCode":"0","sMsg":""}],
        "inTime":"1695190491421339","outTime":"1695190491423240"}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CancelOrderRequest::by_order_id("BTC-USDT", "312269865356374016");

    let result = client.trade().cancel_order(&request).await.unwrap();
    assert_eq!(result[0].ord_id, "12345689");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["ordId"], "312269865356374016");
    assert!(req.is_signed());
}

#[tokio::test]
async fn cancel_multiple_orders_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "clOrdId":"oktswap6","ordId":"12345689","ts":"1695190491421","sCode":"0","sMsg":""},
        {"clOrdId":"oktswap7","ordId":"12344","ts":"1695190491421","sCode":"0","sMsg":""}],
        "inTime":"1695190491421339","outTime":"1695190491423240"}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let requests = vec![CancelOrderRequest::by_order_id(
        "BTC-USDT",
        "312269865356374016",
    )];

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
    assert_eq!(sent[0]["ordId"], "312269865356374016");
    assert!(sent[0].get("clOrdId").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn amend_order_posts_builder_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "clOrdId":"","ordId":"12344","ts":"1695190491421","reqId":"b12344",
        "sCode":"0","sMsg":"","subCode":""}],"inTime":"1695190491421339","outTime":"1695190491423240"}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AmendOrderRequest::new("BTC-USDT")
        .order_id("312269865356374016")
        .request_id("r1")
        .new_price("59300");

    let result = client.trade().amend_order(&request).await.unwrap();
    assert_eq!(result[0].req_id, "b12344");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/trade/amend-order"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["ordId"], "312269865356374016");
    assert_eq!(sent["newPx"], "59300");
    assert!(sent.get("newSz").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn amend_multiple_orders_posts_array_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "clOrdId":"oktswap6","ordId":"12345689","ts":"1695190491421","reqId":"b12344",
        "sCode":"0","sMsg":"","subCode":""},
        {"clOrdId":"oktswap7","ordId":"12344","ts":"1695190491421","reqId":"b12344",
        "sCode":"0","sMsg":"","subCode":""}],"inTime":"1695190491421339","outTime":"1695190491423240"}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let requests = vec![
        AmendOrderRequest::new("BTC-USDT")
            .client_order_id("b15")
            .new_size("0.2"),
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
    assert_eq!(sent[0]["clOrdId"], "b15");
    assert_eq!(sent[0]["newSz"], "0.2");
    assert!(req.is_signed());
}

#[tokio::test]
async fn close_positions_posts_builder_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT-SWAP","posSide":"long","clOrdId":"close1","tag":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = ClosePositionRequest::new("BTC-USDT-SWAP", TradeMode::Cross)
        .position_side(PositionSide::Long)
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
