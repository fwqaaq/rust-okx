use crate::OkxClient;
use crate::test_util::MockTransport;

use super::{
    ContractTakerVolumeRequest, ContractVolumeUnit, CurrencyHistoryRequest,
    InstrumentHistoryRequest, OptionHistoryRequest, OptionStrikeRequest, TakerVolumeInstrumentType,
    TakerVolumeRequest,
};

#[tokio::test]
async fn support_coins_matches_official_object_response() {
    let body = r#"{"code":"0","data":{"contract":["ADA","BTC"],"option":["BTC"],"spot":["ADA","BTC"]},"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let data = client.trading_data().get_support_coins().await.unwrap();

    assert_eq!(data.contract, ["ADA", "BTC"]);
    assert_eq!(data.option, ["BTC"]);
    assert_eq!(data.spot, ["ADA", "BTC"]);
    let req = mock.captured();
    assert!(
        req.uri
            .ends_with("/api/v5/rubik/stat/trading-data/support-coin")
    );
    assert_eq!(req.query(), None);
    assert!(!req.is_signed());
}

#[tokio::test]
async fn instrument_history_endpoints_match_official_array_responses() {
    let request = InstrumentHistoryRequest::new("BTC-USDT-SWAP")
        .period("5m")
        .end("1701417700000")
        .begin("1701417000000")
        .limit(100);

    let body = r#"{"code":"0","msg":"","data":[["1701417600000","731377.57500501","111","8888888"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .trading_data()
        .get_contract_open_interest_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].oi.as_str(), "731377.57500501");
    assert_eq!(rows[0].oi_ccy.as_str(), "111");
    assert_eq!(rows[0].oi_usd.as_str(), "8888888");
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/rubik/stat/contracts/open-interest-history?")
    );
    assert_eq!(
        mock.captured().query(),
        Some(
            "instId=BTC-USDT-SWAP&period=5m&end=1701417700000&begin=1701417000000&limit=100"
        )
    );

    let ratio_body = r#"{"code":"0","msg":"","data":[["1701417600000","1.1739"]]}"#;
    let mock = MockTransport::new(ratio_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .trading_data()
        .get_top_trader_account_ratio(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ratio.as_str(), "1.1739");
    assert!(
        mock.captured()
            .uri
            .contains("long-short-account-ratio-contract-top-trader")
    );
    client
        .trading_data()
        .get_top_trader_position_ratio(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("long-short-position-ratio-contract-top-trader")
    );
    client
        .trading_data()
        .get_contract_long_short_account_ratio(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("long-short-account-ratio-contract?")
    );
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn taker_volume_endpoints_match_official_responses() {
    let body =
        r#"{"code":"0","data":[["1630425600000","7596.2651","7149.4855"]],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = TakerVolumeRequest::new("BTC", TakerVolumeInstrumentType::Spot)
        .begin("1630330000000")
        .end("1630430000000")
        .period("1D");

    let rows = client
        .trading_data()
        .get_taker_volume(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].sell_vol.as_str(), "7596.2651");
    assert_eq!(rows[0].buy_vol.as_str(), "7149.4855");
    assert_eq!(
        mock.captured().query(),
        Some("ccy=BTC&instType=SPOT&begin=1630330000000&end=1630430000000&period=1D")
    );

    let request = ContractTakerVolumeRequest::new("BTC-USDT-SWAP")
        .period("5m")
        .unit(ContractVolumeUnit::Usd)
        .end("1701417700000")
        .begin("1701417000000")
        .limit(100);
    client
        .trading_data()
        .get_contract_taker_volume(&request)
        .await
        .unwrap();
    assert_eq!(
        mock.captured().query(),
        Some(
            "instId=BTC-USDT-SWAP&period=5m&unit=2&end=1701417700000&begin=1701417000000&limit=100"
        )
    );
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/rubik/stat/taker-volume-contract?")
    );
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn currency_history_endpoints_match_official_responses() {
    let ratio_body = r#"{"code":"0","data":[["1630492800000","0.4614"]],"msg":""}"#;
    let mock = MockTransport::new(ratio_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = CurrencyHistoryRequest::new("BTC")
        .begin("1630400000000")
        .end("1630500000000")
        .period("5m");

    let rows = client
        .trading_data()
        .get_margin_loan_ratio(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].ratio.as_str(), "0.4614");
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/rubik/stat/margin/loan-ratio?")
    );
    client
        .trading_data()
        .get_long_short_account_ratio(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/rubik/stat/contracts/long-short-account-ratio?")
    );

    let volume_body =
        r#"{"code":"0","data":[["1630502400000","1713028741.6898","39800873.554"]],"msg":""}"#;
    let mock = MockTransport::new(volume_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .trading_data()
        .get_contract_open_interest_volume(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].oi.as_str(), "1713028741.6898");
    assert_eq!(rows[0].vol.as_str(), "39800873.554");
    assert!(!mock.captured().is_signed());
}

#[tokio::test]
async fn option_endpoints_match_official_responses() {
    let request = OptionHistoryRequest::new("BTC").period("1D");
    let volume_body =
        r#"{"code":"0","data":[["1630368000000","3458.1000","78.8000"]],"msg":""}"#;
    let mock = MockTransport::new(volume_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .trading_data()
        .get_option_open_interest_volume(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].oi.as_str(), "3458.1000");
    assert_eq!(mock.captured().query(), Some("ccy=BTC&period=1D"));

    let ratio_body =
        r#"{"code":"0","data":[["1630512000000","2.7261","2.3447"]],"msg":""}"#;
    let mock = MockTransport::new(ratio_body);
    let client = OkxClient::with_transport(mock).build();
    let rows = client
        .trading_data()
        .get_option_put_call_ratio(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].oi_ratio.as_str(), "2.7261");
    assert_eq!(rows[0].vol_ratio.as_str(), "2.3447");

    let expiry_body =
        r#"{"code":"0","data":[["1630540800000","20210902","6.4","18.4","0.7","0.4"]],"msg":""}"#;
    let mock = MockTransport::new(expiry_body);
    let client = OkxClient::with_transport(mock).build();
    let rows = client
        .trading_data()
        .get_option_open_interest_volume_expiry(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].exp_time, "20210902");
    assert_eq!(rows[0].put_oi.as_str(), "18.4");

    let strike_body =
        r#"{"code":"0","data":[["1630540800000","10000","0","0.5","0","0"]],"msg":""}"#;
    let mock = MockTransport::new(strike_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let strike_request = OptionStrikeRequest::new("BTC", "20210901").period("1D");
    let rows = client
        .trading_data()
        .get_option_open_interest_volume_strike(&strike_request)
        .await
        .unwrap();
    assert_eq!(rows[0].strike.as_str(), "10000");
    assert_eq!(
        mock.captured().query(),
        Some("ccy=BTC&expTime=20210901&period=1D")
    );

    let flow_body =
        r#"{"code":"0","data":["1630512000000","8.55","67.3","16.05","16.3","126.4","40.7"],"msg":""}"#;
    let mock = MockTransport::new(flow_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let flow = client
        .trading_data()
        .get_option_taker_flow(&request)
        .await
        .unwrap();
    assert_eq!(flow.call_buy_vol.as_str(), "8.55");
    assert_eq!(flow.put_block_vol.as_str(), "40.7");
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/rubik/stat/option/taker-block-volume?")
    );
    assert!(!mock.captured().is_signed());
}
