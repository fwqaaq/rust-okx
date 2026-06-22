use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the public system-status endpoints.
///
/// Obtain one via [`OkxClient::status`](crate::OkxClient::status).
pub struct Status<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Status<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve system maintenance status.
    ///
    /// `GET /api/v5/system/status`. Public (unauthenticated). Pass
    /// `state` to filter by a specific maintenance state; omit it to receive
    /// `scheduled`, `ongoing`, and `pre_open` events by default.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or
    /// [`RestError::Transport`](crate::RestError::Transport)/[`RestError::Decode`](crate::RestError::Decode) on transport/parsing failure.
    pub async fn get_status(&self, request: &StatusRequest<'_>) -> Result<Vec<StatusItem>, Error> {
        self.client.get(STATUS, request, false).await
    }
}
