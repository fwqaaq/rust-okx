//! Public trading-statistics endpoints (`/api/v5/rubik/stat/...`).

mod api;
mod endpoints;
mod requests;
mod responses;

pub use api::*;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
