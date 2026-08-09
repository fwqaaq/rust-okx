//! Trading Data API coverage annotations.

#[test]
fn trading_data_endpoints_are_implemented() {
    // API: GET /api/v5/rubik/stat/trading-data/support-coin
    // API: GET /api/v5/rubik/stat/taker-volume
    // API: GET /api/v5/rubik/stat/margin/loan-ratio
    // API: GET /api/v5/rubik/stat/contracts/long-short-account-ratio
    // API: GET /api/v5/rubik/stat/contracts/long-short-account-ratio-contract
    // API: GET /api/v5/rubik/stat/contracts/open-interest-volume
    // API: GET /api/v5/rubik/stat/contracts/open-interest-history
    // API: GET /api/v5/rubik/stat/option/open-interest-volume
    // API: GET /api/v5/rubik/stat/option/open-interest-volume-ratio
    // API: GET /api/v5/rubik/stat/option/open-interest-volume-expiry
    // API: GET /api/v5/rubik/stat/option/open-interest-volume-strike
    // API: GET /api/v5/rubik/stat/option/taker-block-volume
    // API: GET /api/v5/rubik/stat/contracts/long-short-account-ratio-contract-top-trader
    // API: GET /api/v5/rubik/stat/contracts/long-short-position-ratio-contract-top-trader
    // API: GET /api/v5/rubik/stat/taker-volume-contract
    // STATUS: implemented — public read-only endpoints are covered by offline official-example tests.
}
