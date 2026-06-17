//! Typed top-level OKX WebSocket response bodies.
//!
//! [`crate::ws::WsEvent`] is optimized for streaming and keeps channel/operation
//! `data` as raw bytes until the caller chooses a row type. These envelope
//! models are provided for callers and tests that prefer direct Serde decoding
//! of a complete OKX response body.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::model::NumberString;

use super::Arg;

/// Unrecognized top-level response fields retained for forward compatibility.
pub type ResponseExtraFields = BTreeMap<String, Value>;

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

/// Generic WebSocket trade-operation response body.
///
/// `T` is normally one of the operation result rows in
/// [`crate::ws::model`].
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

/// Generic channel data-push response body.
///
/// `T` is the channel row type from [`crate::ws::model`].
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PushResponse<T> {
    /// Subscription argument identifying the channel and filters.
    pub arg: Arg,
    /// Push action, commonly `snapshot` or `update`.
    #[serde(default)]
    pub action: String,
    /// Typed channel rows.
    #[serde(default)]
    pub data: Vec<T>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::model::TickerUpdate;

    #[test]
    fn parses_complete_channel_push_envelope() {
        let response: PushResponse<TickerUpdate> = serde_json::from_str(
            r#"{"arg":{"channel":"tickers","instId":"BTC-USDT"},"data":[{"instType":"SPOT","instId":"BTC-USDT","last":"1","ts":"2"}]}"#,
        )
        .unwrap();
        assert_eq!(response.arg.channel, "tickers");
        assert_eq!(response.data[0].inst_id, "BTC-USDT");
    }

    #[test]
    fn parses_login_acknowledgement_metadata() {
        let response: LoginAcknowledgement = serde_json::from_str(
            r#"{"event":"login","code":"0","msg":"","connId":"abc"}"#,
        )
        .unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.conn_id, "abc");
    }
}
