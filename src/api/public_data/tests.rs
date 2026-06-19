use crate::model::InstType;
use crate::test_util::MockTransport;
use crate::{Error, OkxClient};

#[tokio::test]
async fn get_instruments_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[{
        "instType":"SPOT","instId":"BTC-USDT","uly":"","instFamily":"",
        "baseCcy":"BTC","quoteCcy":"USDT","settleCcy":"","ctVal":"","ctMult":"",
        "ctValCcy":"","optType":"","stk":"","listTime":"1606468572000","expTime":"",
        "lever":"10","tickSz":"0.1","lotSz":"0.00000001","minSz":"0.00001",
        "ctType":"","state":"live","maxLmtSz":"9999999999","maxMktSz":"1000000",
        "maxLmtAmt":"20000000","maxMktAmt":"1000000","ruleType":"normal","riskLimitType":""}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let instruments = client
        .public_data()
        .get_instruments(InstType::Spot, None)
        .await
        .unwrap();

    assert_eq!(instruments.len(), 1);
    assert_eq!(instruments[0].inst_id, "BTC-USDT");
    assert_eq!(instruments[0].base_ccy, "BTC");
    assert_eq!(instruments[0].tick_sz.as_str(), "0.1");

    let req = mock.captured();
    assert_eq!(req.method, http::Method::GET);
    assert!(
        req.uri
            .ends_with("/api/v5/public/instruments?instType=SPOT")
    );
    assert!(!req.is_signed(), "public endpoint must not be signed");
}

#[tokio::test]
async fn get_system_time_parses_time() {
    let body = r#"{"code":"0","msg":"","data":[{"ts":"1597026383085","sysTime":""}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let time = client.public_data().get_system_time().await.unwrap();
    assert_eq!(time[0].ts.as_str(), "1597026383085");

    let req = mock.captured();
    assert!(req.uri.ends_with("/api/v5/public/time"));
    assert_eq!(req.query(), None);
    assert!(!req.is_signed());
}

#[tokio::test]
async fn get_open_interest_uses_family_request() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","oi":"10","oiCcy":"1","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InstrumentFamilyRequest::new(InstType::Swap).inst_id("BTC-USDT-SWAP");

    let rows = client
        .public_data()
        .get_open_interest(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].oi.as_str(), "10");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
}

#[tokio::test]
async fn get_funding_rate_queries_instrument() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","fundingRate":"0.0001","nextFundingRate":"0.0002",
             "fundingTime":"1597026383085","nextFundingTime":"1597030000000"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_funding_rate("BTC-USDT-SWAP")
        .await
        .unwrap();
    assert_eq!(rows[0].funding_rate.as_str(), "0.0001");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP"));
}

#[tokio::test]
async fn get_funding_rate_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","fundingRate":"0.0001","realizedRate":"0.0001",
             "fundingTime":"1597026383085","method":"current_period"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::FundingRateHistoryRequest::new("BTC-USDT-SWAP")
        .before("10")
        .limit(1);

    let rows = client
        .public_data()
        .get_funding_rate_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].method, "current_period");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP&before=10&limit=1"));
    assert!(!req.query().unwrap().contains("after"));
}

#[tokio::test]
async fn get_price_limit_queries_instrument() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","buyLmt":"45000","sellLmt":"39000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_price_limit("BTC-USDT-SWAP")
        .await
        .unwrap();
    assert_eq!(rows[0].buy_lmt.as_str(), "45000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP"));
}

#[tokio::test]
async fn get_mark_price_uses_family_request() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","markPx":"42000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InstrumentFamilyRequest::new(InstType::Swap).inst_family("BTC-USDT");

    let rows = client.public_data().get_mark_price(&request).await.unwrap();
    assert_eq!(rows[0].mark_px.as_str(), "42000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
}

#[tokio::test]
async fn get_delivery_exercise_history_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"FUTURES","instId":"BTC-USD-240628","px":"42000","type":"delivery","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::DeliveryExerciseHistoryRequest::new(InstType::Futures)
        .underlying("BTC-USD")
        .limit(1);

    let rows = client
        .public_data()
        .get_delivery_exercise_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].exercise_type, "delivery");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=FUTURES&uly=BTC-USD&limit=1"));
}

#[tokio::test]
async fn get_position_tiers_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","tdMode":"cross","instId":"BTC-USDT-SWAP","tier":"1",
             "minSz":"0","maxSz":"100","imr":"0.1","mmr":"0.05"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::PositionTiersRequest::new(InstType::Swap, "cross").tier("1");

    let rows = client
        .public_data()
        .get_position_tiers(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].tier, "1");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP&tdMode=cross&tier=1"));
}

#[tokio::test]
async fn get_underlying_queries_inst_type() {
    let body = r#"{"code":"0","msg":"","data":[["BTC-USD"]]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_underlying(&super::UnderlyingRequest::new("SWAP"))
        .await
        .unwrap();
    assert_eq!(rows[0], vec!["BTC-USD"]);

    let req = mock.captured();
    assert_eq!(req.query(), Some("instType=SWAP"));
}

#[tokio::test]
async fn get_insurance_fund_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","type":"regular_update","ccy":"USDT","amt":"100","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InsuranceFundRequest::new(InstType::Swap)
        .fund_type("regular_update")
        .currency("USDT");

    let rows = client
        .public_data()
        .get_insurance_fund(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].amt.as_str(), "100");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("instType=SWAP&type=regular_update&ccy=USDT")
    );
}

#[tokio::test]
async fn get_convert_contract_coin_uses_builder_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","sz":"1","px":"42000","unit":"coin"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::ConvertContractCoinRequest::new("1", "BTC-USDT-SWAP", "1")
        .price("42000")
        .unit("coin");

    let rows = client
        .public_data()
        .get_convert_contract_coin(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].unit, "coin");

    let req = mock.captured();
    assert_eq!(
        req.query(),
        Some("type=1&instId=BTC-USDT-SWAP&sz=1&px=42000&unit=coin")
    );
}

#[tokio::test]
async fn get_option_summary_uses_typed_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"OPTION","instFamily":"BTC-USD","instId":"BTC-USD-240628-50000-C","delta":"0.5","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::OptionSummaryRequest::new("BTC-USD");

    let rows = client
        .public_data()
        .get_option_summary(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].inst_family, "BTC-USD");
    assert_eq!(mock.captured().query(), Some("instFamily=BTC-USD"));
}

#[tokio::test]
async fn get_estimated_price_queries_inst_id() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"FUTURES","instId":"BTC-USD-240628","settlePx":"42000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_estimated_price("BTC-USD-240628")
        .await
        .unwrap();
    assert_eq!(rows[0].settle_px.as_str(), "42000");
    assert_eq!(mock.captured().query(), Some("instId=BTC-USD-240628"));
}

#[tokio::test]
async fn get_discount_rate_interest_free_quota_omits_currency() {
    let body = r#"{"code":"0","msg":"","data":[{
        "ccy":"BTC","amt":"0",
        "discountLv":[{"discountRate":"0.9","maxAmt":"20000","minAmt":"0"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_discount_rate_interest_free_quota(None)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "BTC");
    assert_eq!(rows[0].amt.as_str(), "0");

    let req = mock.captured();
    assert_eq!(req.query(), None);
}

#[tokio::test]
async fn public_edge_requests_use_typed_queries() {
    let tick_body = r#"{"code":"0","msg":"","data":[
        {"instType":"OPTION","instFamily":"BTC-USD","tickBand":[{"minPx":"0","maxPx":"10","tickSz":"0.1"}],"ts":"1"}]}"#;
    let mock = MockTransport::new(tick_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InstrumentTickBandsRequest::new("OPTION");
    let rows = client
        .public_data()
        .get_instrument_tick_bands(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].inst_family, "BTC-USD");
    assert_eq!(mock.captured().query(), Some("instType=OPTION"));

    let trade_body = r#"{"code":"0","msg":"","data":[
        {"instId":"BTC-USD-240628-50000-C","tradeId":"1","px":"10","sz":"1","side":"buy","ts":"1"}]}"#;
    let mock = MockTransport::new(trade_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::PublicOptionTradesRequest::new().inst_family("BTC-USD");
    client
        .public_data()
        .get_option_trades(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("instFamily=BTC-USD"));

    let history_body = r#"{"code":"0","msg":"","data":[
        {"module":"1","instType":"SPOT","instId":"BTC-USDT","dateAggrType":"1D","value":"1","ts":"1"}]}"#;
    let mock = MockTransport::new(history_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::MarketDataHistoryRequest::new()
        .module("1")
        .inst_type("SPOT")
        .inst_id_list("BTC-USDT")
        .date_aggregation("1D");
    client
        .public_data()
        .get_market_data_history(&request)
        .await
        .unwrap();
    assert_eq!(
        mock.captured().query(),
        Some("module=1&instType=SPOT&instIdList=BTC-USDT&dateAggrType=1D")
    );
}

#[tokio::test]
async fn invalid_public_request_fails_before_transport() {
    let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InstrumentTickBandsRequest::new("SPOT");

    let error = client
        .public_data()
        .get_instrument_tick_bands(&request)
        .await
        .unwrap_err();
    assert!(matches!(error, Error::InvalidRequest(_)));
    assert!(!mock.was_called());
}

#[test]
fn public_quota_array_fields_accept_empty_string_or_null() {
    let discount: super::DiscountRateInterestFreeQuota =
        serde_json::from_str(r#"{"ccy":"USDT","amt":"0","discountLv":""}"#)
            .expect("discountLv should accept the empty-string wire representation");
    assert!(discount.discount_lv.is_empty());

    let quota: super::InterestRateLoanQuota = serde_json::from_str(
        r#"{
            "basic":"",
            "vip":null,
            "regular":[],
            "configCcyList":"",
            "config":[]
        }"#,
    )
    .expect("quota arrays should accept OKX empty wire representations");
    assert!(quota.basic.is_empty());
    assert!(quota.vip.is_empty());
    assert!(quota.regular.is_empty());
    assert!(quota.config_ccy_list.is_empty());
    assert!(quota.config.is_empty());

    let tick_band: super::InstrumentTickBand = serde_json::from_str(
        r#"{"instType":"OPTION","instFamily":"BTC-USD","tickBand":"","ts":"1"}"#,
    )
    .expect("tickBand should accept the empty-string wire representation");
    assert!(tick_band.tick_band.is_empty());
}

#[test]
fn array_or_empty_string_rejects_non_empty_strings() {
    let error = serde_json::from_str::<super::DiscountRateInterestFreeQuota>(
        r#"{"ccy":"USDT","amt":"0","discountLv":"not-an-array"}"#,
    )
    .expect_err("a non-empty string must not silently decode as an empty array");

    assert!(
        error
            .to_string()
            .contains("expected an array or empty string")
    );
}
