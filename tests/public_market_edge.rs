//! Integration tests for public market/public-data edge endpoints.

mod common;

use std::time::{SystemTime, UNIX_EPOCH};

use common::public_client;
use rust_okx::api::public_data::{
    InstrumentTickBandsRequest, MarketDataHistoryRequest, OptionSummaryRequest,
    PublicOptionTradesRequest,
};

#[tokio::test]
async fn public_option_and_quota_edges_parse() {
    let client = public_client();

    let _ = client
        .public_data()
        .get_option_summary(&OptionSummaryRequest::new().param("instFamily", "BTC-USD"))
        .await
        .expect("opt-summary request failed");

    let _ = client
        .public_data()
        .get_discount_rate_interest_free_quota(Some("USDT"))
        .await
        .expect("discount-rate-interest-free-quota request failed");

    let _ = client
        .public_data()
        .get_interest_rate_loan_quota(&Default::default())
        .await
        .expect("interest-rate-loan-quota request failed");

    let _ = client
        .public_data()
        .get_vip_interest_rate_loan_quota(&Default::default())
        .await
        .expect("vip-interest-rate-loan-quota request failed");
}

#[tokio::test]
async fn public_market_edge_endpoints_parse() {
    let client = public_client();

    let rows = client
        .market()
        .get_block_tickers(rust_okx::model::InstType::Swap, None)
        .await
        .expect("block-tickers request failed");
    for row in rows {
        assert!(
            !row.inst_id.is_empty(),
            "block ticker rows should include an instrument ID"
        );
    }

    let _ = client
        .market()
        .get_option_instrument_family_trades("BTC-USD")
        .await
        .expect("option instrument-family trades request failed");

    let _ = client
        .public_data()
        .get_instrument_tick_bands(
            &InstrumentTickBandsRequest::new()
                .param("instType", "OPTION")
                .param("instFamily", "BTC-USD"),
        )
        .await
        .expect("instrument-tick-bands request failed");

    let _ = client
        .public_data()
        .get_option_trades(&PublicOptionTradesRequest::new().param("instFamily", "BTC-USD"))
        .await
        .expect("option-trades request failed");
}

#[tokio::test]
async fn public_history_edges_parse() {
    let client = public_client();

    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_millis() as u64;

    let begin_ms = now_ms - 2 * 24 * 60 * 60 * 1000;
    let end_ms = now_ms - 24 * 60 * 60 * 1000;

    let _ = client
        .public_data()
        .get_market_data_history(
            &MarketDataHistoryRequest::new()
                .param("module", "2")
                .param("instType", "SPOT")
                .param("instIdList", "BTC-USDT")
                .param("dateAggrType", "daily")
                .param("begin", begin_ms.to_string())
                .param("end", end_ms.to_string()),
        )
        .await
        .expect("market-data-history request failed");
}
