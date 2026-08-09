#[test]
#[ignore = "requires funded signal-bot strategies"]
fn signal_bot_endpoints_todo() {
    // API: POST /api/v5/tradingBot/signal/create-signal
    // API: GET /api/v5/tradingBot/signal/signals
    // API: POST /api/v5/tradingBot/signal/order-algo
    // API: POST /api/v5/tradingBot/signal/stop-order-algo
    // API: GET /api/v5/tradingBot/signal/orders-algo-details
    // API: GET /api/v5/tradingBot/signal/orders-algo-pending
    // API: GET /api/v5/tradingBot/signal/orders-algo-history
    // API: GET /api/v5/tradingBot/signal/sub-orders
    // API: POST /api/v5/tradingBot/signal/cancel-sub-order
    // API: GET /api/v5/tradingBot/signal/event-history
    // API: GET /api/v5/tradingBot/signal/positions
    // API: GET /api/v5/tradingBot/signal/positions-history
    // API: POST /api/v5/tradingBot/signal/amendTPSL
    // API: POST /api/v5/tradingBot/signal/close-position
    // API: POST /api/v5/tradingBot/signal/margin-balance
    // API: POST /api/v5/tradingBot/signal/set-instruments
    // API: POST /api/v5/tradingBot/signal/sub-order
    // STATUS: TODO — needs a funded signal bot and deterministic cleanup.
    todo!("exercise the full signal-bot lifecycle with strict investment and leverage caps");
}
