use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::requests::{
    ConvertCurrenciesRequest, ConvertCurrencyPairRequest, ConvertHistoryRequest,
    ConvertQuoteRequest, ConvertTradeRequest,
};
use super::responses::{
    ConvertCurrency, ConvertCurrencyPair, ConvertHistory, ConvertQuote, ConvertTradeResult,
};

const CURRENCIES: &str = "/api/v5/asset/convert/currencies";
const CURRENCY_PAIR: &str = "/api/v5/asset/convert/currency-pair";
const ESTIMATE_QUOTE: &str = "/api/v5/asset/convert/estimate-quote";
const TRADE: &str = "/api/v5/asset/convert/trade";
const HISTORY: &str = "/api/v5/asset/convert/history";

/// Accessor for OKX Convert endpoints.
///
/// Obtain one via [`OkxClient::convert`](crate::OkxClient::convert). All
/// methods require credentials.
pub struct Convert<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Convert<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve currencies supported by Convert.
    ///
    /// `GET /api/v5/asset/convert/currencies`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) without credentials, [`RestError::Okx`](crate::RestError::Okx) on a
    /// non-zero OKX code, or transport/decode errors.
    pub async fn get_currencies(&self) -> Result<Vec<ConvertCurrency>, Error> {
        let request = ConvertCurrenciesRequest::new();

        self.client.get(CURRENCIES, &request, true).await
    }

    /// Retrieve limits and metadata for a Convert currency pair.
    ///
    /// `GET /api/v5/asset/convert/currency-pair`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies) for transport and API errors.
    pub async fn get_currency_pair(
        &self,
        request: &ConvertCurrencyPairRequest<'_>,
    ) -> Result<Vec<ConvertCurrencyPair>, Error> {
        self.client.get(CURRENCY_PAIR, request, true).await
    }

    /// Estimate a Convert quote.
    ///
    /// `POST /api/v5/asset/convert/estimate-quote`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies) for transport and API errors.
    pub async fn estimate_quote(
        &self,
        request: &ConvertQuoteRequest<'_>,
    ) -> Result<Vec<ConvertQuote>, Error> {
        self.client.post(ESTIMATE_QUOTE, request, true).await
    }

    /// Execute a Convert trade.
    ///
    /// `POST /api/v5/asset/convert/trade`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies) for transport and API errors.
    pub async fn convert_trade(
        &self,
        request: &ConvertTradeRequest<'_>,
    ) -> Result<Vec<ConvertTradeResult>, Error> {
        self.client.post(TRADE, request, true).await
    }

    /// Retrieve Convert trade history.
    ///
    /// `GET /api/v5/asset/convert/history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies) for transport and API errors.
    pub async fn get_convert_history(
        &self,
        request: &ConvertHistoryRequest<'_>,
    ) -> Result<Vec<ConvertHistory>, Error> {
        self.client.get(HISTORY, request, true).await
    }
}
