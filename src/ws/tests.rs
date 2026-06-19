use serde::Serialize;

use crate::Error;
use crate::api::trade::{CancelOrderRequest, PlaceOrderRequest};
use crate::model::{OrderSide, OrderType, TradeMode};
use crate::signing;
use crate::ws::channels;
use crate::ws::client::login_payload;
use crate::ws::event::parse_text_event;
use crate::ws::model::AccountUpdate;
use crate::ws::ops::operation_payload_with_expiry;
use crate::ws::request::{
    AmendSpreadOrderRequest, CancelSpreadOrderRequest, MassCancelRequest,
    MassCancelSpreadOrdersRequest, PlaceSpreadOrderRequest,
};
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

    let event_markets = channels::public_data::event_contract_markets();
    assert_eq!(event_markets.inst_type.as_deref(), Some("EVENTS"));

    let estimated = channels::public_data::estimated_price("OPTION", "BTC-USD-260626-100000-C");
    assert_eq!(estimated.inst_type.as_deref(), Some("OPTION"));
    assert_eq!(
        estimated.inst_id.as_deref(),
        Some("BTC-USD-260626-100000-C")
    );

    let spot_grid = channels::grid::spot_grid_orders();
    assert_eq!(spot_grid.inst_type.as_deref(), Some("SPOT"));
    let recurring = channels::grid::recurring_buy_orders();
    assert_eq!(recurring.channel, "algo-recurring-buy");

    let account = channels::account::account_by_currency("USDT", None);
    assert_eq!(account.channel, "account");
    assert_eq!(account.extra.get("ccy").map(String::as_str), Some("USDT"));

    let pos = channels::account::positions("ANY", Some("0"));
    assert_eq!(pos.channel, "positions");
    assert_eq!(pos.inst_type.as_deref(), Some("ANY"));
    assert_eq!(
        pos.extra.get("extraParams").map(String::as_str),
        Some(r#"{"updateInterval":"0"}"#)
    );

    let pos_default = channels::account::positions("ANY", None);
    assert_eq!(pos_default.channel, "positions");
    assert!(!pos_default.extra.contains_key("extraParams"));

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
        r#"{"event":"channel-conn-count","channel":"orders","connCount":"2","connId":"abc"}"#,
    )
    .unwrap()
    .unwrap();
    match count {
        WsEvent::ChannelConnectionCount(count) => {
            assert_eq!(count.channel, "orders");
            assert_eq!(count.conn_count.as_str(), "2");
            assert_eq!(count.conn_id, "abc");
        }
        other => panic!("expected channel count, got {other:?}"),
    }

    let limit_error = parse_text_event(
        r#"{"event":"channel-conn-count-error","channel":"orders","connCount":"30","connId":"abc"}"#,
    )
    .unwrap()
    .unwrap();
    match limit_error {
        WsEvent::ChannelConnectionCountError(count) => {
            assert_eq!(count.channel, "orders");
            assert_eq!(count.conn_count.as_str(), "30");
            assert_eq!(count.conn_id, "abc");
        }
        other => panic!("expected channel count error, got {other:?}"),
    }
}

#[test]
fn parse_balance_and_position_push() {
    let event = parse_text_event(
        r#"{"arg":{"channel":"balance_and_position","uid":"77982378738415879"},"data":[{"pTime":"1597026383085","eventType":"snapshot","balData":[{"ccy":"BTC","cashBal":"1","uTime":"1597026383085"}],"posData":[{"posId":"1111111111","tradeId":"2","instId":"BTC-USD-191018","instType":"FUTURES","mgnMode":"cross","posSide":"long","pos":"10","ccy":"BTC","posCcy":"","avgPx":"3320","nonSettleAvgPx":"","settledPnl":"","uTime":"1597026383085"}],"trades":[{"instId":"BTC-USD-191018","tradeId":"2"}]}]}"#,
    )
    .unwrap()
    .unwrap();

    let WsEvent::Push(push) = event else {
        panic!("expected push");
    };
    assert_eq!(push.arg.channel, "balance_and_position");
    assert_eq!(
        push.arg.extra.get("uid").map(String::as_str),
        Some("77982378738415879")
    );

    let rows: Vec<crate::ws::model::BalanceAndPositionUpdate> = push.parse().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].p_time.as_str(), "1597026383085");
    assert_eq!(rows[0].event_type, "snapshot");
    assert_eq!(rows[0].bal_data[0].ccy, "BTC");
    assert_eq!(rows[0].bal_data[0].cash_bal.as_str(), "1");
    assert_eq!(rows[0].pos_data[0].inst_id, "BTC-USD-191018");
    assert_eq!(rows[0].pos_data[0].pos.as_str(), "10");
    assert_eq!(rows[0].trades[0].trade_id, "2");
}

#[test]
fn parse_account_push() {
    let json = r#"{
        "arg":{"channel":"account","uid":"44**584"},
        "eventType":"snapshot",
        "curPage":1,
        "lastPage":true,
        "data":[{
            "adjEq":"55444.12","availEq":"55444.12","borrowFroz":"0",
            "delta":"0","deltaLever":"0","deltaNeutralStatus":"0",
            "imr":"0","isoEq":"0","mgnRatio":"","mmr":"0",
            "notionalUsd":"0","notionalUsdForBorrow":"0","notionalUsdForFutures":"0",
            "notionalUsdForOption":"0","notionalUsdForSwap":"0",
            "ordFroz":"0","totalEq":"55868.06","uTime":"1705564223311","upl":"0",
            "details":[{
                "availBal":"4734.37","availEq":"4734.37","borrowFroz":"0",
                "cashBal":"4750.42","ccy":"USDT","coinUsdPrice":"0.99927",
                "crossLiab":"0","colRes":"0","collateralEnabled":false,
                "colBorrAutoConversion":"0","disEq":"4889.37","eq":"4892.95",
                "eqUsd":"4889.37","smtSyncEq":"0","spotCopyTradingEq":"0",
                "fixedBal":"0","frozenBal":"158.57","frpType":"0",
                "imr":"","interest":"0","isoEq":"0","isoLiab":"0","isoUpl":"0",
                "liab":"0","maxLoan":"0","mgnRatio":"","mmr":"","notionalLever":"",
                "ordFrozen":"0","rewardBal":"0","spotInUseAmt":"","clSpotInUseAmt":"",
                "maxSpotInUseAmt":"","spotIsoBal":"0","stgyEq":"150","twap":"0",
                "uTime":"1705564213903","upl":"-7.47","uplLiab":"0",
                "spotBal":"","openAvgPx":"","accAvgPx":"","spotUpl":"",
                "spotUplRatio":"","totalPnl":"","totalPnlRatio":""
            }]
        }]
    }"#;

    let event = parse_text_event(json).unwrap().unwrap();
    let WsEvent::Push(push) = event else {
        panic!("expected Push, got {:?}", event);
    };
    assert_eq!(push.arg.channel, "account");
    assert_eq!(
        push.arg.extra.get("uid").map(String::as_str),
        Some("44**584")
    );
    assert_eq!(push.action.as_deref(), Some("snapshot"));

    let rows: Vec<AccountUpdate> = push.parse().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].total_eq.as_str(), "55868.06");
    assert_eq!(rows[0].u_time.as_str(), "1705564223311");
    assert_eq!(rows[0].details.len(), 1);
    assert_eq!(rows[0].details[0].ccy, "USDT");
    assert_eq!(rows[0].details[0].cash_bal.as_str(), "4750.42");
    assert_eq!(rows[0].details[0].stgy_eq.as_str(), "150");
    assert!(!rows[0].details[0].collateral_enabled);
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

#[test]
fn operation_payload_serializes_expiry_and_typed_ws_requests() {
    let mmp = MassCancelRequest::option("BTC-USD").lock_interval("1000");
    let payload = operation_payload_with_expiry(
        "mmp-1",
        "mass-cancel",
        Some("1900000000000".to_owned()),
        std::slice::from_ref(&mmp),
    )
    .unwrap();
    let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    assert_eq!(value["expTime"], "1900000000000");
    assert_eq!(value["args"][0]["instType"], "OPTION");
    assert_eq!(value["args"][0]["instFamily"], "BTC-USD");

    let place = PlaceSpreadOrderRequest::new("BTC-USDT_BTC-USDT-SWAP", "buy", "limit", "1")
        .client_order_id("spread-1")
        .price("10");
    let payload =
        operation_payload("spread-place", "sprd-order", std::slice::from_ref(&place)).unwrap();
    let value: serde_json::Value = serde_json::from_str(&payload).unwrap();
    assert_eq!(value["args"][0]["sprdId"], "BTC-USDT_BTC-USDT-SWAP");
    assert_eq!(value["args"][0]["clOrdId"], "spread-1");
    assert_eq!(value["args"][0]["px"], "10");

    let amend = AmendSpreadOrderRequest::by_order_id("ord-1")
        .request_id("req-1")
        .new_price("11");
    let value = serde_json::to_value(amend).unwrap();
    assert_eq!(value["ordId"], "ord-1");
    assert_eq!(value["reqId"], "req-1");
    assert_eq!(value["newPx"], "11");

    let cancel = CancelSpreadOrderRequest::by_client_order_id("spread-1");
    let value = serde_json::to_value(cancel).unwrap();
    assert_eq!(value["clOrdId"], "spread-1");
    assert!(value.get("ordId").is_none());

    let cancel_all = MassCancelSpreadOrdersRequest::all();
    let value = serde_json::to_value(cancel_all).unwrap();
    assert_eq!(value, serde_json::json!({}));
}

fn operation_payload<A: Serialize>(
    id: impl Into<String>,
    op: impl Into<String>,
    args: &[A],
) -> Result<String, Error> {
    operation_payload_with_expiry(id, op, None, args)
}
