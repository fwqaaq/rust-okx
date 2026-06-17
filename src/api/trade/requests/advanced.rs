use std::collections::HashSet;

use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, collection_length, non_empty, non_empty_items,
    optional_one_of, optional_unsigned_integer_string, range_u64,
};

/// Request body for `POST /api/v5/trade/easy-convert`.
#[derive(Debug, Clone, Serialize)]
pub struct EasyConvertRequest {
    #[serde(rename = "fromCcy")]
    from_ccy: Vec<String>,
    #[serde(rename = "toCcy")]
    to_ccy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
}

impl EasyConvertRequest {
    /// Create an easy-convert request with one to five source currencies.
    pub fn new<I, S>(from_ccy: I, to_ccy: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            from_ccy: from_ccy.into_iter().map(Into::into).collect(),
            to_ccy: to_ccy.into(),
            source: None,
        }
    }

    /// Set the funding source: `1` for trading or `2` for funding.
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }
}

impl ValidateRequest for EasyConvertRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        collection_length("fromCcy", self.from_ccy.len(), 1, 5)?;
        non_empty_items("fromCcy", self.from_ccy.iter().map(String::as_str))?;
        non_empty("toCcy", &self.to_ccy)?;
        optional_one_of("source", self.source.as_deref(), &["1", "2"], "1 or 2")?;

        if self.from_ccy.iter().any(|ccy| ccy == &self.to_ccy) {
            return Err(RequestValidationError::InvalidFormat {
                field: "toCcy",
                expected: "a currency different from every fromCcy entry",
            });
        }
        let unique: HashSet<&str> = self.from_ccy.iter().map(String::as_str).collect();
        if unique.len() != self.from_ccy.len() {
            return Err(RequestValidationError::InvalidFormat {
                field: "fromCcy",
                expected: "one to five distinct currencies",
            });
        }
        Ok(())
    }
}

/// Query parameters for easy-convert history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EasyConvertHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl EasyConvertHistoryRequest {
    /// Create an empty history query.
    pub fn new() -> Self {
        Self::default()
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

    /// Set the number of results, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

impl ValidateRequest for EasyConvertHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_unsigned_integer_string("after", self.after.as_deref())?;
        optional_unsigned_integer_string("before", self.before.as_deref())?;
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}

/// Query parameters for one-click-repay currency-list endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OneClickRepayCurrencyListRequest {
    #[serde(rename = "debtType", skip_serializing_if = "Option::is_none")]
    debt_type: Option<String>,
}

impl OneClickRepayCurrencyListRequest {
    /// Create an unfiltered currency-list query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter debt by `cross` or `isolated` type.
    pub fn debt_type(mut self, value: impl Into<String>) -> Self {
        self.debt_type = Some(value.into());
        self
    }
}

impl ValidateRequest for OneClickRepayCurrencyListRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_one_of(
            "debtType",
            self.debt_type.as_deref(),
            &["cross", "isolated"],
            "cross or isolated",
        )
    }
}

/// One or more debt currencies accepted by the legacy and v2 repay APIs.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum DebtCurrencySelection {
    One(String),
    Many(Vec<String>),
}

/// Request body shared by one-click-repay v1 and v2 endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct OneClickRepayRequest {
    #[serde(rename = "debtCcy")]
    debt_ccy: DebtCurrencySelection,
    #[serde(rename = "repayCcy", skip_serializing_if = "Option::is_none")]
    repay_ccy: Option<String>,
    #[serde(rename = "repayCcyList", skip_serializing_if = "Option::is_none")]
    repay_ccy_list: Option<Vec<String>>,
}

impl OneClickRepayRequest {
    /// Build the legacy v1 request (`debtCcy` array + one `repayCcy`).
    pub fn new<I, S>(debt_ccy: I, repay_ccy: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            debt_ccy: DebtCurrencySelection::Many(debt_ccy.into_iter().map(Into::into).collect()),
            repay_ccy: Some(repay_ccy.into()),
            repay_ccy_list: None,
        }
    }

    /// Build the v2 request (`debtCcy` string + prioritized `repayCcyList`).
    pub fn v2<I, S>(debt_ccy: impl Into<String>, repay_ccy_list: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            debt_ccy: DebtCurrencySelection::One(debt_ccy.into()),
            repay_ccy: None,
            repay_ccy_list: Some(repay_ccy_list.into_iter().map(Into::into).collect()),
        }
    }
}

impl ValidateRequest for OneClickRepayRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        match (&self.debt_ccy, &self.repay_ccy, &self.repay_ccy_list) {
            (DebtCurrencySelection::Many(debt), Some(repay), None) => {
                collection_length("debtCcy", debt.len(), 1, 5)?;
                non_empty_items("debtCcy", debt.iter().map(String::as_str))?;
                non_empty("repayCcy", repay)?;
                if debt.iter().any(|ccy| ccy == repay) {
                    return Err(RequestValidationError::InvalidFormat {
                        field: "repayCcy",
                        expected: "a currency different from every debtCcy entry",
                    });
                }
                Ok(())
            }
            (DebtCurrencySelection::One(debt), None, Some(repay_list)) => {
                non_empty("debtCcy", debt)?;
                collection_length("repayCcyList", repay_list.len(), 1, 100)?;
                non_empty_items("repayCcyList", repay_list.iter().map(String::as_str))
            }
            _ => Err(RequestValidationError::InvalidFormat {
                field: "debtCcy",
                expected: "the v1 or v2 one-click-repay request shape",
            }),
        }
    }
}

/// Query parameters for one-click-repay history endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OneClickRepayHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl OneClickRepayHistoryRequest {
    /// Create an empty history query.
    pub fn new() -> Self {
        Self::default()
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

    /// Set the number of results, from 1 through 100.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

impl ValidateRequest for OneClickRepayHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
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
    fn easy_convert_rejects_receiving_source_currency() {
        let request = EasyConvertRequest::new(["BTC"], "BTC");
        assert!(request.validate().is_err());
    }

    #[test]
    fn legacy_repay_is_limited_to_five_debt_currencies() {
        let request = OneClickRepayRequest::new(["A", "B", "C", "D", "E", "F"], "USDT");
        assert!(request.validate().is_err());
    }
}
