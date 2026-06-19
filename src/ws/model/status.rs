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
    /// Unique ID of the maintenance event.
    #[serde(default)]
    pub id: String,
    /// Title of the maintenance event.
    #[serde(default)]
    pub title: String,
    /// Current state of the event.
    ///
    /// Documented values: `scheduled`, `ongoing`, `pre_open`, `completed`.
    #[serde(default)]
    pub state: String,
    /// Maintenance start time (Unix milliseconds).
    #[serde(default)]
    pub begin: NumberString,
    /// Maintenance end time (Unix milliseconds).
    #[serde(default)]
    pub end: NumberString,
    /// Pre-open phase start time (Unix milliseconds).
    ///
    /// Empty string when the event has no pre-open phase.
    #[serde(default)]
    pub pre_open_begin: NumberString,
    /// URL for more information about the event.
    #[serde(default)]
    pub href: String,
    /// Affected service type.
    ///
    /// Documented values: `0` WebSocket, `1` Spot/Leverage, `2` Futures,
    /// `3` Perpetual, `4` Options, `5` Unified account trading service.
    #[serde(default)]
    pub service_type: String,
    /// System affected by the maintenance.
    ///
    /// Documented values: `classic_system`, `unified_system`.
    #[serde(default)]
    pub system: String,
    /// Human-readable description of the maintenance schedule (English).
    #[serde(default)]
    pub sche_desc: String,
    /// Maintenance type.
    ///
    /// Documented values: `0` scheduled, `1` unscheduled.
    #[serde(default)]
    pub maint_type: String,
    /// Environment: `1` production.
    #[serde(default)]
    pub env: String,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
