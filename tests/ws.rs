//! Private WebSocket channel integration tests.
//!
//! Run all with:
//! ```
//! OKX_DEMO=1 cargo test -F websocket --test ws -- --ignored
//! ```
//!
//! Required env vars (or `.env` file): `OKX_API_KEY`, `OKX_API_SECRET`, `OKX_PASSPHRASE`.
//! `OKX_DEMO` defaults to `1` (demo trading) for safety.

#![cfg(feature = "websocket")]

#[path = "ws/account.rs"]
mod account;
#[path = "ws/balance_and_position.rs"]
mod balance_and_position;
#[path = "ws/common.rs"]
mod common;
#[path = "ws/positions.rs"]
mod positions;
