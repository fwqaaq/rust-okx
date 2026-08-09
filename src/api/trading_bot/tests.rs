use http::Method;

use super::{
    GridAiParamRequest, GridInvestmentDataRequest, GridMinInvestmentRequest, GridOrderRequest,
    GridOrdersRequest, GridTriggerRequest, RecurringAlgoIdRequest, RecurringAmendTimeRequest,
    RecurringCurrencyRequest, RecurringOrderRequest, RecurringSubOrdersRequest,
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

    let rows = client
        .trading_bot()
        .grid()
        .place_order(&request)
        .await
        .unwrap();

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

#[tokio::test]
async fn place_recurring_buy_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"algoId":"560472804207104000","algoClOrdId":"","sCode":"0","sMsg":"","tag":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = RecurringOrderRequest {
        stgy_name: "BTC|ETH recurring buy monthly".into(),
        recurring_list: vec![
            RecurringCurrencyRequest {
                ccy: "BTC".into(),
                ratio: "0.2".into(),
                ..Default::default()
            },
            RecurringCurrencyRequest {
                ccy: "ETH".into(),
                ratio: "0.8".into(),
                ..Default::default()
            },
        ],
        period: "monthly".into(),
        recurring_time: "0".into(),
        time_zone: "8".into(),
        amt: "100".into(),
        investment_ccy: "USDT".into(),
        td_mode: "cross".into(),
        recurring_day: Some("1".into()),
        ..Default::default()
    };

    let rows = client
        .trading_bot()
        .recurring_buy()
        .place_order(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].algo_id, "560472804207104000");
    assert_eq!(mock.captured().method, Method::POST);
    assert_eq!(
        mock.captured().body_str(),
        r#"{"stgyName":"BTC|ETH recurring buy monthly","recurringList":[{"ccy":"BTC","ratio":"0.2"},{"ccy":"ETH","ratio":"0.8"}],"period":"monthly","recurringTime":"0","timeZone":"8","amt":"100","investmentCcy":"USDT","tdMode":"cross","recurringDay":"1"}"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn recurring_details_preserve_official_string_precision() {
    let body = r#"{"code":"0","data":[{"algoClOrdId":"","algoId":"644497312047435776","algoOrdType":"recurring","amt":"100","cTime":"1699932133373","cycles":"6","instType":"SPOT","investmentAmt":"0","investmentCcy":"USDC","mktCap":"0","nextInvestTime":"1699956005500","period":"hourly","pnlRatio":"0","recurringDay":"","recurringHour":"1","recurringList":[{"avgPx":"0","ccy":"BTC","profit":"0","px":"36683.2","ratio":"0.2","minPx":"","maxPx":"","totalAmt":"0"}],"recurringTime":"12","state":"running","stgyName":"stg1","tag":"","timeZone":"8","totalAnnRate":"0","totalPnl":"0","uTime":"1699952485451","tradeQuoteCcy":"USDT","source":["1"],"recurringTimeType":"1","recurringTimeMinutes":"0"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = RecurringAlgoIdRequest {
        algo_id: "644497312047435776".into(),
    };

    let rows = client
        .trading_bot()
        .recurring_buy()
        .get_order_details(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].recurring_list[0].px.as_str(), "36683.2");
    assert_eq!(
        mock.captured().query(),
        Some("algoId=644497312047435776")
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn recurring_sub_orders_decode_official_example() {
    let body = r#"{"code":"0","data":[{"accFillSz":"0.045315","algoClOrdId":"","algoId":"560516615079727104","algoOrdType":"recurring","avgPx":"1765.4","cTime":"1679911222200","fee":"-0.0000317205","feeCcy":"ETH","instId":"ETH-USDC","instType":"SPOT","ordId":"560523524230717440","ordType":"market","px":"-1","side":"buy","state":"filled","sz":"80","tag":"","tdMode":"","uTime":"1679911222207"}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = RecurringSubOrdersRequest {
        algo_id: "560516615079727104".into(),
        ..Default::default()
    };

    let rows = client
        .trading_bot()
        .recurring_buy()
        .get_sub_orders(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].fee.as_str(), "-0.0000317205");
    assert_eq!(rows[0].avg_px.as_str(), "1765.4");
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn amend_recurring_time_uses_documented_fields() {
    let body = r#"{"code":"0","msg":"","data":[{"algoId":"2837428373700509696","sCode":"0","sMsg":""}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = RecurringAmendTimeRequest {
        algo_id: "2837428373700509696".into(),
        recurring_time_type: "1".into(),
        time_zone: "8".into(),
        period: "hourly".into(),
        recurring_hour: Some("8".into()),
        recurring_day: Some("1".into()),
        recurring_time: Some("11".into()),
    };

    let rows = client
        .trading_bot()
        .recurring_buy()
        .amend_time(&request)
        .await
        .unwrap();

    assert_eq!(rows[0].s_code, "0");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"algoId":"2837428373700509696","recurringTimeType":"1","timeZone":"8","period":"hourly","recurringHour":"8","recurringDay":"1","recurringTime":"11"}"#
    );
    assert!(mock.captured().is_signed());
}
