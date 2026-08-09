use crate::client::OkxClient;
use crate::error::Error;
use crate::model::Page;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for authenticated Affiliate endpoints.
///
/// Obtain one via [`OkxClient::affiliate`](crate::OkxClient::affiliate).
pub struct Affiliate<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Affiliate<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve aggregate affiliate performance.
    ///
    /// `GET /api/v5/affiliate/performance/summary`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_performance_summary(
        &self,
        request: &PerformanceSummaryRequest,
    ) -> Result<Vec<AffiliatePerformanceSummary>, Error> {
        self.client.get(PERFORMANCE_SUMMARY, request, true).await
    }

    /// Retrieve detailed metrics for one invitee.
    ///
    /// `GET /api/v5/affiliate/invitee/detail`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_invitee_detail(
        &self,
        request: &InviteeDetailRequest,
    ) -> Result<Vec<InviteeDetail>, Error> {
        self.client.get(INVITEE_DETAIL, request, true).await
    }

    /// Retrieve a paginated invitee list.
    ///
    /// `GET /api/v5/affiliate/invitee/list`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_invitees(
        &self,
        request: &InviteeListRequest,
    ) -> Result<Page<Vec<Invitee>>, Error> {
        self.client.get_page(INVITEE_LIST, request, true).await
    }

    /// Retrieve paginated affiliate invitation links.
    ///
    /// `GET /api/v5/affiliate/link/list`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_links(
        &self,
        request: &LinkListRequest,
    ) -> Result<Page<Vec<AffiliateLink>>, Error> {
        self.client.get_page(LINK_LIST, request, true).await
    }

    /// Retrieve paginated co-inviter links.
    ///
    /// `GET /api/v5/affiliate/co-inviter/list`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_co_inviter_links(
        &self,
        request: &CoInviterListRequest,
    ) -> Result<Page<Vec<CoInviterLink>>, Error> {
        self.client.get_page(CO_INVITER_LIST, request, true).await
    }

    /// Retrieve paginated sub-affiliate accounts.
    ///
    /// `GET /api/v5/affiliate/sub-affiliate/list`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_sub_affiliates(
        &self,
        request: &SubAffiliateListRequest,
    ) -> Result<Page<Vec<SubAffiliate>>, Error> {
        self.client
            .get_page(SUB_AFFILIATE_LIST, request, true)
            .await
    }
}
