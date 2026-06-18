//! `trade` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `orders` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-order-channel>
pub fn orders(inst_type: impl Into<String>) -> Arg {
    Arg::new("orders").inst_type(inst_type)
}

/// Subscribe to `fills` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-fills-channel>
pub fn fills(inst_type: impl Into<String>) -> Arg {
    Arg::new("fills").inst_type(inst_type)
}
