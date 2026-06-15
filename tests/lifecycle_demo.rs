//! End-to-end trade lifecycle against the OKX **demo (simulated) account only**.
//!
//! SAFETY: every order/mutating call here runs exclusively on the demo account
//! (`demo_client()` sets `x-simulated-trading: 1`). The limit price is placed
//! far below the market so the buy order rests on the book and never fills.
//! These tests skip entirely when the `OKX_DEMO_API_*` credentials are absent.

mod common;

use common::demo_client;
use rust_okx::api::account::{LeverageRequest, SetLeverageRequest};
use rust_okx::api::trade::PlaceOrderRequest;
use rust_okx::model::{OrderSide, OrderState, OrderType, TradeMode};

/// Full order lifecycle on the demo account:
/// 1. `GET  /api/v5/market/ticker`       — read current price.
/// 2. `POST /api/v5/trade/order`         — place a non-marketable limit buy.
/// 3. `GET  /api/v5/trade/order`         — confirm it is `live`.
/// 4. `POST /api/v5/trade/cancel-order`  — cancel it.
/// 5. `GET  /api/v5/trade/order`         — confirm it is `canceled`.
#[tokio::test]
async fn demo_place_get_cancel_order() {
    let Some(client) = demo_client() else {
        eprintln!("skipping demo_place_get_cancel_order: OKX_DEMO_API_* env vars not set");
        return;
    };

    let ticker = client
        .market()
        .get_ticker("BTC-USDT")
        .await
        .expect("ticker");
    let last: f64 = ticker[0].last.parse().expect("price is numeric");
    // Half the market price: a buy limit here will rest, not fill.
    let price = format!("{:.1}", last * 0.5);

    let request = PlaceOrderRequest::new(
        "BTC-USDT",
        TradeMode::Cash,
        OrderSide::Buy,
        OrderType::Limit,
        "0.0001",
    )
    .price(price);
    let placed = client.trade().place_order(&request).await.expect("place_order");
    assert_eq!(
        placed[0].s_code, "0",
        "order was rejected: {}",
        placed[0].s_msg
    );
    let ord_id = placed[0].ord_id.clone();
    assert!(!ord_id.is_empty(), "expected an order id");

    let live = client
        .trade()
        .get_order("BTC-USDT", &ord_id)
        .await
        .expect("get_order");
    assert_eq!(live[0].state, OrderState::Live);

    let cancelled = client
        .trade()
        .cancel_order("BTC-USDT", &ord_id)
        .await
        .expect("cancel_order");
    assert_eq!(
        cancelled[0].s_code, "0",
        "cancel was rejected: {}",
        cancelled[0].s_msg
    );

    let after = client
        .trade()
        .get_order("BTC-USDT", &ord_id)
        .await
        .expect("get_order after cancel");
    assert_eq!(after[0].state, OrderState::Canceled);
}

/// Mutating account example on the demo account:
/// 1. `POST /api/v5/account/set-leverage`  — set 5x cross on BTC-USDT-SWAP.
/// 2. `GET  /api/v5/account/leverage-info` — read it back.
#[tokio::test]
async fn demo_set_and_get_leverage() {
    let Some(client) = demo_client() else {
        eprintln!("skipping demo_set_and_get_leverage: OKX_DEMO_API_* env vars not set");
        return;
    };

    let set = SetLeverageRequest::new("5", TradeMode::Cross).inst_id("BTC-USDT-SWAP");
    let result = client.account().set_leverage(&set).await.expect("set_leverage");
    assert!(!result.is_empty(), "set-leverage should return a row");

    let info = client
        .account()
        .get_leverage(&LeverageRequest::new(TradeMode::Cross).inst_id("BTC-USDT-SWAP"))
        .await
        .expect("leverage-info");
    assert!(!info.is_empty(), "leverage-info should return a row");
}
