//! The HTTP transport abstraction.
//!
//! [`Transport`] is intentionally minimal: it knows nothing about OKX
//! authentication, endpoints, or response envelopes. It only sends a fully
//! built [`http::Request`] and returns the raw [`http::Response`]. This keeps
//! custom and mock implementations trivial, and lets cross-cutting concerns
//! (retries, logging, rate limiting) be added as wrapper types instead of new
//! trait methods.
//!
//! The trait uses return-position `impl Future` (no `async_trait`), so calling
//! it incurs no boxing or dynamic dispatch — the [`OkxClient`](crate::OkxClient)
//! is generic over `T: Transport`.

use std::error::Error as StdError;
use std::fmt;
use std::future::Future;

use bytes::Bytes;

#[cfg(feature = "reqwest")]
mod reqwest_impl;
#[cfg(feature = "reqwest")]
pub use reqwest_impl::ReqwestTransport;

/// The default transport type used as the `OkxClient<T>` type parameter.
///
/// When the `reqwest` feature is enabled this is [`ReqwestTransport`]. Otherwise
/// it is a placeholder that does not implement [`Transport`]; you must then
/// supply your own transport via [`OkxClient::with_transport`](crate::OkxClient::with_transport).
#[cfg(feature = "reqwest")]
pub type DefaultTransport = ReqwestTransport;

/// See [`DefaultTransport`].
#[cfg(not(feature = "reqwest"))]
pub type DefaultTransport = UnconfiguredTransport;

/// Placeholder used as the default `OkxClient` transport when no built-in
/// transport feature is enabled. It does not implement [`Transport`].
#[cfg(not(feature = "reqwest"))]
#[derive(Debug, Clone, Copy, Default)]
pub struct UnconfiguredTransport;

type BoxError = Box<dyn StdError + Send + Sync>;

/// An opaque transport-layer error.
///
/// The concrete source error (e.g. `reqwest::Error`) is boxed so it does not
/// leak into this crate's public API.
#[derive(Debug)]
pub struct TransportError(BoxError);

impl TransportError {
    /// Wrap any boxable error as a [`TransportError`].
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self(error.into())
    }

    /// Create a [`TransportError`] from a message string.
    pub fn message(message: impl Into<String>) -> Self {
        Self(message.into().into())
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;

        let mut source = self.0.source();
        while let Some(error) = source {
            write!(f, ": {error}")?;
            source = error.source();
        }

        Ok(())
    }
}

impl StdError for TransportError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.0.as_ref())
    }
}

/// Sends HTTP requests on behalf of the client.
///
/// Implementors receive a complete request (URL, method, headers, and body
/// already set, including authentication headers) and return the raw response.
///
/// # Example
///
/// ```
/// use bytes::Bytes;
/// use rust_okx::{Transport, TransportError};
///
/// #[derive(Clone)]
/// struct AlwaysOk;
///
/// impl Transport for AlwaysOk {
///     fn send(
///         &self,
///         _request: http::Request<Bytes>,
///     ) -> impl std::future::Future<Output = Result<http::Response<Bytes>, TransportError>> + Send
///     {
///         async move {
///             Ok(http::Response::builder()
///                 .status(200)
///                 .body(Bytes::from_static(b"{\"code\":\"0\",\"msg\":\"\",\"data\":[]}"))
///                 .unwrap())
///         }
///     }
/// }
/// ```
pub trait Transport: Send + Sync {
    /// Send a request and return the response.
    fn send(
        &self,
        request: http::Request<Bytes>,
    ) -> impl Future<Output = Result<http::Response<Bytes>, TransportError>> + Send;
}
