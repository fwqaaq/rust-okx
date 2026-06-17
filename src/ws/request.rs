//! Typed OKX WebSocket request bodies.
//!
//! These models cover login/subscription envelopes and the WebSocket-only
//! trading operations which previously accepted an untyped `RequestParams`.

use serde::Serialize;

use super::Arg;

/// Subscribe or unsubscribe request body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct ChannelRequest<'a> {
    /// Optional client message ID (up to 32 case-sensitive alphanumeric characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// `subscribe` or `unsubscribe`.
    pub op: &'a str,
    /// Channel arguments.
    pub args: &'a [Arg],
}

impl<'a> ChannelRequest<'a> {
    /// Build a subscription request.
    pub fn subscribe(args: &'a [Arg]) -> Self {
        Self {
            id: None,
            op: "subscribe",
            args,
        }
    }

    /// Build an unsubscription request.
    pub fn unsubscribe(args: &'a [Arg]) -> Self {
        Self {
            id: None,
            op: "unsubscribe",
            args,
        }
    }

    /// Attach a client message ID.
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
        self
    }
}

/// Private WebSocket login request body.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-login>
#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct LoginRequest<'a> {
    /// Always `login`.
    pub op: &'static str,
    /// OKX requires exactly one login argument.
    pub args: [LoginArg<'a>; 1],
}

impl<'a> LoginRequest<'a> {
    /// Build a login request from one authentication argument.
    pub fn new(arg: LoginArg<'a>) -> Self {
        Self {
            op: "login",
            args: [arg],
        }
    }
}

/// Authentication argument inside [`LoginRequest`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoginArg<'a> {
    /// API key.
    pub api_key: &'a str,
    /// API key passphrase.
    pub passphrase: &'a str,
    /// Unix timestamp in seconds used for signing.
    pub timestamp: &'a str,
    /// Base64-encoded HMAC-SHA256 signature.
    pub sign: String,
}

impl<'a> LoginArg<'a> {
    /// Build a login argument from already-computed authentication values.
    pub fn new(
        api_key: &'a str,
        passphrase: &'a str,
        timestamp: &'a str,
        sign: impl Into<String>,
    ) -> Self {
        Self {
            api_key,
            passphrase,
            timestamp,
            sign: sign.into(),
        }
    }
}

/// Generic WebSocket trade-operation envelope.
///
/// `expTime` is supported by order placement/amendment operations and is omitted
/// when `None`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket>
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OperationRequest<'a, A> {
    /// Client-provided request ID.
    pub id: String,
    /// OKX operation name.
    pub op: String,
    /// Optional request effective deadline in Unix milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    /// Operation arguments.
    pub args: &'a [A],
}

impl<'a, A> OperationRequest<'a, A> {
    /// Build an operation request without an expiration deadline.
    pub fn new(id: impl Into<String>, op: impl Into<String>, args: &'a [A]) -> Self {
        Self {
            id: id.into(),
            op: op.into(),
            exp_time: None,
            args,
        }
    }

    /// Set the request effective deadline in Unix milliseconds.
    pub fn exp_time(mut self, exp_time: impl Into<String>) -> Self {
        self.exp_time = Some(exp_time.into());
        self
    }
}

/// MMP mass-cancel request body (`mass-cancel`).
///
/// Only `OPTION` in Portfolio Margin mode is supported by OKX.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-mass-cancel-order>
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MassCancelRequest {
    /// Instrument type. OKX currently requires `OPTION`.
    pub inst_type: String,
    /// Instrument family, e.g. `BTC-USD`.
    pub inst_family: String,
    /// Lock interval in milliseconds, range `0..=10000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_interval: Option<String>,
}

impl MassCancelRequest {
    /// Create an option MMP mass-cancel request.
    pub fn option(inst_family: impl Into<String>) -> Self {
        Self {
            inst_type: "OPTION".to_owned(),
            inst_family: inst_family.into(),
            lock_interval: None,
        }
    }

    /// Set the post-cancel lock interval in milliseconds.
    pub fn lock_interval(mut self, lock_interval: impl Into<String>) -> Self {
        self.lock_interval = Some(lock_interval.into());
        self
    }
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
    /// Order type accepted by the spread API.
    pub ord_type: String,
    /// Order quantity.
    pub sz: String,
    /// Order price. Required for price-based order types.
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
    fn mass_cancel_serializes_documented_keys() {
        let value = serde_json::to_value(
            MassCancelRequest::option("BTC-USD").lock_interval("1000"),
        )
        .unwrap();
        assert_eq!(value["instType"], "OPTION");
        assert_eq!(value["instFamily"], "BTC-USD");
        assert_eq!(value["lockInterval"], "1000");
    }

    #[test]
    fn spread_order_serializes_required_and_selected_optional_fields() {
        let request = PlaceSpreadOrderRequest::new(
            "BTC-USDT_BTC-USDT-SWAP",
            "buy",
            "limit",
            "1",
        )
        .price("10");
        let value = serde_json::to_value(request).unwrap();
        assert!(value.get("clOrdId").is_none());
        assert_eq!(value["px"], "10");
        assert_eq!(value["sprdId"], "BTC-USDT_BTC-USDT-SWAP");
    }
}
