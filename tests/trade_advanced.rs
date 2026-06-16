//! Integration tests for advanced trade endpoints.

mod common;

use std::env;

use common::live_client;
use rust_okx::Error;
use rust_okx::OkxClient;
use rust_okx::api::trade::{
    AlgoOrderListRequest, EasyConvertHistoryRequest, OneClickRepayCurrencyListRequest,
    OneClickRepayHistoryRequest,
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
    let enabled = env::var("OKX_ENABLE_TRADE_MUTATION").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skipping {test}: set OKX_ENABLE_TRADE_MUTATION=1");
    }
    enabled
}

fn expect_ok_or_account_mode<T>(result: Result<T, Error>, label: &str) {
    match result {
        Ok(_) => {}
        Err(Error::Api { code, message }) if code == "51010" => {
            eprintln!("skipping {label}: account mode does not support this endpoint ({message})");
        }
        Err(err) => panic!("{label}: {err}"),
    }
}

#[tokio::test]
async fn advanced_trade_read_only_live() {
    let Some(client) = client_or_skip("advanced_trade_read_only_live") else {
        return;
    };

    expect_ok_or_account_mode(
        client
            .trade()
            .get_algo_order_list(&AlgoOrderListRequest::new().param("ordType", "conditional"))
            .await,
        "trade/orders-algo-pending",
    );
    expect_ok_or_account_mode(
        client.trade().get_easy_convert_currency_list().await,
        "trade/easy-convert-currency-list",
    );
    expect_ok_or_account_mode(
        client
            .trade()
            .get_easy_convert_history(&EasyConvertHistoryRequest::new().param("limit", "10"))
            .await,
        "trade/easy-convert-history",
    );
    expect_ok_or_account_mode(
        client
            .trade()
            .get_one_click_repay_currency_list(&OneClickRepayCurrencyListRequest::new())
            .await,
        "trade/one-click-repay-currency-list",
    );
    expect_ok_or_account_mode(
        client
            .trade()
            .get_one_click_repay_history(&OneClickRepayHistoryRequest::new().param("limit", "10"))
            .await,
        "trade/one-click-repay-history",
    );
}

#[tokio::test]
async fn advanced_trade_mutation_live_is_gated() {
    if !mutation_enabled("advanced_trade_mutation_live_is_gated") {
        return;
    }

    eprintln!(
        "advanced trade mutation test is intentionally a template; configure endpoint-specific inputs before enabling live order placement"
    );
}
