//! WebSocket connection traits and the default tungstenite connector.

use std::future::Future;

use bytes::Bytes;

use super::error::WsError;

/// A WebSocket frame handled by the OKX WebSocket client.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WsFrame {
    /// A text frame.
    Text(String),
    /// A ping frame.
    Ping(Bytes),
    /// A pong frame.
    Pong(Bytes),
    /// A close frame.
    Close,
}

/// A full-duplex WebSocket connection.
///
/// This trait is intentionally minimal so tests can use a fake connection
/// without depending on a concrete WebSocket implementation.
pub trait WsConn: Send {
    /// Send a text frame.
    fn send_text(&mut self, text: String) -> impl Future<Output = Result<(), WsError>> + Send;

    /// Send a pong control frame with the supplied ping payload.
    fn send_pong(&mut self, payload: Bytes) -> impl Future<Output = Result<(), WsError>> + Send;

    /// Receive the next frame.
    fn recv(&mut self) -> impl Future<Output = Result<Option<WsFrame>, WsError>> + Send;

    /// Close the connection.
    fn close(&mut self) -> impl Future<Output = Result<(), WsError>> + Send;
}

/// Creates WebSocket connections for an OKX WebSocket endpoint URL.
pub trait WsConnector: Send + Sync {
    /// The connection type returned by this connector.
    type Conn: WsConn;

    /// Connect to a WebSocket URL.
    fn connect(&self, url: &str) -> impl Future<Output = Result<Self::Conn, WsError>> + Send;
}

#[cfg(feature = "websocket")]
mod tungstenite_impl {
    use futures_util::{SinkExt, StreamExt};
    use tokio::net::TcpStream;
    use tokio_tungstenite::tungstenite::Message;
    use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

    use super::*;
    use crate::TransportError;

    /// Default WebSocket connector backed by `tokio-tungstenite`.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct TungsteniteConnector;

    /// WebSocket connection backed by `tokio-tungstenite`.
    pub struct TungsteniteConn {
        inner: WebSocketStream<MaybeTlsStream<TcpStream>>,
    }

    impl WsConnector for TungsteniteConnector {
        type Conn = TungsteniteConn;

        fn connect(&self, url: &str) -> impl Future<Output = Result<Self::Conn, WsError>> + Send {
            let url = url.to_owned();
            async move {
                let (inner, _) = connect_async(url)
                    .await
                    .map_err(|e| WsError::from(TransportError::new(e)))?;
                Ok(TungsteniteConn { inner })
            }
        }
    }

    #[allow(clippy::manual_async_fn)]
    impl WsConn for TungsteniteConn {
        fn send_text(&mut self, text: String) -> impl Future<Output = Result<(), WsError>> + Send {
            async move {
                self.inner
                    .send(Message::Text(text))
                    .await
                    .map_err(|e| WsError::from(TransportError::new(e)))
            }
        }

        fn send_pong(&mut self, payload: Bytes) -> impl Future<Output = Result<(), WsError>> + Send {
            async move {
                self.inner
                    .send(Message::Pong(payload.to_vec()))
                    .await
                    .map_err(|e| WsError::from(TransportError::new(e)))
            }
        }

        fn recv(&mut self) -> impl Future<Output = Result<Option<WsFrame>, WsError>> + Send {
            async move {
                loop {
                    let Some(message) = self.inner.next().await else {
                        return Ok(None);
                    };
                    let message = message
                        .map_err(|e| WsError::from(TransportError::new(e)))?;
                    match message {
                        Message::Text(text) => return Ok(Some(WsFrame::Text(text))),
                        Message::Ping(bytes) => return Ok(Some(WsFrame::Ping(Bytes::from(bytes)))),
                        Message::Pong(bytes) => return Ok(Some(WsFrame::Pong(Bytes::from(bytes)))),
                        Message::Close(_) => return Ok(Some(WsFrame::Close)),
                        Message::Binary(_) | Message::Frame(_) => continue,
                    }
                }
            }
        }

        fn close(&mut self) -> impl Future<Output = Result<(), WsError>> + Send {
            async move {
                self.inner
                    .close(None)
                    .await
                    .map_err(|e| WsError::from(TransportError::new(e)))
            }
        }
    }
}

#[cfg(feature = "websocket")]
pub use tungstenite_impl::{TungsteniteConn, TungsteniteConnector};
