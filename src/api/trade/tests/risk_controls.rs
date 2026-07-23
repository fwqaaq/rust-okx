use crate::model::{OrderSide, OrderType, TradeMode};
use crate::test_util::MockTransport;

use super::super::{
    CancelAllAfterRequest, MassCancelRequest, OrderPrecheckAttachedAlgoOrderRequest,
    OrderPrecheckRequest,
};
use super::signed_client;

#[tokio::test]
async fn mass_cancel_posts_documented_option_body() {
    let body = r#"{"code":"0","msg":"","data":[{"result":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MassCancelRequest::option("BTC-USD").lock_interval("1000");

    let result = client.trade().mass_cancel(&request).await.unwrap();
    assert!(result[0].result);

    let captured = mock.captured();
    assert_eq!(captured.method, http::Method::POST);
    assert!(captured.uri.ends_with("/api/v5/trade/mass-cancel"));
    assert!(captured.is_signed());
    let sent: serde_json::Value = serde_json::from_str(captured.body_str()).unwrap();
    assert_eq!(sent["instType"], "OPTION");
    assert_eq!(sent["instFamily"], "BTC-USD");
    assert_eq!(sent["lockInterval"], "1000");
}

#[tokio::test]
async fn cancel_all_after_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "triggerTime":"1587971460","tag":"","ts":"1587971400"
    }]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CancelAllAfterRequest::new("60").tag("strategy1");

    let result = client.trade().cancel_all_after(&request).await.unwrap();
    assert_eq!(result[0].trigger_time.as_str(), "1587971460");
    assert_eq!(result[0].tag, "");
    assert_eq!(result[0].ts.as_str(), "1587971400");

    let captured = mock.captured();
    assert!(captured
        .uri
        .ends_with("/api/v5/trade/cancel-all-after"));
    assert!(captured.is_signed());
    let sent: serde_json::Value = serde_json::from_str(captured.body_str()).unwrap();
    assert_eq!(sent["timeOut"], "60");
    assert_eq!(sent["tag"], "strategy1");
}

#[tokio::test]
async fn get_account_rate_limit_sends_signed_empty_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "accRateLimit":"2000","fillRatio":"0.1234","mainFillRatio":"0.1234",
        "nextAccRateLimit":"2000","ts":"123456789000"
    }]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.trade().get_account_rate_limit().await.unwrap();
    assert_eq!(result[0].fill_ratio.as_str(), "0.1234");
    assert_eq!(result[0].main_fill_ratio.as_str(), "0.1234");
    assert_eq!(result[0].acc_rate_limit.as_str(), "2000");
    assert_eq!(result[0].next_acc_rate_limit.as_str(), "2000");
    assert_eq!(result[0].ts.as_str(), "123456789000");

    let captured = mock.captured();
    assert_eq!(captured.method, http::Method::GET);
    assert!(captured
        .uri
        .ends_with("/api/v5/trade/account-rate-limit"));
    assert_eq!(captured.query(), None);
    assert!(captured.is_signed());
}

#[tokio::test]
async fn order_precheck_posts_only_documented_fields_and_decodes_response() {
    let body = r#"{"code":"0","data":[{
        "adjEq":"41.94347460746277","adjEqChg":"-226.05616481626",
        "availBal":"0","availBalChg":"0","imr":"0","imrChg":"57.74709688430927",
        "liab":"0","liabChg":"0","liabChgCcy":"","liqPx":"6764.8556232031115",
        "liqPxDiff":"-57693.044376796888536773622035980224609375",
        "liqPxDiffRatio":"-0.8950500152315991","mgnRatio":"0","mgnRatioChg":"0",
        "mmr":"0","mmrChg":"0","posBal":"","posBalChg":"","type":""
    }],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let attached = OrderPrecheckAttachedAlgoOrderRequest::new()
        .client_algo_order_id("attach1")
        .take_profit("3", "-1")
        .take_profit_order_kind("condition")
        .take_profit_price_type("last")
        .stop_loss("1", "-1")
        .stop_loss_price_type("mark")
        .size("1")
        .callback_ratio("0.01")
        .callback_spread("0.1")
        .active_price("2");
    let request = OrderPrecheckRequest::new(
        "BTC-USDT",
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Limit,
        "2",
    )
    .price("2.15")
    .reduce_only(false)
    .target_ccy("base_ccy")
    .attached_algo_orders([attached]);

    let result = client.trade().precheck_order(&request).await.unwrap();
    assert_eq!(result[0].adj_eq.as_str(), "41.94347460746277");
    assert_eq!(result[0].adj_eq_chg.as_str(), "-226.05616481626");
    assert_eq!(result[0].avail_bal.as_str(), "0");
    assert_eq!(result[0].avail_bal_chg.as_str(), "0");
    assert_eq!(result[0].imr.as_str(), "0");
    assert_eq!(result[0].imr_chg.as_str(), "57.74709688430927");
    assert_eq!(result[0].liab.as_str(), "0");
    assert_eq!(result[0].liab_chg.as_str(), "0");
    assert_eq!(result[0].liq_px.as_str(), "6764.8556232031115");
    assert_eq!(
        result[0].liq_px_diff.as_str(),
        "-57693.044376796888536773622035980224609375"
    );
    assert_eq!(
        result[0].liq_px_diff_ratio.as_str(),
        "-0.8950500152315991"
    );
    assert_eq!(result[0].mgn_ratio.as_str(), "0");
    assert_eq!(result[0].mgn_ratio_chg.as_str(), "0");
    assert_eq!(result[0].mmr.as_str(), "0");
    assert_eq!(result[0].mmr_chg.as_str(), "0");
    assert_eq!(result[0].pos_bal.as_str(), "");
    assert_eq!(result[0].pos_bal_chg.as_str(), "");
    assert_eq!(result[0].liab_chg_ccy, "");
    assert_eq!(result[0].type_, "");

    let captured = mock.captured();
    assert_eq!(captured.method, http::Method::POST);
    assert!(captured.uri.ends_with("/api/v5/trade/order-precheck"));
    assert!(captured.is_signed());
    let sent: serde_json::Value = serde_json::from_str(captured.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT");
    assert_eq!(sent["tdMode"], "cash");
    assert_eq!(sent["side"], "buy");
    assert_eq!(sent["ordType"], "limit");
    assert_eq!(sent["sz"], "2");
    assert_eq!(sent["px"], "2.15");
    assert_eq!(sent["reduceOnly"], false);
    assert_eq!(sent["tgtCcy"], "base_ccy");
    assert_eq!(sent["attachAlgoOrds"][0]["tpOrdKind"], "condition");
    assert_eq!(sent["attachAlgoOrds"][0]["callbackRatio"], "0.01");
    assert!(sent.get("clOrdId").is_none());
}

#[test]
fn order_precheck_serializes_documented_event_outcome_and_elp_order_type() {
    let request = OrderPrecheckRequest::new(
        "EVENTS-US-2026",
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Elp,
        "1",
    )
    .outcome("yes");

    let sent = serde_json::to_value(request).unwrap();
    assert_eq!(sent["ordType"], "elp");
    assert_eq!(sent["outcome"], "yes");
}
