#[test]
#[ignore = "requires funded DCA strategies"]
fn dca_bot_endpoints_todo() {
    // API: POST /api/v5/tradingBot/dca/create
    // API: POST /api/v5/tradingBot/dca/amend-order-algo
    // API: POST /api/v5/tradingBot/dca/stop
    // API: POST /api/v5/tradingBot/dca/margin/add
    // API: POST /api/v5/tradingBot/dca/margin/reduce
    // API: POST /api/v5/tradingBot/dca/orders/manual-buy
    // API: POST /api/v5/tradingBot/dca/settings/reinvestment
    // API: POST /api/v5/tradingBot/dca/settings/take-profit
    // API: GET /api/v5/tradingBot/dca/orders
    // API: GET /api/v5/tradingBot/dca/ongoing-list
    // API: GET /api/v5/tradingBot/dca/history-list
    // API: GET /api/v5/tradingBot/dca/cycle-list
    // API: GET /api/v5/tradingBot/dca/position-details
    // STATUS: TODO — needs funded DCA bots and deterministic stop/close cleanup.
    todo!("exercise contract and spot DCA lifecycles with strict investment caps");
}
