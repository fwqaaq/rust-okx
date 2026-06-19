//! Spread Trading WebSocket trade request models.

use serde::Serialize;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_documented_place_order_example() {
        let request = PlaceSpreadOrderRequest::new("BTC-USDT_BTC-USDT-SWAP", "buy", "limit", "2")
            .client_order_id("b15")
            .price("2.15");
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["sprdId"], "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(value["ordType"], "limit");
        assert_eq!(value["px"], "2.15");
    }

    #[test]
    fn mass_cancel_all_serializes_as_empty_object() {
        let value = serde_json::to_value(MassCancelSpreadOrdersRequest::all()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }
}
