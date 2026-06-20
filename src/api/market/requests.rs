use serde::Serialize;

use crate::model::InstType;

/// Request for [`get_ticker`](crate::api::market::Market::get_ticker) and
/// [`get_block_ticker`](crate::api::market::Market::get_block_ticker).
#[derive(Debug, Clone, Serialize)]
pub struct InstIdRequest<'a> {
    /// Instrument ID, e.g. `"BTC-USDT"`.
    #[serde(rename = "instId")]
    pub inst_id: &'a str,
}

/// Request for [`get_tickers`](crate::api::market::Market::get_tickers) and
/// [`get_block_tickers`](crate::api::market::Market::get_block_tickers).
#[derive(Debug, Clone, Serialize)]
pub struct TickersRequest<'a> {
    /// Instrument type.
    #[serde(rename = "instType")]
    pub inst_type: &'a InstType,
    /// Instrument family filter (optional).
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<&'a str>,
}

/// Request for [`get_index_tickers`](crate::api::market::Market::get_index_tickers).
#[derive(Debug, Clone, Default, Serialize)]
pub struct IndexTickersRequest<'a> {
    /// Quote currency filter, e.g. `Some("USD")`.
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<&'a str>,
    /// Index instrument ID filter.
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<&'a str>,
}

/// Request for [`get_orderbook`](crate::api::market::Market::get_orderbook).
#[derive(Debug, Clone, Serialize)]
pub struct OrderBookRequest<'a> {
    /// Instrument ID.
    #[serde(rename = "instId")]
    pub inst_id: &'a str,
    /// Depth (number of price levels per side). OKX default 1, max 400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<u32>,
}

/// Request for [`get_candlesticks`](crate::api::market::Market::get_candlesticks).
#[derive(Debug, Clone, Serialize)]
pub struct CandlesRequest<'a> {
    /// Instrument ID.
    #[serde(rename = "instId")]
    pub inst_id: &'a str,
    /// Bar size, e.g. `"1m"`, `"1H"`, `"1D"`. OKX default `"1m"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<&'a str>,
    /// Maximum number of bars (max 300).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request for [`get_trades`](crate::api::market::Market::get_trades).
#[derive(Debug, Clone, Serialize)]
pub struct TradesRequest<'a> {
    /// Instrument ID.
    #[serde(rename = "instId")]
    pub inst_id: &'a str,
    /// Maximum number of trades to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Request for [`get_option_instrument_family_trades`](crate::api::market::Market::get_option_instrument_family_trades).
#[derive(Debug, Clone, Serialize)]
pub struct InstFamilyRequest<'a> {
    /// Instrument family, e.g. `"BTC-USD"`.
    #[serde(rename = "instFamily")]
    pub inst_family: &'a str,
}

/// Request for [`get_index_components`](crate::api::market::Market::get_index_components).
#[derive(Debug, Clone, Serialize)]
pub struct IndexRequest<'a> {
    /// Index symbol, e.g. `"BTC-USD"`.
    pub index: &'a str,
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
