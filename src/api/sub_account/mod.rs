//! Sub-account management endpoints.
//!
//! Spans three path prefixes:
//! - `GET/POST /api/v5/users/subaccount/...` — sub-account lifecycle and API keys
//! - `GET/POST /api/v5/account/subaccount/...` — trading-account operations
//! - `GET/POST /api/v5/asset/subaccount/...` — funding-account operations

mod api;
mod endpoints;
mod requests;
mod responses;

pub use api::*;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
