use http::Method;

use super::{
    CopySettingsRequest, InstTypeRequest, LeadPositionsRequest, LeadTraderRanksRequest,
    SetLeadInstrumentsRequest,
};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn current_subpositions_decode_official_precision() {
    let body = r#"{"code":"0","data":[{"algoId":"","ccy":"USDT","instId":"BTC-USDT-SWAP","instType":"SWAP","lever":"3","margin":"12.6417","markPx":"38205.8","mgnMode":"isolated","openAvgPx":"37925.1","openOrdId":"","openTime":"1701231120479","posSide":"net","slOrdPx":"","slTriggerPx":"","subPos":"1","subPosId":"649945658862370816","tpOrdPx":"","tpTriggerPx":"","uniqueCode":"25CD5A80241D6FE6","upl":"0.2807","uplRatio":"0.0222042921442527","availSubPos":"1"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = LeadPositionsRequest {
        inst_id: Some("BTC-USDT-SWAP".into()),
        ..Default::default()
    };

    let rows = client
        .copy_trading()
        .get_current_subpositions(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].upl_ratio.as_str(), "0.0222042921442527");
    assert_eq!(rows[0].avail_sub_pos.as_str(), "1");
    assert_eq!(mock.captured().query(), Some("instId=BTC-USDT-SWAP"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn set_lead_instruments_uses_comma_separated_official_field() {
    let body = r#"{"code":"0","data":[{"enabled":true,"instId":"BTC-USDT-SWAP"},{"enabled":true,"instId":"ETH-USDT-SWAP"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = SetLeadInstrumentsRequest {
        inst_type: None,
        inst_id: "BTC-USDT-SWAP,ETH-USDT-SWAP".into(),
    };

    let rows = client
        .copy_trading()
        .set_instruments(&request)
        .await
        .unwrap();

    assert!(rows.iter().all(|row| row.enabled));
    assert_eq!(mock.captured().method, Method::POST);
    assert_eq!(
        mock.captured().body_str(),
        r#"{"instId":"BTC-USDT-SWAP,ETH-USDT-SWAP"}"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn first_copy_settings_matches_official_body() {
    let body = r#"{"code":"0","data":[{"result":true}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CopySettingsRequest {
        inst_type: Some("SWAP".into()),
        unique_code: "25CD5A80241D6FE6".into(),
        copy_mgn_mode: "cross".into(),
        copy_inst_id_type: "copy".into(),
        copy_mode: Some("ratio_copy".into()),
        copy_total_amt: "500".into(),
        copy_ratio: Some("1".into()),
        sub_pos_close_type: "copy_close".into(),
        ..Default::default()
    };

    let rows = client
        .copy_trading()
        .set_first_copy_settings(&request)
        .await
        .unwrap();

    assert!(rows[0].result);
    assert_eq!(
        mock.captured().body_str(),
        r#"{"instType":"SWAP","uniqueCode":"25CD5A80241D6FE6","copyMgnMode":"cross","copyInstIdType":"copy","copyMode":"ratio_copy","copyTotalAmt":"500","copyRatio":"1","subPosCloseType":"copy_close"}"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn public_lead_trader_ranks_are_unsigned_and_typed() {
    let body = r#"{"code":"0","data":[{"dataVer":"20231129213200","ranks":[{"accCopyTraderNum":"3536","aum":"1509265.3238761567721365","ccy":"USDT","copyState":"0","copyTraderNum":"999","leadDays":"156","maxCopyTraderNum":"1000","nickName":"Crypto to the moon","pnl":"48805.1105999999972258","pnlRatio":"1.6898","pnlRatios":[{"beginTs":"1701187200000","pnlRatio":"1.6744"}],"portLink":"","traderInsts":["ICP-USDT-SWAP"],"uniqueCode":"540D011FDACCB47A","winRatio":"0.6957"}],"totalPage":"1"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = LeadTraderRanksRequest {
        inst_type: Some("SWAP".into()),
        limit: Some("10".into()),
        ..Default::default()
    };

    let rows = client
        .copy_trading()
        .get_public_lead_traders(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].ranks[0].aum.as_str(), "1509265.3238761567721365");
    assert_eq!(rows[0].ranks[0].pnl_ratios[0].pnl_ratio.as_str(), "1.6744");
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn public_config_is_unsigned() {
    let body = r#"{"code":"0","data":[{"maxCopyAmt":"1000","maxCopyRatio":"100","maxCopyTotalAmt":"30000","maxSlRatio":"0.75","maxTpRatio":"1.5","minCopyAmt":"20","minCopyRatio":"0.01"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .copy_trading()
        .get_public_config(&InstTypeRequest {
            inst_type: Some("SWAP".into()),
        })
        .await
        .unwrap();

    assert_eq!(rows[0].max_copy_total_amt.as_str(), "30000");
    assert!(!mock.captured().is_signed());
}
