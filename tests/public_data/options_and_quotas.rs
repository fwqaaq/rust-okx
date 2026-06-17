use crate::common::public_client;
use rust_okx::api::public_data::{
    InstrumentTickBandsRequest, OptionSummaryRequest, PublicOptionTradesRequest,
};

#[tokio::test]
async fn public_option_endpoints_parse() {
    let client = public_client();

    // API: GET /api/v5/public/opt-summary
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_option_summary(&OptionSummaryRequest::new("BTC-USD"))
        .await
        .expect("public/opt-summary");

    // API: GET /api/v5/public/instrument-tick-bands
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_instrument_tick_bands(&InstrumentTickBandsRequest::new("OPTION"))
        .await
        .expect("public/instrument-tick-bands");

    // API: GET /api/v5/public/option-trades
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_option_trades(&PublicOptionTradesRequest::new().inst_family("BTC-USD"))
        .await
        .expect("public/option-trades");
}

#[tokio::test]
async fn public_loan_quota_endpoints_accept_empty_array_wire_values() {
    let client = public_client();

    // API: GET /api/v5/public/discount-rate-interest-free-quota
    // STATUS: LIVE — public; regression for arrays returned as "".
    client
        .public_data()
        .get_discount_rate_interest_free_quota(Some("USDT"))
        .await
        .expect("public/discount-rate-interest-free-quota");

    // API: GET /api/v5/public/interest-rate-loan-quota
    // STATUS: LIVE — public; regression for arrays returned as "" or null.
    client
        .public_data()
        .get_interest_rate_loan_quota(&Default::default())
        .await
        .expect("public/interest-rate-loan-quota");

    // API: GET /api/v5/public/vip-interest-rate-loan-quota
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_vip_interest_rate_loan_quota(&Default::default())
        .await
        .expect("public/vip-interest-rate-loan-quota");
}
