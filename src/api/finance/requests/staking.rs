use std::borrow::Cow;

use serde::Serialize;

/// Query parameters for `GET /api/v5/finance/staking-defi/offers`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StakingDefiOffersRequest<'a> {
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    product_id: Option<Cow<'a, str>>,
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    protocol_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> StakingDefiOffersRequest<'a> {
    /// Create an unfiltered offers query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the response to one product ID.
    pub fn product_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Restrict the response to `staking` or `defi` products.
    pub fn protocol_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.protocol_type = Some(value.into());
        self
    }

    /// Restrict the response to one investment currency.
    pub fn currency(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(value.into());
        self
    }
}

/// Currency and amount invested into one On-chain Earn product.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiInvestment<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
}

impl<'a> StakingDefiInvestment<'a> {
    /// Create one investment item.
    pub fn new(ccy: impl Into<Cow<'a, str>>, amt: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
        }
    }
}

/// Request body for `POST /api/v5/finance/staking-defi/purchase`.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiPurchaseRequest<'a> {
    #[serde(rename = "productId")]
    product_id: Cow<'a, str>,
    #[serde(rename = "investData")]
    invest_data: Vec<StakingDefiInvestment<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    term: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<Cow<'a, str>>,
}

impl<'a> StakingDefiPurchaseRequest<'a> {
    /// Create a purchase with at least one currency/amount item.
    pub fn new(
        product_id: impl Into<Cow<'a, str>>,
        invest_data: Vec<StakingDefiInvestment<'a>>,
    ) -> Self {
        Self {
            product_id: product_id.into(),
            invest_data,
            term: None,
            tag: None,
        }
    }

    /// Set the fixed product term when required by the selected product.
    pub fn term(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.term = Some(value.into());
        self
    }

    /// Set a case-sensitive ASCII alphanumeric tag of at most 16 characters.
    pub fn tag(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.tag = Some(value.into());
        self
    }
}

/// Request body for `POST /api/v5/finance/staking-defi/redeem`.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiRedeemRequest<'a> {
    #[serde(rename = "ordId")]
    ord_id: Cow<'a, str>,
    #[serde(rename = "protocolType")]
    protocol_type: Cow<'a, str>,
    #[serde(rename = "allowEarlyRedeem", skip_serializing_if = "Option::is_none")]
    allow_early_redeem: Option<bool>,
}

impl<'a> StakingDefiRedeemRequest<'a> {
    /// Create a redemption for an existing order.
    pub fn new(ord_id: impl Into<Cow<'a, str>>, protocol_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ord_id: ord_id.into(),
            protocol_type: protocol_type.into(),
            allow_early_redeem: None,
        }
    }

    /// Allow early redemption when the product supports it.
    pub fn allow_early_redeem(mut self, value: bool) -> Self {
        self.allow_early_redeem = Some(value);
        self
    }
}

/// Request body for `POST /api/v5/finance/staking-defi/cancel`.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiCancelRequest<'a> {
    #[serde(rename = "ordId")]
    ord_id: Cow<'a, str>,
    #[serde(rename = "protocolType")]
    protocol_type: Cow<'a, str>,
}

impl<'a> StakingDefiCancelRequest<'a> {
    /// Create a cancellation for a pending On-chain Earn order.
    pub fn new(ord_id: impl Into<Cow<'a, str>>, protocol_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ord_id: ord_id.into(),
            protocol_type: protocol_type.into(),
        }
    }
}

/// Query parameters for `GET /api/v5/finance/staking-defi/orders-active`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StakingDefiActiveOrdersRequest<'a> {
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    product_id: Option<Cow<'a, str>>,
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    protocol_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
}

impl<'a> StakingDefiActiveOrdersRequest<'a> {
    /// Create an unfiltered active-orders query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by product ID.
    pub fn product_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Filter by `staking` or `defi` protocol type.
    pub fn protocol_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.protocol_type = Some(value.into());
        self
    }

    /// Filter by investment currency.
    pub fn currency(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Filter by OKX active-order state (`1`, `2`, `8`, `9`, or `13`).
    pub fn state(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/finance/staking-defi/orders-history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StakingDefiOrderHistoryRequest<'a> {
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    product_id: Option<Cow<'a, str>>,
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    protocol_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> StakingDefiOrderHistoryRequest<'a> {
    /// Create an unfiltered order-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by product ID.
    pub fn product_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Filter by `staking` or `defi` protocol type.
    pub fn protocol_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.protocol_type = Some(value.into());
        self
    }

    /// Filter by investment currency.
    pub fn currency(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Return records before this order-ID cursor.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records after this order-ID cursor.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the result count from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}
