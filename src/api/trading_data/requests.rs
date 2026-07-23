use std::borrow::Cow;

use serde::Serialize;

/// Product family accepted by the aggregate taker-volume endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TakerVolumeInstrumentType {
    /// Spot trading.
    #[serde(rename = "SPOT")]
    Spot,
    /// Expiry-futures and perpetual-futures trading.
    #[serde(rename = "CONTRACTS")]
    Contracts,
}

/// Unit accepted by the contract taker-volume endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ContractVolumeUnit {
    /// Volume in cryptocurrency.
    #[serde(rename = "0")]
    Crypto,
    /// Volume in contracts.
    #[serde(rename = "1")]
    Contracts,
    /// Volume in USD.
    #[serde(rename = "2")]
    Usd,
}

/// Request for instrument-scoped contract history endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentHistoryRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> InstrumentHistoryRequest<'a> {
    /// Create an instrument-scoped history query.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            period: None,
            end: None,
            begin: None,
            limit: None,
        }
    }

    /// Set the documented bar size.
    pub fn period(mut self, period: impl Into<Cow<'a, str>>) -> Self {
        self.period = Some(period.into());
        self
    }

    /// Return records earlier than this timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Return records newer than this timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the maximum number of rows, up to the endpoint's documented limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request for aggregate taker-volume history.
#[derive(Debug, Clone, Serialize)]
pub struct TakerVolumeRequest<'a> {
    ccy: Cow<'a, str>,
    #[serde(rename = "instType")]
    inst_type: TakerVolumeInstrumentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Cow<'a, str>>,
}

impl<'a> TakerVolumeRequest<'a> {
    /// Create a taker-volume query.
    pub fn new(ccy: impl Into<Cow<'a, str>>, inst_type: TakerVolumeInstrumentType) -> Self {
        Self {
            ccy: ccy.into(),
            inst_type,
            begin: None,
            end: None,
            period: None,
        }
    }

    /// Return records newer than this timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Return records earlier than this timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Set the documented bar size.
    pub fn period(mut self, period: impl Into<Cow<'a, str>>) -> Self {
        self.period = Some(period.into());
        self
    }
}

/// Request for instrument-scoped contract taker volume.
#[derive(Debug, Clone, Serialize)]
pub struct ContractTakerVolumeRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<ContractVolumeUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> ContractTakerVolumeRequest<'a> {
    /// Create a contract taker-volume query.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            period: None,
            unit: None,
            end: None,
            begin: None,
            limit: None,
        }
    }

    /// Set the documented bar size.
    pub fn period(mut self, period: impl Into<Cow<'a, str>>) -> Self {
        self.period = Some(period.into());
        self
    }

    /// Set the response volume unit.
    pub fn unit(mut self, unit: ContractVolumeUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Return records earlier than this timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Return records newer than this timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the maximum number of rows, up to the endpoint's documented limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request for currency-scoped contract or margin history.
#[derive(Debug, Clone, Serialize)]
pub struct CurrencyHistoryRequest<'a> {
    ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Cow<'a, str>>,
}

impl<'a> CurrencyHistoryRequest<'a> {
    /// Create a currency-scoped history query.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            begin: None,
            end: None,
            period: None,
        }
    }

    /// Return records newer than this timestamp.
    pub fn begin(mut self, begin: impl Into<Cow<'a, str>>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Return records earlier than this timestamp.
    pub fn end(mut self, end: impl Into<Cow<'a, str>>) -> Self {
        self.end = Some(end.into());
        self
    }

    /// Set the documented bar size.
    pub fn period(mut self, period: impl Into<Cow<'a, str>>) -> Self {
        self.period = Some(period.into());
        self
    }
}

/// Request for currency-scoped option history.
#[derive(Debug, Clone, Serialize)]
pub struct OptionHistoryRequest<'a> {
    ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Cow<'a, str>>,
}

impl<'a> OptionHistoryRequest<'a> {
    /// Create an option-history query.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            period: None,
        }
    }

    /// Set the documented bar size.
    pub fn period(mut self, period: impl Into<Cow<'a, str>>) -> Self {
        self.period = Some(period.into());
        self
    }
}

/// Request for option open-interest and volume by strike.
#[derive(Debug, Clone, Serialize)]
pub struct OptionStrikeRequest<'a> {
    ccy: Cow<'a, str>,
    #[serde(rename = "expTime")]
    exp_time: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Cow<'a, str>>,
}

impl<'a> OptionStrikeRequest<'a> {
    /// Create a strike-history query for one expiry date (`YYYYMMDD`).
    pub fn new(ccy: impl Into<Cow<'a, str>>, exp_time: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            exp_time: exp_time.into(),
            period: None,
        }
    }

    /// Set the documented bar size.
    pub fn period(mut self, period: impl Into<Cow<'a, str>>) -> Self {
        self.period = Some(period.into());
        self
    }
}
