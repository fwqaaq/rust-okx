//! Login, subscription, notice, and connection-count responses.

use serde::Deserialize;

use crate::model::NumberString;
use crate::ws::Arg;

use super::ResponseExtraFields;

/// Subscribe/unsubscribe acknowledgement body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ChannelAcknowledgement {
    /// `subscribe`, `unsubscribe`, or `error`.
    #[serde(default)]
    pub event: String,
    /// Client message ID when one was included in the request.
    #[serde(default)]
    pub id: String,
    /// Acknowledged channel argument.
    #[serde(default)]
    pub arg: Option<Arg>,
    /// OKX response/error code when supplied.
    #[serde(default)]
    pub code: String,
    /// OKX response/error message when supplied.
    #[serde(default)]
    pub msg: String,
    /// WebSocket connection ID.
    #[serde(default)]
    pub conn_id: String,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}

/// Login acknowledgement or login error body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-login>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoginAcknowledgement {
    /// `login` on success or `error` on failure.
    #[serde(default)]
    pub event: String,
    /// OKX response/error code.
    #[serde(default)]
    pub code: String,
    /// OKX response/error message.
    #[serde(default)]
    pub msg: String,
    /// WebSocket connection ID.
    #[serde(default)]
    pub conn_id: String,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}

/// Service notice body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-notice>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NoticeResponse {
    /// Always `notice`.
    #[serde(default)]
    pub event: String,
    /// OKX notice code.
    #[serde(default)]
    pub code: String,
    /// Notice message.
    #[serde(default)]
    pub msg: String,
    /// WebSocket connection ID when supplied.
    #[serde(default)]
    pub conn_id: String,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}

/// Channel connection-count event body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-connection-count-limit>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ChannelConnectionCountResponse {
    /// `channel-conn-count` or `channel-conn-count-error`.
    #[serde(default)]
    pub event: String,
    /// Channel name.
    #[serde(default)]
    pub channel: String,
    /// Current connection count.
    #[serde(default)]
    pub conn_count: NumberString,
    /// WebSocket connection ID.
    #[serde(default)]
    pub conn_id: String,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}
