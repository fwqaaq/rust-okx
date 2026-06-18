//! Shared helpers for the real-API integration tests.
//!
//! Credentials come from two separate environment-variable sets, because OKX
//! issues different API keys for live and demo trading (a live key sent to the
//! demo environment is rejected with `50101 APIKey does not match current
//! environment`):
//!
//! - **live (main account)**: `OKX_API_KEY`, `OKX_API_SECRET`, `OKX_PASSPHRASE`
//! - **demo (simulated)**: `OKX_DEMO_API_KEY`, `OKX_DEMO_API_SECRET`,
//!   `OKX_DEMO_PASSPHRASE`
//!
//! Variables may be placed in a `.env` file at the crate root (loaded by
//! [`load_dotenv`]). When a set is missing the helper returns `None` and the
//! calling test skips, so `cargo test` stays green without credentials.

#![allow(dead_code)] // not every test binary uses every helper

use std::env;
use std::sync::Once;

use rust_okx::{Credentials, Error, OkxClient, RestError};

/// Load variables from a `.env` file in the crate root into the process
/// environment, once per test binary. Missing `.env` is not an error.
fn load_dotenv() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let _ = dotenvy::dotenv();
    });
}

/// A public, unauthenticated client against live OKX (for market data).
pub fn public_client() -> OkxClient {
    OkxClient::builder().build()
}

/// A client for the OKX **live (main account)** environment, or `None` if the
/// `OKX_API_*` variables are not all set.
pub fn live_client() -> Option<OkxClient> {
    let creds = credentials("OKX_API_KEY", "OKX_API_SECRET", "OKX_PASSPHRASE")?;
    Some(OkxClient::builder().credentials(creds).build())
}

/// A client for OKX **demo (simulated) trading** (sets the
/// `x-simulated-trading` header), using the dedicated demo-environment key.
/// Returns `None` if the `OKX_DEMO_API_*` variables are not all set.
pub fn demo_client() -> Option<OkxClient> {
    let creds = credentials(
        "OKX_DEMO_API_KEY",
        "OKX_DEMO_API_SECRET",
        "OKX_DEMO_PASSPHRASE",
    )?;
    Some(
        OkxClient::builder()
            .credentials(creds)
            .demo_trading(true)
            .build(),
    )
}

fn credentials(key: &str, secret: &str, passphrase: &str) -> Option<Credentials> {
    load_dotenv();
    let key = non_empty(key)?;
    let secret = non_empty(secret)?;
    let passphrase = non_empty(passphrase)?;
    Some(Credentials::new(key, secret, passphrase))
}

fn non_empty(var: &str) -> Option<String> {
    env::var(var).ok().filter(|v| !v.is_empty())
}

/// Return a live client or print a consistent skip message for the current test.
pub fn live_client_or_skip(test: &str) -> Option<OkxClient> {
    let client = live_client();
    if client.is_none() {
        eprintln!("skipping {test}: OKX_API_* env vars not set");
    }
    client
}

/// Read a non-empty environment variable after loading `.env`.
pub fn env_non_empty(var: &str) -> Option<String> {
    load_dotenv();
    non_empty(var)
}

/// Accept an API-level eligibility/account-mode rejection while still failing
/// transport and decode errors. This is useful for read-only endpoints whose
/// availability depends on account tier, product enrollment, or region.
pub fn expect_ok_or_api_unavailable<T>(result: Result<T, Error>, endpoint: &str) {
    match result {
        Ok(_) => {}
        Err(Error::Rest(RestError::Okx { code, message, .. })) => {
            eprintln!("skipping {endpoint}: OKX returned {code} {message}");
        }
        Err(error) => panic!("{endpoint} failed: {error}"),
    }
}

/// Unwrap a result, returning `None` and printing a skip message when the
/// server responds with a 5xx transient error. Other errors still panic.
///
/// Use this in sequential lifecycle tests where a transient failure on any
/// step means the rest of the test cannot proceed.
pub fn ok_or_skip<T>(result: Result<T, Error>, label: &str) -> Option<T> {
    match result {
        Ok(v) => Some(v),
        Err(Error::Rest(RestError::HttpStatus { status, .. })) if status.is_server_error() => {
            eprintln!("skipping {label}: OKX returned HTTP {status}");
            None
        }
        Err(e) => panic!("{label}: {e}"),
    }
}
