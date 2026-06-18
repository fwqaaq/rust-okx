#[test]
#[ignore = "requires product enrollment or deterministic account history"]
fn finance_uncovered_read_only_endpoints_todo() {
    // API: GET /api/v5/finance/savings/balance
    // API: GET /api/v5/finance/savings/lending-history
    // API: GET /api/v5/finance/staking-defi/orders-active
    // API: GET /api/v5/finance/staking-defi/orders-history
    // API: POST /api/v5/finance/staking-defi/eth/cancel-redeem
    // API: GET /api/v5/finance/staking-defi/eth/balance
    // API: GET /api/v5/finance/staking-defi/eth/purchase-redeem-history
    // API: GET /api/v5/finance/staking-defi/eth/apy-history
    // API: GET /api/v5/finance/staking-defi/sol/balance
    // API: GET /api/v5/finance/staking-defi/sol/purchase-redeem-history
    // API: GET /api/v5/finance/staking-defi/sol/apy-history
    // API: GET /api/v5/finance/flexible-loan/collateral-assets
    // API: POST /api/v5/finance/flexible-loan/max-loan
    // API: GET /api/v5/finance/flexible-loan/max-collateral-redeem-amount
    // API: GET /api/v5/finance/flexible-loan/loan-info
    // API: GET /api/v5/finance/flexible-loan/loan-history
    // API: GET /api/v5/finance/flexible-loan/interest-accrued
    // STATUS: TODO — needs enrolled products, current order IDs, or non-empty history.
    todo!("add endpoint-specific account fixtures and expected API error handling");
}

#[test]
#[ignore = "locks, redeems, borrows, or moves real assets"]
fn finance_mutating_endpoints_todo() {
    // API: POST /api/v5/finance/savings/purchase-redempt
    // API: POST /api/v5/finance/savings/set-lending-rate
    // API: POST /api/v5/finance/staking-defi/purchase
    // API: POST /api/v5/finance/staking-defi/redeem
    // API: POST /api/v5/finance/staking-defi/cancel
    // API: POST /api/v5/finance/staking-defi/eth/purchase
    // API: POST /api/v5/finance/staking-defi/eth/redeem
    // API: POST /api/v5/finance/staking-defi/sol/purchase
    // API: POST /api/v5/finance/staking-defi/sol/redeem
    // API: POST /api/v5/finance/flexible-loan/adjust-collateral
    // STATUS: TODO — requires real balances and a product-specific rollback plan.
    todo!("run only in an isolated account with strict amount caps");
}
