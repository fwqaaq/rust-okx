use serde::Serialize;

/// Request body for `POST /api/v5/finance/savings/purchase-redempt`.
#[derive(Debug, Clone, Serialize)]
pub struct SavingsPurchaseRedemptionRequest {
    ccy: String,
    amt: String,
    side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate: Option<String>,
}

impl SavingsPurchaseRedemptionRequest {
    /// Create a Simple Earn purchase or redemption request.
    ///
    /// `side` must be `purchase` or `redempt`.
    pub fn new(ccy: impl Into<String>, amt: impl Into<String>, side: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            side: side.into(),
            rate: None,
        }
    }

    /// Set the minimum annual lending rate for a purchase.
    ///
    /// OKX documents the accepted range as `0.01` through `3.65` (1%–365%).
    pub fn rate(mut self, rate: impl Into<String>) -> Self {
        self.rate = Some(rate.into());
        self
    }
}
