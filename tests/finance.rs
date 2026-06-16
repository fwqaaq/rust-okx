//! Integration tests for Finance endpoints.

mod common;

use std::env;

use common::live_client;
use rust_okx::OkxClient;
use rust_okx::api::finance::{FinanceHistoryRequest, StakingDefiOffersRequest};

fn client_or_skip(test: &str) -> Option<OkxClient> {
    let client = live_client();
    if client.is_none() {
        eprintln!("skipping {test}: OKX_API_* env vars not set");
    }
    client
}

fn mutation_enabled(test: &str) -> bool {
    let _ = dotenvy::dotenv();
    let enabled = env::var("OKX_ENABLE_FINANCE_MUTATION").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skipping {test}: set OKX_ENABLE_FINANCE_MUTATION=1");
    }
    enabled
}

#[tokio::test]
async fn finance_read_only_live() {
    let Some(client) = client_or_skip("finance_read_only_live") else {
        return;
    };

    let _ = client
        .finance()
        .savings()
        .get_public_borrow_info(None)
        .await;
    let _ = client
        .finance()
        .savings()
        .get_public_borrow_history(&FinanceHistoryRequest::new().param("ccy", "USDT"))
        .await;
    let _ = client
        .finance()
        .staking_defi()
        .get_offers(&StakingDefiOffersRequest::new())
        .await;
    let _ = client.finance().eth_staking().product_info().await;
    let _ = client.finance().sol_staking().product_info().await;
    let _ = client.finance().flexible_loan().borrow_currencies().await;
}

#[tokio::test]
async fn finance_mutation_live_is_gated() {
    if !mutation_enabled("finance_mutation_live_is_gated") {
        return;
    }

    eprintln!(
        "finance mutation test is intentionally a template; configure product IDs, currencies, and amounts before enabling"
    );
}
