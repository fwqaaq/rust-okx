#[test]
#[ignore = "requires funded grid-bot strategies"]
fn grid_bot_endpoints_todo() {
    // API: POST /api/v5/tradingBot/grid/order-algo
    // API: POST /api/v5/tradingBot/grid/amend-order-algo
    // API: POST /api/v5/tradingBot/grid/stop-order-algo
    // API: GET /api/v5/tradingBot/grid/orders-algo-pending
    // API: GET /api/v5/tradingBot/grid/orders-algo-history
    // API: GET /api/v5/tradingBot/grid/orders-algo-details
    // API: GET /api/v5/tradingBot/grid/sub-orders
    // API: GET /api/v5/tradingBot/grid/positions
    // API: POST /api/v5/tradingBot/grid/close-position
    // API: POST /api/v5/tradingBot/grid/cancel-close-order
    // API: POST /api/v5/tradingBot/grid/withdraw-income
    // API: POST /api/v5/tradingBot/grid/compute-margin-balance
    // API: POST /api/v5/tradingBot/grid/margin-balance
    // API: GET /api/v5/tradingBot/grid/ai-param
    // API: POST /api/v5/tradingBot/grid/min-investment
    // API: POST /api/v5/tradingBot/grid/adjust-investment
    // API: POST /api/v5/tradingBot/grid/order-instant-trigger
    // API: GET /api/v5/tradingBot/grid/grid-quantity
    // API: POST /api/v5/tradingBot/grid/amend-algo-basic-param
    // API: POST /api/v5/tradingBot/grid/copy-order-algo
    // API: GET /api/v5/tradingBot/public/rsi-back-testing
    // STATUS: TODO — needs isolated funded bots and deterministic stop/close cleanup.
    todo!("exercise each grid lifecycle with strict investment and leverage caps");
}
