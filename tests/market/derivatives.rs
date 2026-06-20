use crate::common::public_client;
use rust_okx::api::market::{InstFamilyRequest, TickersRequest};
use rust_okx::model::InstType;

#[tokio::test]
async fn market_block_and_option_endpoints_parse() {
    let client = public_client();

    // API: GET /api/v5/market/block-tickers
    // STATUS: LIVE — public, read-only.
    let rows = client
        .market()
        .get_block_tickers(&TickersRequest {
            inst_type: &InstType::Swap,
            inst_family: None,
        })
        .await
        .expect("market/block-tickers");
    assert!(rows.iter().all(|row| !row.inst_id.is_empty()));

    // API: GET /api/v5/market/option/instrument-family-trades
    // STATUS: LIVE — public, read-only.
    client
        .market()
        .get_option_instrument_family_trades(&InstFamilyRequest {
            inst_family: "BTC-USD",
        })
        .await
        .expect("market/option/instrument-family-trades");
}
