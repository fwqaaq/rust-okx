//! Spread Trading WebSocket trade request models.

use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, at_least_one, non_empty, one_of,
    optional_non_empty, positive_decimal_string, validate_client_request_id,
};

fn optional_signed_decimal_string(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), RequestValidationError> {
    let Some(value) = value else {
        return Ok(());
    };
    non_empty(field, value)?;

    let unsigned = value.strip_prefix('-').unwrap_or(value);
    let mut dot_seen = false;
    let mut digit_seen = false;
    for byte in unsigned.bytes() {
        match byte {
            b'0'..=b'9' => digit_seen = true,
            b'.' if !dot_seen => dot_seen = true,
            _ => {
                return Err(RequestValidationError::InvalidFormat {
                    field,
                    expected: "a signed decimal string without exponent notation",
                });
            }
        }
    }
    if !digit_seen || unsigned.starts_with('.') || unsigned.ends_with('.') {
        return Err(RequestValidationError::InvalidFormat {
            field,
            expected: "a signed decimal string without exponent notation",
        });
    }
    Ok(())
}

/// Place-spread-order request body (`sprd-order`).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-place-order>
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlaceSpreadOrderRequest {
    /// Spread ID.
    pub sprd_id: String,
    /// Client order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Order side: `buy` or `sell`.
    pub side: String,
    /// Spread order type: `limit`, `post_only`, or `ioc`.
    pub ord_type: String,
    /// Order quantity.
    pub sz: String,
    /// Order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
}

impl PlaceSpreadOrderRequest {
    /// Create a spread-order request with the documented required fields.
    pub fn new(
        sprd_id: impl Into<String>,
        side: impl Into<String>,
        ord_type: impl Into<String>,
        size: impl Into<String>,
    ) -> Self {
        Self {
            sprd_id: sprd_id.into(),
            cl_ord_id: None,
            tag: None,
            side: side.into(),
            ord_type: ord_type.into(),
            sz: size.into(),
            px: None,
        }
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, cl_ord_id: impl Into<String>) -> Self {
        self.cl_ord_id = Some(cl_ord_id.into());
        self
    }

    /// Set the order tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Set the order price.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.px = Some(price.into());
        self
    }
}

impl ValidateRequest for PlaceSpreadOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("sprdId", &self.sprd_id)?;
        validate_client_request_id("clOrdId", self.cl_ord_id.as_deref())?;
        optional_non_empty("tag", self.tag.as_deref())?;
        one_of("side", &self.side, &["buy", "sell"], "buy or sell")?;
        one_of(
            "ordType",
            &self.ord_type,
            &["limit", "post_only", "ioc"],
            "limit, post_only, or ioc",
        )?;
        positive_decimal_string("sz", &self.sz)?;
        optional_signed_decimal_string("px", self.px.as_deref())?;
        Ok(())
    }
}

/// Amend-spread-order request body (`sprd-amend-order`).
///
/// Either `ordId` or `clOrdId` is required; when both are supplied OKX uses
/// `ordId`. At least one of `newSz` or `newPx` must be supplied.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-amend-order>
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AmendSpreadOrderRequest {
    /// OKX order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Client amendment request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// New total order quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,
    /// New order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_px: Option<String>,
}

impl AmendSpreadOrderRequest {
    /// Identify the spread order by OKX order ID.
    pub fn by_order_id(ord_id: impl Into<String>) -> Self {
        Self {
            ord_id: Some(ord_id.into()),
            ..Self::default()
        }
    }

    /// Identify the spread order by client order ID.
    pub fn by_client_order_id(cl_ord_id: impl Into<String>) -> Self {
        Self {
            cl_ord_id: Some(cl_ord_id.into()),
            ..Self::default()
        }
    }

    /// Set the client amendment request ID.
    pub fn request_id(mut self, req_id: impl Into<String>) -> Self {
        self.req_id = Some(req_id.into());
        self
    }

    /// Set the new total order size.
    pub fn new_size(mut self, size: impl Into<String>) -> Self {
        self.new_sz = Some(size.into());
        self
    }

    /// Set the new order price.
    pub fn new_price(mut self, price: impl Into<String>) -> Self {
        self.new_px = Some(price.into());
        self
    }
}

impl ValidateRequest for AmendSpreadOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        at_least_one("ordId, clOrdId", &[self.ord_id.is_some(), self.cl_ord_id.is_some()])?;
        optional_non_empty("ordId", self.ord_id.as_deref())?;
        validate_client_request_id("clOrdId", self.cl_ord_id.as_deref())?;
        validate_client_request_id("reqId", self.req_id.as_deref())?;
        at_least_one("newSz, newPx", &[self.new_sz.is_some(), self.new_px.is_some()])?;
        if let Some(value) = self.new_sz.as_deref() {
            positive_decimal_string("newSz", value)?;
        }
        optional_signed_decimal_string("newPx", self.new_px.as_deref())?;
        Ok(())
    }
}

/// Cancel-spread-order request body (`sprd-cancel-order`).
///
/// Either `ordId` or `clOrdId` is required; when both are supplied OKX uses
/// `ordId`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-order>
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelSpreadOrderRequest {
    /// OKX order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

impl CancelSpreadOrderRequest {
    /// Identify the spread order by OKX order ID.
    pub fn by_order_id(ord_id: impl Into<String>) -> Self {
        Self {
            ord_id: Some(ord_id.into()),
            cl_ord_id: None,
        }
    }

    /// Identify the spread order by client order ID.
    pub fn by_client_order_id(cl_ord_id: impl Into<String>) -> Self {
        Self {
            ord_id: None,
            cl_ord_id: Some(cl_ord_id.into()),
        }
    }
}

impl ValidateRequest for CancelSpreadOrderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        at_least_one("ordId, clOrdId", &[self.ord_id.is_some(), self.cl_ord_id.is_some()])?;
        optional_non_empty("ordId", self.ord_id.as_deref())?;
        validate_client_request_id("clOrdId", self.cl_ord_id.as_deref())?;
        Ok(())
    }
}

/// Cancel-all-spread-orders request body (`sprd-mass-cancel`).
///
/// When `sprdId` is omitted, OKX cancels pending orders across all spreads.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-all-orders>
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MassCancelSpreadOrdersRequest {
    /// Optional spread ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
}

impl MassCancelSpreadOrdersRequest {
    /// Cancel pending orders across all spreads.
    pub fn all() -> Self {
        Self::default()
    }

    /// Cancel pending orders for one spread only.
    pub fn for_spread(sprd_id: impl Into<String>) -> Self {
        Self {
            sprd_id: Some(sprd_id.into()),
        }
    }
}

impl ValidateRequest for MassCancelSpreadOrdersRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("sprdId", self.sprd_id.as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_documented_place_order_example() {
        let request = PlaceSpreadOrderRequest::new(
            "BTC-USDT_BTC-USDT-SWAP",
            "buy",
            "limit",
            "2",
        )
        .client_order_id("b15")
        .price("2.15");
        request.validate().unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["sprdId"], "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(value["ordType"], "limit");
        assert_eq!(value["px"], "2.15");
    }

    #[test]
    fn rejects_incomplete_amend_request() {
        let error = AmendSpreadOrderRequest::by_order_id("1").validate().unwrap_err();
        assert!(matches!(error, RequestValidationError::AtLeastOneRequired { .. }));
    }

    #[test]
    fn accepts_negative_spread_prices() {
        PlaceSpreadOrderRequest::new("A_B", "sell", "limit", "1")
            .price("-2.5")
            .validate()
            .unwrap();
        AmendSpreadOrderRequest::by_order_id("1")
            .new_price("-1.25")
            .validate()
            .unwrap();
    }

    #[test]
    fn mass_cancel_all_serializes_as_empty_object() {
        let value = serde_json::to_value(MassCancelSpreadOrdersRequest::all()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }
}
