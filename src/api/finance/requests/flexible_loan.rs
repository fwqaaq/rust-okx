use serde::Serialize;

/// Query parameters for `GET /api/v5/finance/flexible-loan/collateral-assets`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlexibleLoanCollateralAssetsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl FlexibleLoanCollateralAssetsRequest {
    /// Create an unfiltered collateral-assets query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by collateral currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Scope the query to one flexible-loan order.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }
}

/// Request body for `POST /api/v5/finance/flexible-loan/max-loan`.
#[derive(Debug, Clone, Serialize)]
pub struct FlexibleLoanMaxLoanRequest {
    #[serde(rename = "borrowCcy")]
    borrow_ccy: String,
    #[serde(rename = "collateralCcy", skip_serializing_if = "Option::is_none")]
    collateral_ccy: Option<String>,
    #[serde(rename = "collateralAmt", skip_serializing_if = "Option::is_none")]
    collateral_amt: Option<String>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl FlexibleLoanMaxLoanRequest {
    /// Create a maximum-loan estimate for a borrowing currency.
    pub fn new(borrow_ccy: impl Into<String>) -> Self {
        Self {
            borrow_ccy: borrow_ccy.into(),
            collateral_ccy: None,
            collateral_amt: None,
            ord_id: None,
        }
    }

    /// Estimate using a collateral currency and amount.
    pub fn collateral(mut self, ccy: impl Into<String>, amt: impl Into<String>) -> Self {
        self.collateral_ccy = Some(ccy.into());
        self.collateral_amt = Some(amt.into());
        self
    }

    /// Estimate additional borrowing for one existing flexible-loan order.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/finance/flexible-loan/max-collateral-redeem-amount`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlexibleLoanMaxRedeemRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl FlexibleLoanMaxRedeemRequest {
    /// Create an unfiltered maximum-redeem query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by collateral currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Scope the calculation to one flexible-loan order.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }
}

/// Request body for `POST /api/v5/finance/flexible-loan/adjust-collateral`.
#[derive(Debug, Clone, Serialize)]
pub struct FlexibleLoanAdjustCollateralRequest {
    #[serde(rename = "ordId")]
    ord_id: String,
    #[serde(rename = "collateralCcy")]
    collateral_ccy: String,
    amt: String,
    #[serde(rename = "type")]
    adjustment_type: String,
}

impl FlexibleLoanAdjustCollateralRequest {
    /// Create a collateral adjustment; `type` must be `add` or `reduce`.
    pub fn new(
        ord_id: impl Into<String>,
        collateral_ccy: impl Into<String>,
        amt: impl Into<String>,
        adjustment_type: impl Into<String>,
    ) -> Self {
        Self {
            ord_id: ord_id.into(),
            collateral_ccy: collateral_ccy.into(),
            amt: amt.into(),
            adjustment_type: adjustment_type.into(),
        }
    }
}

/// Query parameters for `GET /api/v5/finance/flexible-loan/loan-info`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlexibleLoanInfoRequest {
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl FlexibleLoanInfoRequest {
    /// Create a query for all current flexible-loan orders.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one flexible-loan order.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/finance/flexible-loan/loan-history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlexibleLoanHistoryRequest {
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FlexibleLoanHistoryRequest {
    /// Create an unfiltered loan-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one flexible-loan order.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }

    /// Set the endpoint's `after` cursor.
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Set the endpoint's `before` cursor.
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

/// Query parameters for `GET /api/v5/finance/flexible-loan/interest-accrued`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FlexibleLoanInterestAccruedRequest {
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FlexibleLoanInterestAccruedRequest {
    /// Create an unfiltered accrued-interest query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one flexible-loan order.
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.ord_id = Some(value.into());
        self
    }

    /// Restrict the result to one currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Set the endpoint's `after` cursor.
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Set the endpoint's `before` cursor.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_loan_requires_complete_collateral_pair() {
        let request = FlexibleLoanMaxLoanRequest::new("USDT");

        let mut value = serde_json::to_value(request).unwrap();
        value["collateralCcy"] = serde_json::Value::String("BTC".into());
        assert_eq!(value["borrowCcy"], "USDT");
    }
}
