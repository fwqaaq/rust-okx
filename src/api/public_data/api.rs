use crate::client::OkxClient;
use crate::error::Error;
use crate::model::EmptyRequest;
use crate::transport::Transport;

use super::endpoints::*;
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
    /// `GET /api/v5/public/instruments`. Public.
    ///
    /// `inst_family` is required for `FUTURES`, `SWAP`, and `OPTION` and ignored for `SPOT`/`MARGIN`.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Okx`](crate::RestError::Okx) if OKX rejects the request, or
    /// [`RestError::Transport`](crate::RestError::Transport)/[`RestError::Decode`](crate::RestError::Decode) on transport/parsing failure.
    pub async fn get_instruments(
        &self,
        request: &InstrumentsRequest<'_>,
    ) -> Result<Vec<Instrument>, Error> {
        self.client.get(INSTRUMENTS, request, false).await
    }

    /// Retrieve OKX system time.
    ///
    /// `GET /api/v5/public/time`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Okx`](crate::RestError::Okx) if OKX rejects the request, or transport/decode
    /// errors.
    pub async fn get_system_time(&self) -> Result<Vec<SystemTime>, Error> {
        self.client.get(SYSTEM_TIME, &EmptyRequest {}, false).await
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
        request: &InstrumentFamilyRequest<'_>,
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
    pub async fn get_funding_rate(
        &self,
        request: &InstIdRequest<'_>,
    ) -> Result<Vec<FundingRate>, Error> {
        self.client.get(FUNDING_RATE, request, false).await
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
        request: &FundingRateHistoryRequest<'_>,
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
    pub async fn get_price_limit(
        &self,
        request: &InstIdRequest<'_>,
    ) -> Result<Vec<PriceLimit>, Error> {
        self.client.get(PRICE_LIMIT, request, false).await
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
        request: &InstrumentFamilyRequest<'_>,
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
        request: &DeliveryExerciseHistoryRequest<'_>,
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
        request: &PositionTiersRequest<'_>,
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
        request: &UnderlyingRequest<'_>,
    ) -> Result<Vec<Vec<String>>, Error> {
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
        request: &InsuranceFundRequest<'_>,
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
        request: &ConvertContractCoinRequest<'_>,
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
        request: &OptionSummaryRequest<'_>,
    ) -> Result<Vec<OptionSummary>, Error> {
        self.client.get(OPTION_SUMMARY, request, false).await
    }

    /// Retrieve the estimated delivery/exercise price for an instrument.
    ///
    /// `GET /api/v5/public/estimated-price`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_estimated_price(
        &self,
        request: &InstIdRequest<'_>,
    ) -> Result<Vec<EstimatedPrice>, Error> {
        self.client.get(ESTIMATED_PRICE, request, false).await
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
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<DiscountRateInterestFreeQuota>, Error> {
        self.client
            .get(DISCOUNT_RATE_INTEREST_FREE_QUOTA, request, false)
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
        request: &InstrumentTickBandsRequest<'_>,
    ) -> Result<Vec<InstrumentTickBand>, Error> {
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
        request: &PublicOptionTradesRequest<'_>,
    ) -> Result<Vec<PublicOptionTrade>, Error> {
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
        request: &MarketDataHistoryRequest<'_>,
    ) -> Result<Vec<MarketDataHistory>, Error> {
        self.client.get(MARKET_DATA_HISTORY, request, false).await
    }

    /// Retrieve MM Program instrument-type classifications.
    ///
    /// `GET /api/v5/public/mm-instrument-types`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_mm_instrument_types(
        &self,
        request: &MmInstrumentTypesRequest<'_>,
    ) -> Result<Vec<MmInstrumentType>, Error> {
        self.client.get(MM_INSTRUMENT_TYPES, request, false).await
    }

    /// Retrieve macro-economic calendar data within the last three months.
    ///
    /// `GET /api/v5/public/economic-calendar`. Authenticated: this endpoint
    /// requires signing and is only supported in the production environment.
    /// Historical data older than three months requires trading fee tier VIP1
    /// or above.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_economic_calendar(
        &self,
        request: &EconomicCalendarRequest<'_>,
    ) -> Result<Vec<EconomicCalendar>, Error> {
        self.client.get(ECONOMIC_CALENDAR, request, true).await
    }

    /// Retrieve swap premium history from the past six months.
    ///
    /// `GET /api/v5/public/premium-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_premium_history(
        &self,
        request: &PremiumHistoryRequest<'_>,
    ) -> Result<Vec<PremiumHistory>, Error> {
        self.client.get(PREMIUM_HISTORY, request, false).await
    }

    /// Retrieve prediction-market series metadata.
    ///
    /// `GET /api/v5/public/event-contract/series`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_event_contract_series(
        &self,
        request: &EventContractSeriesRequest<'_>,
    ) -> Result<Vec<EventContractSeries>, Error> {
        self.client.get(EVENT_CONTRACT_SERIES, request, false).await
    }

    /// Retrieve prediction-market events.
    ///
    /// `GET /api/v5/public/event-contract/events`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_event_contract_events(
        &self,
        request: &EventContractEventsRequest<'_>,
    ) -> Result<Vec<EventContractEvent>, Error> {
        self.client.get(EVENT_CONTRACT_EVENTS, request, false).await
    }

    /// Retrieve prediction-market instruments.
    ///
    /// `GET /api/v5/public/event-contract/markets`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_event_contract_markets(
        &self,
        request: &EventContractMarketsRequest<'_>,
    ) -> Result<Vec<EventContractMarket>, Error> {
        self.client.get(EVENT_CONTRACT_MARKETS, request, false).await
    }

    /// Retrieve delayed public single-leg block trades for an instrument.
    ///
    /// `GET /api/v5/public/block-trades`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_public_block_trades(
        &self,
        request: &InstIdRequest<'_>,
    ) -> Result<Vec<PublicBlockTrade>, Error> {
        self.client.get(BLOCK_TRADES, request, false).await
    }

    /// Retrieve the estimated futures settlement price.
    ///
    /// `GET /api/v5/public/estimated-settlement-info`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_estimated_settlement_info(
        &self,
        request: &InstIdRequest<'_>,
    ) -> Result<Vec<EstimatedSettlementInfo>, Error> {
        self.client
            .get(ESTIMATED_SETTLEMENT_INFO, request, false)
            .await
    }

    /// Retrieve futures settlement history from the past three months.
    ///
    /// `GET /api/v5/public/settlement-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_settlement_history(
        &self,
        request: &SettlementHistoryRequest<'_>,
    ) -> Result<Vec<SettlementHistory>, Error> {
        self.client.get(SETTLEMENT_HISTORY, request, false).await
    }
}
