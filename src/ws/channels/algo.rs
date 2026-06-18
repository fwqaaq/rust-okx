//! `algo` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `orders-algo` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-algo-orders-channel>
pub fn orders_algo(inst_type: impl Into<String>) -> Arg {
    Arg::new("orders-algo").inst_type(inst_type)
}

/// Subscribe to `algo-advance` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-advance-algo-orders-channel>
pub fn algo_advance(inst_type: impl Into<String>) -> Arg {
    Arg::new("algo-advance").inst_type(inst_type)
}
