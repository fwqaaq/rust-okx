#[test]
#[ignore = "requires dynamic expiries, product families, or stable historical fixtures"]
fn remaining_public_data_endpoints_todo() {
    // API: GET /api/v5/public/open-interest
    // API: GET /api/v5/public/funding-rate-history
    // API: GET /api/v5/public/mark-price
    // API: GET /api/v5/public/delivery-exercise-history
    // API: GET /api/v5/public/position-tiers
    // API: GET /api/v5/public/insurance-fund
    // API: GET /api/v5/public/convert-contract-coin
    // API: GET /api/v5/public/estimated-price
    // STATUS: TODO — select current instruments and define stable field assertions.
    todo!("derive fixtures from /public/instruments before invoking these endpoints");
}

#[test]
#[ignore = "requires current event-contract, swap, futures, and block-trade fixtures"]
fn public_data_completion_live_fixtures_todo() {
    // API: GET /api/v5/public/premium-history
    // API: GET /api/v5/public/event-contract/series
    // API: GET /api/v5/public/event-contract/events
    // API: GET /api/v5/public/event-contract/markets
    // API: GET /api/v5/public/block-trades
    // API: GET /api/v5/public/estimated-settlement-info
    // API: GET /api/v5/public/settlement-history
    // STATUS: TODO — select live instruments and event series before invoking.
    todo!("derive current fixtures from public instruments and event-contract series");
}
