use serde::Serialize;

use crate::model::InstType;

/// Request for [`get_positions`](crate::api::account::Account::get_positions).
///
/// All fields are optional; omit both to return all open positions.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsRequest<'a> {
    /// Instrument type filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<&'a InstType>,
    /// Instrument ID filter (e.g. `"BTC-USDT"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<&'a str>,
}

/// Request for [`get_position_risk`](crate::api::account::Account::get_position_risk).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionRiskRequest<'a> {
    /// Instrument type filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<&'a InstType>,
}
