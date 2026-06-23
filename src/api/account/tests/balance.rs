use crate::api::account::{BalanceRequest, PositionRiskRequest, PositionsRequest};
use crate::model::{InstType, PositionSide};
use crate::test_util::MockTransport;

use super::signed_client;

#[tokio::test]
async fn get_balance_signs_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "adjEq":"10679.3181948406542","borrowFroz":"","imr":"3372.2942371050594",
        "isoEq":"3372.2942371050594","mgnRatio":"","mmr":"134.891693","notionalUsd":"",
        "ordFroz":"0","totalEq":"11827.8008","uTime":"1597026383085",
        "details":[{
            "availBal":"9000","availEq":"9.9950792","cashBal":"9.9950792",
            "ccy":"BTC","crossLiab":"","disEq":"10000","eq":"9.9950792",
            "eqUsd":"10000","fixedBal":"0","frozenBal":"1000",
            "interest":"","isoEq":"0","isoLiab":"","isoUpl":"0",
            "liab":"","maxLoan":"10000","mgnRatio":"","notionalLever":"0.0022195262185864",
            "ordFrozen":"0","spotInUseAmt":"","stgyEq":"0","twap":"0",
            "uTime":"1597026383085","upl":"0.0009","uplLiab":""}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let balances = client
        .account()
        .get_balance(BalanceRequest::default())
        .await
        .unwrap();
    assert_eq!(balances[0].total_eq.as_str(), "11827.8008");
    assert_eq!(balances[0].details[0].ccy, "BTC");
    assert_eq!(balances[0].details[0].avail_bal.as_str(), "9000");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert!(req.uri.ends_with("/api/v5/account/balance"));
    assert!(req.is_signed(), "authenticated endpoint must be signed");
}

#[tokio::test]
async fn get_positions_passes_filters() {
    let body = r#"{"code":"0","msg":"","data":[{
        "adl":"1","availPos":"1","avgPx":"2566.31","cTime":"1619507758793",
        "ccy":"ETH","deltaBS":"","deltaPA":"","gammaBS":"","gammaPA":"",
        "imr":"","instId":"ETH-USD-210430","instType":"FUTURES",
        "interest":"0","last":"2566.22","lever":"10",
        "liab":"","liabCcy":"","liqPx":"2352.8496681818204","markPx":"2353.849",
        "margin":"0.0003896645377994","mgnMode":"isolated","mgnRatio":"11.731726509588816",
        "mmr":"0.0000311811092","notionalUsd":"2276.2546990924644",
        "optVal":"","pTime":"1619507761462","pos":"1","posCcy":"","posId":"307173036051017730",
        "posSide":"long","spotInUseAmt":"","spotInUseCcy":"","bizRefId":"","bizRefType":"",
        "thetaBS":"","thetaPA":"","tradeId":"109844","uTime":"1619507761462",
        "upl":"-0.0003913207474226","uplRatio":"-1.0047685","vegaBS":"","vegaPA":"",
        "realizedPnl":"0","pnl":"0","fee":"-0.000246405","fundingFee":"0","liqPenalty":"0",
        "closeOrderAlgo":[]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let positions = client
        .account()
        .get_positions(
            &PositionsRequest::new()
                .inst_type(InstType::Futures)
                .inst_id("ETH-USD-210430"),
        )
        .await
        .unwrap();
    assert_eq!(positions[0].inst_id, "ETH-USD-210430");
    assert_eq!(positions[0].pos_side, PositionSide::Long);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=FUTURES&instId=ETH-USD-210430"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_position_risk_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "adjEq":"9850","ts":"1620282288836",
        "balData":[{"ccy":"USDT","eq":"9850"}],
        "posData":[{
            "baseBal":"","ccy":"","instId":"BTC-USD-210430","instType":"FUTURES",
            "mgnMode":"cross","notionalCcy":"","notionalUsd":"","pos":"1",
            "posCcy":"","posSide":"long","quoteBal":""}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let risk = client
        .account()
        .get_position_risk(&PositionRiskRequest::new().inst_type(InstType::Futures))
        .await
        .unwrap();
    assert_eq!(risk[0].adj_eq.as_str(), "9850");
    assert_eq!(risk[0].pos_data[0].inst_id, "BTC-USD-210430");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=FUTURES"));
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_account_config_signs_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "uid":"12345","acctLv":"2","posMode":"net_mode","autoLoan":false,"greeksType":"PA",
        "level":"Lv1","levelTmp":"","ctIsoMode":"automatic","mgnIsoMode":"automatic",
        "spotOffsetType":"","roleType":"0","traderInsts":[],"spotRoleType":"0",
        "spotTraderInsts":[],"opAuth":"0","ip":"","perm":"read_only,trade","mainUid":"",
        "kycLv":"1","label":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let config = client.account().get_account_config().await.unwrap();
    assert_eq!(config[0].pos_mode, "net_mode");
    assert_eq!(config[0].acct_lv, "2");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/account/config"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}

#[tokio::test]
async fn get_risk_state_signs_and_parses() {
    let body = r#"{"code":"0","data":[{"debt":"0.85893159114900247077000000000000","interest":"0.00000000000000000000000000000000","loanAlloc":"","nextDiscountTime":"1729490400000","nextInterestTime":"1729490400000","records":[{"availLoan":"","avgRate":"","ccy":"BTC","interest":"0","loanQuota":"175.00000000","posLoan":"","rate":"0.0000276","surplusLmt":"175.00000000","surplusLmtDetails":{},"usedLmt":"0.00000000","usedLoan":"","interestFreeLiab":"","potentialBorrowingAmt":""}]}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let result = client.account().get_risk_state().await.unwrap();
    assert_eq!(result[0].records[0].ccy, "BTC");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/account/risk-state"));
    assert_eq!(req.query(), None);
    assert!(req.is_signed());
}
