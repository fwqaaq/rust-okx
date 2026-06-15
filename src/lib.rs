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
//! use rust_okx::{Credentials, OkxClient};
//!
//! // Public, unauthenticated client.
//! let client = OkxClient::builder().build();
//! let ticker = client.market().get_ticker("BTC-USDT").await?;
//! println!("last price: {}", ticker[0].last.as_str());
//!
//! // Authenticated client.
//! let creds = Credentials::new("key", "secret", "passphrase");
//! let client = OkxClient::builder().credentials(creds).build();
//! let balance = client.account().get_balance(None).await?;
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

pub use client::{OkxClient, OkxClientBuilder};
pub use credentials::Credentials;
pub use error::Error;
pub use model::NumberString;
#[cfg(feature = "reqwest")]
pub use transport::ReqwestTransport;
pub use transport::{Transport, TransportError};

/// Default OKX REST API base URL.
pub const API_URL: &str = "https://www.okx.com";
