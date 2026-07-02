use crate::api::account::BalanceRequest;
use crate::model::InstType;
use crate::test_util::MockTransport;

use super::super::{
    AccountPositionTiersRequest, PositionBuilderRequest, SetRiskOffsetAmountRequest,
    SimulatedAsset, SimulatedMarginRequest, SimulatedPosition,
};
use super::signed_client;

#[tokio::test]
async fn get_greeks_queries_currency() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","deltaBS":"1","deltaPA":"0.91","gammaBS":"0.00111","gammaPA":"0.00121",
        "thetaBS":"-24.46655352","thetaPA":"-23.25","ts":"1630982931600","vegaBS":"4601.26","vegaPA":"4626.55"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .get_greeks(BalanceRequest::new().currency("BTC"))
        .await
        .unwrap();
    assert_eq!(result[0].delta_pa.as_str(), "0.91");

    let req = mock.captured();
    assert_eq!(req.query(), Some("ccy=BTC"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_position_tiers_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"OPTION","uly":"BTC-USD","instFamily":"BTC-USD","posType":"1",
        "minSz":"0","maxSz":"100","mmrFactor":"0.1","imrFactor":"0.3","netDeltaAmount":"0.02"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = AccountPositionTiersRequest::new(InstType::Option).underlying("BTC-USD");

    let result = client
        .account()
        .get_account_position_tiers(&request)
        .await
        .unwrap();
    assert_eq!(result[0].max_sz.as_str(), "100");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=OPTION&uly=BTC-USD"));
    assert!(req.is_signed());
}

#[test]
fn account_position_tiers_request_serializes_inst_family_filter() {
    let request = AccountPositionTiersRequest::new(InstType::Swap).inst_family("BTC-USDT");

    let query = serde_urlencoded::to_string(&request).unwrap();
    assert_eq!(query, "instType=SWAP&instFamily=BTC-USDT");
}

#[tokio::test]
async fn get_simulated_margin_posts_body_and_omits_unset_fields() {
    let body = r#"{"code":"0","msg":"","data":[{
        "imr":"3174.99999","mmr":"126.39999","mr":"54085.3541","notionalUsd":"","ts":"1629453864251",
        "details":[{
            "instId":"BTC-USDT-SWAP","pos":"1","imr":"3174.99999","mmr":"126.39999",
            "upl":"12.20","optVal":"","delta":"","gamma":"","vega":"","theta":""}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SimulatedMarginRequest::new()
        .inst_type(InstType::Swap)
        .simulated_positions(vec![
            SimulatedPosition::new("BTC-USDT-SWAP")
                .position("1")
                .leverage("10"),
        ]);

    let result = client
        .account()
        .get_simulated_margin(&request)
        .await
        .unwrap();
    assert_eq!(result[0].details[0].upl.as_str(), "12.20");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/simulated_margin"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["instType"], "SWAP");
    assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["simPos"][0]["pos"], "1");
    assert_eq!(sent["simPos"][0]["lever"], "10");
    assert!(sent.get("inclRealPos").is_none());
    assert!(sent["simPos"][0].get("avgPx").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn set_risk_offset_amount_posts_body() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"BTC","clSpotInUseAmt":"0.5"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client
        .account()
        .set_risk_offset_amount(&SetRiskOffsetAmountRequest::new("BTC", "0.5"))
        .await
        .unwrap();
    assert_eq!(result[0].ccy, "BTC");
    assert_eq!(result[0].cl_spot_in_use_amt.as_str(), "0.5");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/set-riskOffset-amt"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["ccy"], "BTC");
    assert_eq!(sent["clSpotInUseAmt"], "0.5");
    assert!(req.is_signed());
}

#[tokio::test]
async fn position_builder_posts_body_and_omits_unset_fields() {
    let body = r#"{"code":"0","msg":"","data":[{
        "acctLv":"2","adjEq":"10000","imr":"3174.99","mmr":"126.39","mr":"0","notionalUsd":"0",
        "riskUnit":"USD","ts":"1629453864251",
        "posData":[{
            "instType":"SWAP","instId":"BTC-USDT-SWAP","pos":"1","avgPx":"42000","upl":"2",
            "delta":"1","gamma":"0","vega":"0","theta":"0"}],
        "assetData":[{"ccy":"USDT","eq":"10000","borrowMmd":"","borrowImr":""}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = PositionBuilderRequest::new()
        .account_level("2")
        .include_real_positions_and_equity(false)
        .simulated_positions(vec![SimulatedPosition::new("BTC-USDT-SWAP").position("1")])
        .simulated_assets(vec![SimulatedAsset::new("USDT").equity("10000")]);

    let result = client.account().position_builder(&request).await.unwrap();
    assert_eq!(result[0].pos_data[0].inst_id, "BTC-USDT-SWAP");
    assert_eq!(result[0].asset_data[0].eq.as_str(), "10000");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/position-builder"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["acctLv"], "2");
    assert_eq!(sent["inclRealPosAndEq"], false);
    assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["simAsset"][0]["ccy"], "USDT");
    assert!(sent.get("lever").is_none());
    assert!(sent["simPos"][0].get("avgPx").is_none());
    assert!(req.is_signed());
}
