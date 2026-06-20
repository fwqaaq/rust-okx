//! Async Rust client for the [OKX v5 REST API](https://www.okx.com/docs-v5/en/).
//!
//! The crate is built around three layers that map cleanly onto the OKX request
//! lifecycle:
//!
//! 1. **Request building** — typed request/response models live under [`api`].
//! 2. **Authentication** — credentials and HMAC-SHA256 request signing.
//! 3. **Transport** — sending raw HTTP. The [`Transport`] trait abstracts this
//!    so the default [`ReqwestTransport`] can be swapped for a custom or mock
//!    implementation without changing any calling code.
//!
//! # Example
//!
//! ```no_run
//! # #[cfg(feature = "reqwest")]
//! # async fn run() -> Result<(), rust_okx::Error> {
//! use rust_okx::{Credentials, OkxClient, api::market::InstIdRequest};
//!
//! // Public, unauthenticated client.
//! let client = OkxClient::builder().build();
//! let ticker = client
//!     .market()
//!     .get_ticker(&InstIdRequest {
//!         inst_id: "BTC-USDT",
//!     })
//!     .await?;
//! println!("last price: {}", ticker[0].last.as_str());
//!
//! // Authenticated client.
//! let creds = Credentials::new("key", "secret", "passphrase");
//! let client = OkxClient::builder().credentials(creds).build();
//! let balance = client.account().get_balance(rust_okx::api::account::BalanceRequest::default()).await?;
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "reqwest"))]
//! # fn run() {}
//! ```
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod api;
mod client;
mod credentials;
mod error;
pub mod model;
mod signing;
#[cfg(test)]
mod test_util;
pub mod transport;
#[cfg(feature = "websocket")]
pub mod ws;

pub use client::{OkxClient, OkxClientBuilder};
pub use credentials::Credentials;
#[cfg(feature = "websocket")]
pub use error::WsError;
pub use error::{Error, RestError, Result};
pub use model::NumberString;
#[cfg(feature = "reqwest")]
pub use transport::ReqwestTransport;
pub use transport::{Transport, TransportError};
#[cfg(feature = "websocket")]
pub use ws::{
    Arg, OkxWs, OkxWsBuilder, Push, WsChannelConnectionCount, WsChannelGroup, WsConn, WsConnector,
    WsEvent, WsFrame, WsNotice, WsOperation,
};

/// Global OKX REST API base URL.
pub const GLOBAL_API_URL: &str = "https://www.okx.com";

/// US and AU OKX REST API base URL.
pub const US_API_URL: &str = "https://us.okx.com";

/// EEA OKX REST API base URL.
pub const EEA_API_URL: &str = "https://eea.okx.com";

/// Default global OKX REST API base URL.
///
/// This alias is retained for compatibility. Use [`OkxRegion`] with
/// [`OkxClientBuilder::region`] when building a client for a regional account.
pub const API_URL: &str = GLOBAL_API_URL;

/// OKX REST API region.
///
/// Regional accounts must use the matching API domain. US and AU users
/// registered on `app.okx.com` should use [`OkxRegion::Us`]. EU users
/// registered on `my.okx.com` should use [`OkxRegion::Eea`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OkxRegion {
    /// Global OKX REST API domain.
    Global,
    /// US and AU OKX REST API domain.
    Us,
    /// EEA OKX REST API domain.
    Eea,
}

impl OkxRegion {
    /// Return the REST API base URL for this region.
    pub const fn api_url(self) -> &'static str {
        match self {
            Self::Global => GLOBAL_API_URL,
            Self::Us => US_API_URL,
            Self::Eea => EEA_API_URL,
        }
    }
}
