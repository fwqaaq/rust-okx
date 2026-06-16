//! Integration tests for account VIP/fixed loan endpoints.

mod common;

use std::env;

use common::live_client;
use rust_okx::OkxClient;
use rust_okx::api::account::{
    FixedLoanBorrowingLimitRequest, FixedLoanBorrowingOrdersListRequest,
    SpotBorrowRepayHistoryRequest, VipInterestAccruedRequest, VipLoanOrderListRequest,
};

fn client_or_skip(test: &str) -> Option<OkxClient> {
    let client = live_client();
    if client.is_none() {
        eprintln!("skipping {test}: OKX_API_* env vars not set");
    }
    client
}

fn mutation_enabled(test: &str) -> bool {
    let _ = dotenvy::dotenv();
    let enabled = env::var("OKX_ENABLE_ACCOUNT_LOAN_MUTATION").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skipping {test}: set OKX_ENABLE_ACCOUNT_LOAN_MUTATION=1");
    }
    enabled
}

#[tokio::test]
async fn account_loan_read_only_live() {
    let Some(client) = client_or_skip("account_loan_read_only_live") else {
        return;
    };

    let _ = client
        .account()
        .get_vip_interest_accrued(&VipInterestAccruedRequest::new().param("ccy", "USDT"))
        .await;
    let _ = client
        .account()
        .get_vip_loan_order_list(&VipLoanOrderListRequest::new().param("ccy", "USDT"))
        .await;
    let _ = client
        .account()
        .get_fixed_loan_borrowing_limit(&FixedLoanBorrowingLimitRequest::new().param("ccy", "USDT"))
        .await;
    let _ = client
        .account()
        .get_fixed_loan_borrowing_orders_list(&FixedLoanBorrowingOrdersListRequest::new())
        .await;
    let _ = client
        .account()
        .get_spot_borrow_repay_history(&SpotBorrowRepayHistoryRequest::new().param("ccy", "USDT"))
        .await;
}

#[tokio::test]
async fn account_loan_mutation_live_is_gated() {
    if !mutation_enabled("account_loan_mutation_live_is_gated") {
        return;
    }

    eprintln!(
        "account loan mutation test is intentionally a template; configure loan IDs and amounts before enabling"
    );
}
