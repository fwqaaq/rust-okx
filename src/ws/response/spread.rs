//! Typed aliases for Spread Trading WebSocket operation responses.

use crate::ws::model::{
    SpreadAmendOrderResult, SpreadCancelOrderResult, SpreadMassCancelResult,
    SpreadPlaceOrderResult,
};

use super::OperationResponse;

/// Response returned by `sprd-order`.
pub type PlaceSpreadOrderResponse = OperationResponse<SpreadPlaceOrderResult>;
/// Response returned by `sprd-amend-order`.
pub type AmendSpreadOrderResponse = OperationResponse<SpreadAmendOrderResult>;
/// Response returned by `sprd-cancel-order`.
pub type CancelSpreadOrderResponse = OperationResponse<SpreadCancelOrderResult>;
/// Response returned by `sprd-mass-cancel`.
pub type MassCancelSpreadOrdersResponse = OperationResponse<SpreadMassCancelResult>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mass_cancel_response() {
        let response: MassCancelSpreadOrdersResponse = serde_json::from_str(
            r#"{"id":"1","op":"sprd-mass-cancel","code":"0","data":[{"result":true}]}"#,
        )
        .unwrap();
        assert!(response.data[0].result);
    }
}
