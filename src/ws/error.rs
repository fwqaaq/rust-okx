//! WebSocket-layer error type.

use crate::transport::TransportError;

/// Errors from the WebSocket layer.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WsError {
    /// A WebSocket transport error (connection, TLS, I/O, …).
    #[error("WebSocket transport error: {source}")]
    Transport {
        /// Finding the position of the error
        #[from]
        source: TransportError,
    },

    /// A WebSocket event or push payload could not be decoded.
    #[error("failed to decode WebSocket message ({context})")]
    Decode {
        /// Human-readable context: channel name, event type, or a snippet of
        /// the raw payload.
        context: String,
        /// The underlying deserialization error.
        #[source]
        source: serde_json::Error,
    },

    /// A push event was missing the required `arg` field.
    #[error("missing `arg` field in WebSocket event: {raw}")]
    MissingArg {
        /// The raw JSON text of the offending event (truncated if large).
        raw: String,
    },

    /// An outgoing WebSocket request could not be serialized.
    #[error("failed to encode WebSocket request")]
    Encode {
        /// The underlying serialization error.
        #[source]
        source: serde_json::Error,
    },

    /// The WebSocket client was used in an invalid way (wrong channel group,
    /// missing credentials, …).
    #[error("invalid configuration: {0}")]
    Configuration(String),
}
