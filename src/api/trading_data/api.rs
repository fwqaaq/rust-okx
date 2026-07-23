use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the public trading-statistics endpoints.
///
/// Obtain one via [`OkxClient::trading_data`](crate::OkxClient::trading_data).
pub struct TradingData<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> TradingData<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve currencies supported by trading statistics.
    ///
    /// `GET /api/v5/rubik/stat/trading-data/support-coin`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_support_coins(&self) -> Result<TradingDataSupportCoins, Error> {
        self.client.get(SUPPORT_COINS, &(), false).await
    }

    /// Retrieve contract open-interest history.
    ///
    /// `GET /api/v5/rubik/stat/contracts/open-interest-history`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_contract_open_interest_history(
        &self,
        request: &InstrumentHistoryRequest<'_>,
    ) -> Result<Vec<ContractOpenInterestHistory>, Error> {
        self.client
            .get(CONTRACT_OPEN_INTEREST_HISTORY, request, false)
            .await
    }

    /// Retrieve aggregate taker volume.
    ///
    /// `GET /api/v5/rubik/stat/taker-volume`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_taker_volume(
        &self,
        request: &TakerVolumeRequest<'_>,
    ) -> Result<Vec<TakerVolume>, Error> {
        self.client.get(TAKER_VOLUME, request, false).await
    }

    /// Retrieve contract taker volume.
    ///
    /// `GET /api/v5/rubik/stat/taker-volume-contract`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_contract_taker_volume(
        &self,
        request: &ContractTakerVolumeRequest<'_>,
    ) -> Result<Vec<TakerVolume>, Error> {
        self.client.get(CONTRACT_TAKER_VOLUME, request, false).await
    }

    /// Retrieve the margin long/short loan ratio.
    ///
    /// `GET /api/v5/rubik/stat/margin/loan-ratio`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_margin_loan_ratio(
        &self,
        request: &CurrencyHistoryRequest<'_>,
    ) -> Result<Vec<RatioPoint>, Error> {
        self.client.get(MARGIN_LOAN_RATIO, request, false).await
    }

    /// Retrieve the long/short account ratio for top traders.
    ///
    /// `GET /api/v5/rubik/stat/contracts/long-short-account-ratio-contract-top-trader`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_top_trader_account_ratio(
        &self,
        request: &InstrumentHistoryRequest<'_>,
    ) -> Result<Vec<RatioPoint>, Error> {
        self.client
            .get(TOP_TRADER_ACCOUNT_RATIO, request, false)
            .await
    }

    /// Retrieve the long/short position ratio for top traders.
    ///
    /// `GET /api/v5/rubik/stat/contracts/long-short-position-ratio-contract-top-trader`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_top_trader_position_ratio(
        &self,
        request: &InstrumentHistoryRequest<'_>,
    ) -> Result<Vec<RatioPoint>, Error> {
        self.client
            .get(TOP_TRADER_POSITION_RATIO, request, false)
            .await
    }

    /// Retrieve the account long/short ratio for one contract.
    ///
    /// `GET /api/v5/rubik/stat/contracts/long-short-account-ratio-contract`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_contract_long_short_account_ratio(
        &self,
        request: &InstrumentHistoryRequest<'_>,
    ) -> Result<Vec<RatioPoint>, Error> {
        self.client
            .get(CONTRACT_LONG_SHORT_ACCOUNT_RATIO, request, false)
            .await
    }

    /// Retrieve the currency-wide long/short account ratio.
    ///
    /// `GET /api/v5/rubik/stat/contracts/long-short-account-ratio`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_long_short_account_ratio(
        &self,
        request: &CurrencyHistoryRequest<'_>,
    ) -> Result<Vec<RatioPoint>, Error> {
        self.client
            .get(LONG_SHORT_ACCOUNT_RATIO, request, false)
            .await
    }

    /// Retrieve contracts open interest and volume.
    ///
    /// `GET /api/v5/rubik/stat/contracts/open-interest-volume`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_contract_open_interest_volume(
        &self,
        request: &CurrencyHistoryRequest<'_>,
    ) -> Result<Vec<OpenInterestVolume>, Error> {
        self.client
            .get(CONTRACT_OPEN_INTEREST_VOLUME, request, false)
            .await
    }

    /// Retrieve options open interest and volume.
    ///
    /// `GET /api/v5/rubik/stat/option/open-interest-volume`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_option_open_interest_volume(
        &self,
        request: &OptionHistoryRequest<'_>,
    ) -> Result<Vec<OpenInterestVolume>, Error> {
        self.client
            .get(OPTION_OPEN_INTEREST_VOLUME, request, false)
            .await
    }

    /// Retrieve option put/call open-interest and volume ratios.
    ///
    /// `GET /api/v5/rubik/stat/option/open-interest-volume-ratio`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_option_put_call_ratio(
        &self,
        request: &OptionHistoryRequest<'_>,
    ) -> Result<Vec<PutCallRatio>, Error> {
        self.client.get(OPTION_PUT_CALL_RATIO, request, false).await
    }

    /// Retrieve option open interest and volume by expiry.
    ///
    /// `GET /api/v5/rubik/stat/option/open-interest-volume-expiry`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_option_open_interest_volume_expiry(
        &self,
        request: &OptionHistoryRequest<'_>,
    ) -> Result<Vec<OptionExpiryVolume>, Error> {
        self.client
            .get(OPTION_OPEN_INTEREST_VOLUME_EXPIRY, request, false)
            .await
    }

    /// Retrieve option open interest and volume by strike.
    ///
    /// `GET /api/v5/rubik/stat/option/open-interest-volume-strike`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_option_open_interest_volume_strike(
        &self,
        request: &OptionStrikeRequest<'_>,
    ) -> Result<Vec<OptionStrikeVolume>, Error> {
        self.client
            .get(OPTION_OPEN_INTEREST_VOLUME_STRIKE, request, false)
            .await
    }

    /// Retrieve the current option taker flow.
    ///
    /// `GET /api/v5/rubik/stat/option/taker-block-volume`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_option_taker_flow(
        &self,
        request: &OptionHistoryRequest<'_>,
    ) -> Result<OptionTakerFlow, Error> {
        self.client.get(OPTION_TAKER_FLOW, request, false).await
    }
}
