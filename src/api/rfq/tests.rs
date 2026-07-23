use http::Method;

use super::{
    CreateQuoteRequest, CreateRfqRequest, MakerInstrumentRequest, MakerInstrumentSettingsRequest,
    PublicRfqTradesRequest, QuoteLegRequest, RfqLegRequest, RfqMmpConfigRequest, RfqsRequest,
};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn counterparties_match_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"traderName":"Trader One","traderCode":"Trader1","type":"LP"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client.rfq().get_counterparties().await.unwrap();

    assert_eq!(rows[0].trader_code, "Trader1");
    assert_eq!(rows[0].r#type, "LP");
    assert_eq!(mock.captured().method, Method::GET);
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn create_rfq_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"cTime":"1611033737572","uTime":"1611033737572","traderCode":"SATOSHI","tag":"123456","rfqId":"22534","clRfqId":"rfq01","allowPartialExecution":false,"state":"active","validUntil":"1611033857557","counterparties":["Trader1","Trader2"],"legs":[{"instId":"BTC-USD-221208-100000-C","tdMode":"cross","ccy":"USDT","sz":"25","side":"buy","posSide":"long","tgtCcy":""}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CreateRfqRequest::new(
        vec!["Trader1".into(), "Trader2".into()],
        vec![
            RfqLegRequest::new("BTC-USD-221208-100000-C", "25", "buy")
                .trade_mode("cross")
                .currency("USDT")
                .position_side("long"),
        ],
    )
    .anonymous(true)
    .allow_partial_execution(false)
    .client_rfq_id("rfq01")
    .tag("123456");

    let rows = client.rfq().create_rfq(&request).await.unwrap();

    assert_eq!(rows[0].rfq_id, "22534");
    assert_eq!(rows[0].legs[0].sz.as_str(), "25");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"counterparties":["Trader1","Trader2"],"legs":[{"instId":"BTC-USD-221208-100000-C","sz":"25","side":"buy","tdMode":"cross","ccy":"USDT","posSide":"long"}],"anonymous":true,"clRfqId":"rfq01","tag":"123456","allowPartialExecution":false}"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn create_quote_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"validUntil":"1608997227834","uTime":"1608267227834","cTime":"1608267227834","legs":[{"px":"46000","sz":"25","instId":"BTC-USD-220114-25000-C","tdMode":"cross","ccy":"USDT","side":"sell","posSide":"long","tgtCcy":""}],"quoteId":"25092","rfqId":"18753","tag":"123456","quoteSide":"sell","state":"active","reason":"mmp_canceled","clQuoteId":"","clRfqId":"","traderCode":"Aksha"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = CreateQuoteRequest::new(
        "22539",
        "buy",
        vec![
            QuoteLegRequest::new("BTC-USDT-SWAP", "200000", "39450.0", "buy")
                .trade_mode("cross")
                .currency("USDT")
                .position_side("long"),
        ],
    )
    .client_quote_id("q001")
    .tag("123456")
    .anonymous(true)
    .expires_in("30");

    let rows = client.rfq().create_quote(&request).await.unwrap();

    assert_eq!(rows[0].quote_id, "25092");
    assert_eq!(rows[0].legs[0].px.as_str(), "46000");
    assert_eq!(mock.captured().method, Method::POST);
    assert!(mock.captured().body_str().contains(r#""clQuoteId":"q001""#));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn maker_settings_use_documented_array_body() {
    let body = r#"{"code":"0","msg":"","data":[{"result":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let settings = [MakerInstrumentSettingsRequest::new(
        "OPTION",
        vec![
            MakerInstrumentRequest::by_family("BTC-USD")
                .max_block_size("10000")
                .maker_price_band("5"),
        ],
    )];

    let rows = client
        .rfq()
        .set_maker_instrument_settings(&settings)
        .await
        .unwrap();

    assert!(rows[0].result);
    assert_eq!(
        mock.captured().body_str(),
        r#"[{"instType":"OPTION","data":[{"instFamily":"BTC-USD","maxBlockSz":"10000","makerPxBand":"5"}]}]"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn mmp_config_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"frozenInterval":"2000","countLimit":"100","timeInterval":"5000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = RfqMmpConfigRequest::new("5000", "2000", "100");

    let rows = client.rfq().set_mmp_config(&request).await.unwrap();

    assert_eq!(rows[0].time_interval.as_str(), "5000");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"timeInterval":"5000","frozenInterval":"2000","countLimit":"100"}"#
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn private_and_public_queries_have_correct_auth() {
    let private_body = r#"{"code":"0","msg":"","data":[{"rfqId":"22534","clRfqId":"rfq01","state":"active","legs":[]}]}"#;
    let mock = MockTransport::new(private_body);
    let client = signed_client(mock.clone());
    let rows = client
        .rfq()
        .get_rfqs(&RfqsRequest::new().state("active").limit("10"))
        .await
        .unwrap();
    assert_eq!(rows[0].rfq_id, "22534");
    assert_eq!(mock.captured().query(), Some("state=active&limit=10"));
    assert!(mock.captured().is_signed());

    let public_body = r#"{"code":"0","msg":"","data":[{"blockTdId":"439161457415012352","groupId":"","legs":[{"instId":"BTC-USD-210826","side":"sell","sz":"100","px":"11000","tradeId":"439161457415012354"}],"strategy":"CALL_CALENDAR_SPREAD","cTime":"1650976251241"}]}"#;
    let mock = MockTransport::new(public_body);
    let client = OkxClient::with_transport(mock.clone()).build();
    let rows = client
        .rfq()
        .get_public_trades(&PublicRfqTradesRequest::new().limit("100"))
        .await
        .unwrap();
    assert_eq!(rows[0].legs[0].px.as_str(), "11000");
    assert_eq!(mock.captured().query(), Some("limit=100"));
    assert!(!mock.captured().is_signed());
}
