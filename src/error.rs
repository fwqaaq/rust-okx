//! Error types returned by the crate.

use crate::transport::TransportError;
#[cfg(feature = "websocket")]
pub use crate::ws::WsError;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// A type alias for [`std::result::Result`] using this crate's [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Top-level error type.
///
/// Wraps [`RestError`] (REST layer), optionally [`WsError`] (WebSocket layer),
/// and [`RequestValidationError`] (shared pre-flight validation).
///
/// The enum is [`#[non_exhaustive]`](https://doc.rust-lang.org/reference/attributes/type_system.html)
/// so new variants can be added without a breaking change.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An error from the REST API layer.
    #[error(transparent)]
    Rest(#[from] RestError),

    /// An error from the WebSocket layer.
    #[cfg(feature = "websocket")]
    #[error(transparent)]
    Ws(#[from] WsError),
}

/// Errors from the REST API layer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RestError {
    /// The underlying HTTP transport failed (connection, TLS, timeout, …).
    #[error("transport error: {source}")]
    Transport {
        /// Finding the position of the error
        #[from]
        source: TransportError,
    },

    /// The HTTP response had a non-success status before the OKX envelope
    /// could be decoded.
    #[error("HTTP {status} from {endpoint}")]
    HttpStatus {
        /// The endpoint path, e.g. `/api/v5/account/balance`.
        endpoint: &'static str,
        /// HTTP response status code.
        status: http::StatusCode,
        /// Response body, decoded lossily as UTF-8 for diagnostics.
        body: String,
    },

    /// OKX returned a non-zero response code.
    #[error("OKX API error {code} from {endpoint}: {message}")]
    Okx {
        /// The endpoint path.
        endpoint: &'static str,
        /// OKX error code (e.g. `"50011"`). Kept as a string deliberately.
        code: String,
        /// Human-readable error message from OKX.
        message: String,
    },

    /// The response body could not be decoded into the expected model.
    #[error("failed to decode response from {endpoint}")]
    Decode {
        /// The endpoint path.
        endpoint: &'static str,
        /// The underlying deserialization error.
        #[source]
        source: serde_json::Error,
    },

    /// The request could not be encoded (query string, JSON body, or headers).
    #[error("failed to encode request")]
    Encode {
        /// The underlying encoding error.
        #[source]
        source: BoxError,
    },

    /// The client was used in an invalid way (e.g. an authenticated endpoint
    /// was called without credentials).
    #[error("invalid configuration: {0}")]
    Configuration(String),
}
