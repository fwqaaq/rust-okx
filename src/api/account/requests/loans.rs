use serde::Serialize;
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
