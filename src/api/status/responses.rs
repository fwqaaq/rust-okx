use serde::Deserialize;

use crate::model::NumberString;

/// A single system-maintenance record returned by
/// [`get_status`](crate::api::status::Status::get_status).
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#rest-api-status-get-system-status>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StatusItem {
    /// Title of the system maintenance event.
    #[serde(default)]
    pub title: String,
    /// Current maintenance state.
    ///
    /// Documented values: `scheduled`, `ongoing`, `pre_open`, `completed`, `canceled`.
    #[serde(default)]
    pub state: String,
    /// Maintenance start time (Unix milliseconds).
    #[serde(default)]
    pub begin: NumberString,
    /// Maintenance end time (Unix milliseconds).
    ///
    /// Expected end time before `completed`; actual end time after `completed`.
    #[serde(default)]
    pub end: NumberString,
    /// Pre-open phase start time (Unix milliseconds).
    ///
    /// Empty string when the event has no pre-open phase.
    #[serde(default)]
    pub pre_open_begin: NumberString,
    /// URL for system maintenance details. Empty string when not provided.
    #[serde(default)]
    pub href: String,
    /// Affected service type.
    ///
    /// Documented values: `0` WebSocket; `5` Trading service; `6` Block trading;
    /// `7` Trading bot; `8` Trading service (in batches of accounts);
    /// `9` Trading service (in batches of products); `10` Spread trading;
    /// `11` Copy trading; `99` Others.
    #[serde(default)]
    pub service_type: String,
    /// System affected by the maintenance. Documented values: `unified` Trading account.
    #[serde(default)]
    pub system: String,
    /// Rescheduled description.
    #[serde(default)]
    pub sche_desc: String,
    /// Maintenance type.
    ///
    /// Documented values: `1` Scheduled maintenance; `2` Unscheduled maintenance;
    /// `3` System disruption.
    #[serde(default)]
    pub maint_type: String,
    /// Environment: `1` Production Trading; `2` Demo Trading.
    #[serde(default)]
    pub env: String,
}
