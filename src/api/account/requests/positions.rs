use std::borrow::Cow;

use serde::Serialize;

use crate::model::InstType;

/// Request for [`get_positions`](crate::api::account::Account::get_positions).
///
/// All fields are optional; omit both to return all open positions.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos_id: Option<Cow<'a, str>>,
}

impl<'a> PositionsRequest<'a> {
    /// Create an unfiltered positions query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the position ID filter.
    pub fn position_id(mut self, pos_id: impl Into<Cow<'a, str>>) -> Self {
        self.pos_id = Some(pos_id.into());
        self
    }
}

/// Request for [`get_position_risk`](crate::api::account::Account::get_position_risk).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
}

impl PositionRiskRequest {
    /// Create an unfiltered position-risk query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }
}
