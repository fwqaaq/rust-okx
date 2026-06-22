use crate::client::OkxClient;
use crate::error::Error;
use crate::model::EmptyRequest;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the public support and announcement endpoints.
///
/// Obtain one via [`OkxClient::support`](crate::OkxClient::support).
pub struct Support<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Support<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve paginated announcements.
    ///
    /// `GET /api/v5/support/announcements`. Public (unauthenticated). The
    /// returned `Vec` always contains one [`AnnouncementsPage`]; the actual
    /// announcement list is in `[0].details`.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or
    /// [`RestError::Transport`](crate::RestError::Transport)/[`RestError::Decode`](crate::RestError::Decode) on transport/parsing failure.
    pub async fn get_announcements(
        &self,
        request: &AnnouncementsRequest<'_>,
    ) -> Result<Vec<AnnouncementsPage>, Error> {
        self.client.get(ANNOUNCEMENTS, request, false).await
    }

    /// Retrieve all available announcement types.
    ///
    /// `GET /api/v5/support/announcement-types`. Public (unauthenticated). Use
    /// the returned `ann_type` values to filter
    /// [`get_announcements`](Self::get_announcements).
    ///
    /// # Errors
    ///
    /// See [`get_announcements`](Self::get_announcements).
    pub async fn get_announcement_types(&self) -> Result<Vec<AnnouncementType>, Error> {
        self.client
            .get(ANNOUNCEMENT_TYPES, &EmptyRequest {}, false)
            .await
    }
}
