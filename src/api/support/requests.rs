use serde::Serialize;

/// Request for [`get_announcements`](crate::api::support::Support::get_announcements).
#[derive(Debug, Clone, Default, Serialize)]
pub struct AnnouncementsRequest<'a> {
    /// Filter by announcement type (values from
    /// [`get_announcement_types`](crate::api::support::Support::get_announcement_types)).
    /// Omit to return all types.
    #[serde(rename = "annType", skip_serializing_if = "Option::is_none")]
    pub ann_type: Option<&'a str>,
    /// Page number for pagination. Default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}
