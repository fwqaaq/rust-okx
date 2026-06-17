use crate::common::public_client;
use rust_okx::model::InstType;

#[tokio::test]
async fn public_instruments_and_time_parse() {
    let client = public_client();

    // API: GET /api/v5/public/instruments
    // STATUS: LIVE — public, read-only.
    let instruments = client
        .public_data()
        .get_instruments(InstType::Spot, None)
        .await
        .expect("public/instruments");
    assert!(instruments.iter().any(|row| row.inst_id == "BTC-USDT"));

    // API: GET /api/v5/public/time
    // STATUS: LIVE — public, read-only.
    let rows = client
        .public_data()
        .get_system_time()
        .await
        .expect("public/time");
    assert!(!rows.is_empty());
}

#[tokio::test]
async fn public_derivatives_reference_data_parse() {
    let client = public_client();

    // API: GET /api/v5/public/funding-rate
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_funding_rate("BTC-USDT-SWAP")
        .await
        .expect("public/funding-rate");

    // API: GET /api/v5/public/price-limit
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_price_limit("BTC-USDT-SWAP")
        .await
        .expect("public/price-limit");

    // API: GET /api/v5/public/underlying
    // STATUS: LIVE — public, read-only.
    client
        .public_data()
        .get_underlying(InstType::Option)
        .await
        .expect("public/underlying");
}
