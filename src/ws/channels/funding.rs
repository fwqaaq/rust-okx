//! `funding` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `deposit-info`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-deposit-info-channel>
pub fn deposit_info() -> Arg {
    Arg::new("deposit-info")
}

/// Subscribe to `withdrawal-info`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-withdrawal-info-channel>
pub fn withdrawal_info() -> Arg {
    Arg::new("withdrawal-info")
}
