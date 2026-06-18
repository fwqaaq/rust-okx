use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, at_least_one, non_empty, one_of, optional_non_empty,
    optional_one_of, optional_unsigned_integer_string,
};

/// Query parameters for `GET /api/v5/public/opt-summary`.
#[derive(Debug, Clone, Serialize)]
pub struct OptionSummaryRequest {
    #[serde(rename = "instFamily")]
    inst_family: String,
    #[serde(rename = "expTime", skip_serializing_if = "Option::is_none")]
    exp_time: Option<String>,
}

impl OptionSummaryRequest {
    /// Create a query for an option instrument family, such as `BTC-USD`.
    pub fn new(inst_family: impl Into<String>) -> Self {
        Self {
            inst_family: inst_family.into(),
            exp_time: None,
        }
    }

    /// Restrict the result to one expiration timestamp in milliseconds.
    pub fn expiration(mut self, value: impl Into<String>) -> Self {
        self.exp_time = Some(value.into());
        self
    }
}

impl ValidateRequest for OptionSummaryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("instFamily", &self.inst_family)?;
        optional_unsigned_integer_string("expTime", self.exp_time.as_deref())
    }
}

/// Empty query for `GET /api/v5/public/interest-rate-loan-quota`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestRateLoanQuotaRequest {}

impl InterestRateLoanQuotaRequest {
    /// Create the parameterless query.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ValidateRequest for InterestRateLoanQuotaRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        Ok(())
    }
}

/// Query parameters for `GET /api/v5/public/instrument-tick-bands`.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentTickBandsRequest {
    #[serde(rename = "instType")]
    inst_type: String,
}

impl InstrumentTickBandsRequest {
    /// Create a tick-band query for `FUTURES` or `OPTION` instruments.
    pub fn new(inst_type: impl Into<String>) -> Self {
        Self {
            inst_type: inst_type.into(),
        }
    }
}

impl ValidateRequest for InstrumentTickBandsRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        one_of(
            "instType",
            &self.inst_type,
            &["FUTURES", "OPTION"],
            "FUTURES or OPTION",
        )
    }
}

/// Query parameters for `GET /api/v5/public/underlying`.
#[derive(Debug, Clone, Serialize)]
pub struct UnderlyingRequest {
    #[serde(rename = "instType")]
    inst_type: String,
}

impl UnderlyingRequest {
    /// Create a query for `SWAP`, `FUTURES`, or `OPTION` instruments.
    pub fn new(inst_type: impl Into<String>) -> Self {
        Self {
            inst_type: inst_type.into(),
        }
    }
}

impl ValidateRequest for UnderlyingRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        one_of(
            "instType",
            &self.inst_type,
            &["SWAP", "FUTURES", "OPTION"],
            "SWAP, FUTURES, or OPTION",
        )
    }
}

/// Query parameters for `GET /api/v5/public/option-trades`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PublicOptionTradesRequest {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(rename = "optType", skip_serializing_if = "Option::is_none")]
    option_type: Option<String>,
}

impl PublicOptionTradesRequest {
    /// Create an unfiltered option-trades query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict results to one option instrument.
    pub fn inst_id(mut self, value: impl Into<String>) -> Self {
        self.inst_id = Some(value.into());
        self
    }

    /// Restrict results to one option instrument family.
    pub fn inst_family(mut self, value: impl Into<String>) -> Self {
        self.inst_family = Some(value.into());
        self
    }

    /// Restrict results to calls (`C`) or puts (`P`).
    pub fn option_type(mut self, value: impl Into<String>) -> Self {
        self.option_type = Some(value.into());
        self
    }
}

impl ValidateRequest for PublicOptionTradesRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("instId", self.inst_id.as_deref())?;
        optional_non_empty("instFamily", self.inst_family.as_deref())?;
        optional_one_of(
            "optType",
            self.option_type.as_deref(),
            &["C", "P"],
            "C or P",
        )?;
        at_least_one(
            "instId, instFamily",
            &[self.inst_id.is_some(), self.inst_family.is_some()],
        )
    }
}

/// Query parameters for `GET /api/v5/public/market-data-history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MarketDataHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    module: Option<String>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<String>,
    #[serde(rename = "instIdList", skip_serializing_if = "Option::is_none")]
    inst_id_list: Option<String>,
    #[serde(rename = "dateAggrType", skip_serializing_if = "Option::is_none")]
    date_aggr_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
}

impl MarketDataHistoryRequest {
    /// Create an empty market-data-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the OKX market-data module identifier.
    pub fn module(mut self, value: impl Into<String>) -> Self {
        self.module = Some(value.into());
        self
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, value: impl Into<String>) -> Self {
        self.inst_type = Some(value.into());
        self
    }

    /// Set a comma-separated instrument-ID list.
    pub fn inst_id_list(mut self, value: impl Into<String>) -> Self {
        self.inst_id_list = Some(value.into());
        self
    }

    /// Set the documented date aggregation type.
    pub fn date_aggregation(mut self, value: impl Into<String>) -> Self {
        self.date_aggr_type = Some(value.into());
        self
    }

    /// Set the inclusive begin timestamp in milliseconds.
    pub fn begin(mut self, value: impl Into<String>) -> Self {
        self.begin = Some(value.into());
        self
    }

    /// Set the inclusive end timestamp in milliseconds.
    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }
}

impl ValidateRequest for MarketDataHistoryRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_non_empty("module", self.module.as_deref())?;
        optional_one_of(
            "instType",
            self.inst_type.as_deref(),
            &["SPOT", "MARGIN", "SWAP", "FUTURES", "OPTION"],
            "SPOT, MARGIN, SWAP, FUTURES, or OPTION",
        )?;
        optional_non_empty("instIdList", self.inst_id_list.as_deref())?;
        optional_non_empty("dateAggrType", self.date_aggr_type.as_deref())?;
        optional_unsigned_integer_string("begin", self.begin.as_deref())?;
        optional_unsigned_integer_string("end", self.end.as_deref())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn market_history_uses_okx_field_names() {
        let request = MarketDataHistoryRequest::new()
            .module("2")
            .inst_type("SPOT")
            .inst_id_list("BTC-USDT")
            .date_aggregation("daily");
        request.validate().unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["instType"], "SPOT");
        assert_eq!(value["instIdList"], "BTC-USDT");
        assert_eq!(value["dateAggrType"], "daily");
    }

    #[test]
    fn option_trades_requires_an_instrument_scope() {
        assert!(PublicOptionTradesRequest::new().validate().is_err());
    }
}
