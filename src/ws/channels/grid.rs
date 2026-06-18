//! `grid` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `grid-orders-spot`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
pub fn spot_grid_orders() -> Arg {
    Arg::new("grid-orders-spot").inst_type("SPOT")
}

/// Subscribe to `grid-orders-contract`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
pub fn contract_grid_orders() -> Arg {
    Arg::new("grid-orders-contract").inst_type("ANY")
}

/// Subscribe to `grid-orders-moon`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
pub fn moon_grid_orders() -> Arg {
    Arg::new("grid-orders-moon").inst_type("SPOT")
}

/// Subscribe to `grid-positions` for one algo order.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-positions-channel>
pub fn positions(algo_id: impl Into<String>) -> Arg {
    Arg::new("grid-positions").algo_id(algo_id)
}

/// Subscribe to `grid-sub-orders` for one algo order.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-sub-orders-channel>
pub fn sub_orders(algo_id: impl Into<String>) -> Arg {
    Arg::new("grid-sub-orders").algo_id(algo_id)
}

/// Subscribe to recurring-buy algo updates (`algo-recurring-buy`).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-recurring-buy-orders-channel>
pub fn recurring_buy_orders() -> Arg {
    Arg::new("algo-recurring-buy")
}

/// Subscribe to `copytrading-notification`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#copy-trading-websocket-copy-trading-notification-channel>
pub fn copy_trading_notification() -> Arg {
    Arg::new("copytrading-notification")
}
