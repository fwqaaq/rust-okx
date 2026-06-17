use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, non_empty, one_of, optional_non_empty,
    optional_one_of, optional_unsigned_integer_string, positive_decimal_string, range_u64,
};

fn validate_pagination(
    after: Option<&str>,
    before: Option<&str>,
    limit: Option<u32>,
) -> Result<(), RequestValidationError> {
    optional_unsigned_integer_string("after", after)?;
    optional_unsigned_integer_string("before", before)?;
    if let Some(limit) = limit {
        range_u64("limit", u64::from(limit), 1, 100)?;
    }
    Ok(())
}

/// Request body for `POST /api/v5/account/spot-manual-borrow-repay`.
#[derive(Debug, Clone, Serialize)]
pub struct SpotManualBorrowRepayRequest {
    ccy: String,
    side: String,
    amt: String,
}

impl SpotManualBorrowRepayRequest {
    /// Create a manual spot borrow or repay request.
    pub fn new(ccy: impl Into<String>, side: impl Into<String>, amt: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            side: side.into(),
            amt: amt.into(),
        }
    }
}

impl ValidateRequest for SpotManualBorrowRepayRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("ccy", &self.ccy)?;
        one_of("side", &self.side, &["borrow", "repay"], "borrow or repay")?;
        positive_decimal_string("amt", &self.amt)
    }
}

/// Request body for `POST /api/v5/account/set-auto-repay`.
#[derive(Debug, Clone, Serialize)]
pub struct SetAutoRepayRequest {
    #[serde(rename = "autoRepay")]
    auto_repay: bool,
}

impl SetAutoRepayRequest {
    /// Enable or disable automatic repayment.
    pub fn new(auto_repay: bool) -> Self {
        Self { auto_repay }
    }
}

impl ValidateRequest for SetAutoRepayRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        Ok(())
    }
}

/// Query parameters for `GET /api/v5/account/spot-borrow-repay-history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SpotBorrowRepayHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    event_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl SpotBorrowRepayHistoryRequest {
    /// Create an unfiltered history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.ccy = Some(value.into());
        self
    }

    /// Filter by event type.
    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    /// Return records earlier than this millisecond timestamp.
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records newer than this millisecond timestamp.
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

impl ValidateRequest for SpotBorrowRepayHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("ccy", self.ccy.as_deref())?;
        optional_one_of(
            "type",
            self.event_type.as_deref(),
            &["auto_borrow", "auto_repay", "manual_borrow", "manual_repay"],
            "auto_borrow, auto_repay, manual_borrow, or manual_repay",
        )?;
        validate_pagination(self.after.as_deref(), self.before.as_deref(), self.limit)
    }
}

/// Request body for `POST /api/v5/account/set-auto-earn`.
#[derive(Debug, Clone, Serialize)]
pub struct SetAutoEarnRequest {
    #[serde(rename = "earnType")]
    earn_type: String,
    ccy: String,
    action: String,
}

impl SetAutoEarnRequest {
    /// Create an auto-earn update.
    ///
    /// `earn_type` is `0` for auto-lend/stake and `1` for USDG-style earn;
    /// `action` is `turn_on` or `turn_off`.
    pub fn new(
        earn_type: impl Into<String>,
        ccy: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            earn_type: earn_type.into(),
            ccy: ccy.into(),
            action: action.into(),
        }
    }
}

impl ValidateRequest for SetAutoEarnRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        one_of("earnType", &self.earn_type, &["0", "1"], "0 or 1")?;
        non_empty("ccy", &self.ccy)?;
        one_of(
            "action",
            &self.action,
            &["turn_on", "turn_off"],
            "turn_on or turn_off",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spot_manual_borrow_repay_rejects_invalid_side() {
        let request = SpotManualBorrowRepayRequest::new("USDT", "lend", "10");
        assert!(request.validate().is_err());
    }

    #[test]
    fn spot_history_rejects_limit_over_one_hundred() {
        let request = SpotBorrowRepayHistoryRequest::new().limit(101);
        assert!(request.validate().is_err());
    }

    #[test]
    fn auto_earn_uses_current_wire_fields() {
        let request = SetAutoEarnRequest::new("0", "BTC", "turn_on");
        request.validate().unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["earnType"], "0");
        assert_eq!(value["action"], "turn_on");
        assert!(value.get("autoEarn").is_none());
    }
}
