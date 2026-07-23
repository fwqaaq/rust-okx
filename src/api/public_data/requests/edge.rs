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

/// Query parameters for `GET /api/v5/public/mm-instrument-types`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MmInstrumentTypesRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
}

impl<'a> MmInstrumentTypesRequest<'a> {
    /// Create an unfiltered MM instrument-types query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict results to one instrument type (`SPOT` or `SWAP`).
    pub fn inst_type(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_type = Some(value.into());
        self
    }

    /// Restrict results to one instrument ID, e.g. `BTC-USDT-SWAP`.
    pub fn inst_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/economic-calendar`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EconomicCalendarRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    importance: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Cow<'a, str>>,
}

impl<'a> EconomicCalendarRequest<'a> {
    /// Create an unfiltered economic-calendar query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict results to one country, region, or entity, e.g. `united_states`.
    pub fn region(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Restrict results to one importance level (`1` low, `2` medium, `3` high).
    pub fn importance(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.importance = Some(value.into());
        self
    }

    /// Return records newer than this `date` timestamp (Unix milliseconds).
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Return records earlier than this `date` timestamp (Unix milliseconds).
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Set the number of results per request. The maximum and default are 100.
    pub fn limit(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.limit = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/premium-history`.
#[derive(Debug, Clone, Serialize)]
pub struct PremiumHistoryRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Cow<'a, str>>,
}

impl<'a> PremiumHistoryRequest<'a> {
    /// Create a premium-history query for a swap instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Return records earlier than this timestamp, exclusive.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records newer than this timestamp, exclusive.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the number of results. The documented maximum is 100.
    pub fn limit(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.limit = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/event-contract/series`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EventContractSeriesRequest<'a> {
    #[serde(rename = "seriesId", skip_serializing_if = "Option::is_none")]
    series_id: Option<Cow<'a, str>>,
}

impl<'a> EventContractSeriesRequest<'a> {
    /// Create an unfiltered event-contract series query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the result to one series ID.
    pub fn series_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.series_id = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/event-contract/events`.
#[derive(Debug, Clone, Serialize)]
pub struct EventContractEventsRequest<'a> {
    #[serde(rename = "seriesId")]
    series_id: Cow<'a, str>,
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    event_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
}

impl<'a> EventContractEventsRequest<'a> {
    /// Create an event query for one series.
    pub fn new(series_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            series_id: series_id.into(),
            event_id: None,
            state: None,
            limit: None,
            before: None,
            after: None,
        }
    }

    /// Restrict the result to one event ID.
    pub fn event_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    /// Restrict the result to one documented event state.
    pub fn state(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Set the number of results. The documented maximum is 100.
    pub fn limit(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.limit = Some(value.into());
        self
    }

    /// Return records newer than this expiration timestamp, exclusive.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Return records earlier than this expiration timestamp, exclusive.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/event-contract/markets`.
#[derive(Debug, Clone, Serialize)]
pub struct EventContractMarketsRequest<'a> {
    #[serde(rename = "seriesId")]
    series_id: Cow<'a, str>,
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    event_id: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
}

impl<'a> EventContractMarketsRequest<'a> {
    /// Create a market query for one series.
    pub fn new(series_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            series_id: series_id.into(),
            event_id: None,
            inst_id: None,
            state: None,
            limit: None,
            before: None,
            after: None,
        }
    }

    /// Restrict the result to one event ID.
    pub fn event_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    /// Restrict the result to one instrument ID.
    pub fn inst_id(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(value.into());
        self
    }

    /// Restrict the result to one documented market state.
    pub fn state(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Set the number of results. The documented maximum is 100.
    pub fn limit(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.limit = Some(value.into());
        self
    }

    /// Return records newer than this expiration timestamp, exclusive.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Return records earlier than this expiration timestamp, exclusive.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }
}

/// Query parameters for `GET /api/v5/public/settlement-history`.
#[derive(Debug, Clone, Serialize)]
pub struct SettlementHistoryRequest<'a> {
    #[serde(rename = "instFamily")]
    inst_family: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Cow<'a, str>>,
}

impl<'a> SettlementHistoryRequest<'a> {
    /// Create a futures settlement-history query.
    pub fn new(inst_family: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_family: inst_family.into(),
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Return records earlier than this timestamp, exclusive.
    pub fn after(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Return records newer than this timestamp, exclusive.
    pub fn before(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Set the number of results. The documented maximum is 100.
    pub fn limit(mut self, value: impl Into<Cow<'a, str>>) -> Self {
        self.limit = Some(value.into());
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
