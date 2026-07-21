use crate::api::account::{
    MovePositionFrom, MovePositionLeg, MovePositionTo, MovePositionsRequest,
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
