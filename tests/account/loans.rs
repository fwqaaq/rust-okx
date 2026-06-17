use crate::common::{expect_ok_or_api_unavailable, live_client_or_skip};
use rust_okx::api::account::SpotBorrowRepayHistoryRequest;

#[tokio::test]
async fn loan_read_only_endpoints_parse_when_eligible() {
    let Some(client) = live_client_or_skip("loan_read_only_endpoints_parse_when_eligible") else {
        return;
    };

    // API: GET /api/v5/account/spot-borrow-repay-history
    // STATUS: LIVE/ELIGIBILITY-TODO — decode errors fail; product rejection skips.
    expect_ok_or_api_unavailable(
        client
            .account()
            .get_spot_borrow_repay_history(&SpotBorrowRepayHistoryRequest::new().currency("USDT"))
            .await,
        "account/spot-borrow-repay-history",
    );
}
