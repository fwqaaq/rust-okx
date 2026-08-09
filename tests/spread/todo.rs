#[test]
#[ignore = "requires Nitro Spread permissions and live spreads"]
fn spread_endpoints_todo() {
    // API: GET /api/v5/sprd/spreads
    // API: GET /api/v5/sprd/books
    // API: GET /api/v5/sprd/public-trades
    // API: POST /api/v5/sprd/order
    // API: POST /api/v5/sprd/cancel-order
    // API: POST /api/v5/sprd/cancel-all-after
    // API: POST /api/v5/sprd/mass-cancel
    // API: POST /api/v5/sprd/amend-order
    // API: GET /api/v5/sprd/order
    // API: GET /api/v5/sprd/orders-pending
    // API: GET /api/v5/sprd/orders-history
    // API: GET /api/v5/sprd/orders-history-archive
    // API: GET /api/v5/sprd/trades
    // STATUS: TODO — needs live spreads and a dedicated account with strict order caps.
    todo!("exercise the place/amend/cancel lifecycle with isolated Nitro Spread permissions");
}
