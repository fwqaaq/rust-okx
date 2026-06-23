//! Support and announcement endpoints (`/api/v5/support/*`).

mod api;
mod endpoints;
mod requests;
mod responses;

pub use api::*;
pub use requests::*;
pub use responses::*;

#[cfg(test)]
mod tests;
