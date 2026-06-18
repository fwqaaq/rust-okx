//! Typed OKX WebSocket request bodies.
//!
//! Models are grouped by envelope and API family while preserving the original
//! flat `crate::ws::request::*` imports.

mod envelope;
mod spread;
mod trade;

pub use envelope::{ChannelRequest, LoginArg, LoginRequest, OperationRequest};
pub use spread::{
    AmendSpreadOrderRequest, CancelSpreadOrderRequest, MassCancelSpreadOrdersRequest,
    PlaceSpreadOrderRequest,
};
pub use trade::MassCancelRequest;
