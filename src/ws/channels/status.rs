//! `status` WebSocket channel helpers.

use crate::ws::Arg;

/// Subscribe to `status`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#status-websocket-status-channel>
pub fn status() -> Arg {
    Arg::new("status")
}
