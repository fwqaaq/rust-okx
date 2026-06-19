//! Generic channel data-push response envelope.

use serde::Deserialize;

use crate::ws::Arg;

use super::ResponseExtraFields;

/// Generic channel data-push response body.
///
/// `T` is the channel row type from [`crate::ws::model`].
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PushResponse<T> {
    /// Subscription argument identifying the channel and filters.
    pub arg: Arg,
    /// Push action, commonly `snapshot` or `update`.
    ///
    /// Used by most channels. For the `positions` channel OKX uses
    /// [`Self::event_type`] instead.
    #[serde(default)]
    pub action: String,
    /// Event type used by the `positions` channel.
    ///
    /// Values: `snapshot` (initial / regular snapshot push) or
    /// `event_update` (event-driven update). Empty string for channels
    /// that use [`Self::action`] instead.
    #[serde(default)]
    pub event_type: String,
    /// Current page number for paginated snapshot pushes.
    ///
    /// Only present on `positions` snapshot events (`eventType = "snapshot"`).
    #[serde(default)]
    pub cur_page: Option<u32>,
    /// Whether this is the last page of a paginated snapshot push.
    ///
    /// Only present on `positions` snapshot events (`eventType = "snapshot"`).
    #[serde(default)]
    pub last_page: Option<bool>,
    /// Typed channel rows.
    #[serde(default)]
    pub data: Vec<T>,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}
