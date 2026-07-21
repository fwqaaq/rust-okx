//! Unsafe or account-specific Account API cases.

#[test]
#[ignore = "changes account configuration or product enrollment"]
fn account_configuration_mutations_todo() {
    // API: POST /api/v5/account/set-position-mode
    // API: POST /api/v5/account/set-greeks
    // API: POST /api/v5/account/set-isolated-mode
    // API: POST /api/v5/account/set-settle-currency
    // API: POST /api/v5/account/set-auto-loan
    // API: POST /api/v5/account/set-account-level
    // API: POST /api/v5/account/set-collateral-assets
    // API: POST /api/v5/account/activate-option
    // API: POST /api/v5/account/set-auto-repay
    // API: POST /api/v5/account/set-auto-earn
    // STATUS: TODO — these calls persistently change account configuration.
    todo!("run only against a dedicated account with explicit expected-state assertions");
}

#[test]
#[ignore = "may borrow, repay, reserve, or move real collateral"]
fn account_borrowing_and_margin_mutations_todo() {
    // API: POST /api/v5/account/position/margin-balance
    // API: POST /api/v5/account/move-positions
    // API: POST /api/v5/account/spot-manual-borrow-repay
    // STATUS: TODO — requires eligible products and real balances.
    todo!("configure eligible currencies, order IDs, and a reversible safety plan");
}

#[test]
#[ignore = "requires complex portfolio inputs or special account state"]
fn account_calculation_and_uncovered_reads_todo() {
    // API: GET /api/v5/account/bills-archive
    // API: POST /api/v5/account/bills-history-archive
    // API: GET /api/v5/account/bills-history-archive
    // API: GET /api/v5/account/max-size
    // API: GET /api/v5/account/max-loan
    // API: GET /api/v5/account/interest-accrued
    // API: GET /api/v5/account/interest-rate
    // API: GET /api/v5/account/max-withdrawal
    // API: GET /api/v5/account/move-positions-history
    // API: GET /api/v5/account/interest-limits
    // API: POST /api/v5/account/simulated_margin
    // API: GET /api/v5/account/positions-history
    // API: GET /api/v5/account/position-tiers
    // API: GET /api/v5/account/precheck-set-delta-neutral
    // API: GET /api/v5/account/risk-state
    // API: POST /api/v5/account/position-builder
    // API: POST /api/v5/account/set-riskOffset-amt
    // STATUS: TODO — needs deterministic fixtures, IDs, account eligibility, or avoids triggering server-side archive generation / time-limited download-link workflows.
    todo!("add endpoint-specific fixtures before promoting these to live tests");
}
