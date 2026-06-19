use serde::Deserialize;

use crate::{NumberString, model::InstType};

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
    /// Opening price (UTC 0).
    #[serde(default)]
    pub sod_utc0: NumberString,
    /// Opening price (UTC 8).
    #[serde(default)]
    pub sod_utc8: NumberString,
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
    /// Opening price (UTC 0).
    #[serde(default)]
    pub sod_utc0: NumberString,
    /// Opening price (UTC 8).
    #[serde(default)]
    pub sod_utc8: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// An order book snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    /// Trade source.
    #[serde(default)]
    pub source: String,
    /// Trade timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// A block-trading ticker snapshot.
///
/// OKX block ticker rows are sparser than regular ticker rows, so fields that
/// are required by [`Ticker`] are optional/defaulted here.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BlockTicker {
    /// Instrument type, when present.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g. `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Last traded price, when present.
    #[serde(default)]
    pub last: NumberString,
    /// Last traded size, when present.
    #[serde(default)]
    pub last_sz: NumberString,
    /// Best ask price, when present.
    #[serde(default)]
    pub ask_px: NumberString,
    /// Best ask size, when present.
    #[serde(default)]
    pub ask_sz: NumberString,
    /// Best bid price, when present.
    #[serde(default)]
    pub bid_px: NumberString,
    /// Best bid size, when present.
    #[serde(default)]
    pub bid_sz: NumberString,
    /// Open price over the last 24 hours, when present.
    #[serde(default)]
    pub open24h: NumberString,
    /// Highest price over the last 24 hours, when present.
    #[serde(default)]
    pub high24h: NumberString,
    /// Lowest price over the last 24 hours, when present.
    #[serde(default)]
    pub low24h: NumberString,
    /// Trading volume over the last 24 hours, when present.
    #[serde(default)]
    pub vol24h: NumberString,
    /// Trading volume in currency units over the last 24 hours, when present.
    #[serde(default)]
    pub vol_ccy24h: NumberString,
    /// Opening price (UTC 0), when present.
    #[serde(default)]
    pub sod_utc0: NumberString,
    /// Opening price (UTC 8), when present.
    #[serde(default)]
    pub sod_utc8: NumberString,
    /// Ticker timestamp (Unix milliseconds), when present.
    #[serde(default)]
    pub ts: NumberString,
}

/// A public block-trade record.
pub type BlockTrade = MarketTrade;

/// A public option trade record grouped by instrument family.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OptionInstrumentFamilyTrade {
    /// Instrument ID, when present.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument family, when present.
    #[serde(default)]
    pub inst_family: String,
    /// Trade ID, when present.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade size.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade side (`buy` or `sell`), when present.
    #[serde(default)]
    pub side: String,
    /// Trade timestamp (Unix milliseconds), when present.
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

/// A single candlestick bar for index/mark-price endpoints (6-element array).
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "IndexCandleRaw")]
#[non_exhaustive]
pub struct IndexCandle {
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
    /// `1` if the bar is closed/confirmed, `0` otherwise.
    pub confirm: NumberString,
}

type IndexCandleRaw = (
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
);

impl From<IndexCandleRaw> for IndexCandle {
    fn from(raw: IndexCandleRaw) -> Self {
        Self {
            ts: raw.0,
            open: raw.1,
            high: raw.2,
            low: raw.3,
            close: raw.4,
            confirm: raw.5,
        }
    }
}

/// Grouped option trades returned by the instrument-family-trades endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OptionFamilyTradeGroup {
    /// 24-hour volume.
    #[serde(default)]
    pub vol24h: NumberString,
    /// Option type (`C` or `P`).
    #[serde(default)]
    pub opt_type: String,
    /// Individual trades within this group.
    #[serde(default)]
    pub trade_info: Vec<OptionInstrumentFamilyTrade>,
}
