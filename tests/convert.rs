//! Integration tests for Convert endpoints.

mod common;

use std::env;

use common::live_client;
use rust_okx::api::convert::{
    ConvertCurrencyPairRequest, ConvertHistoryRequest, ConvertQuoteRequest, ConvertTradeRequest,
};
use rust_okx::model::OrderSide;
use rust_okx::OkxClient;

fn client_or_skip(test: &str) -> Option<OkxClient> {
    let client = live_client();
    if client.is_none() {
        eprintln!("skipping {test}: OKX_API_* env vars not set");
    }
    client
}

fn env_non_empty(var: &str) -> Option<String> {
    let _ = dotenvy::dotenv();
    env::var(var).ok().filter(|value| !value.is_empty())
}

fn mutation_enabled(test: &str) -> bool {
    let _ = dotenvy::dotenv();
    let enabled = env::var("OKX_ENABLE_CONVERT_MUTATION").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skipping {test}: set OKX_ENABLE_CONVERT_MUTATION=1");
    }
    enabled
}

#[tokio::test]
async fn convert_read_only_live() {
    let Some(client) = client_or_skip("convert_read_only_live") else {
        return;
    };

    let _ = client
        .convert()
        .get_currencies()
        .await
        .expect("convert/currencies");

    let pair = ConvertCurrencyPairRequest::new("BTC", "USDT");
    let _ = client
        .convert()
        .get_currency_pair(&pair)
        .await
        .expect("convert/currency-pair");

    let _ = client
        .convert()
        .get_convert_history(&ConvertHistoryRequest::new().limit(10))
        .await
        .expect("convert/history");
}

#[tokio::test]
async fn convert_quote_and_trade_live_when_enabled() {
    let Some(client) = client_or_skip("convert_quote_and_trade_live_when_enabled") else {
        return;
    };
    if !mutation_enabled("convert_quote_and_trade_live_when_enabled") {
        return;
    }

    let base_ccy = env_non_empty("OKX_TEST_CONVERT_FROM_CCY").expect("from ccy");
    let quote_ccy = env_non_empty("OKX_TEST_CONVERT_TO_CCY").expect("to ccy");
    let side = match env_non_empty("OKX_TEST_CONVERT_SIDE")
        .expect("side")
        .as_str()
    {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        other => panic!("OKX_TEST_CONVERT_SIDE must be buy or sell, got {other}"),
    };
    let amount = env_non_empty("OKX_TEST_CONVERT_AMOUNT").expect("amount");
    let size_ccy = env_non_empty("OKX_TEST_CONVERT_SIZE_CCY").expect("size ccy");

    let quote = ConvertQuoteRequest::new(
        base_ccy.clone(),
        quote_ccy.clone(),
        side.clone(),
        amount.clone(),
        size_ccy.clone(),
    );
    let quotes = client
        .convert()
        .estimate_quote(&quote)
        .await
        .expect("convert/estimate-quote");

    let quote_id = env_non_empty("OKX_TEST_CONVERT_QUOTE_ID")
        .or_else(|| quotes.first().map(|row| row.quote_id.clone()))
        .expect("quote id");

    let trade = ConvertTradeRequest::new(
        quote_id,
        base_ccy,
        quote_ccy,
        side,
        amount,
        size_ccy,
    );
    let _ = client
        .convert()
        .convert_trade(&trade)
        .await
        .expect("convert/trade");
}
