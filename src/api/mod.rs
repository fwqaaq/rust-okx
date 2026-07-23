//! Typed request/response models and endpoint methods, grouped by OKX API area.
//!
//! Each submodule exposes an accessor type (e.g. [`market::Market`]) reachable
//! from the [`OkxClient`](crate::OkxClient), plus the request and response
//! models for that area.

pub mod account;
pub mod convert;
pub mod fiat;
pub mod finance;
pub mod funding;
pub mod market;
pub mod public_data;
pub mod status;
pub mod sub_account;
pub mod support;
pub mod trade;
pub mod trading_data;
