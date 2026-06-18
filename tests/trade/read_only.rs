use crate::common::{expect_ok_or_api_unavailable, live_client_or_skip};
use rust_okx::api::trade::{
    AlgoOrderListRequest, EasyConvertHistoryRequest, FillsRequest,
    OneClickRepayCurrencyListRequest, OneClickRepayHistoryRequest, OrderHistoryRequest,
    OrderListRequest,
};
use rust_okx::model::InstType;

#[tokio::test]
async fn standard_trade_read_only_endpoints_parse() {
    let Some(client) = live_client_or_skip("standard_trade_read_only_endpoints_parse") else {
        return;
    };

    // API: GET /api/v5/trade/orders-pending
    // STATUS: LIVE — authenticated, read-only.
    client
        .trade()
        .get_order_list(&OrderListRequest::new().limit(10))
        .await
        .expect("trade/orders-pending");

    // API: GET /api/v5/trade/orders-history
    // STATUS: LIVE — authenticated, read-only.
    client
        .trade()
        .get_orders_history(&OrderHistoryRequest::new(InstType::Spot))
        .await
        .expect("trade/orders-history");

    // API: GET /api/v5/trade/fills
    // STATUS: LIVE — authenticated, read-only.
    client
        .trade()
        .get_fills(&FillsRequest::new().limit(10))
        .await
        .expect("trade/fills");

    // API: GET /api/v5/trade/fills-history
    // STATUS: LIVE — authenticated, read-only.
    client
        .trade()
        .get_fills_history(&FillsRequest::new().inst_type(InstType::Spot).limit(10))
        .await
        .expect("trade/fills-history");
}

#[tokio::test]
async fn advanced_trade_read_only_endpoints_parse_when_supported() {
    let Some(client) =
        live_client_or_skip("advanced_trade_read_only_endpoints_parse_when_supported")
    else {
        return;
    };

    // API: GET /api/v5/trade/orders-algo-pending
    // STATUS: LIVE/ELIGIBILITY-TODO — decode errors fail; mode rejection skips.
    expect_ok_or_api_unavailable(
        client
            .trade()
            .get_algo_order_list(&AlgoOrderListRequest::new("conditional"))
            .await,
        "trade/orders-algo-pending",
    );

    // API: GET /api/v5/trade/easy-convert-currency-list
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client.trade().get_easy_convert_currency_list().await,
        "trade/easy-convert-currency-list",
    );

    // API: GET /api/v5/trade/easy-convert-history
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client
            .trade()
            .get_easy_convert_history(&EasyConvertHistoryRequest::new().limit(10))
            .await,
        "trade/easy-convert-history",
    );

    // API: GET /api/v5/trade/one-click-repay-currency-list
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client
            .trade()
            .get_one_click_repay_currency_list(&OneClickRepayCurrencyListRequest::new())
            .await,
        "trade/one-click-repay-currency-list",
    );

    // API: GET /api/v5/trade/one-click-repay-history
    // STATUS: LIVE/ELIGIBILITY-TODO.
    expect_ok_or_api_unavailable(
        client
            .trade()
            .get_one_click_repay_history(&OneClickRepayHistoryRequest::new().limit(10))
            .await,
        "trade/one-click-repay-history",
    );
}
