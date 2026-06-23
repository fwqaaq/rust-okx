use std::borrow::Cow;

use serde::Serialize;

/// Query parameters for `GET /api/v5/public/opt-summary`.
#[derive(Debug, Clone, Serialize)]
pub struct OptionSummaryRequest<'a> {
    #[serde(rename = "instFamily")]
    inst_family: Cow<'a, str>,
    #[serde(rename = "expTime", skip_serializing_if = "Option::is_none")]
    exp_time: Option<Cow<'a, str>>,
}

impl<'a> OptionSummaryRequest<'a> {
    /// Create a query for an option instrument family, such as `BTC-USD`.
    pub fn new(inst_family: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_family: inst_family.into(),
            exp_time: None,
        }
    }

    /// Restrict the result to one expiration timestamp in milliseconds.
    pub fn expiration(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.exp_time = Some(value.into());
        self
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

/// Query parameters for `GET /api/v5/public/instrument-tick-bands`.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentTickBandsRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: Cow<'a, str>,
}

impl<'a> InstrumentTickBandsRequest<'a> {
    /// Create a tick-band query for `FUTURES` or `OPTION` instruments.
    pub fn new(inst_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_type: inst_type.into(),
        }
    }
}

/// Query parameters for `GET /api/v5/public/underlying`.
#[derive(Debug, Clone, Serialize)]
pub struct UnderlyingRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: Cow<'a, str>,
}

impl<'a> UnderlyingRequest<'a> {
    /// Create a query for `SWAP`, `FUTURES`, or `OPTION` instruments.
    pub fn new(inst_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_type: inst_type.into(),
        }
    }
}

/// Query parameters for `GET /api/v5/public/option-trades`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PublicOptionTradesRequest<'a> {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
    #[serde(rename = "optType", skip_serializing_if = "Option::is_none")]
    option_type: Option<Cow<'a, str>>,
}

impl<'a> PublicOptionTradesRequest<'a> {
    /// Create an unfiltered option-trades query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict results to one option instrument.
    pub fn inst_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(value.into());
        self
    }

    /// Restrict results to one option instrument family.
    pub fn inst_family(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(value.into());
        self
    }

    /// Restrict results to calls (`C`) or puts (`P`).
    pub fn option_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.option_type = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/market-data-history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MarketDataHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    module: Option<Cow<'a, str>>,
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<Cow<'a, str>>,
    #[serde(rename = "instIdList", skip_serializing_if = "Option::is_none")]
    inst_id_list: Option<Cow<'a, str>>,
    #[serde(rename = "dateAggrType", skip_serializing_if = "Option::is_none")]
    date_aggr_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
}

impl<'a> MarketDataHistoryRequest<'a> {
    /// Create an empty market-data-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the OKX market-data module identifier.
    pub fn module(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.module = Some(value.into());
        self
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_type = Some(value.into());
        self
    }

    /// Set a comma-separated instrument-ID list.
    pub fn inst_id_list(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id_list = Some(value.into());
        self
    }

    /// Set the documented date aggregation type.
    pub fn date_aggregation(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.date_aggr_type = Some(value.into());
        self
    }

    /// Set the inclusive begin timestamp in milliseconds.
    pub fn begin(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(value.into());
        self
    }

    /// Set the inclusive end timestamp in milliseconds.
    pub fn end(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(value.into());
        self
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
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["instType"], "SPOT");
        assert_eq!(value["instIdList"], "BTC-USDT");
        assert_eq!(value["dateAggrType"], "daily");
    }
}
