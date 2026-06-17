use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, decimal_string_range, non_empty, one_of,
    optional_positive_decimal_string, positive_decimal_string, reject_when_present,
};

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

impl ValidateRequest for SavingsPurchaseRedemptionRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("ccy", &self.ccy)?;
        positive_decimal_string("amt", &self.amt)?;
        one_of(
            "side",
            &self.side,
            &["purchase", "redempt"],
            "purchase or redempt",
        )?;

        if self.side == "redempt" {
            reject_when_present("rate", self.rate.as_ref(), "side is redempt")?;
        }
        if let Some(rate) = self.rate.as_deref() {
            optional_positive_decimal_string("rate", Some(rate))?;
            decimal_string_range("rate", rate, 0.01, 3.65, "0.01", "3.65")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redemption_rejects_lending_rate() {
        let request = SavingsPurchaseRedemptionRequest::new("USDT", "1", "redempt").rate("0.02");
        assert!(request.validate().is_err());
    }

    #[test]
    fn purchase_rate_must_be_in_documented_range() {
        let request = SavingsPurchaseRedemptionRequest::new("USDT", "1", "purchase").rate("3.66");
        assert!(request.validate().is_err());
    }
}
