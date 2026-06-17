use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, optional_non_empty, optional_unsigned_integer_string,
    range_u64,
};

/// Common currency and cursor pagination query used by finance history endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FinanceHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FinanceHistoryRequest {
    /// Create an unfiltered history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the endpoint-specific `after` cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the endpoint-specific `before` cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the number of records to return, from 1 through 100.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl ValidateRequest for FinanceHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("ccy", self.ccy.as_deref())?;
        optional_unsigned_integer_string("after", self.after.as_deref())?;
        optional_unsigned_integer_string("before", self.before.as_deref())?;
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}
