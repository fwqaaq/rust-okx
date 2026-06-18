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
    #[serde(default)]
    pub action: String,
    /// Typed channel rows.
    #[serde(default)]
    pub data: Vec<T>,
    /// Fields introduced by OKX after this crate version.
    #[serde(flatten, default)]
    pub extra: ResponseExtraFields,
}
