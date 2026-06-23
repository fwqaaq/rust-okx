use std::borrow::Cow;

use serde::Serialize;

/// Request for [`get_announcements`](crate::api::support::Support::get_announcements).
#[derive(Debug, Clone, Default, Serialize)]
pub struct AnnouncementsRequest<'a> {
    #[serde(rename = "annType", skip_serializing_if = "Option::is_none")]
    ann_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u32>,
}

impl<'a> AnnouncementsRequest<'a> {
    /// Create an unfiltered announcements query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the announcement type filter.
    pub fn announcement_type(mut self, ann_type: impl Into<Cow<'a, str>>) -> Self {
        self.ann_type = Some(ann_type.into());
        self
    }

    /// Set the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}
