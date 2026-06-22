use serde::Serialize;

/// Request for [`get_status`](crate::api::status::Status::get_status).
#[derive(Debug, Clone, Default, Serialize)]
pub struct StatusRequest<'a> {
    /// Filter by maintenance state.
    ///
    /// Documented values: `scheduled`, `ongoing`, `pre_open`, `completed`, `canceled`.
    /// Omit to receive `scheduled`, `ongoing`, and `pre_open` events by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
}
