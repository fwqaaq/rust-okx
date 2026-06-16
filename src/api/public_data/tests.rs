use crate::OkxClient;
use crate::model::InstType;
use crate::test_util::MockTransport;

#[tokio::test]
async fn get_instruments_builds_request_and_parses() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","uly":"","instFamily":"",
             "baseCcy":"BTC","quoteCcy":"USDT","settleCcy":"","lotSz":"0.00000001",
             "tickSz":"0.1","minSz":"0.00001","state":"live"}]}"#;
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
    let body = r#"{"code":"0","msg":"","data":[{"ts":"1597026383085"}]}"#;
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
    let body = r#"{"code":"0","msg":"","data":["BTC-USD"]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_underlying(InstType::Swap)
        .await
        .unwrap();
    assert_eq!(rows[0], "BTC-USD");

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
async fn get_option_summary_uses_flexible_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"OPTION","instFamily":"BTC-USD","instId":"BTC-USD-240628-50000-C","delta":"0.5","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::OptionSummaryRequest::new().param("instFamily", "BTC-USD");

    let rows = client
        .public_data()
        .get_option_summary(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].inst_family, "BTC-USD");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instFamily=BTC-USD"));
}

#[tokio::test]
async fn get_estimated_price_queries_inst_id() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USD-240628","px":"42000","ts":"1597026383085"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_estimated_price("BTC-USD-240628")
        .await
        .unwrap();
    assert_eq!(rows[0].px.as_str(), "42000");

    let req = mock.captured();
    assert_eq!(req.query(), Some("instId=BTC-USD-240628"));
}

#[tokio::test]
async fn get_discount_rate_interest_free_quota_omits_currency() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","amt":"100"}]}"#;
    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();

    let rows = client
        .public_data()
        .get_discount_rate_interest_free_quota(None)
        .await
        .unwrap();
    assert_eq!(rows[0].ccy, "USDT");

    let req = mock.captured();
    assert_eq!(req.query(), None);
}

#[tokio::test]
async fn public_edge_requests_use_flexible_query() {
    let body = r#"{"code":"0","msg":"","data":[
            {"instType":"OPTION","instFamily":"BTC-USD","instId":"BTC-USD-240628-50000-C","px":"10","ts":"1597026383085"}]}"#;

    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InstrumentTickBandsRequest::new().param("instFamily", "BTC-USD");
    let rows = client
        .public_data()
        .get_instrument_tick_bands(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].inst_id, "BTC-USD-240628-50000-C");
    assert_eq!(mock.captured().query(), Some("instFamily=BTC-USD"));

    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::PublicOptionTradesRequest::new().param("instFamily", "BTC-USD");
    client
        .public_data()
        .get_option_trades(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("instFamily=BTC-USD"));

    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::MarketDataHistoryRequest::new()
        .param("instId", "BTC-USDT")
        .param("period", "5m");
    client
        .public_data()
        .get_market_data_history(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("instId=BTC-USDT&period=5m"));
}

#[tokio::test]
async fn loan_quota_requests_are_public_and_flexible() {
    let body =
        r#"{"code":"0","msg":"","data":[{"ccy":"USDT","rate":"0.01","ts":"1597026383085"}]}"#;

    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::InterestRateLoanQuotaRequest::new().param("ccy", "USDT");
    let rows = client
        .public_data()
        .get_interest_rate_loan_quota(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].rate.as_str(), "0.01");
    assert!(!mock.captured().is_signed());

    let mock = MockTransport::new(body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let request = super::VipInterestRateLoanQuotaRequest::new().param("ccy", "USDT");
    client
        .public_data()
        .get_vip_interest_rate_loan_quota(&request)
        .await
        .unwrap();
    assert_eq!(mock.captured().query(), Some("ccy=USDT"));
    assert!(!mock.captured().is_signed());
}
