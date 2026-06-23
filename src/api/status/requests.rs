use std::borrow::Cow;

use serde::Serialize;

/// Request for [`get_status`](crate::api::status::Status::get_status).
#[derive(Debug, Clone, Default, Serialize)]
pub struct StatusRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
}

impl<'a> StatusRequest<'a> {
    /// Create an unfiltered status query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the maintenance-state filter.
    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }
}
