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
