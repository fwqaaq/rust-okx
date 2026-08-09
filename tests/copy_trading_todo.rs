#[test]
#[ignore = "requires lead and copy-trader accounts with deterministic cleanup"]
fn copy_trading_endpoints_todo() {
    // API: GET /api/v5/copytrading/current-subpositions
    // API: GET /api/v5/copytrading/subpositions-history
    // API: POST /api/v5/copytrading/algo-order
    // API: POST /api/v5/copytrading/close-subposition
    // API: GET /api/v5/copytrading/instruments
    // API: POST /api/v5/copytrading/set-instruments
    // API: GET /api/v5/copytrading/profit-sharing-details
    // API: GET /api/v5/copytrading/total-profit-sharing
    // API: GET /api/v5/copytrading/unrealized-profit-sharing-details
    // API: GET /api/v5/copytrading/total-unrealized-profit-sharing
    // API: POST /api/v5/copytrading/amend-profit-sharing-ratio
    // API: GET /api/v5/copytrading/config
    // API: POST /api/v5/copytrading/first-copy-settings
    // API: POST /api/v5/copytrading/amend-copy-settings
    // API: POST /api/v5/copytrading/stop-copy-trading
    // API: GET /api/v5/copytrading/copy-settings
    // API: GET /api/v5/copytrading/current-lead-traders
    // API: GET /api/v5/copytrading/public-config
    // API: GET /api/v5/copytrading/public-lead-traders
    // API: GET /api/v5/copytrading/public-weekly-pnl
    // API: GET /api/v5/copytrading/public-pnl
    // API: GET /api/v5/copytrading/public-stats
    // API: GET /api/v5/copytrading/public-preference-currency
    // API: GET /api/v5/copytrading/public-current-subpositions
    // API: GET /api/v5/copytrading/public-subpositions-history
    // API: GET /api/v5/copytrading/public-copy-traders
    // STATUS: TODO — requires linked lead/copy accounts and deterministic position cleanup.
    todo!("exercise Copy Trading lifecycle endpoints with strict loss caps");
}
