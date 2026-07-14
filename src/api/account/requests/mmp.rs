use std::borrow::Cow;

use serde::Serialize;

use crate::model::InstType;

/// Request body for [`set_mmp_config`](crate::api::account::Account::set_mmp_config).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpConfigRequest<'a> {
    inst_family: Cow<'a, str>,
    time_interval: Cow<'a, str>,
    frozen_interval: Cow<'a, str>,
    qty_limit: Cow<'a, str>,
}

impl<'a> SetMmpConfigRequest<'a> {
    /// Create a Market Maker Protection configuration request.
    pub fn new(
        inst_family: impl Into<Cow<'a, str>>,
        time_interval: impl Into<Cow<'a, str>>,
        frozen_interval: impl Into<Cow<'a, str>>,
        qty_limit: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            inst_family: inst_family.into(),
            time_interval: time_interval.into(),
            frozen_interval: frozen_interval.into(),
            qty_limit: qty_limit.into(),
        }
    }
}

/// Request body for [`reset_mmp_status`](crate::api::account::Account::reset_mmp_status).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetMmpStatusRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    inst_family: Cow<'a, str>,
}

impl<'a> ResetMmpStatusRequest<'a> {
    /// Create a reset request for an instrument family.
    ///
    /// When `inst_type` is omitted, OKX defaults it to `OPTION`.
    pub fn new(inst_family: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_type: None,
            inst_family: inst_family.into(),
        }
    }

    /// Set the instrument type explicitly.
    pub fn instrument_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }
}
