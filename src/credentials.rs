//! API credentials.

use std::fmt;

/// OKX API credentials: an API key, secret key, and passphrase.
///
/// The secret key and passphrase are redacted from the [`Debug`] output to
/// avoid accidental disclosure in logs.
#[derive(Clone)]
pub struct Credentials {
    api_key: String,
    secret_key: String,
    passphrase: String,
}

impl Credentials {
    /// Create a new set of credentials.
    pub fn new(
        api_key: impl Into<String>,
        secret_key: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            passphrase: passphrase.into(),
        }
    }

    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }

    pub(crate) fn secret_key(&self) -> &str {
        &self.secret_key
    }

    pub(crate) fn passphrase(&self) -> &str {
        &self.passphrase
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("api_key", &self.api_key)
            .field("secret_key", &"<redacted>")
            .field("passphrase", &"<redacted>")
            .finish()
    }
}
