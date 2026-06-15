//! OKX WebSocket client.
//!
//! Enable the `websocket` feature to use this module. The WebSocket client is
//! separate from the REST [`OkxClient`](crate::OkxClient), but reuses
//! [`Credentials`], [`OkxRegion`], and the crate-wide [`Error`] type.

mod conn;
pub mod model;

use std::collections::VecDeque;

use bytes::Bytes;
#[cfg(feature = "websocket")]
pub use conn::{TungsteniteConn, TungsteniteConnector};
pub use conn::{WsConn, WsConnector, WsFrame};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::credentials::Credentials;
use crate::{Error, OkxRegion, signing};

/// A WebSocket channel argument.
///
/// Use [`Arg::new`] for a channel-only subscription, then add instrument filters
/// with consuming setters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arg {
    /// OKX channel name, e.g. `tickers`, `books5`, `account`, or `orders`.
    pub channel: String,
    /// Instrument ID, e.g. `BTC-USDT`.
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Instrument type, e.g. `SPOT` or `SWAP`.
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument family, e.g. `BTC-USD`.
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

impl Arg {
    /// Create a channel argument.
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            inst_id: None,
            inst_type: None,
            inst_family: None,
        }
    }

    /// Set the instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the instrument type.
    pub fn inst_type(mut self, inst_type: impl Into<String>) -> Self {
        self.inst_type = Some(inst_type.into());
        self
    }

    /// Set the instrument family.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

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

/// A WebSocket event surfaced by [`OkxWs::next_event`].
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
    /// Channel data push.
    Push(Push),
    /// The client reconnected and started subscription recovery.
    Reconnected,
    /// The remote side closed the connection.
    Disconnected,
}

/// A channel data push.
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
    /// Deserialize the push `data` array into typed rows.
    pub fn parse<T: DeserializeOwned>(&self) -> Result<Vec<T>, Error> {
        serde_json::from_slice(&self.raw).map_err(Error::decode)
    }

    /// Return the raw JSON `data` bytes.
    pub fn raw_data(&self) -> &Bytes {
        &self.raw
    }
}

/// Builder for an [`OkxWs`] client.
pub struct OkxWsBuilder<C: WsConnector = TungsteniteConnector> {
    connector: C,
    region: OkxRegion,
    group: WsChannelGroup,
    credentials: Option<Credentials>,
    demo: bool,
}

impl<C: WsConnector> OkxWsBuilder<C> {
    /// Create a builder from a connector, channel group, and region.
    pub fn new(connector: C, group: WsChannelGroup, region: OkxRegion) -> Self {
        Self {
            connector,
            region,
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
        let url = ws_url(self.region, self.group, self.demo);
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
            queued: VecDeque::new(),
        })
    }
}

impl OkxWsBuilder<TungsteniteConnector> {
    /// Create a public WebSocket builder using the default connector.
    pub fn public(region: OkxRegion) -> Self {
        Self::new(TungsteniteConnector, WsChannelGroup::Public, region)
    }

    /// Create a business WebSocket builder using the default connector.
    pub fn business(region: OkxRegion) -> Self {
        Self::new(TungsteniteConnector, WsChannelGroup::Business, region)
    }

    /// Create a private WebSocket builder using the default connector.
    pub fn private(credentials: Credentials, region: OkxRegion) -> Self {
        Self::new(TungsteniteConnector, WsChannelGroup::Private, region).credentials(credentials)
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
    queued: VecDeque<WsEvent>,
}

impl OkxWs<TungsteniteConnector> {
    /// Start building a public WebSocket client.
    pub fn public(region: OkxRegion) -> OkxWsBuilder<TungsteniteConnector> {
        OkxWsBuilder::public(region)
    }

    /// Start building a business WebSocket client.
    pub fn business(region: OkxRegion) -> OkxWsBuilder<TungsteniteConnector> {
        OkxWsBuilder::business(region)
    }

    /// Start building a private WebSocket client.
    pub fn private(
        credentials: Credentials,
        region: OkxRegion,
    ) -> OkxWsBuilder<TungsteniteConnector> {
        OkxWsBuilder::private(credentials, region)
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
            Some("login") => {
                self.logged_in = true;
                self.login_sent = false;
                if !self.pending_after_login.is_empty() {
                    let args = std::mem::take(&mut self.pending_after_login);
                    self.send_op("subscribe", &args).await?;
                }
                Ok(Some(WsEvent::Login))
            }
            Some("error") => Ok(Some(WsEvent::Error {
                code: value
                    .get("code")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                msg: value
                    .get("msg")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
            })),
            _ if value.get("arg").is_some() && value.get("data").is_some() => {
                let arg = parse_arg(&value)?;
                let raw = serde_json::to_vec(value.get("data").expect("checked above"))
                    .map_err(Error::decode)?;
                Ok(Some(WsEvent::Push(Push {
                    arg,
                    action: value
                        .get("action")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    raw: Bytes::from(raw),
                })))
            }
            _ => Ok(None),
        }
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

fn login_payload(credentials: &Credentials, timestamp: &str) -> Result<String, Error> {
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

fn parse_arg(value: &Value) -> Result<Arg, Error> {
    let arg = value
        .get("arg")
        .ok_or_else(|| Error::Decode("missing WebSocket arg".into()))?;
    serde_json::from_value(arg.clone()).map_err(Error::decode)
}

fn ws_url(region: OkxRegion, group: WsChannelGroup, demo: bool) -> String {
    let host = if demo {
        "wspap.okx.com"
    } else {
        match region {
            OkxRegion::Global => "ws.okx.com",
            OkxRegion::Us => "wsus.okx.com",
            OkxRegion::Eea => "wseea.okx.com",
        }
    };
    format!("wss://{host}:8443/ws/v5/{}", group.path())
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    use crate::TransportError;
    use crate::api::market::Ticker;

    use super::*;

    #[derive(Clone)]
    struct MockConnector {
        conns: Arc<Mutex<VecDeque<MockWsConn>>>,
    }

    impl MockConnector {
        fn new(conns: Vec<MockWsConn>) -> Self {
            Self {
                conns: Arc::new(Mutex::new(conns.into())),
            }
        }
    }

    impl WsConnector for MockConnector {
        type Conn = MockWsConn;

        fn connect(
            &self,
            _url: &str,
        ) -> impl std::future::Future<Output = Result<Self::Conn, Error>> + Send {
            let conn = self.conns.lock().unwrap().pop_front();
            async move {
                conn.ok_or_else(|| Error::Transport(TransportError::message("no mock connection")))
            }
        }
    }

    #[derive(Clone)]
    struct MockWsConn {
        frames: Arc<Mutex<VecDeque<MockFrame>>>,
        sent: Arc<Mutex<Vec<String>>>,
    }

    impl MockWsConn {
        fn new(frames: Vec<MockFrame>) -> Self {
            Self {
                frames: Arc::new(Mutex::new(frames.into())),
                sent: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn sent(&self) -> Vec<String> {
            self.sent.lock().unwrap().clone()
        }
    }

    #[derive(Clone)]
    enum MockFrame {
        Frame(WsFrame),
        Error,
    }

    #[allow(clippy::manual_async_fn)]
    impl WsConn for MockWsConn {
        fn send_text(
            &mut self,
            text: String,
        ) -> impl std::future::Future<Output = Result<(), Error>> + Send {
            self.sent.lock().unwrap().push(text);
            async { Ok(()) }
        }

        fn recv(
            &mut self,
        ) -> impl std::future::Future<Output = Result<Option<WsFrame>, Error>> + Send {
            let frame = self.frames.lock().unwrap().pop_front();
            async move {
                match frame {
                    Some(MockFrame::Frame(frame)) => Ok(Some(frame)),
                    Some(MockFrame::Error) => {
                        Err(Error::Transport(TransportError::message("disconnect")))
                    }
                    None => Ok(None),
                }
            }
        }

        fn close(&mut self) -> impl std::future::Future<Output = Result<(), Error>> + Send {
            async { Ok(()) }
        }
    }

    #[test]
    fn login_payload_signs_expected_message() {
        let credentials = Credentials::new("key", "secret", "pass");
        let payload = login_payload(&credentials, "1700000000").unwrap();
        let value: Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(value["op"], "login");
        assert_eq!(value["args"][0]["apiKey"], "key");
        assert_eq!(value["args"][0]["passphrase"], "pass");
        assert_eq!(value["args"][0]["timestamp"], "1700000000");
        assert_eq!(
            value["args"][0]["sign"],
            signing::ws_login_sign("1700000000", "secret")
        );
    }

    #[test]
    fn arg_serializes_optional_fields_only_when_set() {
        let arg = Arg::new("tickers").inst_id("BTC-USDT");
        let value = serde_json::to_value(&arg).unwrap();
        assert_eq!(value["channel"], "tickers");
        assert_eq!(value["instId"], "BTC-USDT");
        assert!(value.get("instType").is_none());
        assert!(value.get("instFamily").is_none());
    }

    #[tokio::test]
    async fn subscribe_sends_expected_payload_and_ack_parses() {
        let conn = MockWsConn::new(vec![MockFrame::Frame(WsFrame::Text(
            r#"{"event":"subscribe","arg":{"channel":"tickers","instId":"BTC-USDT"}}"#.to_owned(),
        ))]);
        let sent = conn.clone();
        let connector = MockConnector::new(vec![conn]);
        let mut ws = OkxWsBuilder::new(connector, WsChannelGroup::Public, OkxRegion::Global)
            .connect()
            .await
            .unwrap();

        ws.subscribe(&[Arg::new("tickers").inst_id("BTC-USDT")])
            .await
            .unwrap();
        let payload: Value = serde_json::from_str(&sent.sent()[0]).unwrap();
        assert_eq!(payload["op"], "subscribe");
        assert_eq!(payload["args"][0]["channel"], "tickers");

        let event = ws.next_event().await.unwrap().unwrap();
        assert!(matches!(event, WsEvent::Subscribed(_)));
    }

    #[tokio::test]
    async fn push_event_parses_typed_rows() {
        let conn = MockWsConn::new(vec![MockFrame::Frame(WsFrame::Text(
            r#"{"arg":{"channel":"tickers","instId":"BTC-USDT"},"data":[{"instType":"SPOT","instId":"BTC-USDT","last":"42000","ts":"1597026383085"}]}"#.to_owned(),
        ))]);
        let connector = MockConnector::new(vec![conn]);
        let mut ws = OkxWsBuilder::new(connector, WsChannelGroup::Public, OkxRegion::Global)
            .connect()
            .await
            .unwrap();

        let event = ws.next_event().await.unwrap().unwrap();
        let WsEvent::Push(push) = event else {
            panic!("expected push");
        };
        let tickers: Vec<Ticker> = push.parse().unwrap();
        assert_eq!(tickers[0].last.as_str(), "42000");
    }

    #[tokio::test]
    async fn private_subscribe_logs_in_before_subscribing() {
        let conn = MockWsConn::new(vec![MockFrame::Frame(WsFrame::Text(
            r#"{"event":"login","code":"0","msg":""}"#.to_owned(),
        ))]);
        let sent = conn.clone();
        let connector = MockConnector::new(vec![conn]);
        let credentials = Credentials::new("key", "secret", "pass");
        let mut ws = OkxWsBuilder::new(connector, WsChannelGroup::Private, OkxRegion::Global)
            .credentials(credentials)
            .connect()
            .await
            .unwrap();

        ws.subscribe(&[Arg::new("account")]).await.unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&sent.sent()[0]).unwrap()["op"],
            "login"
        );

        let event = ws.next_event().await.unwrap().unwrap();
        assert!(matches!(event, WsEvent::Login));
        assert_eq!(
            serde_json::from_str::<Value>(&sent.sent()[1]).unwrap()["op"],
            "subscribe"
        );
    }

    #[tokio::test]
    async fn reconnect_replays_public_subscriptions() {
        let first = MockWsConn::new(vec![MockFrame::Error]);
        let second = MockWsConn::new(vec![]);
        let sent_second = second.clone();
        let connector = MockConnector::new(vec![first, second]);
        let mut ws = OkxWsBuilder::new(connector, WsChannelGroup::Public, OkxRegion::Global)
            .connect()
            .await
            .unwrap();

        ws.subscribe(&[Arg::new("tickers").inst_id("BTC-USDT")])
            .await
            .unwrap();
        let event = ws.next_event().await.unwrap().unwrap();
        assert!(matches!(event, WsEvent::Reconnected));
        assert_eq!(
            serde_json::from_str::<Value>(&sent_second.sent()[0]).unwrap()["op"],
            "subscribe"
        );
    }

    #[tokio::test]
    async fn text_ping_sends_text_pong_without_user_event() {
        let conn = MockWsConn::new(vec![
            MockFrame::Frame(WsFrame::Text("ping".to_owned())),
            MockFrame::Frame(WsFrame::Text(
                r#"{"event":"error","code":"1","msg":"bad"}"#.to_owned(),
            )),
        ]);
        let sent = conn.clone();
        let connector = MockConnector::new(vec![conn]);
        let mut ws = OkxWsBuilder::new(connector, WsChannelGroup::Public, OkxRegion::Global)
            .connect()
            .await
            .unwrap();

        let event = ws.next_event().await.unwrap().unwrap();
        assert!(matches!(event, WsEvent::Error { .. }));
        assert_eq!(sent.sent()[0], "pong");
    }

    #[test]
    fn endpoint_urls_follow_region_and_demo() {
        assert_eq!(
            ws_url(OkxRegion::Global, WsChannelGroup::Public, false),
            "wss://ws.okx.com:8443/ws/v5/public"
        );
        assert_eq!(
            ws_url(OkxRegion::Us, WsChannelGroup::Private, false),
            "wss://wsus.okx.com:8443/ws/v5/private"
        );
        assert_eq!(
            ws_url(OkxRegion::Eea, WsChannelGroup::Business, false),
            "wss://wseea.okx.com:8443/ws/v5/business"
        );
        assert_eq!(
            ws_url(OkxRegion::Eea, WsChannelGroup::Public, true),
            "wss://wspap.okx.com:8443/ws/v5/public"
        );
    }
}
