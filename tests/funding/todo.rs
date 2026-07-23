#[test]
#[ignore = "moves funds between OKX accounts"]
fn funding_transfer_todo() {
    // API: POST /api/v5/asset/transfer
    // STATUS: TODO — changes balances and needs a reversible source/destination fixture.
    todo!("use a dedicated account and assert both transfer result and transfer-state");
}

#[test]
#[ignore = "requires withdrawal permission, allow-listing, fees, and real assets"]
fn funding_withdrawal_endpoints_todo() {
    // API: POST /api/v5/asset/withdrawal
    // API: POST /api/v5/asset/cancel-withdrawal
    // STATUS: TODO — unsafe for the default integration suite.
    todo!("configure an allow-listed destination, network, fee, amount cap, and cancellation plan");
}

#[test]
#[ignore = "changes real assets or creates Lightning invoices"]
fn funding_asset_conversion_and_lightning_todo() {
    // API: GET /api/v5/asset/deposit-lightning
    // API: POST /api/v5/asset/withdrawal-lightning
    // STATUS: TODO — needs product-specific balances and expiring invoice fixtures.
    todo!("supply dedicated fixtures and explicit amount caps");
}

#[test]
#[ignore = "statement generation is limited to 20 applications per month"]
fn funding_monthly_statement_todo() {
    // API: POST /api/v5/asset/monthly-statement
    // API: GET /api/v5/asset/monthly-statement
    // STATUS: TODO — requires a deliberately selected month and asynchronous generation fixture.
    todo!(
        "apply once for a selected month, then poll the generated statement without exhausting quota"
    );
}
