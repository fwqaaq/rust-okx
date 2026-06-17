#[test]
#[ignore = "public endpoints still need stable response assertions"]
fn remaining_market_endpoints_todo() {
    // API: GET /api/v5/market/tickers
    // API: GET /api/v5/market/index-tickers
    // API: GET /api/v5/market/history-candles
    // API: GET /api/v5/market/index-candles
    // API: GET /api/v5/market/mark-price-candles
    // API: GET /api/v5/market/history-trades
    // API: GET /api/v5/market/platform-24-volume
    // API: GET /api/v5/market/index-components
    // API: GET /api/v5/market/exchange-rate
    // API: GET /api/v5/market/block-ticker
    // API: GET /api/v5/market/block-trades
    // STATUS: TODO — add stable instrument/index fixtures and field-level assertions.
    todo!("promote each endpoint to a live test after defining a stable fixture");
}
