#[test]
#[ignore = "places, amends, cancels, or closes real orders"]
fn standard_trade_mutations_todo() {
    // API: POST /api/v5/trade/batch-orders
    // API: POST /api/v5/trade/cancel-batch-orders
    // API: POST /api/v5/trade/amend-order
    // API: POST /api/v5/trade/amend-batch-orders
    // API: POST /api/v5/trade/close-position
    // STATUS: TODO — use simulated trading and deterministic order cleanup.
    todo!("extend the demo lifecycle with batch/amend/close scenarios");
}

#[test]
#[ignore = "requires a current real order identifier"]
fn standard_trade_identifier_reads_todo() {
    // API: GET /api/v5/trade/orders-history-archive
    // STATUS: TODO — archive access and a stable historical fixture are required.
    todo!("configure an archive-capable account and a deterministic time range");
}

#[test]
#[ignore = "places or modifies algo orders and may convert or repay real assets"]
fn advanced_trade_mutations_todo() {
    // API: POST /api/v5/trade/order-algo
    // API: POST /api/v5/trade/cancel-algos
    // API: POST /api/v5/trade/amend-algos
    // API: POST /api/v5/trade/easy-convert
    // API: POST /api/v5/trade/one-click-repay
    // API: POST /api/v5/trade/one-click-repay-v2
    // API: POST /api/v5/trade/cancel-all-after
    // API: POST /api/v5/trade/mass-cancel
    // STATUS: TODO — unsafe for the default live suite.
    todo!("use demo support where available and strict real-asset caps otherwise");
}

#[test]
#[ignore = "requires existing algo/repay identifiers or v2 eligibility"]
fn advanced_trade_uncovered_reads_todo() {
    // API: GET /api/v5/trade/orders-algo-history
    // API: GET /api/v5/trade/order-algo
    // API: GET /api/v5/trade/one-click-repay-currency-list-v2
    // API: GET /api/v5/trade/one-click-repay-history-v2
    // STATUS: TODO — needs current IDs or an account eligible for the v2 product.
    todo!("create deterministic fixtures before enabling these reads");
}
