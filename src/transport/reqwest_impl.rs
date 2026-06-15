//! Default [`Transport`] implementation for the built-in HTTP client.

use std::future::Future;

use bytes::Bytes;

use super::{Transport, TransportError};

/// The default HTTP [`Transport`].
///
/// This is the default transport when the `reqwest` feature is enabled.
#[derive(Debug, Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}

impl ReqwestTransport {
    /// Create a transport with a default [`reqwest::Client`].
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

}

impl Default for ReqwestTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for ReqwestTransport {
    fn send(
        &self,
        request: http::Request<Bytes>,
    ) -> impl Future<Output = Result<http::Response<Bytes>, TransportError>> + Send {
        let client = self.client.clone();
        async move {
            let (parts, body) = request.into_parts();
            let response = client
                .request(parts.method, parts.uri.to_string())
                .headers(parts.headers)
                .body(body)
                .send()
                .await
                .map_err(TransportError::new)?;

            let status = response.status();
            let headers = response.headers().clone();
            let body = response.bytes().await.map_err(TransportError::new)?;

            let mut builder = http::Response::builder().status(status);
            if let Some(out_headers) = builder.headers_mut() {
                *out_headers = headers;
            }
            builder.body(body).map_err(TransportError::new)
        }
    }
}
