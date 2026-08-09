//! Copy Trading endpoints (`/api/v5/copytrading/*`).

mod api;
mod endpoints;
mod requests;
mod responses;

pub use api::*;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
