use std::borrow::Cow;

use serde::Serialize;

/// Option type accepted by dual-investment product queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DualInvestmentOptionType {
    /// Call.
    #[serde(rename = "C")]
    Call,
    /// Put.
    #[serde(rename = "P")]
    Put,
}

/// Query for dual-investment products.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualInvestmentProductsRequest<'a> {
    base_ccy: Cow<'a, str>,
    quote_ccy: Cow<'a, str>,
    opt_type: DualInvestmentOptionType,
}

impl<'a> DualInvestmentProductsRequest<'a> {
    /// Select a documented currency pair and option type.
    pub fn new(
        base_ccy: impl Into<Cow<'a, str>>,
        quote_ccy: impl Into<Cow<'a, str>>,
        opt_type: DualInvestmentOptionType,
    ) -> Self {
        Self {
            base_ccy: base_ccy.into(),
            quote_ccy: quote_ccy.into(),
            opt_type,
        }
    }
}

/// Request for a live dual-investment quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualInvestmentQuoteRequest<'a> {
    product_id: Cow<'a, str>,
    notional_sz: Cow<'a, str>,
    notional_ccy: Cow<'a, str>,
}

impl<'a> DualInvestmentQuoteRequest<'a> {
    /// Create a quote request.
    pub fn new(
        product_id: impl Into<Cow<'a, str>>,
        notional_sz: impl Into<Cow<'a, str>>,
        notional_ccy: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            product_id: product_id.into(),
            notional_sz: notional_sz.into(),
            notional_ccy: notional_ccy.into(),
        }
    }
}

/// Request identifying a dual-investment quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualInvestmentQuoteIdRequest<'a> {
    quote_id: Cow<'a, str>,
}

impl<'a> DualInvestmentQuoteIdRequest<'a> {
    /// Select a quote ID.
    pub fn new(quote_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            quote_id: quote_id.into(),
        }
    }
}

/// Request identifying a dual-investment order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualInvestmentOrderIdRequest<'a> {
    ord_id: Cow<'a, str>,
}

impl<'a> DualInvestmentOrderIdRequest<'a> {
    /// Select an order ID.
    pub fn new(ord_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ord_id: ord_id.into(),
        }
    }
}

/// Request confirming a dual-investment early redemption.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualInvestmentRedeemRequest<'a> {
    ord_id: Cow<'a, str>,
    quote_id: Cow<'a, str>,
}

impl<'a> DualInvestmentRedeemRequest<'a> {
    /// Select the order and redeem quote.
    pub fn new(
        ord_id: impl Into<Cow<'a, str>>,
        quote_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            ord_id: ord_id.into(),
            quote_id: quote_id.into(),
        }
    }
}

/// Query for dual-investment order history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualInvestmentHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ord_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uly: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> DualInvestmentHistoryRequest<'a> {
    /// Create an unfiltered order-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by order ID.
    pub fn order_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.ord_id = Some(value.into());
        self
    }

    /// Filter by product ID.
    pub fn product_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Filter by underlying index.
    pub fn underlying(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.uly = Some(value.into());
        self
    }

    /// Filter by order state.
    pub fn state(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Return records newer than this order ID.
    pub fn begin_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.begin_id = Some(value.into());
        self
    }

    /// Return records earlier than this order ID.
    pub fn end_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.end_id = Some(value.into());
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(value.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(value.into());
        self
    }

    /// Set the maximum number of results.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}
