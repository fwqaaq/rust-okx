//! Public market-data endpoints (`/api/v5/market/*`).

use serde::{Deserialize, Serialize};

use crate::client::OkxClient;
use crate::error::Error;
use crate::model::{InstType, NumberString};
use crate::transport::Transport;

const TICKER: &str = "/api/v5/market/ticker";
const TICKERS: &str = "/api/v5/market/tickers";
const INDEX_TICKERS: &str = "/api/v5/market/index-tickers";
const BOOKS: &str = "/api/v5/market/books";
const BOOKS_LITE: &str = "/api/v5/market/books-lite";
const CANDLES: &str = "/api/v5/market/candles";
const HISTORY_CANDLES: &str = "/api/v5/market/history-candles";
const INDEX_CANDLES: &str = "/api/v5/market/index-candles";
const MARK_PRICE_CANDLES: &str = "/api/v5/market/mark-price-candles";
const TRADES: &str = "/api/v5/market/trades";
const HISTORY_TRADES: &str = "/api/v5/market/history-trades";
const PLATFORM_24_VOLUME: &str = "/api/v5/market/platform-24-volume";
const INDEX_COMPONENTS: &str = "/api/v5/market/index-components";
const EXCHANGE_RATE: &str = "/api/v5/market/exchange-rate";

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
    /// Returns [`Error::Api`] on a non-zero OKX code, or
    /// [`Error::Transport`]/[`Error::Decode`] on transport/parsing failure.
    pub async fn get_ticker(&self, inst_id: &str) -> Result<Vec<Ticker>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(TICKER, &query, false).await
    }

    /// Retrieve tickers for an instrument type.
    ///
    /// `GET /api/v5/market/tickers`. Public. `underlying` and `inst_family`
    /// are useful for derivatives and omitted when `None`.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_tickers(
        &self,
        inst_type: InstType,
        underlying: Option<&str>,
        inst_family: Option<&str>,
    ) -> Result<Vec<Ticker>, Error> {
        let query = TickersQuery {
            inst_type: &inst_type,
            underlying,
            inst_family,
        };
        self.client.get(TICKERS, &query, false).await
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
        quote_ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> Result<Vec<IndexTicker>, Error> {
        let query = IndexTickersQuery { quote_ccy, inst_id };
        self.client.get(INDEX_TICKERS, &query, false).await
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
        inst_id: &str,
        depth: Option<u32>,
    ) -> Result<Vec<OrderBook>, Error> {
        let query = OrderBookQuery { inst_id, sz: depth };
        self.client.get(BOOKS, &query, false).await
    }

    /// Retrieve the lightweight order book for an instrument.
    ///
    /// `GET /api/v5/market/books-lite`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_order_lite_book(&self, inst_id: &str) -> Result<Vec<OrderBook>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(BOOKS_LITE, &query, false).await
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
        inst_id: &str,
        bar: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Candle>, Error> {
        let query = CandlesQuery {
            inst_id,
            bar,
            limit,
        };
        self.client.get(CANDLES, &query, false).await
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
    ) -> Result<Vec<Candle>, Error> {
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
    ) -> Result<Vec<Candle>, Error> {
        self.client.get(MARK_PRICE_CANDLES, request, false).await
    }

    /// Retrieve recent trades for an instrument.
    ///
    /// `GET /api/v5/market/trades`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_trades(
        &self,
        inst_id: &str,
        limit: Option<u32>,
    ) -> Result<Vec<MarketTrade>, Error> {
        let query = TradesQuery { inst_id, limit };
        self.client.get(TRADES, &query, false).await
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
        self.client.get(PLATFORM_24_VOLUME, &NoQuery, false).await
    }

    /// Retrieve index components.
    ///
    /// `GET /api/v5/market/index-components`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_index_components(&self, index: &str) -> Result<Vec<IndexComponents>, Error> {
        let query = IndexComponentsQuery { index };
        self.client.get(INDEX_COMPONENTS, &query, false).await
    }

    /// Retrieve the USD/CNY exchange rate used by OKX.
    ///
    /// `GET /api/v5/market/exchange-rate`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_ticker`](Self::get_ticker).
    pub async fn get_exchange_rate(&self) -> Result<Vec<ExchangeRate>, Error> {
        self.client.get(EXCHANGE_RATE, &NoQuery, false).await
    }
}

#[derive(Serialize)]
struct NoQuery;

#[derive(Serialize)]
struct InstIdQuery<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
}

#[derive(Serialize)]
struct TickersQuery<'a> {
    #[serde(rename = "instType")]
    inst_type: &'a InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<&'a str>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<&'a str>,
}

#[derive(Serialize)]
struct IndexTickersQuery<'a> {
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    quote_ccy: Option<&'a str>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<&'a str>,
}

#[derive(Serialize)]
struct OrderBookQuery<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    sz: Option<u32>,
}

#[derive(Serialize)]
struct CandlesQuery<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

/// Query parameters for historical/index/mark-price candlestick endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct CandlesticksRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl CandlesticksRequest {
    /// Create a candlestick query for an instrument.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        }
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the bar size, e.g. `1m`, `1H`, or `1D`.
    pub fn bar(mut self, bar: impl Into<String>) -> Self {
        self.bar = Some(bar.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Serialize)]
struct TradesQuery<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

/// Query parameters for historical trades.
#[derive(Debug, Clone, Serialize)]
pub struct HistoryTradesRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    trade_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl HistoryTradesRequest {
    /// Create a historical trades query for an instrument.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            trade_type: None,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Set the OKX trade type filter.
    pub fn trade_type(mut self, trade_type: impl Into<String>) -> Self {
        self.trade_type = Some(trade_type.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Serialize)]
struct IndexComponentsQuery<'a> {
    index: &'a str,
}

/// The latest ticker snapshot for an instrument.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Ticker {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID, e.g. `BTC-USDT`.
    pub inst_id: String,
    /// Last traded price.
    pub last: NumberString,
    /// Last traded size.
    #[serde(default)]
    pub last_sz: NumberString,
    /// Best ask price.
    #[serde(default)]
    pub ask_px: NumberString,
    /// Best ask size.
    #[serde(default)]
    pub ask_sz: NumberString,
    /// Best bid price.
    #[serde(default)]
    pub bid_px: NumberString,
    /// Best bid size.
    #[serde(default)]
    pub bid_sz: NumberString,
    /// Open price over the last 24 hours.
    #[serde(default)]
    pub open24h: NumberString,
    /// Highest price over the last 24 hours.
    #[serde(default)]
    pub high24h: NumberString,
    /// Lowest price over the last 24 hours.
    #[serde(default)]
    pub low24h: NumberString,
    /// Trading volume (base ccy) over the last 24 hours.
    #[serde(default)]
    pub vol24h: NumberString,
    /// Trading volume (quote ccy) over the last 24 hours.
    #[serde(default)]
    pub vol_ccy24h: NumberString,
    /// Ticker timestamp (Unix milliseconds).
    pub ts: NumberString,
}

/// The latest index ticker snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IndexTicker {
    /// Index instrument ID, e.g. `BTC-USD`.
    pub inst_id: String,
    /// Index price.
    #[serde(default)]
    pub idx_px: NumberString,
    /// Open price over the last 24 hours.
    #[serde(default)]
    pub open24h: NumberString,
    /// Highest price over the last 24 hours.
    #[serde(default)]
    pub high24h: NumberString,
    /// Lowest price over the last 24 hours.
    #[serde(default)]
    pub low24h: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// An order book snapshot.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct OrderBook {
    /// Ask levels, sorted from best (lowest) price.
    pub asks: Vec<OrderBookLevel>,
    /// Bid levels, sorted from best (highest) price.
    pub bids: Vec<OrderBookLevel>,
    /// Snapshot timestamp (Unix milliseconds).
    pub ts: NumberString,
}

/// A single price level in an [`OrderBook`].
///
/// On the wire OKX encodes a level as the array
/// `[price, size, deprecated, order_count]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "BookLevelRaw")]
#[non_exhaustive]
pub struct OrderBookLevel {
    /// Price at this level.
    pub price: NumberString,
    /// Aggregated size available at this level.
    pub size: NumberString,
    /// Deprecated field (always `0`), retained for wire compatibility.
    pub deprecated: NumberString,
    /// Number of orders aggregated at this level.
    pub order_count: NumberString,
}

type BookLevelRaw = (NumberString, NumberString, NumberString, NumberString);

impl From<BookLevelRaw> for OrderBookLevel {
    fn from(raw: BookLevelRaw) -> Self {
        Self {
            price: raw.0,
            size: raw.1,
            deprecated: raw.2,
            order_count: raw.3,
        }
    }
}

/// A single candlestick (OHLCV) bar.
///
/// On the wire OKX encodes a bar as a 9-element string array.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "CandleRaw")]
#[non_exhaustive]
pub struct Candle {
    /// Opening timestamp (Unix milliseconds).
    pub ts: NumberString,
    /// Open price.
    pub open: NumberString,
    /// Highest price.
    pub high: NumberString,
    /// Lowest price.
    pub low: NumberString,
    /// Close price.
    pub close: NumberString,
    /// Trading volume in contracts / base currency.
    pub vol: NumberString,
    /// Trading volume in quote currency.
    pub vol_ccy: NumberString,
    /// Trading volume in quote currency (alternate calculation).
    pub vol_ccy_quote: NumberString,
    /// `1` if the bar is closed/confirmed, `0` otherwise.
    pub confirm: NumberString,
}

type CandleRaw = (
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
);

impl From<CandleRaw> for Candle {
    fn from(raw: CandleRaw) -> Self {
        Self {
            ts: raw.0,
            open: raw.1,
            high: raw.2,
            low: raw.3,
            close: raw.4,
            vol: raw.5,
            vol_ccy: raw.6,
            vol_ccy_quote: raw.7,
            confirm: raw.8,
        }
    }
}

/// A public trade record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MarketTrade {
    /// Instrument ID.
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade size.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade side (`buy` or `sell`).
    #[serde(default)]
    pub side: String,
    /// Trade timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// OKX platform 24-hour trading volume.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlatformVolume {
    /// 24-hour volume in USD.
    #[serde(default)]
    pub vol_usd: NumberString,
    /// 24-hour volume in CNY.
    #[serde(default)]
    pub vol_cny: NumberString,
}

/// The components that make up an OKX index.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IndexComponents {
    /// Index name.
    #[serde(default)]
    pub index: String,
    /// Component list.
    #[serde(default)]
    pub components: Vec<IndexComponent>,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// A single index component.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IndexComponent {
    /// Exchange name.
    #[serde(default)]
    pub exch: String,
    /// Symbol used by the component exchange.
    #[serde(default)]
    pub symbol: String,
    /// Component price.
    #[serde(default)]
    pub sym_px: NumberString,
    /// Component weight.
    #[serde(default)]
    pub wgt: NumberString,
    /// Conversion price.
    #[serde(default)]
    pub cnv_px: NumberString,
}

/// The USD/CNY exchange rate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ExchangeRate {
    /// USD/CNY rate.
    #[serde(default)]
    pub usd_cny: NumberString,
}

#[cfg(test)]
mod tests {
    use crate::OkxClient;
    use crate::test_util::MockTransport;

    #[tokio::test]
    async fn get_ticker_builds_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","last":"42000.1","lastSz":"0.1",
             "askPx":"42000.2","askSz":"1","bidPx":"42000.0","bidSz":"2",
             "open24h":"41000","high24h":"43000","low24h":"40000","vol24h":"1000",
             "volCcy24h":"42000000","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let tickers = client.market().get_ticker("BTC-USDT").await.unwrap();
        assert_eq!(tickers[0].inst_id, "BTC-USDT");
        assert_eq!(tickers[0].last.as_str(), "42000.1");
        assert_eq!(tickers[0].bid_px.parse::<f64>().unwrap(), 42000.0);

        let req = mock.captured();
        assert_eq!(req.method, http::Method::GET);
        assert_eq!(req.query(), Some("instId=BTC-USDT"));
        assert!(!req.is_signed());
    }

    #[tokio::test]
    async fn get_tickers_builds_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","last":"42000.1","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let tickers = client
            .market()
            .get_tickers(crate::model::InstType::Swap, None, Some("BTC-USDT"))
            .await
            .unwrap();
        assert_eq!(tickers[0].inst_id, "BTC-USDT-SWAP");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::GET);
        assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
        assert!(!req.is_signed());
    }

    #[tokio::test]
    async fn get_index_tickers_builds_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USD","idxPx":"42000.1","open24h":"41000","high24h":"43000","low24h":"40000","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let tickers = client
            .market()
            .get_index_tickers(Some("USD"), None)
            .await
            .unwrap();
        assert_eq!(tickers[0].inst_id, "BTC-USD");
        assert_eq!(tickers[0].idx_px.as_str(), "42000.1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("quoteCcy=USD"));
    }

    #[tokio::test]
    async fn get_orderbook_parses_levels_and_passes_depth() {
        let body = r#"{"code":"0","msg":"","data":[
            {"asks":[["42000.2","1","0","3"]],"bids":[["42000.0","2","0","1"]],
             "ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let books = client
            .market()
            .get_orderbook("BTC-USDT", Some(5))
            .await
            .unwrap();
        let book = &books[0];
        assert_eq!(book.asks[0].price.as_str(), "42000.2");
        assert_eq!(book.asks[0].order_count.as_str(), "3");
        assert_eq!(book.bids[0].size.as_str(), "2");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT&sz=5"));
    }

    #[tokio::test]
    async fn get_order_lite_book_queries_instrument() {
        let body = r#"{"code":"0","msg":"","data":[
            {"asks":[["42000.2","1","0","3"]],"bids":[["42000.0","2","0","1"]],
             "ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let books = client
            .market()
            .get_order_lite_book("BTC-USDT")
            .await
            .unwrap();
        assert_eq!(books[0].asks[0].price.as_str(), "42000.2");

        let req = mock.captured();
        assert!(
            req.uri
                .ends_with("/api/v5/market/books-lite?instId=BTC-USDT")
        );
    }

    #[tokio::test]
    async fn get_candlesticks_parses_array_rows() {
        let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let candles = client
            .market()
            .get_candlesticks("BTC-USDT", Some("1H"), Some(1))
            .await
            .unwrap();
        assert_eq!(candles[0].open.as_str(), "42000");
        assert_eq!(candles[0].close.as_str(), "42500");
        assert_eq!(candles[0].confirm.as_str(), "1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT&bar=1H&limit=1"));
    }

    #[tokio::test]
    async fn get_history_candlesticks_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::CandlesticksRequest::new("BTC-USDT")
            .bar("1H")
            .limit(2);

        let candles = client
            .market()
            .get_history_candlesticks(&request)
            .await
            .unwrap();
        assert_eq!(candles[0].high.as_str(), "43000");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT&bar=1H&limit=2"));
        assert!(!req.query().unwrap().contains("after"));
    }

    #[tokio::test]
    async fn get_index_candlesticks_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::CandlesticksRequest::new("BTC-USD").after("10");

        let candles = client
            .market()
            .get_index_candlesticks(&request)
            .await
            .unwrap();
        assert_eq!(candles[0].low.as_str(), "41000");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USD&after=10"));
    }

    #[tokio::test]
    async fn get_mark_price_candlesticks_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            ["1597026383085","42000","43000","41000","42500","100","4250000","4250000","1"]]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::CandlesticksRequest::new("BTC-USDT-SWAP").before("20");

        let candles = client
            .market()
            .get_mark_price_candlesticks(&request)
            .await
            .unwrap();
        assert_eq!(candles[0].close.as_str(), "42500");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP&before=20"));
    }

    #[tokio::test]
    async fn get_trades_builds_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","tradeId":"1","px":"42000.1","sz":"0.02","side":"buy","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let trades = client
            .market()
            .get_trades("BTC-USDT", Some(1))
            .await
            .unwrap();
        assert_eq!(trades[0].trade_id, "1");
        assert_eq!(trades[0].px.as_str(), "42000.1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT&limit=1"));
    }

    #[tokio::test]
    async fn get_history_trades_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","tradeId":"1","px":"42000.1","sz":"0.02","side":"sell","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::HistoryTradesRequest::new("BTC-USDT")
            .trade_type("1")
            .before("100")
            .limit(1);

        let trades = client.market().get_history_trades(&request).await.unwrap();
        assert_eq!(trades[0].side, "sell");

        let req = mock.captured();
        assert_eq!(
            req.query(),
            Some("instId=BTC-USDT&type=1&before=100&limit=1")
        );
        assert!(!req.query().unwrap().contains("after"));
    }

    #[tokio::test]
    async fn get_platform_24_volume_parses_volume() {
        let body = r#"{"code":"0","msg":"","data":[{"volUsd":"1000","volCny":"7100"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let volume = client.market().get_platform_24_volume().await.unwrap();
        assert_eq!(volume[0].vol_usd.as_str(), "1000");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/market/platform-24-volume"));
        assert_eq!(req.query(), None);
    }

    #[tokio::test]
    async fn get_index_components_builds_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"index":"BTC-USD","ts":"1597026383085","components":[
                {"exch":"okx","symbol":"BTC-USDT","symPx":"42000","wgt":"1","cnvPx":"42000"}]}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let components = client
            .market()
            .get_index_components("BTC-USD")
            .await
            .unwrap();
        assert_eq!(components[0].components[0].symbol, "BTC-USDT");

        let req = mock.captured();
        assert_eq!(req.query(), Some("index=BTC-USD"));
    }

    #[tokio::test]
    async fn get_exchange_rate_parses_rate() {
        let body = r#"{"code":"0","msg":"","data":[{"usdCny":"7.1"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let rate = client.market().get_exchange_rate().await.unwrap();
        assert_eq!(rate[0].usd_cny.as_str(), "7.1");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/market/exchange-rate"));
        assert_eq!(req.query(), None);
    }
}
