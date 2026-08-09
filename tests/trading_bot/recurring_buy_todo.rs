#[test]
#[ignore = "requires funded recurring-buy strategies"]
fn recurring_buy_endpoints_todo() {
    // API: POST /api/v5/tradingBot/recurring/order-algo
    // API: POST /api/v5/tradingBot/recurring/amend-order-algo
    // API: POST /api/v5/tradingBot/recurring/stop-order-algo
    // API: GET /api/v5/tradingBot/recurring/orders-algo-pending
    // API: GET /api/v5/tradingBot/recurring/orders-algo-history
    // API: GET /api/v5/tradingBot/recurring/orders-algo-details
    // API: GET /api/v5/tradingBot/recurring/sub-orders
    // API: POST /api/v5/tradingBot/recurring/add-investment
    // API: POST /api/v5/tradingBot/recurring/amend-price-range
    // API: POST /api/v5/tradingBot/recurring/amend-recurring-amount
    // API: POST /api/v5/tradingBot/recurring/amend-recurring-time
    // API: POST /api/v5/tradingBot/recurring/pause
    // API: POST /api/v5/tradingBot/recurring/restart
    // STATUS: TODO — needs a funded recurring-buy strategy and deterministic cleanup.
    todo!("exercise the full recurring-buy lifecycle with a strict investment cap");
}
