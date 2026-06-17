//! WebSocket event and response decoding.

use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{Error, NumberString};

use super::arg::Arg;

/// A WebSocket event surfaced by [`OkxWs::next_event`](crate::ws::OkxWs::next_event).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WsEvent {
    /// Subscription acknowledgement.
    Subscribed(Arg),
    /// Unsubscription acknowledgement.
    Unsubscribed(Arg),
    /// Private login acknowledgement.
    Login,
    /// OKX WebSocket error event.
    Error {
        /// OKX error code.
        code: String,
        /// OKX error message.
        msg: String,
    },
    /// WebSocket service notice, usually for service upgrades.
    Notice(WsNotice),
    /// Connection-count notification for channels with connection limits.
    ChannelConnectionCount(WsChannelConnectionCount),
    /// Connection-count limit error. The server terminated this channel subscription.
    ChannelConnectionCountError(WsChannelConnectionCount),
    /// WebSocket operation response, e.g. order placement or cancellation.
    Operation(WsOperation),
    /// Channel data push.
    Push(Push),
    /// The client reconnected and started subscription recovery.
    Reconnected,
    /// The remote side closed the connection.
    Disconnected,
}

/// A service notice pushed by OKX.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-notice>
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct WsNotice {
    /// OKX notice code, if supplied.
    pub code: String,
    /// OKX notice message.
    pub msg: String,
}

/// A channel connection-count notification.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-connection-count-limit>
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct WsChannelConnectionCount {
    /// Channel name associated with the notification.
    pub channel: String,
    /// Current connection count for the channel.
    pub conn_count: NumberString,
    /// WebSocket connection ID.
    pub conn_id: String,
}

/// A WebSocket operation response.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct WsOperation {
    /// Client-provided message ID.
    pub id: Option<String>,
    /// OKX operation name.
    pub op: String,
    /// Top-level OKX response code.
    pub code: String,
    /// Top-level OKX response message.
    pub msg: String,
    /// Gateway receive timestamp, when supplied.
    pub in_time: Option<NumberString>,
    /// Gateway send timestamp, when supplied.
    pub out_time: Option<NumberString>,
    raw: Bytes,
}

impl WsOperation {
    /// Deserialize the response `data` array into typed rows.
    pub fn parse<T: DeserializeOwned>(&self) -> Result<Vec<T>, Error> {
        serde_json::from_slice(&self.raw).map_err(Error::decode)
    }

    /// Return the raw JSON `data` bytes.
    pub fn raw_data(&self) -> &Bytes {
        &self.raw
    }
}

/// A channel data push.
///
/// The concrete rows are available in [`crate::ws::model`] and can be decoded
/// with [`Push::parse`].
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Push {
    /// Channel argument associated with this push.
    pub arg: Arg,
    /// OKX push action, usually `snapshot` or `update`.
    pub action: Option<String>,
    raw: Bytes,
}

impl Push {
    pub(crate) fn new(arg: Arg, action: Option<String>, raw: Bytes) -> Self {
        Self { arg, action, raw }
    }

    /// Deserialize the push `data` array into typed rows.
    pub fn parse<T: DeserializeOwned>(&self) -> Result<Vec<T>, Error> {
        serde_json::from_slice(&self.raw).map_err(Error::decode)
    }

    /// Return the raw JSON `data` bytes.
    pub fn raw_data(&self) -> &Bytes {
        &self.raw
    }
}

pub(crate) fn parse_text_event(text: &str) -> Result<Option<WsEvent>, Error> {
    let value: Value = serde_json::from_str(text).map_err(Error::decode)?;
    match value.get("event").and_then(Value::as_str) {
        Some("subscribe") => {
            let arg = parse_arg(&value)?;
            Ok(Some(WsEvent::Subscribed(arg)))
        }
        Some("unsubscribe") => {
            let arg = parse_arg(&value)?;
            Ok(Some(WsEvent::Unsubscribed(arg)))
        }
        Some("login") => Ok(Some(WsEvent::Login)),
        Some("error") => Ok(Some(WsEvent::Error {
            code: string_field(&value, "code"),
            msg: string_field(&value, "msg"),
        })),
        Some("notice") => Ok(Some(WsEvent::Notice(WsNotice {
            code: string_field(&value, "code"),
            msg: string_field(&value, "msg"),
        }))),
        Some("channel-conn-count") => Ok(Some(WsEvent::ChannelConnectionCount(
            parse_channel_connection_count(&value),
        ))),
        Some("channel-conn-count-error") => Ok(Some(WsEvent::ChannelConnectionCountError(
            parse_channel_connection_count(&value),
        ))),
        _ if value.get("arg").is_some() && value.get("data").is_some() => {
            let arg = parse_arg(&value)?;
            let raw = data_bytes(&value)?;
            Ok(Some(WsEvent::Push(Push::new(
                arg,
                value
                    .get("action")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned),
                raw,
            ))))
        }
        _ if value.get("op").is_some() => Ok(Some(WsEvent::Operation(parse_operation(&value)?))),
        _ => Ok(None),
    }
}

fn parse_channel_connection_count(value: &Value) -> WsChannelConnectionCount {
    WsChannelConnectionCount {
        channel: string_field(value, "channel"),
        conn_count: string_field(value, "connCount").into(),
        conn_id: string_field(value, "connId"),
    }
}

fn parse_operation(value: &Value) -> Result<WsOperation, Error> {
    Ok(WsOperation {
        id: value
            .get("id")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        op: string_field(value, "op"),
        code: string_field(value, "code"),
        msg: string_field(value, "msg"),
        in_time: optional_number_string(value, "inTime"),
        out_time: optional_number_string(value, "outTime"),
        raw: data_bytes(value)?,
    })
}

fn parse_arg(value: &Value) -> Result<Arg, Error> {
    let arg = value
        .get("arg")
        .ok_or_else(|| Error::Decode("missing WebSocket arg".into()))?;
    serde_json::from_value(arg.clone()).map_err(Error::decode)
}

fn data_bytes(value: &Value) -> Result<Bytes, Error> {
    let data = value
        .get("data")
        .cloned()
        .unwrap_or_else(|| Value::Array(vec![]));
    let raw = serde_json::to_vec(&data).map_err(Error::decode)?;
    Ok(Bytes::from(raw))
}

fn string_field(value: &Value, field: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn optional_number_string(value: &Value, field: &str) -> Option<NumberString> {
    value.get(field).and_then(Value::as_str).map(Into::into)
}
