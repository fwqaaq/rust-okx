use serde::Serialize;

use crate::model::{
    RequestValidationError, TradeMode, ValidateRequest, collection_length, exactly_one, non_empty,
    non_empty_items, one_of, optional_non_empty, optional_one_of, optional_unsigned_integer_string,
    positive_decimal_string, range_u64, reject_when_present,
};

/// Query parameters for maximum loan.
#[derive(Debug, Clone, Serialize)]
pub struct MaxLoanRequest {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "mgnCcy", skip_serializing_if = "Option::is_none")]
    mgn_ccy: Option<String>,
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl MaxLoanRequest {
    /// Create an instrument-based maximum-loan query.
    ///
    /// `inst_id` may contain one to five comma-separated instrument IDs, as
    /// documented by OKX. Use [`Self::by_currency`] for Spot-mode manual-borrow
    /// quota queries.
    pub fn new(inst_id: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self::by_instrument(inst_id, mgn_mode)
    }

    /// Create an instrument-based maximum-loan query.
    pub fn by_instrument(inst_id: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self {
            mgn_mode,
            inst_id: Some(inst_id.into()),
            ccy: None,
            mgn_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Create a currency-based Spot-mode manual-borrow quota query.
    pub fn by_currency(ccy: impl Into<String>) -> Self {
        Self {
            mgn_mode: TradeMode::Cross,
            inst_id: None,
            ccy: Some(ccy.into()),
            mgn_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Replace the selector with a currency-based Spot-mode query.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.inst_id = None;
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin currency.
    pub fn margin_currency(mut self, mgn_ccy: impl Into<String>) -> Self {
        self.mgn_ccy = Some(mgn_ccy.into());
        self
    }

    /// Set the trade quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// Query parameters for interest-accrued records.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestAccruedRequest {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl InterestAccruedRequest {
    /// Create an empty interest-accrued query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request body for borrow/repay.
#[derive(Debug, Clone, Serialize)]
pub struct BorrowRepayRequest {
    ccy: String,
    side: String,
    amt: String,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl BorrowRepayRequest {
    /// Create a borrow/repay request.
    pub fn new(ccy: impl Into<String>, side: impl Into<String>, amt: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            side: side.into(),
            amt: amt.into(),
            ord_id: None,
        }
    }

    /// Set the related order ID.
    pub fn order_id(mut self, ord_id: impl Into<String>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }
}

/// Query parameters for borrow/repay history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BorrowRepayHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl BorrowRepayHistoryRequest {
    /// Create an empty borrow/repay-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for interest limits.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestLimitsRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    limit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl InterestLimitsRequest {
    /// Create an empty interest-limits query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the OKX interest-limit type.
    pub fn limit_type(mut self, limit_type: impl Into<String>) -> Self {
        self.limit_type = Some(limit_type.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

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

impl ValidateRequest for MaxLoanRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        match &self.mgn_mode {
            TradeMode::Cross | TradeMode::Isolated => {}
            _ => {
                return Err(RequestValidationError::InvalidFormat {
                    field: "mgnMode",
                    expected: "cross or isolated",
                });
            }
        }
        optional_non_empty("instId", self.inst_id.as_deref())?;
        optional_non_empty("ccy", self.ccy.as_deref())?;
        exactly_one("instId, ccy", &[self.inst_id.is_some(), self.ccy.is_some()])?;

        if let Some(inst_ids) = self.inst_id.as_deref() {
            let instruments: Vec<_> = inst_ids.split(',').collect();
            collection_length("instId", instruments.len(), 1, 5)?;
            non_empty_items("instId", instruments)?;
        }

        optional_non_empty("mgnCcy", self.mgn_ccy.as_deref())?;
        optional_non_empty("tradeQuoteCcy", self.trade_quote_ccy.as_deref())?;
        if self.ccy.is_some() {
            if !matches!(self.mgn_mode, TradeMode::Cross) {
                return Err(RequestValidationError::InvalidFormat {
                    field: "mgnMode",
                    expected: "cross when ccy is used",
                });
            }
            reject_when_present("mgnCcy", self.mgn_ccy.as_ref(), "ccy is used")?;
            reject_when_present(
                "tradeQuoteCcy",
                self.trade_quote_ccy.as_ref(),
                "ccy is used",
            )?;
        }
        Ok(())
    }
}

impl ValidateRequest for InterestAccruedRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("instId", self.inst_id.as_deref())?;
        optional_non_empty("ccy", self.ccy.as_deref())?;
        if let Some(mode) = &self.mgn_mode {
            match mode {
                TradeMode::Cross | TradeMode::Isolated => {}
                _ => {
                    return Err(RequestValidationError::InvalidFormat {
                        field: "mgnMode",
                        expected: "cross or isolated",
                    });
                }
            }
        }
        validate_pagination(self.after.as_deref(), self.before.as_deref(), self.limit)
    }
}

impl ValidateRequest for BorrowRepayRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("ccy", &self.ccy)?;
        one_of("side", &self.side, &["borrow", "repay"], "borrow or repay")?;
        positive_decimal_string("amt", &self.amt)?;
        optional_non_empty("ordId", self.ord_id.as_deref())
    }
}

impl ValidateRequest for BorrowRepayHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("ccy", self.ccy.as_deref())?;
        validate_pagination(self.after.as_deref(), self.before.as_deref(), self.limit)
    }
}

impl ValidateRequest for InterestLimitsRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_one_of(
            "type",
            self.limit_type.as_deref(),
            &["1", "2"],
            "1 (loan quota) or 2 (interest rate)",
        )?;
        optional_non_empty("ccy", self.ccy.as_deref())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_loan_accepts_currency_selector() {
        MaxLoanRequest::by_currency("USDT").validate().unwrap();
    }

    #[test]
    fn max_loan_rejects_more_than_five_instruments() {
        let request = MaxLoanRequest::new("A-B,B-C,C-D,D-E,E-F,F-G", TradeMode::Cross);
        assert!(request.validate().is_err());
    }
}
