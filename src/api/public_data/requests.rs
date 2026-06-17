use serde::Serialize;

use crate::model::InstType;

mod edge;

pub use edge::*;
/// Query parameters for public endpoints filtered by instrument family.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentFamilyRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl InstrumentFamilyRequest {
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
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for funding-rate history.
#[derive(Debug, Clone, Serialize)]
pub struct FundingRateHistoryRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FundingRateHistoryRequest {
    /// Create a funding-rate history query.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
            limit: None,
        }
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

/// Query parameters for delivery/exercise history.
#[derive(Debug, Clone, Serialize)]
pub struct DeliveryExerciseHistoryRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl DeliveryExerciseHistoryRequest {
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
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
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

/// Query parameters for public position tiers.
#[derive(Debug, Clone, Serialize)]
pub struct PositionTiersRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "tdMode")]
    td_mode: String,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl PositionTiersRequest {
    /// Create a position-tiers query.
    pub fn new(inst_type: InstType, td_mode: impl Into<String>) -> Self {
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
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
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

    /// Set the tier filter.
    pub fn tier(mut self, tier: impl Into<String>) -> Self {
        self.tier = Some(tier.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for insurance fund snapshots.
#[derive(Debug, Clone, Serialize)]
pub struct InsuranceFundRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    fund_type: Option<String>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl InsuranceFundRequest {
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
    pub fn fund_type(mut self, fund_type: impl Into<String>) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for contract/coin conversion.
#[derive(Debug, Clone, Serialize)]
pub struct ConvertContractCoinRequest {
    #[serde(rename = "type")]
    conversion_type: String,
    #[serde(rename = "instId")]
    inst_id: String,
    sz: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
}

impl ConvertContractCoinRequest {
    /// Create a contract/coin conversion query.
    pub fn new(
        conversion_type: impl Into<String>,
        inst_id: impl Into<String>,
        sz: impl Into<String>,
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
    pub fn price(mut self, px: impl Into<String>) -> Self {
        self.px = Some(px.into());
        self
    }

    /// Set the unit used for conversion.
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}
