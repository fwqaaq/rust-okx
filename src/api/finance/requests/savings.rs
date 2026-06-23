use std::borrow::Cow;

use serde::Serialize;

/// Request body for `POST /api/v5/finance/savings/purchase-redempt`.
#[derive(Debug, Clone, Serialize)]
pub struct SavingsPurchaseRedemptionRequest<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
    side: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate: Option<Cow<'a, str>>,
}

impl<'a> SavingsPurchaseRedemptionRequest<'a> {
    /// Create a Simple Earn purchase or redemption request.
    ///
    /// `side` must be `purchase` or `redempt`.
    pub fn new(
        ccy: impl Into<Cow<'a, str>>,
        amt: impl Into<Cow<'a, str>>,
        side: impl Into<Cow<'a, str>>,
    ) -> Self {
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
    pub fn rate(mut self, rate: impl Into<Cow<'a, str>>) -> Self {
        self.rate = Some(rate.into());
        self
    }
}
