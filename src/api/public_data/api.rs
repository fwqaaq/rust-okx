use crate::client::OkxClient;
use crate::error::Error;
use crate::model::{InstType, ValidateRequest};
use crate::transport::Transport;

use super::endpoints::*;
use super::internal::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the public reference-data endpoints.
///
/// Obtain one via [`OkxClient::public_data`](crate::OkxClient::public_data).
pub struct PublicData<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> PublicData<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve the list of tradable instruments.
    ///
    /// `GET /api/v5/public/instruments`
    ///
    /// `inst_family` is required for `FUTURES`, `SWAP`, and `OPTION` and ignored for `SPOT`/`MARGIN`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] if OKX rejects the request, or
    /// [`Error::Transport`]/[`Error::Decode`] on transport/parsing failure.
    pub async fn get_instruments(
        &self,
        inst_type: InstType,
        inst_family: Option<&str>,
    ) -> Result<Vec<Instrument>, Error> {
        let query = InstrumentsQuery {
            inst_type: &inst_type,
            inst_family,
        };
        self.client.get(INSTRUMENTS, &query, false).await
    }

    /// Retrieve OKX system time.
    ///
    /// `GET /api/v5/public/time`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] if OKX rejects the request, or transport/decode
    /// errors.
    pub async fn get_system_time(&self) -> Result<Vec<SystemTime>, Error> {
        self.client.get(SYSTEM_TIME, &NoQuery, false).await
    }

    /// Retrieve open interest.
    ///
    /// `GET /api/v5/public/open-interest`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_open_interest(
        &self,
        request: &InstrumentFamilyRequest,
    ) -> Result<Vec<OpenInterest>, Error> {
        self.client.get(OPEN_INTEREST, request, false).await
    }

    /// Retrieve the current funding rate for a derivatives instrument.
    ///
    /// `GET /api/v5/public/funding-rate`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_funding_rate(&self, inst_id: &str) -> Result<Vec<FundingRate>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(FUNDING_RATE, &query, false).await
    }

    /// Retrieve historical funding rates.
    ///
    /// `GET /api/v5/public/funding-rate-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_funding_rate_history(
        &self,
        request: &FundingRateHistoryRequest,
    ) -> Result<Vec<FundingRateHistory>, Error> {
        self.client.get(FUNDING_RATE_HISTORY, request, false).await
    }

    /// Retrieve the price limit for an instrument.
    ///
    /// `GET /api/v5/public/price-limit`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_price_limit(&self, inst_id: &str) -> Result<Vec<PriceLimit>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(PRICE_LIMIT, &query, false).await
    }

    /// Retrieve mark prices.
    ///
    /// `GET /api/v5/public/mark-price`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_mark_price(
        &self,
        request: &InstrumentFamilyRequest,
    ) -> Result<Vec<MarkPrice>, Error> {
        self.client.get(MARK_PRICE, request, false).await
    }

    /// Retrieve delivery/exercise history.
    ///
    /// `GET /api/v5/public/delivery-exercise-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_delivery_exercise_history(
        &self,
        request: &DeliveryExerciseHistoryRequest,
    ) -> Result<Vec<DeliveryExercise>, Error> {
        self.client
            .get(DELIVERY_EXERCISE_HISTORY, request, false)
            .await
    }

    /// Retrieve position tiers.
    ///
    /// `GET /api/v5/public/position-tiers`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_position_tiers(
        &self,
        request: &PositionTiersRequest,
    ) -> Result<Vec<PositionTier>, Error> {
        self.client.get(POSITION_TIERS, request, false).await
    }

    /// Retrieve underlying values for an instrument type.
    ///
    /// `GET /api/v5/public/underlying`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_underlying(
        &self,
        request: &UnderlyingRequest,
    ) -> Result<Vec<Vec<String>>, Error> {
        request.validate()?;
        self.client.get(UNDERLYING, request, false).await
    }

    /// Retrieve insurance-fund snapshots.
    ///
    /// `GET /api/v5/public/insurance-fund`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_insurance_fund(
        &self,
        request: &InsuranceFundRequest,
    ) -> Result<Vec<InsuranceFund>, Error> {
        self.client.get(INSURANCE_FUND, request, false).await
    }

    /// Convert between contract count and coin amount.
    ///
    /// `GET /api/v5/public/convert-contract-coin`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_convert_contract_coin(
        &self,
        request: &ConvertContractCoinRequest,
    ) -> Result<Vec<ConvertContractCoin>, Error> {
        self.client.get(CONVERT_CONTRACT_COIN, request, false).await
    }

    /// Retrieve option summary data.
    ///
    /// `GET /api/v5/public/opt-summary`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_option_summary(
        &self,
        request: &OptionSummaryRequest,
    ) -> Result<Vec<OptionSummary>, Error> {
        request.validate()?;
        self.client.get(OPTION_SUMMARY, request, false).await
    }

    /// Retrieve the estimated delivery/exercise price for an instrument.
    ///
    /// `GET /api/v5/public/estimated-price`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_estimated_price(&self, inst_id: &str) -> Result<Vec<EstimatedPrice>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(ESTIMATED_PRICE, &query, false).await
    }

    /// Retrieve discount-rate and interest-free quota data.
    ///
    /// `GET /api/v5/public/discount-rate-interest-free-quota`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_discount_rate_interest_free_quota(
        &self,
        ccy: Option<&str>,
    ) -> Result<Vec<DiscountRateInterestFreeQuota>, Error> {
        let query = CurrencyQuery { ccy };
        self.client
            .get(DISCOUNT_RATE_INTEREST_FREE_QUOTA, &query, false)
            .await
    }

    /// Retrieve interest-rate loan quota data.
    ///
    /// `GET /api/v5/public/interest-rate-loan-quota`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_interest_rate_loan_quota(
        &self,
        request: &InterestRateLoanQuotaRequest,
    ) -> Result<Vec<InterestRateLoanQuota>, Error> {
        request.validate()?;
        self.client
            .get(INTEREST_RATE_LOAN_QUOTA, request, false)
            .await
    }

    /// Retrieve option tick bands.
    ///
    /// `GET /api/v5/public/instrument-tick-bands`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_instrument_tick_bands(
        &self,
        request: &InstrumentTickBandsRequest,
    ) -> Result<Vec<InstrumentTickBand>, Error> {
        request.validate()?;
        self.client.get(INSTRUMENT_TICK_BANDS, request, false).await
    }

    /// Retrieve public option trade data.
    ///
    /// `GET /api/v5/public/option-trades`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_option_trades(
        &self,
        request: &PublicOptionTradesRequest,
    ) -> Result<Vec<PublicOptionTrade>, Error> {
        request.validate()?;
        self.client.get(OPTION_TRADES, request, false).await
    }

    /// Retrieve public market-data history.
    ///
    /// `GET /api/v5/public/market-data-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_market_data_history(
        &self,
        request: &MarketDataHistoryRequest,
    ) -> Result<Vec<MarketDataHistory>, Error> {
        request.validate()?;
        self.client.get(MARKET_DATA_HISTORY, request, false).await
    }
}
