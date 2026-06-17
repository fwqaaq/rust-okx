use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::public_client;
use rust_okx::api::public_data::MarketDataHistoryRequest;

#[tokio::test]
async fn public_market_data_history_parses() {
    let client = public_client();
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before Unix epoch")
        .as_millis() as u64;

    // API: GET /api/v5/public/market-data-history
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_market_data_history(
            &MarketDataHistoryRequest::new()
                .module("2")
                .inst_type("SPOT")
                .inst_id_list("BTC-USDT")
                .date_aggregation("daily")
                .begin((now_ms - 2 * 24 * 60 * 60 * 1000).to_string())
                .end((now_ms - 24 * 60 * 60 * 1000).to_string()),
        )
        .await
        .expect("public/market-data-history");
}
