//! WebSocket channel argument helpers.
//!
//! Each helper links to the matching OKX channel definition so the required
//! filters and the response model can be checked against the upstream schema.

pub mod market;
pub mod public_data;
pub mod account;
pub mod trade;
pub mod algo;
pub mod grid;
pub mod block;
pub mod spread;
pub mod funding;
pub mod status;
