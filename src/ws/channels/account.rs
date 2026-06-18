//! `account` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to the complete `account` channel.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-channel>
pub fn account() -> Arg {
    Arg::new("account")
}

/// Subscribe to `account` filtered by currency.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-channel>
pub fn account_by_currency(ccy: impl Into<String>) -> Arg {
    Arg::new("account").ccy(ccy)
}

/// Subscribe to `positions` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-positions-channel>
pub fn positions(inst_type: impl Into<String>) -> Arg {
    Arg::new("positions").inst_type(inst_type)
}

/// Subscribe to `balance_and_position`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-balance-and-position-channel>
pub fn balance_and_position() -> Arg {
    Arg::new("balance_and_position")
}

/// Subscribe to `liquidation-warning` for one instrument type.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-liquidation-warning-channel>
pub fn liquidation_warning(inst_type: impl Into<String>) -> Arg {
    Arg::new("liquidation-warning").inst_type(inst_type)
}

/// Subscribe to `account-greeks` for one currency.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#trading-account-websocket-account-greeks-channel>
pub fn account_greeks(ccy: impl Into<String>) -> Arg {
    Arg::new("account-greeks").ccy(ccy)
}
