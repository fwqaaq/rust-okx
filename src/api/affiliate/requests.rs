use serde::Serialize;

/// Time range for an affiliate performance summary.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceSummaryRequest {
    /// Period type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<String>,
    /// Inclusive range start timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// Inclusive range end timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Request selecting one invitee.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteeDetailRequest {
    /// Invitee user ID.
    pub uid: String,
}

/// Filters for the paginated invitee list.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteeListRequest {
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Period type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<String>,
    /// Inclusive range start timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// Inclusive range end timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// User ID, email, or phone search keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Commission category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_category: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_dir: Option<String>,
    /// KYC status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kyc_status: Option<String>,
    /// Sub-affiliate user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_affiliate_uid: Option<String>,
}

/// Filters for affiliate invitation links.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkListRequest {
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Link type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    /// Link status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_status: Option<String>,
}

/// Filters for co-inviter links.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoInviterListRequest {
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Link status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_status: Option<String>,
}

/// Filters for sub-affiliate accounts.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAffiliateListRequest {
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// User ID, email, or phone search keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    /// Commission category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_category: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_dir: Option<String>,
}
