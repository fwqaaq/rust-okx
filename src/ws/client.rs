//! WebSocket client and connection lifecycle.

use std::collections::VecDeque;

use serde::Serialize;

use crate::credentials::Credentials;
use crate::{Error, signing};

use super::arg::Arg;
use super::conn::{TungsteniteConnector, WsConn, WsConnector, WsFrame};
use super::event::{WsEvent, parse_text_event};

/// OKX WebSocket channel group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WsChannelGroup {
    /// Public market-data channels.
    Public,
    /// Private account and order channels.
    Private,
    /// Business channels such as candle variants and some advanced feeds.
    Business,
}

impl WsChannelGroup {
    fn path(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Business => "business",
        }
    }
}

/// Builder for an [`OkxWs`] client.
pub struct OkxWsBuilder<C: WsConnector = TungsteniteConnector> {
    connector: C,
    group: WsChannelGroup,
    credentials: Option<Credentials>,
    demo: bool,
}

impl<C: WsConnector> OkxWsBuilder<C> {
    /// Create a builder from a connector, channel group, and region.
    pub fn new(connector: C, group: WsChannelGroup) -> Self {
        Self {
            connector,
            group,
            credentials: None,
            demo: false,
        }
    }

    /// Set API credentials for private WebSocket login.
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Toggle OKX demo trading WebSocket endpoints.
    pub fn demo_trading(mut self, demo: bool) -> Self {
        self.demo = demo;
        self
    }

    /// Connect and build the WebSocket client.
    pub async fn connect(self) -> Result<OkxWs<C>, Error> {
        let url = ws_url(self.group, self.demo);
        let conn = self.connector.connect(&url).await?;
        Ok(OkxWs {
            connector: self.connector,
            conn,
            url,
            group: self.group,
            credentials: self.credentials,
            logged_in: false,
            login_sent: false,
            subscriptions: Vec::new(),
            pending_after_login: Vec::new(),
            pending_payloads_after_login: Vec::new(),
            queued: VecDeque::new(),
        })
    }
}

impl OkxWsBuilder<TungsteniteConnector> {
    /// Create a public WebSocket builder using the default connector.
    pub fn public() -> Self {
        Self::new(TungsteniteConnector, WsChannelGroup::Public)
    }

    /// Create a business WebSocket builder using the default connector.
    pub fn business() -> Self {
        Self::new(TungsteniteConnector, WsChannelGroup::Business)
    }

    /// Create a private WebSocket builder using the default connector.
    pub fn private(credentials: Credentials) -> Self {
        Self::new(TungsteniteConnector, WsChannelGroup::Private).credentials(credentials)
    }
}

/// OKX WebSocket client.
pub struct OkxWs<C: WsConnector = TungsteniteConnector> {
    connector: C,
    conn: C::Conn,
    url: String,
    group: WsChannelGroup,
    credentials: Option<Credentials>,
    logged_in: bool,
    login_sent: bool,
    subscriptions: Vec<Arg>,
    pending_after_login: Vec<Arg>,
    pending_payloads_after_login: Vec<String>,
    queued: VecDeque<WsEvent>,
}

impl OkxWs<TungsteniteConnector> {
    /// Start building a public WebSocket client.
    pub fn public() -> OkxWsBuilder<TungsteniteConnector> {
        OkxWsBuilder::public()
    }

    /// Start building a business WebSocket client.
    pub fn business() -> OkxWsBuilder<TungsteniteConnector> {
        OkxWsBuilder::business()
    }

    /// Start building a private WebSocket client.
    pub fn private(credentials: Credentials) -> OkxWsBuilder<TungsteniteConnector> {
        OkxWsBuilder::private(credentials)
    }
}

impl<C: WsConnector> OkxWs<C> {
    /// Subscribe to one or more channel arguments.
    pub async fn subscribe(&mut self, args: &[Arg]) -> Result<(), Error> {
        self.track_subscriptions(args);
        if self.needs_login()? && !self.logged_in {
            self.pending_after_login.extend_from_slice(args);
            if !self.login_sent {
                self.send_login().await?;
            }
            return Ok(());
        }
        self.send_op("subscribe", args).await
    }

    /// Unsubscribe from one or more channel arguments.
    pub async fn unsubscribe(&mut self, args: &[Arg]) -> Result<(), Error> {
        self.subscriptions.retain(|arg| !args.contains(arg));
        self.send_op("unsubscribe", args).await
    }

    /// Receive and decode the next WebSocket event.
    pub async fn next_event(&mut self) -> Result<Option<WsEvent>, Error> {
        if let Some(event) = self.queued.pop_front() {
            return Ok(Some(event));
        }

        loop {
            let frame = match self.conn.recv().await {
                Ok(Some(frame)) => frame,
                Ok(None) | Err(_) => {
                    self.reconnect().await?;
                    return Ok(Some(WsEvent::Reconnected));
                }
            };

            match frame {
                WsFrame::Text(text) if text == "pong" => continue,
                WsFrame::Text(text) if text == "ping" => {
                    self.conn.send_text("pong".to_owned()).await?;
                    continue;
                }
                WsFrame::Text(text) => {
                    if let Some(event) = self.parse_text(&text).await? {
                        return Ok(Some(event));
                    }
                }
                WsFrame::Ping(_) => {
                    self.conn.send_text("pong".to_owned()).await?;
                }
                WsFrame::Pong(_) => continue,
                WsFrame::Close => {
                    self.reconnect().await?;
                    return Ok(Some(WsEvent::Reconnected));
                }
            }
        }
    }

    /// Close the WebSocket connection.
    pub async fn close(&mut self) -> Result<(), Error> {
        self.conn.close().await
    }

    pub(crate) async fn send_operation_payload(&mut self, payload: String) -> Result<(), Error> {
        if self.needs_login()? && !self.logged_in {
            self.pending_payloads_after_login.push(payload);
            if !self.login_sent {
                self.send_login().await?;
            }
            return Ok(());
        }
        self.conn.send_text(payload).await
    }

    fn track_subscriptions(&mut self, args: &[Arg]) {
        for arg in args {
            if !self.subscriptions.contains(arg) {
                self.subscriptions.push(arg.clone());
            }
        }
    }

    fn needs_login(&self) -> Result<bool, Error> {
        if self.group == WsChannelGroup::Private && self.credentials.is_none() {
            return Err(Error::Configuration(
                "private WebSocket requires credentials".to_owned(),
            ));
        }
        Ok(self.group == WsChannelGroup::Private
            || (self.group == WsChannelGroup::Business && self.credentials.is_some()))
    }

    async fn reconnect(&mut self) -> Result<(), Error> {
        self.conn = self.connector.connect(&self.url).await?;
        self.logged_in = false;
        self.login_sent = false;
        self.pending_payloads_after_login.clear();

        if self.needs_login()? {
            self.pending_after_login = self.subscriptions.clone();
            self.send_login().await?;
        } else if !self.subscriptions.is_empty() {
            let args = self.subscriptions.clone();
            self.send_op("subscribe", &args).await?;
        }
        Ok(())
    }

    async fn send_login(&mut self) -> Result<(), Error> {
        let credentials = self.credentials.as_ref().ok_or_else(|| {
            Error::Configuration("WebSocket login requires credentials".to_owned())
        })?;
        let payload = login_payload(credentials, &signing::ws_timestamp())?;
        self.conn.send_text(payload).await?;
        self.login_sent = true;
        Ok(())
    }

    async fn send_op(&mut self, op: &str, args: &[Arg]) -> Result<(), Error> {
        let payload = serde_json::to_string(&WsRequest { op, args }).map_err(Error::encode)?;
        self.conn.send_text(payload).await
    }

    async fn parse_text(&mut self, text: &str) -> Result<Option<WsEvent>, Error> {
        let Some(event) = parse_text_event(text)? else {
            return Ok(None);
        };

        if matches!(event, WsEvent::Login) {
            self.logged_in = true;
            self.login_sent = false;
            if !self.pending_after_login.is_empty() {
                let args = std::mem::take(&mut self.pending_after_login);
                self.send_op("subscribe", &args).await?;
            }
            if !self.pending_payloads_after_login.is_empty() {
                let payloads = std::mem::take(&mut self.pending_payloads_after_login);
                for payload in payloads {
                    self.conn.send_text(payload).await?;
                }
            }
        }

        Ok(Some(event))
    }
}

#[derive(Serialize)]
struct WsRequest<'a> {
    op: &'a str,
    args: &'a [Arg],
}

#[derive(Serialize)]
struct LoginRequest<'a> {
    op: &'static str,
    args: [LoginArg<'a>; 1],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LoginArg<'a> {
    api_key: &'a str,
    passphrase: &'a str,
    timestamp: &'a str,
    sign: String,
}

pub(crate) fn login_payload(credentials: &Credentials, timestamp: &str) -> Result<String, Error> {
    let payload = LoginRequest {
        op: "login",
        args: [LoginArg {
            api_key: credentials.api_key(),
            passphrase: credentials.passphrase(),
            timestamp,
            sign: signing::ws_login_sign(timestamp, credentials.secret_key()),
        }],
    };
    serde_json::to_string(&payload).map_err(Error::encode)
}

pub(crate) fn ws_url(group: WsChannelGroup, demo: bool) -> String {
    let host = if demo { "wspap.okx.com" } else { "ws.okx.com" };
    format!("wss://{host}:8443/ws/v5/{}", group.path())
}
