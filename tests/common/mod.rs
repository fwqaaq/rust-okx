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

use rust_okx::{Credentials, OkxClient};

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
