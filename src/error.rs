//! Error types returned by the crate.

use crate::model::RequestValidationError;
use crate::transport::TransportError;

/// A boxed, thread-safe error used as the `source` of opaque variants so that
/// concrete dependency error types (e.g. `serde_json::Error`) do not leak into
/// this crate's public API.
type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Errors that can occur while making an OKX API request.
///
/// The enum is [`#[non_exhaustive]`](https://doc.rust-lang.org/reference/attributes/type_system.html)
/// so new variants can be added without a breaking change. Downstream code can
/// match on [`Error::Api`] and inspect the OKX error `code` (kept as a `String`,
/// since OKX codes are not guaranteed to be parseable integers).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The underlying HTTP transport failed (connection, TLS, timeout, …).
    #[error("transport error: {0}")]
    Transport(#[from] TransportError),

    /// The typed request failed client-side validation before it was sent.
    #[error("invalid request: {0}")]
    InvalidRequest(#[from] RequestValidationError),

    /// The request could not be encoded (query string or JSON body).
    #[error("failed to encode request: {0}")]
    Encode(#[source] BoxError),

    /// The response body could not be decoded into the expected model.
    #[error("failed to decode response: {0}")]
    Decode(#[source] BoxError),

    /// OKX returned a non-zero response code.
    #[error("OKX API error {code}: {message}")]
    Api {
        /// OKX error code (e.g. `"50011"`). Kept as a string deliberately.
        code: String,
        /// Human-readable error message from OKX.
        message: String,
    },

    /// The HTTP response had a non-success status before the OKX response
    /// envelope could be decoded.
    #[error("HTTP status {status}: {body}")]
    HttpStatus {
        /// HTTP response status code.
        status: http::StatusCode,
        /// Response body, decoded lossily as UTF-8 for diagnostics.
        body: String,
    },

    /// The client was used in an invalid way (e.g. an authenticated endpoint
    /// was called without credentials).
    #[error("invalid configuration: {0}")]
    Configuration(String),
}

impl Error {
    /// Construct an [`Error::Encode`] from any boxable error.
    pub(crate) fn encode(err: impl Into<BoxError>) -> Self {
        Error::Encode(err.into())
    }

    /// Construct an [`Error::Decode`] from any boxable error.
    pub(crate) fn decode(err: impl Into<BoxError>) -> Self {
        Error::Decode(err.into())
    }
}
