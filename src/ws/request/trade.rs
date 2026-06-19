//! Standard trade WebSocket request models.

use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, non_empty, one_of, optional_unsigned_integer_string,
    range_u64,
};

/// MMP mass-cancel request body (`mass-cancel`).
///
/// Only `OPTION` in Portfolio Margin mode is supported by OKX.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-mass-cancel-order>
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MassCancelRequest {
    /// Instrument type. OKX currently requires `OPTION`.
    pub inst_type: String,
    /// Instrument family, e.g. `BTC-USD`.
    pub inst_family: String,
    /// Lock interval in milliseconds, range `0..=10000`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_interval: Option<String>,
}

impl MassCancelRequest {
    /// Create an option MMP mass-cancel request.
    pub fn option(inst_family: impl Into<String>) -> Self {
        Self {
            inst_type: "OPTION".to_owned(),
            inst_family: inst_family.into(),
            lock_interval: None,
        }
    }

    /// Set the post-cancel lock interval in milliseconds.
    pub fn lock_interval(mut self, lock_interval: impl Into<String>) -> Self {
        self.lock_interval = Some(lock_interval.into());
        self
    }
}

impl ValidateRequest for MassCancelRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        one_of("instType", &self.inst_type, &["OPTION"], "OPTION")?;
        non_empty("instFamily", &self.inst_family)?;
        optional_unsigned_integer_string("lockInterval", self.lock_interval.as_deref())?;
        if let Some(value) = self.lock_interval.as_deref() {
            let value =
                value
                    .parse::<u64>()
                    .map_err(|_| RequestValidationError::InvalidFormat {
                        field: "lockInterval",
                        expected: "an integer from 0 through 10000",
                    })?;
            range_u64("lockInterval", value, 0, 10_000)?;
        }
        Ok(())
    }
}
