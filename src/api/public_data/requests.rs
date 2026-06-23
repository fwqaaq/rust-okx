use std::borrow::Cow;

use serde::Serialize;

use crate::model::InstType;

/// Request for [`get_instruments`](crate::api::public_data::PublicData::get_instruments).
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentsRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> InstrumentsRequest<'a> {
    /// Create an instruments query for an instrument type.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            inst_family: None,
        }
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Request for [`get_funding_rate`](crate::api::public_data::PublicData::get_funding_rate),
/// [`get_price_limit`](crate::api::public_data::PublicData::get_price_limit), and
/// [`get_estimated_price`](crate::api::public_data::PublicData::get_estimated_price).
#[derive(Debug, Clone, Serialize)]
pub struct InstIdRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
}

impl<'a> InstIdRequest<'a> {
    /// Create a query for one instrument ID.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
        }
    }
}

/// Request for [`get_discount_rate_interest_free_quota`](crate::api::public_data::PublicData::get_discount_rate_interest_free_quota).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CurrencyRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> CurrencyRequest<'a> {
    /// Create an unfiltered currency query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

mod edge;

pub use edge::*;

/// Query parameters for public endpoints filtered by instrument family.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentFamilyRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> InstrumentFamilyRequest<'a> {
    /// Create a query for an instrument type.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            underlying: None,
            inst_id: None,
            inst_family: None,
        }
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for funding-rate history.
#[derive(Debug, Clone, Serialize)]
pub struct FundingRateHistoryRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> FundingRateHistoryRequest<'a> {
    /// Create a funding-rate history query.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for delivery/exercise history.
#[derive(Debug, Clone, Serialize)]
pub struct DeliveryExerciseHistoryRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> DeliveryExerciseHistoryRequest<'a> {
    /// Create a delivery/exercise history query.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            underlying: None,
            inst_family: None,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for public position tiers.
#[derive(Debug, Clone, Serialize)]
pub struct PositionTiersRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "tdMode")]
    td_mode: Cow<'a, str>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<Cow<'a, str>>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> PositionTiersRequest<'a> {
    /// Create a position-tiers query.
    pub fn new(inst_type: InstType, td_mode: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_type,
            td_mode: td_mode.into(),
            underlying: None,
            inst_id: None,
            ccy: None,
            tier: None,
            inst_family: None,
        }
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the tier filter.
    pub fn tier(mut self, tier: impl Into<Cow<'a, str>>) -> Self {
        self.tier = Some(tier.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for insurance fund snapshots.
#[derive(Debug, Clone, Serialize)]
pub struct InsuranceFundRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    fund_type: Option<Cow<'a, str>>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> InsuranceFundRequest<'a> {
    /// Create an insurance fund query.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            fund_type: None,
            underlying: None,
            ccy: None,
            before: None,
            after: None,
            limit: None,
            inst_family: None,
        }
    }

    /// Set the OKX fund type filter.
    pub fn fund_type(mut self, fund_type: impl Into<Cow<'a, str>>) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for contract/coin conversion.
#[derive(Debug, Clone, Serialize)]
pub struct ConvertContractCoinRequest<'a> {
    #[serde(rename = "type")]
    conversion_type: Cow<'a, str>,
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    sz: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    px: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<Cow<'a, str>>,
}

impl<'a> ConvertContractCoinRequest<'a> {
    /// Create a contract/coin conversion query.
    pub fn new(
        conversion_type: impl Into<Cow<'a, str>>,
        inst_id: impl Into<Cow<'a, str>>,
        sz: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            conversion_type: conversion_type.into(),
            inst_id: inst_id.into(),
            sz: sz.into(),
            px: None,
            unit: None,
        }
    }

    /// Set the price used for conversion.
    pub fn price(mut self, px: impl Into<Cow<'a, str>>) -> Self {
        self.px = Some(px.into());
        self
    }

    /// Set the unit used for conversion.
    pub fn unit(mut self, unit: impl Into<Cow<'a, str>>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}
