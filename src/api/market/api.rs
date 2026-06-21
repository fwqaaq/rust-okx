use crate::client::OkxClient;
use crate::error::Error;
use crate::model::EmptyRequest;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the public market-data endpoints.
///
/// Obtain one via [`OkxClient::market`](crate::OkxClient::market).
pub struct Market<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Market<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve the latest ticker for a single instrument.
    ///
    /// `GET /api/v5/market/ticker`. Public (unauthenticated). The returned
    /// vector contains exactly one [`Ticker`].
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or
    /// [`RestError::Transport`](crate::RestError::Transport)/[`RestError::Decode`](crate::RestError::Decode) on transport/parsing failure.
    pub async fn get_ticker(&self, request: &InstIdRequest<'_>) -> Result<Vec<Ticker>, Error> {
        self.client.get(TICKER, request, false).await
    }

    /// Retrieve tickers for an instrument type.
    ///
    /// `GET /api/v5/market/tickers`. Public. `underlying` and `inst_family`
    /// are useful for derivatives and omitted when `None`.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_tickers(&self, request: &TickersRequest<'_>) -> Result<Vec<Ticker>, Error> {
        self.client.get(TICKERS, request, false).await
    }

    /// Retrieve index tickers.
    ///
    /// `GET /api/v5/market/index-tickers`. Public. Filter by quote currency,
    /// index instrument ID, or neither.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_index_tickers(
        &self,
        request: &IndexTickersRequest<'_>,
    ) -> Result<Vec<IndexTicker>, Error> {
        self.client.get(INDEX_TICKERS, request, false).await
    }

    /// Retrieve the order book for an instrument.
    ///
    /// `GET /api/v5/market/books`. `depth` is the number of levels per side
    /// (OKX default 1, max 400). Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_orderbook(
        &self,
        request: &OrderBookRequest<'_>,
    ) -> Result<Vec<OrderBook>, Error> {
        self.client.get(BOOKS, request, false).await
    }

    /// Retrieve candlestick (OHLCV) data.
    ///
    /// `GET /api/v5/market/candles`. `bar` is the bar size, e.g. `1m`, `1H`,
    /// `1D` (OKX default `1m`). `limit` caps the number of bars (max 300).
    /// Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_candlesticks(
        &self,
        request: &CandlesRequest<'_>,
    ) -> Result<Vec<Candle>, Error> {
        self.client.get(CANDLES, request, false).await
    }

    /// Retrieve historical candlestick data for top currencies.
    ///
    /// `GET /api/v5/market/history-candles`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_history_candlesticks(
        &self,
        request: &CandlesticksRequest,
    ) -> Result<Vec<Candle>, Error> {
        self.client.get(HISTORY_CANDLES, request, false).await
    }

    /// Retrieve index candlestick data.
    ///
    /// `GET /api/v5/market/index-candles`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_index_candlesticks(
        &self,
        request: &CandlesticksRequest,
    ) -> Result<Vec<IndexCandle>, Error> {
        self.client.get(INDEX_CANDLES, request, false).await
    }

    /// Retrieve mark-price candlestick data.
    ///
    /// `GET /api/v5/market/mark-price-candles`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_mark_price_candlesticks(
        &self,
        request: &CandlesticksRequest,
    ) -> Result<Vec<IndexCandle>, Error> {
        self.client.get(MARK_PRICE_CANDLES, request, false).await
    }

    /// Retrieve recent trades for an instrument.
    ///
    /// `GET /api/v5/market/trades`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_trades(&self, request: &TradesRequest<'_>) -> Result<Vec<MarketTrade>, Error> {
        self.client.get(TRADES, request, false).await
    }

    /// Retrieve historical trades for an instrument.
    ///
    /// `GET /api/v5/market/history-trades`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_history_trades(
        &self,
        request: &HistoryTradesRequest,
    ) -> Result<Vec<MarketTrade>, Error> {
        self.client.get(HISTORY_TRADES, request, false).await
    }

    /// Retrieve OKX platform 24-hour volume.
    ///
    /// `GET /api/v5/market/platform-24-volume`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_platform_24_volume(&self) -> Result<Vec<PlatformVolume>, Error> {
        self.client
            .get(PLATFORM_24_VOLUME, &EmptyRequest {}, false)
            .await
    }

    /// Retrieve index components.
    ///
    /// `GET /api/v5/market/index-components`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_index_components(
        &self,
        request: &IndexRequest<'_>,
    ) -> Result<Vec<IndexComponents>, Error> {
        self.client.get(INDEX_COMPONENTS, request, false).await
    }

    /// Retrieve the USD/CNY exchange rate used by OKX.
    ///
    /// `GET /api/v5/market/exchange-rate`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_exchange_rate(&self) -> Result<Vec<ExchangeRate>, Error> {
        self.client
            .get(EXCHANGE_RATE, &EmptyRequest {}, false)
            .await
    }

    /// Retrieve a block-trading ticker for a single instrument.
    ///
    /// `GET /api/v5/market/block-ticker`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_block_ticker(
        &self,
        request: &InstIdRequest<'_>,
    ) -> Result<Vec<BlockTicker>, Error> {
        self.client.get(BLOCK_TICKER, request, false).await
    }

    /// Retrieve block-trading tickers for an instrument type.
    ///
    /// `GET /api/v5/market/block-tickers`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_block_tickers(
        &self,
        request: &TickersRequest<'_>,
    ) -> Result<Vec<BlockTicker>, Error> {
        self.client.get(BLOCK_TICKERS, request, false).await
    }

    /// Retrieve option trades aggregated by instrument family.
    ///
    /// `GET /api/v5/market/option/instrument-family-trades`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_option_instrument_family_trades(
        &self,
        request: &InstFamilyRequest<'_>,
    ) -> Result<Vec<OptionFamilyTradeGroup>, Error> {
        self.client
            .get(OPTION_INSTRUMENT_FAMILY_TRADES, request, false)
            .await
    }
}
