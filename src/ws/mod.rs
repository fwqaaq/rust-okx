//! OKX WebSocket client.
//!
//! Enable the `websocket` feature to use this module. The WebSocket client is
//! separate from the REST [`OkxClient`](crate::OkxClient), but reuses
//! [`Credentials`](crate::Credentials), [`OkxRegion`](crate::OkxRegion), and
//! the crate-wide [`Error`](crate::Error) type.

pub mod arg;
pub mod channels;
pub mod client;
mod conn;
pub mod event;
pub mod model;
pub mod ops;

pub use arg::Arg;
pub use client::{OkxWs, OkxWsBuilder, WsChannelGroup};
#[cfg(feature = "websocket")]
pub use conn::{TungsteniteConn, TungsteniteConnector};
pub use conn::{WsConn, WsConnector, WsFrame};
pub use event::{Push, WsChannelConnectionCount, WsEvent, WsNotice, WsOperation};

#[cfg(test)]
mod tests;
