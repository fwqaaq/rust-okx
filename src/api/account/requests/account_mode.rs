use std::borrow::Cow;

use serde::Serialize;

/// Query parameters for
/// [`precheck_set_delta_neutral`](crate::api::account::Account::precheck_set_delta_neutral).
#[derive(Debug, Clone, Serialize)]
pub struct PrecheckSetDeltaNeutralRequest<'a> {
    #[serde(rename = "stgyType")]
    stgy_type: Cow<'a, str>,
}

impl<'a> PrecheckSetDeltaNeutralRequest<'a> {
    /// Create a delta-neutral strategy precheck.
    ///
    /// OKX currently accepts `0` for general mode and `1` for delta-neutral mode.
    pub fn new(stgy_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            stgy_type: stgy_type.into(),
        }
    }
}
