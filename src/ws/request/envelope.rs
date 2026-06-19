//! Common WebSocket request envelopes.

use serde::Serialize;

use crate::ws::Arg;

/// Subscribe or unsubscribe request body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct ChannelRequest<'a> {
    /// Optional client message ID (up to 32 case-sensitive alphanumeric characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// `subscribe` or `unsubscribe`.
    pub op: &'a str,
    /// Channel arguments.
    pub args: &'a [Arg],
}

impl<'a> ChannelRequest<'a> {
    /// Build a subscription request.
    pub fn subscribe(args: &'a [Arg]) -> Self {
        Self {
            id: None,
            op: "subscribe",
            args,
        }
    }

    /// Build an unsubscription request.
    pub fn unsubscribe(args: &'a [Arg]) -> Self {
        Self {
            id: None,
            op: "unsubscribe",
            args,
        }
    }

    /// Attach a client message ID.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

/// Private WebSocket login request body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-login>
#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct LoginRequest<'a> {
    /// Always `login`.
    pub op: &'static str,
    /// OKX requires exactly one login argument.
    pub args: [LoginArg<'a>; 1],
}

impl<'a> LoginRequest<'a> {
    /// Build a login request from one authentication argument.
    pub fn new(arg: LoginArg<'a>) -> Self {
        Self {
            op: "login",
            args: [arg],
        }
    }
}

/// Authentication argument inside [`LoginRequest`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoginArg<'a> {
    /// API key.
    pub api_key: &'a str,
    /// API key passphrase.
    pub passphrase: &'a str,
    /// Unix timestamp in seconds used for signing.
    pub timestamp: &'a str,
    /// Base64-encoded HMAC-SHA256 signature.
    pub sign: String,
}

impl<'a> LoginArg<'a> {
    /// Build a login argument from already-computed authentication values.
    pub fn new(
        api_key: &'a str,
        passphrase: &'a str,
        timestamp: &'a str,
        sign: impl Into<String>,
    ) -> Self {
        Self {
            api_key,
            passphrase,
            timestamp,
            sign: sign.into(),
        }
    }
}

/// Generic WebSocket trade-operation envelope.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationRequest<'a, A> {
    /// Client-provided request ID.
    pub id: String,
    /// OKX operation name.
    pub op: String,
    /// Optional request effective deadline in Unix milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    /// Operation arguments.
    pub args: &'a [A],
}

impl<'a, A> OperationRequest<'a, A> {
    /// Build an operation request without an expiration deadline.
    pub fn new(id: impl Into<String>, op: impl Into<String>, args: &'a [A]) -> Self {
        Self {
            id: id.into(),
            op: op.into(),
            exp_time: None,
            args,
        }
    }

    /// Set the request effective deadline in Unix milliseconds.
    pub fn exp_time(mut self, exp_time: impl Into<String>) -> Self {
        self.exp_time = Some(exp_time.into());
        self
    }
}
