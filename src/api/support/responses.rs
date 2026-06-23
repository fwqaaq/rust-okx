use serde::Deserialize;

use crate::model::NumberString;

/// A single announcement entry within [`AnnouncementsPage::details`].
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AnnouncementDetail {
    /// Announcement title.
    #[serde(default)]
    pub title: String,
    /// Announcement type identifier.
    #[serde(default)]
    pub ann_type: String,
    /// Announcement URL.
    #[serde(default)]
    pub url: String,
    /// Actual publish time (Unix milliseconds). May be delayed ~5 minutes.
    #[serde(default)]
    pub p_time: NumberString,
    /// Display time shown on the announcement page (Unix milliseconds).
    #[serde(default)]
    pub business_p_time: NumberString,
}

/// Top-level element returned by
/// [`get_announcements`](crate::api::support::Support::get_announcements).
///
/// The OKX `data` array always contains exactly one object of this type.
/// The announcement list itself is in [`details`](AnnouncementsPage::details).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#rest-api-support-get-announcements>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AnnouncementsPage {
    /// Total number of pages available.
    #[serde(default)]
    pub total_page: String,
    /// Announcement entries for the requested page.
    #[serde(default)]
    pub details: Vec<AnnouncementDetail>,
}

/// An announcement type returned by
/// [`get_announcement_types`](crate::api::support::Support::get_announcement_types).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#rest-api-support-get-announcement-types>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AnnouncementType {
    /// Announcement type identifier (use as `ann_type` filter in
    /// [`AnnouncementsRequest`](crate::api::support::AnnouncementsRequest)).
    #[serde(default)]
    pub ann_type: String,
    /// Human-readable description of the announcement type.
    #[serde(default)]
    pub ann_type_desc: String,
}
