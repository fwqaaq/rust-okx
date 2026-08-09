use crate::client::OkxClient;
use crate::error::Error;
use crate::model::EmptyRequest;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for block-trading request-for-quote endpoints.
///
/// Obtain one via [`OkxClient::rfq`](crate::OkxClient::rfq).
pub struct Rfq<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Rfq<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve available block-trading counterparties.
    ///
    /// `GET /api/v5/rfq/counterparties`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_counterparties(&self) -> Result<Vec<RfqCounterparty>, Error> {
        self.client
            .get(COUNTERPARTIES, &EmptyRequest {}, true)
            .await
    }

    /// Create a block-trading RFQ.
    ///
    /// `POST /api/v5/rfq/create-rfq`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn create_rfq(&self, request: &CreateRfqRequest) -> Result<Vec<RfqOrder>, Error> {
        self.client.post(CREATE_RFQ, request, true).await
    }

    /// Cancel one active RFQ.
    ///
    /// `POST /api/v5/rfq/cancel-rfq`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_rfq(
        &self,
        request: &CancelRfqRequest,
    ) -> Result<Vec<RfqCancelResult>, Error> {
        self.client.post(CANCEL_RFQ, request, true).await
    }

    /// Cancel up to 100 active RFQs.
    ///
    /// `POST /api/v5/rfq/cancel-batch-rfqs`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_batch_rfqs(
        &self,
        request: &CancelBatchRfqsRequest,
    ) -> Result<Vec<RfqCancelResult>, Error> {
        self.client.post(CANCEL_BATCH_RFQS, request, true).await
    }

    /// Cancel all active RFQs.
    ///
    /// `POST /api/v5/rfq/cancel-all-rfqs`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_all_rfqs(&self) -> Result<Vec<RfqTimestamp>, Error> {
        self.client
            .post(CANCEL_ALL_RFQS, &EmptyRequest {}, true)
            .await
    }

    /// Execute a quote for an RFQ created by the caller.
    ///
    /// `POST /api/v5/rfq/execute-quote`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn execute_quote(
        &self,
        request: &ExecuteQuoteRequest,
    ) -> Result<Vec<RfqExecution>, Error> {
        self.client.post(EXECUTE_QUOTE, request, true).await
    }

    /// Create a maker quote for an RFQ.
    ///
    /// `POST /api/v5/rfq/create-quote`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn create_quote(&self, request: &CreateQuoteRequest) -> Result<Vec<RfqQuote>, Error> {
        self.client.post(CREATE_QUOTE, request, true).await
    }

    /// Cancel one active quote.
    ///
    /// `POST /api/v5/rfq/cancel-quote`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_quote(
        &self,
        request: &CancelQuoteRequest,
    ) -> Result<Vec<RfqQuoteCancelResult>, Error> {
        self.client.post(CANCEL_QUOTE, request, true).await
    }

    /// Cancel several active quotes.
    ///
    /// `POST /api/v5/rfq/cancel-batch-quotes`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_batch_quotes(
        &self,
        request: &CancelBatchQuotesRequest,
    ) -> Result<Vec<RfqQuoteCancelResult>, Error> {
        self.client.post(CANCEL_BATCH_QUOTES, request, true).await
    }

    /// Cancel all active quotes.
    ///
    /// `POST /api/v5/rfq/cancel-all-quotes`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_all_quotes(&self) -> Result<Vec<RfqTimestamp>, Error> {
        self.client
            .post(CANCEL_ALL_QUOTES, &EmptyRequest {}, true)
            .await
    }

    /// Retrieve RFQs.
    ///
    /// `GET /api/v5/rfq/rfqs`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_rfqs(&self, request: &RfqsRequest) -> Result<Vec<RfqOrder>, Error> {
        self.client.get(RFQS, request, true).await
    }

    /// Retrieve quotes.
    ///
    /// `GET /api/v5/rfq/quotes`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_quotes(&self, request: &QuotesRequest) -> Result<Vec<RfqQuote>, Error> {
        self.client.get(QUOTES, request, true).await
    }

    /// Retrieve private block trades.
    ///
    /// `GET /api/v5/rfq/trades`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_trades(&self, request: &RfqTradesRequest) -> Result<Vec<RfqTrade>, Error> {
        self.client.get(TRADES, request, true).await
    }

    /// Retrieve public multi-leg block trades.
    ///
    /// `GET /api/v5/rfq/public-trades`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error for transport/decode failures or an OKX error.
    pub async fn get_public_trades(
        &self,
        request: &PublicRfqTradesRequest,
    ) -> Result<Vec<PublicRfqTrade>, Error> {
        self.client.get(PUBLIC_TRADES, request, false).await
    }

    /// Retrieve the maker's quote product settings.
    ///
    /// `GET /api/v5/rfq/maker-instrument-settings`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_maker_instrument_settings(
        &self,
    ) -> Result<Vec<MakerInstrumentSettings>, Error> {
        self.client
            .get(MAKER_INSTRUMENT_SETTINGS, &EmptyRequest {}, true)
            .await
    }

    /// Replace the maker's quote product settings.
    ///
    /// `POST /api/v5/rfq/maker-instrument-settings`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn set_maker_instrument_settings(
        &self,
        request: &[MakerInstrumentSettingsRequest],
    ) -> Result<Vec<RfqBooleanResult>, Error> {
        self.client
            .post(MAKER_INSTRUMENT_SETTINGS, request, true)
            .await
    }

    /// Retrieve the maker's MMP configuration.
    ///
    /// `GET /api/v5/rfq/mmp-config`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn get_mmp_config(&self) -> Result<Vec<RfqMmpConfig>, Error> {
        self.client.get(MMP_CONFIG, &EmptyRequest {}, true).await
    }

    /// Configure block-trading maker protection.
    ///
    /// `POST /api/v5/rfq/mmp-config`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn set_mmp_config(
        &self,
        request: &RfqMmpConfigRequest,
    ) -> Result<Vec<RfqMmpConfig>, Error> {
        self.client.post(MMP_CONFIG, request, true).await
    }

    /// Reset the triggered block-trading MMP state.
    ///
    /// `POST /api/v5/rfq/mmp-reset`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn reset_mmp(&self) -> Result<Vec<RfqTimestamp>, Error> {
        self.client.post(MMP_RESET, &EmptyRequest {}, true).await
    }

    /// Configure cancel-all-after protection for RFQs and quotes.
    ///
    /// `POST /api/v5/rfq/cancel-all-after`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns an error for missing credentials, transport/decode failures, or an OKX error.
    pub async fn cancel_all_after(
        &self,
        request: &RfqCancelAllAfterRequest,
    ) -> Result<Vec<RfqCancelAllAfter>, Error> {
        self.client.post(CANCEL_ALL_AFTER, request, true).await
    }
}
