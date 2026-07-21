use crate::api::account::{
    MovePositionFrom, MovePositionLeg, MovePositionTo, MovePositionsHistoryRequest,
    MovePositionsRequest, PositionBuilderGraphAsset, PositionBuilderGraphMmrConfig,
    PositionBuilderGraphPosition, PositionBuilderGraphRequest,
};
use crate::model::{OrderSide, PositionSide, TradeMode};
use crate::test_util::MockTransport;

use super::signed_client;

#[tokio::test]
async fn move_positions_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"blockTdId":"2066393411110139648","clientId":"transfer1","state":"filled","ts":"1734085725000","fromAcct":"0","toAcct":"sub1","legs":[{"from":{"posId":"2065477911110792832","instId":"BTC-USD-SWAP","px":"100123.8","side":"sell","sz":"1","sCode":"0","sMsg":""},"to":{"instId":"BTC-USD-SWAP","px":"100123.8","side":"buy","sz":"1","tdMode":"cross","posSide":"net","ccy":"","sCode":"0","sMsg":""}}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MovePositionsRequest::new(
        "0",
        "sub1",
        vec![MovePositionLeg::new(
            MovePositionFrom::new("2065477911110792832", OrderSide::Sell, "1"),
            MovePositionTo::new()
                .trade_mode(TradeMode::Cross)
                .position_side(PositionSide::Net),
        )],
        "transfer1",
    );

    let result = client.account().move_positions(&request).await.unwrap();
    assert_eq!(result[0].block_td_id, "2066393411110139648");
    assert_eq!(result[0].legs[0].from.pos_id, "2065477911110792832");
    assert_eq!(result[0].legs[0].to.px.as_str(), "100123.8");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req.uri.ends_with("/api/v5/account/move-positions"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["fromAcct"], "0");
    assert_eq!(sent["toAcct"], "sub1");
    assert_eq!(sent["clientId"], "transfer1");
    assert_eq!(sent["legs"][0]["from"]["posId"], "2065477911110792832");
    assert_eq!(sent["legs"][0]["from"]["side"], "sell");
    assert_eq!(sent["legs"][0]["to"]["tdMode"], "cross");
    assert_eq!(sent["legs"][0]["to"]["posSide"], "net");
    assert!(sent["legs"][0]["to"].get("ccy").is_none());
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_move_positions_history_uses_documented_query() {
    let body = r#"{"code":"0","msg":"","data":[{"clientId":"transfer1","blockTdId":"2066393411110139648","state":"filled","ts":"1734085725000","fromAcct":"0","toAcct":"sub1","legs":[{"from":{"posId":"2065477911110792832","instId":"BTC-USD-SWAP","px":"100123.8","side":"sell","sz":"1"},"to":{"instId":"BTC-USD-SWAP","px":"100123.8","side":"buy","sz":"1","tdMode":"cross","posSide":"net","ccy":""}}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = MovePositionsHistoryRequest::new()
        .client_id("transfer1")
        .begin_timestamp("1734085000000")
        .state("filled");

    let result = client
        .account()
        .get_move_positions_history(&request)
        .await
        .unwrap();
    assert_eq!(result[0].state, "filled");
    assert_eq!(result[0].ts.as_str(), "1734085725000");
    assert_eq!(result[0].legs[0].to.pos_side, PositionSide::Net);

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert_eq!(
        req.query(),
        Some("clientId=transfer1&beginTs=1734085000000&state=filled")
    );
    assert!(req.uri.contains("/api/v5/account/move-positions-history?"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn position_builder_graph_posts_documented_body() {
    let body = r#"{"code":"0","msg":"","data":[{"type":"mmr","mmrData":[{"shockFactor":"-0.94","mmr":"1415.0254039225917","mmrRatio":"-47.45603627655477"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = PositionBuilderGraphRequest::new(
        PositionBuilderGraphMmrConfig::new()
            .account_level("3")
            .leverage("1"),
    )
    .include_real_positions_and_equity(false)
    .simulated_positions(vec![PositionBuilderGraphPosition::new(
        "BTC-USDT-SWAP",
        "-10",
        "100000",
    )])
    .simulated_assets(vec![PositionBuilderGraphAsset::new("USDT", "100")])
    .greeks_type("CASH");

    let result = client
        .account()
        .position_builder_graph(&request)
        .await
        .unwrap();
    assert_eq!(result[0].graph_type, "mmr");
    assert_eq!(result[0].mmr_data[0].shock_factor.as_str(), "-0.94");
    assert_eq!(result[0].mmr_data[0].mmr.as_str(), "1415.0254039225917");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::POST);
    assert!(req
        .uri
        .ends_with("/api/v5/account/position-builder-graph"));
    let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
    assert_eq!(sent["inclRealPosAndEq"], false);
    assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
    assert_eq!(sent["simPos"][0]["pos"], "-10");
    assert_eq!(sent["simPos"][0]["avgPx"], "100000");
    assert_eq!(sent["simAsset"][0]["amt"], "100");
    assert_eq!(sent["greeksType"], "CASH");
    assert_eq!(sent["type"], "mmr");
    assert_eq!(sent["mmrConfig"]["acctLv"], "3");
    assert_eq!(sent["mmrConfig"]["lever"], "1");
    assert!(req.is_signed());
}
