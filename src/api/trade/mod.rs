//! Authenticated trading endpoints (`/api/v5/trade/*`).

mod api;
mod endpoints;
mod internal;
mod requests;
mod responses;

pub use api::*;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
