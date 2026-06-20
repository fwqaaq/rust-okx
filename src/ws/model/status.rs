//! System status channel model (`status`).
//!
//! Public channel; no authentication required.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

/// Public `status` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#status-websocket-status-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StatusUpdate {
    /// Title of the maintenance event.
    #[serde(default)]
    pub title: String,
    /// Current state of the event.
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
    /// URL for more information about the event.
    ///
    /// Empty string when not provided.
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
    /// System affected by the maintenance.
    ///
    /// Documented values: `unified` Trading account.
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
    /// Push time due to change event (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_docs_push_data_example() {
        let json = r#"{
            "begin": "1672823400000",
            "end": "1672825980000",
            "href": "",
            "preOpenBegin": "",
            "scheDesc": "",
            "serviceType": "0",
            "state": "completed",
            "system": "unified",
            "maintType": "1",
            "env": "1",
            "title": "Trading account WebSocket system upgrade",
            "ts": "1672826038470"
        }"#;

        let row: StatusUpdate = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(row.title, "Trading account WebSocket system upgrade");
        assert_eq!(row.state, "completed");
        assert_eq!(row.begin.as_str(), "1672823400000");
        assert_eq!(row.end.as_str(), "1672825980000");
        assert!(row.pre_open_begin.is_empty());
        assert_eq!(row.service_type, "0");
        assert_eq!(row.system, "unified");
        assert_eq!(row.maint_type, "1");
        assert_eq!(row.env, "1");
        assert_eq!(row.ts.as_str(), "1672826038470");
        assert!(row.extra.is_empty());
    }
}
