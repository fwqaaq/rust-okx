use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for Copy Trading endpoints.
///
/// Obtain one via [`OkxClient::copy_trading`](crate::OkxClient::copy_trading).
pub struct CopyTrading<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> CopyTrading<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve existing lead positions.
    ///
    /// `GET /api/v5/copytrading/current-subpositions`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_current_subpositions(
        &self,
        request: &LeadPositionsRequest,
    ) -> Result<Vec<LeadPosition>, Error> {
        self.client.get(CURRENT_SUBPOSITIONS, request, true).await
    }

    /// Retrieve lead position history.
    ///
    /// `GET /api/v5/copytrading/subpositions-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_subpositions_history(
        &self,
        request: &LeadPositionsRequest,
    ) -> Result<Vec<LeadPosition>, Error> {
        self.client.get(SUBPOSITIONS_HISTORY, request, true).await
    }

    /// Place a take-profit or stop-loss order for a lead position.
    ///
    /// `POST /api/v5/copytrading/algo-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn place_lead_stop_order(
        &self,
        request: &LeadStopOrderRequest,
    ) -> Result<Vec<LeadPositionAction>, Error> {
        self.client.post(ALGO_ORDER, request, true).await
    }

    /// Close a lead position.
    ///
    /// `POST /api/v5/copytrading/close-subposition`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn close_lead_position(
        &self,
        request: &CloseLeadPositionRequest,
    ) -> Result<Vec<LeadPositionAction>, Error> {
        self.client.post(CLOSE_SUBPOSITION, request, true).await
    }

    /// Retrieve instruments available for lead trading.
    ///
    /// `GET /api/v5/copytrading/instruments`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_instruments(
        &self,
        request: &InstTypeRequest,
    ) -> Result<Vec<LeadInstrument>, Error> {
        self.client.get(INSTRUMENTS, request, true).await
    }

    /// Set lead-trading instruments.
    ///
    /// `POST /api/v5/copytrading/set-instruments`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn set_instruments(
        &self,
        request: &SetLeadInstrumentsRequest,
    ) -> Result<Vec<LeadInstrument>, Error> {
        self.client.post(SET_INSTRUMENTS, request, true).await
    }

    /// Retrieve realized profit-sharing details.
    ///
    /// `GET /api/v5/copytrading/profit-sharing-details`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_profit_sharing_details(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<Vec<ProfitSharingDetail>, Error> {
        self.client.get(PROFIT_SHARING_DETAILS, request, true).await
    }

    /// Retrieve total realized profit sharing.
    ///
    /// `GET /api/v5/copytrading/total-profit-sharing`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_total_profit_sharing(
        &self,
        request: &InstTypeRequest,
    ) -> Result<Vec<TotalProfitSharing>, Error> {
        self.client.get(TOTAL_PROFIT_SHARING, request, true).await
    }

    /// Retrieve unrealized profit-sharing details.
    ///
    /// `GET /api/v5/copytrading/unrealized-profit-sharing-details`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_unrealized_profit_sharing_details(
        &self,
        request: &InstTypeRequest,
    ) -> Result<Vec<UnrealizedProfitSharingDetail>, Error> {
        self.client
            .get(UNREALIZED_PROFIT_SHARING_DETAILS, request, true)
            .await
    }

    /// Retrieve total unrealized profit sharing.
    ///
    /// `GET /api/v5/copytrading/total-unrealized-profit-sharing`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_total_unrealized_profit_sharing(
        &self,
        request: &InstTypeRequest,
    ) -> Result<Vec<TotalUnrealizedProfitSharing>, Error> {
        self.client
            .get(TOTAL_UNREALIZED_PROFIT_SHARING, request, true)
            .await
    }

    /// Amend the lead-trader profit-sharing ratio.
    ///
    /// `POST /api/v5/copytrading/amend-profit-sharing-ratio`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_profit_sharing_ratio(
        &self,
        request: &AmendProfitSharingRequest,
    ) -> Result<Vec<CopySettingResult>, Error> {
        self.client
            .post(AMEND_PROFIT_SHARING_RATIO, request, true)
            .await
    }

    /// Retrieve Copy Trading account configuration.
    ///
    /// `GET /api/v5/copytrading/config`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_config(&self) -> Result<Vec<CopyAccountConfig>, Error> {
        self.client.get(CONFIG, &(), true).await
    }

    /// Set first-time Copy Trading settings.
    ///
    /// `POST /api/v5/copytrading/first-copy-settings`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn set_first_copy_settings(
        &self,
        request: &CopySettingsRequest,
    ) -> Result<Vec<CopySettingResult>, Error> {
        self.client.post(FIRST_COPY_SETTINGS, request, true).await
    }

    /// Amend Copy Trading settings.
    ///
    /// `POST /api/v5/copytrading/amend-copy-settings`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn amend_copy_settings(
        &self,
        request: &CopySettingsRequest,
    ) -> Result<Vec<CopySettingResult>, Error> {
        self.client.post(AMEND_COPY_SETTINGS, request, true).await
    }

    /// Stop copying a lead trader.
    ///
    /// `POST /api/v5/copytrading/stop-copy-trading`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn stop_copy_trading(
        &self,
        request: &StopCopyTradingRequest,
    ) -> Result<Vec<CopySettingResult>, Error> {
        self.client.post(STOP_COPY_TRADING, request, true).await
    }

    /// Retrieve settings for copying one lead trader.
    ///
    /// `GET /api/v5/copytrading/copy-settings`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_copy_settings(
        &self,
        request: &LeadTraderRequest,
    ) -> Result<Vec<CopySettings>, Error> {
        self.client.get(COPY_SETTINGS, request, true).await
    }

    /// Retrieve the account's current lead traders.
    ///
    /// `GET /api/v5/copytrading/current-lead-traders`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_current_lead_traders(
        &self,
        request: &InstTypeRequest,
    ) -> Result<Vec<CurrentLeadTrader>, Error> {
        self.client.get(CURRENT_LEAD_TRADERS, request, true).await
    }

    /// Retrieve public Copy Trading limits.
    ///
    /// `GET /api/v5/copytrading/public-config`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_config(
        &self,
        request: &InstTypeRequest,
    ) -> Result<Vec<CopyTradingConfig>, Error> {
        self.client.get(PUBLIC_CONFIG, request, false).await
    }

    /// Retrieve ranked public lead traders.
    ///
    /// `GET /api/v5/copytrading/public-lead-traders`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_lead_traders(
        &self,
        request: &LeadTraderRanksRequest,
    ) -> Result<Vec<LeadTraderRanks>, Error> {
        self.client.get(PUBLIC_LEAD_TRADERS, request, false).await
    }

    /// Retrieve weekly public profit and loss for a lead trader.
    ///
    /// `GET /api/v5/copytrading/public-weekly-pnl`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_weekly_pnl(
        &self,
        request: &LeadTraderRequest,
    ) -> Result<Vec<LeadTraderPnl>, Error> {
        self.client.get(PUBLIC_WEEKLY_PNL, request, false).await
    }

    /// Retrieve daily public profit and loss for a lead trader.
    ///
    /// `GET /api/v5/copytrading/public-pnl`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_pnl(
        &self,
        request: &LeadTraderPerformanceRequest,
    ) -> Result<Vec<LeadTraderPnl>, Error> {
        self.client.get(PUBLIC_PNL, request, false).await
    }

    /// Retrieve public aggregate statistics for a lead trader.
    ///
    /// `GET /api/v5/copytrading/public-stats`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_stats(
        &self,
        request: &LeadTraderPerformanceRequest,
    ) -> Result<Vec<LeadTraderStats>, Error> {
        self.client.get(PUBLIC_STATS, request, false).await
    }

    /// Retrieve public lead-trader currency preferences.
    ///
    /// `GET /api/v5/copytrading/public-preference-currency`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_preference_currency(
        &self,
        request: &LeadTraderRequest,
    ) -> Result<Vec<CurrencyPreference>, Error> {
        self.client
            .get(PUBLIC_PREFERENCE_CURRENCY, request, false)
            .await
    }

    /// Retrieve a lead trader's public current lead positions.
    ///
    /// `GET /api/v5/copytrading/public-current-subpositions`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_current_subpositions(
        &self,
        request: &PublicLeadPositionsRequest,
    ) -> Result<Vec<LeadPosition>, Error> {
        self.client
            .get(PUBLIC_CURRENT_SUBPOSITIONS, request, false)
            .await
    }

    /// Retrieve a lead trader's public lead-position history.
    ///
    /// `GET /api/v5/copytrading/public-subpositions-history`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_subpositions_history(
        &self,
        request: &PublicLeadPositionsRequest,
    ) -> Result<Vec<LeadPosition>, Error> {
        self.client
            .get(PUBLIC_SUBPOSITIONS_HISTORY, request, false)
            .await
    }

    /// Retrieve public copy-trader information for a lead trader.
    ///
    /// `GET /api/v5/copytrading/public-copy-traders`. Public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error response.
    pub async fn get_public_copy_traders(
        &self,
        request: &CopyTradersRequest,
    ) -> Result<Vec<CopyTraders>, Error> {
        self.client.get(PUBLIC_COPY_TRADERS, request, false).await
    }
}
