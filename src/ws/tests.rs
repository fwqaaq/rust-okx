use crate::api::trade::{CancelOrderRequest, PlaceOrderRequest};
use crate::model::{OrderSide, OrderType, TradeMode};
use crate::signing;
use crate::ws::channels;
use crate::ws::client::login_payload;
use crate::ws::event::parse_text_event;
use crate::ws::ops::operation_payload;
use crate::{Arg, Credentials, WsEvent};

#[test]
fn login_payload_signs_expected_message() {
    let credentials = Credentials::new("key", "secret", "pass");
    let payload = login_payload(&credentials, "1700000000").unwrap();
    let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    assert_eq!(value["op"], "login");
    assert_eq!(value["args"][0]["apiKey"], "key");
    assert_eq!(value["args"][0]["passphrase"], "pass");
    assert_eq!(value["args"][0]["timestamp"], "1700000000");
    assert_eq!(
        value["args"][0]["sign"],
        signing::ws_login_sign("1700000000", "secret")
    );
}

#[test]
fn arg_serializes_optional_and_extra_fields() {
    let arg = Arg::new("sprd-orders")
        .sprd_id("BTC-USDT_BTC-USDT-SWAP")
        .param("customField", "custom-value");
    let value = serde_json::to_value(&arg).unwrap();
    assert_eq!(value["channel"], "sprd-orders");
    assert_eq!(value["sprdId"], "BTC-USDT_BTC-USDT-SWAP");
    assert_eq!(value["customField"], "custom-value");
    assert!(value.get("instType").is_none());
}

#[test]
fn arg_deserializes_extra_fields() {
    let arg: Arg =
        serde_json::from_str(r#"{"channel":"account-greeks","ccy":"USDT","instType":"OPTION"}"#)
            .unwrap();
    assert_eq!(arg.channel, "account-greeks");
    assert_eq!(arg.inst_type.as_deref(), Some("OPTION"));
    assert_eq!(arg.extra.get("ccy").map(String::as_str), Some("USDT"));
}

#[test]
fn channel_helpers_build_expected_args() {
    let ticker = channels::market::tickers("BTC-USDT");
    assert_eq!(ticker.channel, "tickers");
    assert_eq!(ticker.inst_id.as_deref(), Some("BTC-USDT"));

    let funding = channels::public_data::funding_rate("BTC-USDT-SWAP");
    assert_eq!(funding.channel, "funding-rate");
    assert_eq!(funding.inst_id.as_deref(), Some("BTC-USDT-SWAP"));

    let account = channels::account::account_by_currency("USDT");
    assert_eq!(account.channel, "account");
    assert_eq!(account.extra.get("ccy").map(String::as_str), Some("USDT"));

    let spread = channels::spread::orders_by_spread("BTC-USDT_BTC-USDT-SWAP");
    assert_eq!(spread.channel, "sprd-orders");
    assert_eq!(
        spread.extra.get("sprdId").map(String::as_str),
        Some("BTC-USDT_BTC-USDT-SWAP")
    );
}

#[test]
fn parse_notice_and_channel_connection_count_events() {
    let notice =
        parse_text_event(r#"{"event":"notice","code":"64008","msg":"service upgrade soon"}"#)
            .unwrap()
            .unwrap();
    match notice {
        WsEvent::Notice(notice) => assert_eq!(notice.msg, "service upgrade soon"),
        other => panic!("expected notice, got {other:?}"),
    }

    let count = parse_text_event(
        r#"{"event":"channel-conn-count","arg":{"channel":"orders","instType":"ANY"},"connCount":"2","connId":"abc"}"#,
    )
    .unwrap()
    .unwrap();
    match count {
        WsEvent::ChannelConnectionCount(count) => {
            assert_eq!(count.arg.channel, "orders");
            assert_eq!(count.arg.inst_type.as_deref(), Some("ANY"));
            assert_eq!(count.conn_count.as_str(), "2");
            assert_eq!(count.conn_id, "abc");
        }
        other => panic!("expected channel count, got {other:?}"),
    }
}

#[test]
fn parse_operation_response_and_rows() {
    let event = parse_text_event(
        r#"{"id":"1512","op":"cancel-order","code":"0","msg":"","data":[{"ordId":"1","clOrdId":"","sCode":"0","sMsg":""}],"inTime":"1695190491421339","outTime":"1695190491423240"}"#,
    )
    .unwrap()
    .unwrap();
    let WsEvent::Operation(operation) = event else {
        panic!("expected operation");
    };
    assert_eq!(operation.id.as_deref(), Some("1512"));
    assert_eq!(operation.op, "cancel-order");
    assert_eq!(
        operation.in_time.as_ref().unwrap().as_str(),
        "1695190491421339"
    );
    let rows: Vec<crate::api::trade::CancelOrderResult> = operation.parse().unwrap();
    assert_eq!(rows[0].s_code, "0");
}

#[test]
fn operation_payload_serializes_trade_requests() {
    let request = PlaceOrderRequest::new(
        "BTC-USDT",
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Limit,
        "0.001",
    )
    .price("100");
    let payload = operation_payload("1", "order", std::slice::from_ref(&request)).unwrap();
    let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    assert_eq!(value["id"], "1");
    assert_eq!(value["op"], "order");
    assert_eq!(value["args"][0]["instId"], "BTC-USDT");
    assert_eq!(value["args"][0]["tdMode"], "cash");

    let cancel = CancelOrderRequest::by_order_id("BTC-USDT", "ord-1");
    let payload = operation_payload("2", "cancel-order", std::slice::from_ref(&cancel)).unwrap();
    let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    assert_eq!(value["op"], "cancel-order");
    assert_eq!(value["args"][0]["ordId"], "ord-1");
}
