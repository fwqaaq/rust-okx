use crate::common::{env_non_empty, live_client_or_skip};
use rust_okx::api::convert::{
    ConvertCurrencyPairRequest, ConvertHistoryRequest, ConvertQuoteRequest,
};
use rust_okx::model::OrderSide;

#[tokio::test]
async fn convert_reference_and_history_parse() {
    let Some(client) = live_client_or_skip("convert_reference_and_history_parse") else {
        return;
    };

    // API: GET /api/v5/asset/convert/currencies
    // STATUS: LIVE — authenticated, read-only.
    client
        .convert()
        .get_currencies()
        .await
        .expect("convert/currencies");

    // API: GET /api/v5/asset/convert/currency-pair
    // STATUS: LIVE — authenticated, read-only.
    let pair = ConvertCurrencyPairRequest::new("BTC", "USDT");
    client
        .convert()
        .get_currency_pair(&pair)
        .await
        .expect("convert/currency-pair");

    // API: GET /api/v5/asset/convert/history
    // STATUS: LIVE — authenticated, read-only.
    client
        .convert()
        .get_convert_history(&ConvertHistoryRequest::new().limit(10))
        .await
        .expect("convert/history");
}

#[tokio::test]
async fn convert_estimate_quote_parses_when_inputs_are_configured() {
    let test = "convert_estimate_quote_parses_when_inputs_are_configured";
    let Some(client) = live_client_or_skip(test) else {
        return;
    };

    let Some(from_ccy) = env_non_empty("OKX_TEST_CONVERT_FROM_CCY") else {
        eprintln!("skipping {test}: OKX_TEST_CONVERT_FROM_CCY is not set");
        return;
    };
    let Some(to_ccy) = env_non_empty("OKX_TEST_CONVERT_TO_CCY") else {
        eprintln!("skipping {test}: OKX_TEST_CONVERT_TO_CCY is not set");
        return;
    };
    let Some(amount) = env_non_empty("OKX_TEST_CONVERT_AMOUNT") else {
        eprintln!("skipping {test}: OKX_TEST_CONVERT_AMOUNT is not set");
        return;
    };
    let size_ccy = env_non_empty("OKX_TEST_CONVERT_SIZE_CCY").unwrap_or_else(|| from_ccy.clone());
    let side = match env_non_empty("OKX_TEST_CONVERT_SIDE").as_deref() {
        None | Some("sell") => OrderSide::Sell,
        Some("buy") => OrderSide::Buy,
        Some(other) => panic!("OKX_TEST_CONVERT_SIDE must be buy or sell, got {other}"),
    };

    // API: POST /api/v5/asset/convert/estimate-quote
    // STATUS: LIVE/ENV — authenticated but does not execute a conversion.
    let request = ConvertQuoteRequest::new(from_ccy, to_ccy, side, amount, size_ccy);
    client
        .convert()
        .estimate_quote(&request)
        .await
        .expect("convert/estimate-quote");
}
