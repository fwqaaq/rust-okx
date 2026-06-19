//! Test-only helpers: an offline [`Transport`] that records the request it was
//! given and replays a canned response.

use std::sync::{Arc, Mutex};

use bytes::Bytes;
use http::{HeaderMap, Method};

use crate::transport::{Transport, TransportError};

/// A captured copy of the request a [`MockTransport`] received.
#[derive(Debug, Clone, Default)]
pub(crate) struct CapturedRequest {
    pub method: Method,
    pub uri: String,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl CapturedRequest {
    /// The query string portion of the URI, if any.
    pub fn query(&self) -> Option<&str> {
        self.uri.split_once('?').map(|(_, q)| q)
    }

    /// The request body decoded as UTF-8.
    pub fn body_str(&self) -> &str {
        std::str::from_utf8(&self.body).unwrap_or_default()
    }

    /// Whether the OKX signature headers are present.
    pub fn is_signed(&self) -> bool {
        [
            "ok-access-key",
            "ok-access-sign",
            "ok-access-timestamp",
            "ok-access-passphrase",
        ]
        .iter()
        .all(|h| self.headers.contains_key(*h))
    }
}

/// A [`Transport`] that captures the outgoing request and returns a fixed body.
#[derive(Clone)]
pub(crate) struct MockTransport {
    status: u16,
    response: Bytes,
    captured: Arc<Mutex<Option<CapturedRequest>>>,
}

impl MockTransport {
    /// A transport that returns `body` with HTTP 200.
    pub fn new(body: &str) -> Self {
        Self {
            status: 200,
            response: Bytes::copy_from_slice(body.as_bytes()),
            captured: Arc::new(Mutex::new(None)),
        }
    }

    /// The request that was sent. Panics if nothing was sent.
    pub fn captured(&self) -> CapturedRequest {
        self.captured
            .lock()
            .unwrap()
            .clone()
            .expect("no request was sent through the mock transport")
    }
}

impl Transport for MockTransport {
    fn send(
        &self,
        request: http::Request<Bytes>,
    ) -> impl std::future::Future<Output = Result<http::Response<Bytes>, TransportError>> + Send
    {
        let (parts, body) = request.into_parts();
        *self.captured.lock().unwrap() = Some(CapturedRequest {
            method: parts.method,
            uri: parts.uri.to_string(),
            headers: parts.headers,
            body,
        });
        let response = http::Response::builder()
            .status(self.status)
            .body(self.response.clone())
            .expect("valid mock response");
        std::future::ready(Ok(response))
    }
}
