//! WebSocket trade-operation response envelope.

use serde::Deserialize;

use crate::model::NumberString;

use super::ResponseExtraFields;

/// Generic WebSocket trade-operation response body.
///
/// `T` is normally one of the operation result rows in [`crate::ws::model`].
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationResponse<T> {
    /// Client request ID.
    #[serde(default)]
    pub id: String,
    /// Operation name, e.g. `order` or `sprd-order`.
    #[serde(default)]
    pub op: String,
    /// Top-level OKX response code.
    #[serde(default)]
    pub code: String,
    /// Top-level OKX response message.
    #[serde(default)]
    pub msg: String,
    /// Per-request result rows.
    #[serde(default)]
    pub data: Vec<T>,
    /// Gateway receive timestamp in microseconds.
    #[serde(default)]
    pub in_time: NumberString,
    /// Gateway send timestamp in microseconds.
    #[serde(default)]
    pub out_time: NumberString,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}
