use serde::Serialize;

/// Optional instrument and pagination filters for private lead positions.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadPositionsRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Return records earlier than this lead-position ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this lead-position ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to place a stop order on a lead position.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadStopOrderRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead-position ID.
    pub sub_pos_id: String,
    /// Take-profit trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Stop-loss trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Take-profit order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss order price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Take-profit trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    /// Stop-loss trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Request to close a lead position.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseLeadPositionRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead-position ID.
    pub sub_pos_id: String,
    /// Closing order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<String>,
    /// Limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Optional instrument-type filter.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstTypeRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
}

/// Request to set lead instruments.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeadInstrumentsRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Comma-separated instrument IDs.
    pub inst_id: String,
}

/// Pagination filters for profit-sharing details.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfitSharingRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Return records earlier than this profit-sharing ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this profit-sharing ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Request to amend the lead-trader profit-sharing ratio.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendProfitSharingRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Profit-sharing ratio.
    pub profit_sharing_ratio: String,
}

/// First-time or amended copy-trading settings.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopySettingsRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead trader unique code.
    pub unique_code: String,
    /// Copy margin mode.
    pub copy_mgn_mode: String,
    /// Copy instrument selection mode.
    pub copy_inst_id_type: String,
    /// Comma-separated custom instrument IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Copy sizing mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_mode: Option<String>,
    /// Maximum total copy amount.
    pub copy_total_amt: String,
    /// Fixed amount per copied order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_amt: Option<String>,
    /// Copy ratio per order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_ratio: Option<String>,
    /// Take-profit ratio per order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ratio: Option<String>,
    /// Stop-loss ratio per order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ratio: Option<String>,
    /// Total stop-loss amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_total_amt: Option<String>,
    /// Action for open positions when copying stops.
    pub sub_pos_close_type: String,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Request to stop copying a lead trader.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StopCopyTradingRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead trader unique code.
    pub unique_code: String,
    /// Action for open copied positions.
    pub sub_pos_close_type: String,
}

/// Request selecting a lead trader.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadTraderRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead trader unique code.
    pub unique_code: String,
}

/// Filters for ranked lead traders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadTraderRanksRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Rank sort type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    /// Lead-trader state filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Minimum lead days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_lead_days: Option<String>,
    /// Minimum assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_assets: Option<String>,
    /// Maximum assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_assets: Option<String>,
    /// Minimum assets under management.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_aum: Option<String>,
    /// Maximum assets under management.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_aum: Option<String>,
    /// Data version for pagination consistency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_ver: Option<String>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Public lead-trader performance query.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeadTraderPerformanceRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead trader unique code.
    pub unique_code: String,
    /// Performance window in days.
    pub last_days: String,
}

/// Public lead-position pagination query.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicLeadPositionsRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead trader unique code.
    pub unique_code: String,
    /// Return records earlier than this lead-position ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Return records newer than this lead-position ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Public query for the copy traders of a lead trader.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyTradersRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Lead trader unique code.
    pub unique_code: String,
    /// Number of copy traders to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}
