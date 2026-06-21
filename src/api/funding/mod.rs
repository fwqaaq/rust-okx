//! Authenticated funding endpoints (`/api/v5/assert/*`).

mod api;
mod endpoints;
mod requests;
mod responses;

pub use api::*;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
