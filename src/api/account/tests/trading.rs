use crate::model::{InstType, PositionSide, TradeMode};
use crate::test_util::MockTransport;

use super::super::{
    AccountInstrumentsRequest, AdjustLeverageInfoRequest, AdjustMarginRequest, BalanceRequest,
    FeeRatesRequest, LeverageRequest, MaxAvailableSizeRequest, MaxOrderSizeRequest,
    SetAccountLevelRequest, SetCollateralAssetsRequest, SetGreeksRequest, SetIsolatedModeRequest,
    SetLeverageRequest, SetPositionModeRequest,
};
use super::signed_client;

#[tokio::test]
async fn set_position_mode_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"posMode":"net_mode"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_position_mode(&SetPositionModeRequest::new("net_mode"))
        .await
        .unwrap();
    assert_eq!(result[0].pos_mode, "net_mode");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-position-mode"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["posMode"], "net_mode");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_collateral_assets_posts_all_body() {
    let body = r#"{"code":"0","msg":"","data":[{"type":"all","ccyList":["BTC","ETH"],"collateralEnabled":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetCollateralAssetsRequest::all(true);

    let result = client
        .account()
        .set_collateral_assets(&request)
        .await
        .unwrap();
    assert_eq!(result[0].collateral_type, "all");
    assert_eq!(result[0].ccy_list, ["BTC", "ETH"]);
    assert!(result[0].collateral_enabled);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-collateral-assets"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["type"], "all");
    assert_eq!(sent["collateralEnabled"], true);
    assert!(sent.get("ccyList").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_collateral_assets_posts_custom_body() {
    let body = r#"{"code":"0","msg":"","data":[{"type":"custom","ccyList":["BTC","ETH"],"collateralEnabled":false}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetCollateralAssetsRequest::custom(["BTC", "ETH"], false);

    client
        .account()
        .set_collateral_assets(&request)
        .await
        .unwrap();

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-collateral-assets"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["type"], "custom");
    assert_eq!(sent["collateralEnabled"], false);
    assert_eq!(sent["ccyList"], serde_json::json!(["BTC", "ETH"]));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_leverage_posts_builder_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT-SWAP","lever":"5","mgnMode":"cross","posSide":"long"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetLeverageRequest::new("5", TradeMode::Cross)
        .inst_id("BTC-USDT-SWAP")
        .position_side(PositionSide::Long);

    let result = client.account().set_leverage(&request).await.unwrap();
    assert_eq!(result[0].lever.as_str(), "5");

    let req = mock.captured();
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["lever"], "5");
    assert_eq!(sent["mgnMode"], "cross");
    assert_eq!(sent["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["posSide"], "long");
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_leverage_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT-SWAP","lever":"5","mgnMode":"cross","posSide":"long"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = LeverageRequest::new(TradeMode::Cross).inst_id("BTC-USDT-SWAP");

    let result = client.account().get_leverage(&request).await.unwrap();
    assert_eq!(result[0].mgn_mode, TradeMode::Cross);

    let req = mock.captured();
    assert_eq!(req.query(), Some("mgnMode=cross&instId=BTC-USDT-SWAP"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_adjust_leverage_info_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "estAvailQuoteTrans":"","estAvailTrans":"1.1398040558348279",
        "estLiqPx":"","estMaxAmt":"10.6095865868904898",
        "estMgn":"0.0701959441651721","estQuoteMgn":"",
        "estQuoteMaxAmt":"176889.6871254563042714","existOrd":false,
        "maxLever":"10","minLever":"0.01"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AdjustLeverageInfoRequest::new(InstType::Margin, TradeMode::Isolated, "5")
        .inst_id("BTC-USDT")
        .currency("BTC")
        .position_side(PositionSide::Net);

    let result = client
        .account()
        .get_adjust_leverage_info(&request)
        .await
        .unwrap();
    assert_eq!(result[0].est_avail_trans.as_str(), "1.1398040558348279");
    assert!(!result[0].exist_ord);
    assert_eq!(result[0].max_lever.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(
        req.query(),
        Some("instType=MARGIN&mgnMode=isolated&lever=5&instId=BTC-USDT&ccy=BTC&posSide=net")
    );
    assert!(req.uri.contains("/api/v5/account/adjust-leverage-info?"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_order_size_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT","ccy":"","maxBuy":"9.6782340237","maxSell":"0.0049"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MaxOrderSizeRequest::new("BTC-USDT", TradeMode::Cash).price("41960.8");

    let result = client.account().get_max_order_size(&request).await.unwrap();
    assert_eq!(result[0].max_buy.as_str(), "9.6782340237");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT&tdMode=cash&px=41960.8"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_avail_size_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT","availBuy":"186.0717690","availSell":"0.12"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MaxAvailableSizeRequest::new("BTC-USDT", TradeMode::Cash).reduce_only(false);

    let result = client.account().get_max_avail_size(&request).await.unwrap();
    assert_eq!(result[0].avail_sell.as_str(), "0.12");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instId=BTC-USDT&tdMode=cash&reduceOnly=false")
    );
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fee_rates_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "category":"1","delivery":"","exercise":"",
        "feeGroup":[{"elpMaker":"-0.0008","groupId":"1","maker":"-0.0008","taker":"-0.001"}],
        "fiat":[{"ccy":"USD","maker":"0.001","taker":"0.002"}],
        "instType":"SPOT","level":"Lv1","maker":"-0.0008","makerU":"","makerUSDC":"",
        "ruleType":"normal","settle":"","taker":"-0.001","takerU":"","takerUSDC":"",
        "ts":"1763979985847"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FeeRatesRequest::new(InstType::Spot).inst_id("BTC-USDT");

    let result = client.account().get_fee_rates(&request).await.unwrap();
    assert_eq!(result[0].maker.as_str(), "-0.0008");
    assert_eq!(result[0].level, "Lv1");
    assert_eq!(result[0].fee_group[0].group_id, "1");
    assert_eq!(result[0].fee_group[0].elp_maker.as_str(), "-0.0008");
    assert_eq!(result[0].taker.as_str(), "-0.001");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&instId=BTC-USDT"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fee_rates_can_query_group_id() {
    let body = r#"{"code":"0","msg":"","data":[{
        "category":"1","delivery":"","exercise":"","feeGroup":[],"fiat":[],
        "instType":"SPOT","level":"Lv1","maker":"-0.0008","makerU":"","makerUSDC":"",
        "ruleType":"normal","settle":"","taker":"-0.001","takerU":"","takerUSDC":"",
        "ts":"1763979985847"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FeeRatesRequest::new(InstType::Spot).group_id("1");

    let result = client.account().get_fee_rates(&request).await.unwrap();
    assert_eq!(result[0].rule_type, "normal");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SPOT&groupId=1"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_fee_rates_can_query_instrument_family() {
    let body = r#"{"code":"0","msg":"","data":[{
        "category":"1","delivery":"","exercise":"","feeGroup":[],"fiat":[],
        "instType":"SWAP","level":"Lv1","maker":"","makerU":"-0.0002","makerUSDC":"",
        "ruleType":"normal","settle":"","taker":"","takerU":"0.0005","takerUSDC":"",
        "ts":"1763979985847"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FeeRatesRequest::new(InstType::Swap).inst_family("BTC-USD");

    let result = client.account().get_fee_rates(&request).await.unwrap();
    assert_eq!(result[0].inst_type, InstType::Swap);
    assert_eq!(result[0].level, "Lv1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USD"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_max_withdrawal_queries_currency() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","maxWd":"124.82837647","maxWdEx":"125.7","spotOffsetMaxWd":"","spotOffsetMaxWdEx":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .get_max_withdrawal(BalanceRequest::new().currency("BTC"))
        .await
        .unwrap();
    assert_eq!(result[0].max_wd.as_str(), "124.82837647");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn adjust_margin_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instId":"BTC-USDT-SWAP","posSide":"long","type":"add","amt":"100","cTime":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AdjustMarginRequest::new("BTC-USDT-SWAP", PositionSide::Long, "add", "100")
        .loan_transfer(true);

    let result = client.account().adjust_margin(&request).await.unwrap();
    assert_eq!(result[0].amt.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/position/margin-balance"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["posSide"], "long");
    assert_eq!(sent["type"], "add");
    assert_eq!(sent["amt"], "100");
    assert_eq!(sent["loanTrans"], true);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_instruments_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SWAP","instId":"BTC-USDT-SWAP","uly":"BTC-USDT","instFamily":"BTC-USDT",
        "baseCcy":"","quoteCcy":"","settleCcy":"USDT","ctVal":"0.01","ctMult":"1",
        "ctValCcy":"BTC","optType":"","stk":"","listTime":"1597026383085","expTime":"",
        "lever":"125","tickSz":"0.1","lotSz":"1","minSz":"1","ctType":"linear","alias":"",
        "state":"live","maxLmtSz":"100","maxMktSz":"100","maxMktIcebergSz":"100",
        "maxTwapSz":"100","maxIcebergSz":"100","maxStopSz":"100","maxTriggerSz":"100",
        "maxLmtAmt":"100","maxMktAmt":"100","ruleType":"","riskLimitType":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AccountInstrumentsRequest::new(InstType::Swap).inst_id("BTC-USDT-SWAP");

    let result = client
        .account()
        .get_account_instruments(&request)
        .await
        .unwrap();
    assert_eq!(result[0].settle_ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_greeks_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"greeksType":"PA"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_greeks(&SetGreeksRequest::new("PA"))
        .await
        .unwrap();
    assert_eq!(result[0].greeks_type, "PA");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-greeks"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["greeksType"], "PA");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_isolated_mode_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"isoMode":"automatic","type":"MARGIN"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_isolated_mode(&SetIsolatedModeRequest::new("automatic", "MARGIN"))
        .await
        .unwrap();
    assert_eq!(result[0].iso_mode, "automatic");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-isolated-mode"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["isoMode"], "automatic");
    assert_eq!(sent["type"], "MARGIN");
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_account_level_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"acctLv":"2"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_account_level(&SetAccountLevelRequest::new("2"))
        .await
        .unwrap();
    assert_eq!(result[0].acct_lv, "2");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-account-level"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["acctLv"], "2");
    assert!(req.is_signed());
}

#[tokio::test]
async fn activate_option_posts_empty_body() {
    let body = r#"{"code":"0","msg":"","data":[{"result":"true"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().activate_option().await.unwrap();
    assert_eq!(result[0].result, "true");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/activate-option"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent, serde_json::json!({}));
    assert!(req.is_signed());
}
