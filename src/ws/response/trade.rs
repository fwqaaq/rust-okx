//! Typed aliases for Trade WebSocket operation responses.

use crate::ws::model::{
    AmendOrderResult, CancelOrderResult, MassCancelOperationResult, PlaceOrderResult,
};

use super::OperationResponse;

/// Response returned by `order` and `batch-orders`.
pub type PlaceOrderResponse = OperationResponse<PlaceOrderResult>;
/// Response returned by `cancel-order` and `batch-cancel-orders`.
pub type CancelOrderResponse = OperationResponse<CancelOrderResult>;
/// Response returned by `amend-order` and `batch-amend-orders`.
pub type AmendOrderResponse = OperationResponse<AmendOrderResult>;
/// Response returned by `mass-cancel`.
pub type MassCancelOrdersResponse = OperationResponse<MassCancelOperationResult>;
