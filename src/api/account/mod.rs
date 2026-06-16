//! Authenticated trading-account endpoints (`/api/v5/account/*`).

mod api;
mod endpoints;
mod internal;
mod requests;
mod responses;

pub use api::Account;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
