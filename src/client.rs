//! The OKX client and its builder.

use bytes::Bytes;
use http::Method;
use http::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::OkxRegion;
use crate::api::account::Account;
use crate::api::convert::Convert;
use crate::api::finance::Finance;
use crate::api::funding::Funding;
use crate::api::market::Market;
use crate::api::public_data::PublicData;
use crate::api::sub_account::SubAccount;
use crate::api::trade::Trade;
use crate::credentials::Credentials;
use crate::error::{Error, RestError};
use crate::model::OkxResponse;
use crate::signing;
use crate::transport::{DefaultTransport, Transport};

/// An OKX v5 REST API client, generic over the HTTP [`Transport`].
///
/// Construct one with [`OkxClient::builder`] (uses the default
/// [`ReqwestTransport`](crate::ReqwestTransport)) or [`OkxClient::with_transport`]
/// for a custom transport. The client is cheap to share behind an `Arc` and all
/// request methods take `&self`.
///
/// API groups are reached through accessor methods: [`market`](Self::market),
/// [`public_data`](Self::public_data), [`account`](Self::account),
/// [`funding`](Self::funding), [`convert`](Self::convert),
/// [`finance`](Self::finance), and [`trade`](Self::trade).
pub struct OkxClient<T = DefaultTransport> {
    transport: T,
    credentials: Option<Credentials>,
    base_url: String,
    demo: bool,
}

#[cfg(feature = "reqwest")]
impl OkxClient {
    /// Start building a client backed by the default
    /// [`ReqwestTransport`](crate::ReqwestTransport).
    pub fn builder() -> OkxClientBuilder<DefaultTransport> {
        OkxClientBuilder::from_transport(crate::transport::ReqwestTransport::new())
    }
}

impl<T> OkxClient<T> {
    /// Start building a client around a custom [`Transport`].
    pub fn with_transport(transport: T) -> OkxClientBuilder<T> {
        OkxClientBuilder::from_transport(transport)
    }
}

impl<T: Transport> OkxClient<T> {
    /// Access the public market-data endpoints.
    pub fn market(&self) -> Market<'_, T> {
        Market::new(self)
    }

    /// Access the public reference-data endpoints.
    pub fn public_data(&self) -> PublicData<'_, T> {
        PublicData::new(self)
    }

    /// Access the (authenticated) account endpoints.
    pub fn account(&self) -> Account<'_, T> {
        Account::new(self)
    }

    /// Access the (authenticated) funding-account and asset endpoints.
    pub fn funding(&self) -> Funding<'_, T> {
        Funding::new(self)
    }

    /// Access the (authenticated) asset conversion endpoints.
    pub fn convert(&self) -> Convert<'_, T> {
        Convert::new(self)
    }

    /// Access the finance endpoints.
    pub fn finance(&self) -> Finance<'_, T> {
        Finance::new(self)
    }

    /// Access the (authenticated) sub-account management endpoints.
    pub fn sub_account(&self) -> SubAccount<'_, T> {
        SubAccount::new(self)
    }

    /// Access the (authenticated) trading endpoints.
    pub fn trade(&self) -> Trade<'_, T> {
        Trade::new(self)
    }

    /// Send a `GET` request, serializing `query` into the URL query string and
    /// returning the deserialized `data` array.
    pub(crate) async fn get<Q, D>(
        &self,
        endpoint: &'static str,
        query: &Q,
        authenticated: bool,
    ) -> Result<D, Error>
    where
        Q: Serialize,
        D: DeserializeOwned,
    {
        let qs = serde_urlencoded::to_string(query)
            .map_err(|e| RestError::Encode { source: e.into() })?;
        let request_path = if qs.is_empty() {
            endpoint.to_owned()
        } else {
            format!("{endpoint}?{qs}")
        };
        self.send(
            endpoint,
            Method::GET,
            &request_path,
            Bytes::new(),
            authenticated,
        )
        .await
    }

    /// Send a `POST` request with `body` serialized as JSON and return the
    /// deserialized `data` array.
    pub(crate) async fn post<B, D>(
        &self,
        endpoint: &'static str,
        body: &B,
        authenticated: bool,
    ) -> Result<D, Error>
    where
        B: Serialize,
        D: DeserializeOwned,
    {
        let body = serde_json::to_vec(body).map_err(|e| RestError::Encode { source: e.into() })?;
        self.send(
            endpoint,
            Method::POST,
            endpoint,
            Bytes::from(body),
            authenticated,
        )
        .await
    }

    async fn send<D>(
        &self,
        endpoint: &'static str,
        method: Method,
        request_path: &str,
        body: Bytes,
        authenticated: bool,
    ) -> Result<D, Error>
    where
        D: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, request_path);
        let mut builder = http::Request::builder().method(method.clone()).uri(url);

        let headers = builder
            .headers_mut()
            .expect("a freshly constructed request builder has no error");
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if self.demo {
            headers.insert(
                HeaderName::from_static("x-simulated-trading"),
                HeaderValue::from_static("1"),
            );
        }
        if authenticated {
            let credentials = self.credentials.as_ref().ok_or_else(|| {
                RestError::Configuration("authenticated endpoint requires credentials".to_owned())
            })?;
            let timestamp = signing::timestamp();
            let body_str = std::str::from_utf8(&body).unwrap_or_default();
            let prehash = signing::pre_hash(&timestamp, method.as_str(), request_path, body_str);
            let signature = signing::sign(&prehash, credentials.secret_key());
            insert_header(headers, "ok-access-key", credentials.api_key())?;
            insert_header(headers, "ok-access-sign", &signature)?;
            insert_header(headers, "ok-access-timestamp", &timestamp)?;
            insert_header(headers, "ok-access-passphrase", credentials.passphrase())?;
        }

        let request = builder
            .body(body)
            .map_err(|e| RestError::Encode { source: e.into() })?;
        let response = self
            .transport
            .send(request)
            .await
            .map_err(RestError::from)?;
        let status = response.status();
        let bytes = response.into_body();
        if !status.is_success() {
            return Err(RestError::HttpStatus {
                endpoint,
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            }
            .into());
        }
        let envelope: OkxResponse<D> =
            serde_json::from_slice(&bytes).map_err(|e| RestError::Decode {
                endpoint,
                source: e,
            })?;
        if envelope.code != "0" {
            return Err(RestError::Okx {
                endpoint,
                code: envelope.code,
                message: envelope.msg,
            }
            .into());
        }
        Ok(envelope.data)
    }
}

fn insert_header(headers: &mut HeaderMap, name: &'static str, value: &str) -> Result<(), Error> {
    let value = HeaderValue::from_str(value)
        .map_err(|e| RestError::Configuration(format!("invalid header value for {name}: {e}")))?;
    headers.insert(HeaderName::from_static(name), value);
    Ok(())
}

/// Builder for [`OkxClient`].
///
/// Created by [`OkxClient::builder`] or [`OkxClient::with_transport`].
pub struct OkxClientBuilder<T = DefaultTransport> {
    transport: T,
    credentials: Option<Credentials>,
    base_url: String,
    demo: bool,
}

impl<T> OkxClientBuilder<T> {
    fn from_transport(transport: T) -> Self {
        Self {
            transport,
            credentials: None,
            base_url: crate::API_URL.to_owned(),
            demo: false,
        }
    }

    /// Set the API credentials used to sign authenticated requests.
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Select the OKX REST API region.
    ///
    /// The default is [`OkxRegion::Global`]. US and AU users registered on
    /// `app.okx.com` should use [`OkxRegion::Us`]. EU users registered on
    /// `my.okx.com` should use [`OkxRegion::Eea`].
    pub fn region(mut self, region: OkxRegion) -> Self {
        self.base_url = region.api_url().to_owned();
        self
    }

    /// Override the API base URL.
    ///
    /// This overrides the default global domain and any region selected through
    /// [`Self::region`].
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Toggle the `x-simulated-trading` header for OKX demo trading.
    pub fn demo_trading(mut self, demo: bool) -> Self {
        self.demo = demo;
        self
    }

    /// Replace the transport, changing the client's transport type.
    pub fn transport<U>(self, transport: U) -> OkxClientBuilder<U> {
        OkxClientBuilder {
            transport,
            credentials: self.credentials,
            base_url: self.base_url,
            demo: self.demo,
        }
    }

    /// Build the [`OkxClient`].
    pub fn build(self) -> OkxClient<T> {
        OkxClient {
            transport: self.transport,
            credentials: self.credentials,
            base_url: self.base_url,
            demo: self.demo,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{Error, RestError};
    use crate::test_util::MockTransport;
    use crate::{OkxClient, OkxRegion};

    /// A non-zero OKX response code is surfaced as [`RestError::Okx`] with the
    /// code and message preserved (offline unit test; network path covered by
    /// integration tests).
    #[tokio::test]
    async fn non_zero_code_is_api_error() {
        let mock = MockTransport::new(r#"{"code":"51000","msg":"Parameter error","data":[]}"#);
        let client = OkxClient::with_transport(mock).build();
        let err = client.market().get_ticker("BAD").await.unwrap_err();
        match err {
            Error::Rest(RestError::Okx { code, message, .. }) => {
                assert_eq!(code, "51000");
                assert_eq!(message, "Parameter error");
            }
            other => panic!("expected Error::Rest(RestError::Okx), got {other:?}"),
        }
    }

    #[test]
    fn okx_region_returns_expected_api_urls() {
        assert_eq!(OkxRegion::Global.api_url(), "https://www.okx.com");
        assert_eq!(OkxRegion::Us.api_url(), "https://us.okx.com");
        assert_eq!(OkxRegion::Eea.api_url(), "https://eea.okx.com");
    }

    #[tokio::test]
    async fn region_sets_request_base_url() {
        let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
        let client = OkxClient::with_transport(mock.clone())
            .region(OkxRegion::Us)
            .build();

        client.market().get_ticker("BTC-USDT").await.unwrap();

        let req = mock.captured();
        assert!(
            req.uri
                .starts_with("https://us.okx.com/api/v5/market/ticker"),
            "unexpected URI: {}",
            req.uri
        );
    }

    #[tokio::test]
    async fn base_url_overrides_selected_region() {
        let mock = MockTransport::new(r#"{"code":"0","msg":"","data":[]}"#);
        let client = OkxClient::with_transport(mock.clone())
            .region(OkxRegion::Eea)
            .base_url("https://example.test")
            .build();

        client.market().get_ticker("BTC-USDT").await.unwrap();

        let req = mock.captured();
        assert!(
            req.uri
                .starts_with("https://example.test/api/v5/market/ticker"),
            "unexpected URI: {}",
            req.uri
        );
    }
}
