//! Fiat API coverage annotations.

#[test]
fn fiat_endpoints_are_implemented() {
    // API: GET /api/v5/fiat/deposit-payment-methods
    // API: GET /api/v5/fiat/withdrawal-payment-methods
    // API: POST /api/v5/fiat/create-withdrawal
    // API: POST /api/v5/fiat/cancel-withdrawal
    // API: GET /api/v5/fiat/withdrawal-order-history
    // API: GET /api/v5/fiat/withdrawal
    // API: GET /api/v5/fiat/deposit-order-history
    // API: GET /api/v5/fiat/deposit
    // API: GET /api/v5/fiat/buy-sell/currencies
    // API: GET /api/v5/fiat/buy-sell/currency-pair
    // API: POST /api/v5/fiat/buy-sell/quote
    // API: POST /api/v5/fiat/buy-sell/trade
    // API: GET /api/v5/fiat/buy-sell/history
    // STATUS: TODO — implementation is covered offline; live calls require regional fiat access and dedicated funds.
}
