use crate::common::{expect_ok_or_api_unavailable, live_client_or_skip};
use rust_okx::api::finance::{CurrencyRequest, FinanceHistoryRequest, StakingDefiOffersRequest};

#[tokio::test]
async fn finance_read_only_endpoints_parse_when_eligible() {
    let Some(client) = live_client_or_skip("finance_read_only_endpoints_parse_when_eligible")
    else {
        return;
    };

    // API: GET /api/v5/finance/savings/lending-rate-summary
    // STATUS: LIVE — public finance data exposed through an authenticated client.
    expect_ok_or_api_unavailable(
        client
            .finance()
            .savings()
            .get_public_borrow_info(&CurrencyRequest::default())
            .await,
        "finance/savings/lending-rate-summary",
    );

    // API: GET /api/v5/finance/savings/lending-rate-history
    // STATUS: LIVE — read-only.
    expect_ok_or_api_unavailable(
        client
            .finance()
            .savings()
            .get_public_borrow_history(&FinanceHistoryRequest::new().currency("USDT"))
            .await,
        "finance/savings/lending-rate-history",
    );

    // API: GET /api/v5/finance/staking-defi/offers
    // STATUS: LIVE/ELIGIBILITY-TODO — availability varies by region/product.
    expect_ok_or_api_unavailable(
        client
            .finance()
            .staking_defi()
            .get_offers(&StakingDefiOffersRequest::new())
            .await,
        "finance/staking-defi/offers",
    );

    // API: GET /api/v5/finance/staking-defi/eth/product-info
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client.finance().eth_staking().product_info().await,
        "finance/staking-defi/eth/product-info",
    );

    // API: GET /api/v5/finance/staking-defi/sol/product-info
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client.finance().sol_staking().product_info().await,
        "finance/staking-defi/sol/product-info",
    );

    // API: GET /api/v5/finance/flexible-loan/borrow-currencies
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client.finance().flexible_loan().borrow_currencies().await,
        "finance/flexible-loan/borrow-currencies",
    );
}
