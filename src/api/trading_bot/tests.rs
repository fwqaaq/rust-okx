use http::Method;

use super::{
    GridAiParamRequest, GridInvestmentDataRequest, GridMinInvestmentRequest, GridOrderRequest,
    GridOrdersRequest, GridTriggerRequest,
};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn place_spot_grid_matches_official_example() {
    let body = r#"{"code":"0","data":[{"algoClOrdId":"","algoId":"447053782921515008","sCode":"0","sMsg":"","tag":""}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = GridOrderRequest {
        inst_id: "BTC-USDT".into(),
        algo_ord_type: "grid".into(),
        max_px: "5000".into(),
        min_px: "400".into(),
        grid_num: "10".into(),
        run_type: Some("1".into()),
        quote_sz: Some("25".into()),
        trigger_params: Some(vec![GridTriggerRequest {
            trigger_action: "stop".into(),
            trigger_strategy: "price".into(),
            trigger_px: Some("1000".into()),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let rows = client.trading_bot().grid().place_order(&request).await.unwrap();

    assert_eq!(rows[0].algo_id, "447053782921515008");
    assert_eq!(mock.captured().method, Method::POST);
    assert_eq!(
        mock.captured().body_str(),
        r#"{"instId":"BTC-USDT","algoOrdType":"grid","maxPx":"5000","minPx":"400","gridNum":"10","runType":"1","triggerParams":[{"triggerAction":"stop","triggerStrategy":"price","triggerPx":"1000"}],"quoteSz":"25"}"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn pending_grid_orders_preserve_string_precision() {
    let body = r#"{"code":"0","msg":"","data":[{"algoId":"448965992920907776","algoClOrdId":"","instType":"SPOT","instId":"BTC-USDT","cTime":"1645769208701","uTime":"1645769208701","algoOrdType":"grid","state":"running","maxPx":"50000","minPx":"40000","gridNum":"10","runType":"1","totalPnl":"0.0101","investment":"25","gridProfit":"0.001","floatProfit":"0.0091","quoteSz":"25","baseSz":"","tag":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = GridOrdersRequest {
        algo_ord_type: "grid".into(),
        limit: Some("10".into()),
        ..Default::default()
    };

    let rows = client
        .trading_bot()
        .grid()
        .get_pending_orders(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].total_pnl.as_str(), "0.0101");
    assert_eq!(mock.captured().query(), Some("algoOrdType=grid&limit=10"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn public_ai_parameters_are_unsigned() {
    let body = r#"{"code":"0","msg":"","data":[{"instId":"BTC-USDT","algoOrdType":"grid","duration":"7D","gridNum":"50","maxPx":"50000","minPx":"30000","perMaxProfitRate":"0.01","perMinProfitRate":"0.005","perGridProfitRatio":"0.007","annualizedRate":"0.12","minInvestment":"100","ccy":"USDT","runType":"1","direction":"","lever":"","sourceCcy":"USDT"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = GridAiParamRequest {
        algo_ord_type: "grid".into(),
        inst_id: "BTC-USDT".into(),
        duration: Some("7D".into()),
        ..Default::default()
    };

    let rows = client
        .trading_bot()
        .grid()
        .get_ai_parameters(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].min_investment.as_str(), "100");
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn public_min_investment_uses_documented_nested_body() {
    let body = r#"{"code":"0","msg":"","data":[{"minInvestmentData":[{"amt":"100","ccy":"USDT"}],"singleAmt":"10"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = GridMinInvestmentRequest {
        inst_id: "BTC-USDT".into(),
        algo_ord_type: "grid".into(),
        max_px: "50000".into(),
        min_px: "40000".into(),
        grid_num: "10".into(),
        run_type: "1".into(),
        investment_data: Some(vec![GridInvestmentDataRequest {
            amt: "100".into(),
            ccy: "USDT".into(),
        }]),
        ..Default::default()
    };

    let rows = client
        .trading_bot()
        .grid()
        .compute_min_investment(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].min_investment_data[0].amt.as_str(), "100");
    assert_eq!(mock.captured().method, Method::POST);
    assert!(!mock.captured().is_signed());
}
