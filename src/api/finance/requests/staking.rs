use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, collection_length, length_range, non_empty, one_of,
    optional_non_empty, optional_one_of, optional_unsigned_integer_string, positive_decimal_string,
    range_u64,
};

const PROTOCOL_TYPES: &[&str] = &["staking", "defi"];

/// Query parameters for `GET /api/v5/finance/staking-defi/offers`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StakingDefiOffersRequest {
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    product_id: Option<String>,
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    protocol_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl StakingDefiOffersRequest {
    /// Create an unfiltered offers query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the response to one product ID.
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Restrict the response to `staking` or `defi` products.
    pub fn protocol_type(mut self, value: impl Into<String>) -> Self {
        self.protocol_type = Some(value.into());
        self
    }

    /// Restrict the response to one investment currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }
}

impl ValidateRequest for StakingDefiOffersRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("productId", self.product_id.as_deref())?;
        optional_one_of(
            "protocolType",
            self.protocol_type.as_deref(),
            PROTOCOL_TYPES,
            "staking or defi",
        )?;
        optional_non_empty("ccy", self.ccy.as_deref())
    }
}

/// Currency and amount invested into one On-chain Earn product.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiInvestment {
    ccy: String,
    amt: String,
}

impl StakingDefiInvestment {
    /// Create one investment item.
    pub fn new(ccy: impl Into<String>, amt: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
        }
    }
}

impl ValidateRequest for StakingDefiInvestment {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("investData.ccy", &self.ccy)?;
        positive_decimal_string("investData.amt", &self.amt)
    }
}

/// Request body for `POST /api/v5/finance/staking-defi/purchase`.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiPurchaseRequest {
    #[serde(rename = "productId")]
    product_id: String,
    #[serde(rename = "investData")]
    invest_data: Vec<StakingDefiInvestment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}

impl StakingDefiPurchaseRequest {
    /// Create a purchase with at least one currency/amount item.
    pub fn new(product_id: impl Into<String>, invest_data: Vec<StakingDefiInvestment>) -> Self {
        Self {
            product_id: product_id.into(),
            invest_data,
            term: None,
            tag: None,
        }
    }

    /// Set the fixed product term when required by the selected product.
    pub fn term(mut self, value: impl Into<String>) -> Self {
        self.term = Some(value.into());
        self
    }

    /// Set a case-sensitive ASCII alphanumeric tag of at most 16 characters.
    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }
}

impl ValidateRequest for StakingDefiPurchaseRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("productId", &self.product_id)?;
        collection_length("investData", self.invest_data.len(), 1, usize::MAX)?;
        for investment in &self.invest_data {
            investment.validate()?;
        }
        optional_non_empty("term", self.term.as_deref())?;
        if let Some(tag) = self.tag.as_deref() {
            length_range("tag", tag, 1, 16)?;
            if !tag.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
                return Err(RequestValidationError::InvalidFormat {
                    field: "tag",
                    expected: "1-16 ASCII alphanumeric characters",
                });
            }
        }
        Ok(())
    }
}

/// Request body for `POST /api/v5/finance/staking-defi/redeem`.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiRedeemRequest {
    #[serde(rename = "ordId")]
    ord_id: String,
    #[serde(rename = "protocolType")]
    protocol_type: String,
    #[serde(rename = "allowEarlyRedeem", skip_serializing_if = "Option::is_none")]
    allow_early_redeem: Option<bool>,
}

impl StakingDefiRedeemRequest {
    /// Create a redemption for an existing order.
    pub fn new(ord_id: impl Into<String>, protocol_type: impl Into<String>) -> Self {
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

impl ValidateRequest for StakingDefiRedeemRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("ordId", &self.ord_id)?;
        one_of(
            "protocolType",
            &self.protocol_type,
            PROTOCOL_TYPES,
            "staking or defi",
        )
    }
}

/// Request body for `POST /api/v5/finance/staking-defi/cancel`.
#[derive(Debug, Clone, Serialize)]
pub struct StakingDefiCancelRequest {
    #[serde(rename = "ordId")]
    ord_id: String,
    #[serde(rename = "protocolType")]
    protocol_type: String,
}

impl StakingDefiCancelRequest {
    /// Create a cancellation for a pending On-chain Earn order.
    pub fn new(ord_id: impl Into<String>, protocol_type: impl Into<String>) -> Self {
        Self {
            ord_id: ord_id.into(),
            protocol_type: protocol_type.into(),
        }
    }
}

impl ValidateRequest for StakingDefiCancelRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("ordId", &self.ord_id)?;
        one_of(
            "protocolType",
            &self.protocol_type,
            PROTOCOL_TYPES,
            "staking or defi",
        )
    }
}

/// Query parameters for `GET /api/v5/finance/staking-defi/orders-active`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StakingDefiActiveOrdersRequest {
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    product_id: Option<String>,
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    protocol_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
}

impl StakingDefiActiveOrdersRequest {
    /// Create an unfiltered active-orders query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by product ID.
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Filter by `staking` or `defi` protocol type.
    pub fn protocol_type(mut self, value: impl Into<String>) -> Self {
        self.protocol_type = Some(value.into());
        self
    }

    /// Filter by investment currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Filter by OKX active-order state (`1`, `2`, `8`, `9`, or `13`).
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
}

impl ValidateRequest for StakingDefiActiveOrdersRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("productId", self.product_id.as_deref())?;
        optional_one_of(
            "protocolType",
            self.protocol_type.as_deref(),
            PROTOCOL_TYPES,
            "staking or defi",
        )?;
        optional_non_empty("ccy", self.ccy.as_deref())?;
        optional_one_of(
            "state",
            self.state.as_deref(),
            &["1", "2", "8", "9", "13"],
            "1, 2, 8, 9, or 13",
        )
    }
}

/// Query parameters for `GET /api/v5/finance/staking-defi/orders-history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StakingDefiOrderHistoryRequest {
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    product_id: Option<String>,
    #[serde(rename = "protocolType", skip_serializing_if = "Option::is_none")]
    protocol_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl StakingDefiOrderHistoryRequest {
    /// Create an unfiltered order-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by product ID.
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Filter by `staking` or `defi` protocol type.
    pub fn protocol_type(mut self, value: impl Into<String>) -> Self {
        self.protocol_type = Some(value.into());
        self
    }

    /// Filter by investment currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Return records before this order-ID cursor.
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records after this order-ID cursor.
    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the result count from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

impl ValidateRequest for StakingDefiOrderHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("productId", self.product_id.as_deref())?;
        optional_one_of(
            "protocolType",
            self.protocol_type.as_deref(),
            PROTOCOL_TYPES,
            "staking or defi",
        )?;
        optional_non_empty("ccy", self.ccy.as_deref())?;
        optional_unsigned_integer_string("after", self.after.as_deref())?;
        optional_unsigned_integer_string("before", self.before.as_deref())?;
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn purchase_serializes_nested_investment_data() {
        let request = StakingDefiPurchaseRequest::new(
            "product-1",
            vec![StakingDefiInvestment::new("ETH", "0.5")],
        );
        request.validate().unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["productId"], "product-1");
        assert_eq!(value["investData"][0]["ccy"], "ETH");
    }

    #[test]
    fn active_orders_reject_unknown_state() {
        assert!(
            StakingDefiActiveOrdersRequest::new()
                .state("3")
                .validate()
                .is_err()
        );
    }
}
