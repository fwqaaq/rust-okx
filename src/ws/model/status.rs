//! System status channel model (`status`).
//!
//! Public channel; no authentication required.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Public `status` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#status-websocket-status-channel>
    StatusUpdate {
        id: String,
        title: String,
        state: String,
        begin: NumberString,
        end: NumberString,
        pre_open_begin: NumberString,
        href: String,
        service_type: String,
        system: String,
        sche_desc: String,
        maint_type: String,
        env: String,
        ts: NumberString
    }
}
