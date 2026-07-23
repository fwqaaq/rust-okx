use std::borrow::Cow;

use serde::Serialize;

use crate::model::InstType;

/// Request for market endpoints that require one instrument ID.
///
/// Used by [`get_ticker`](crate::api::market::Market::get_ticker),
/// [`get_block_ticker`](crate::api::market::Market::get_block_ticker), and
/// [`get_call_auction_details`](crate::api::market::Market::get_call_auction_details).
#[derive(Debug, Clone, Serialize)]
pub struct InstIdRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
}

impl<'a> InstIdRequest<'a> {
    /// Create a query for one instrument ID.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
        }
    }
}

/// Request for [`get_tickers`](crate::api::market::Market::get_tickers) and
/// [`get_block_tickers`](crate::api::market::Market::get_block_tickers).
#[derive(Debug, Clone, Serialize)]
pub struct TickersRequest<'a> {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> TickersRequest<'a> {
    /// Create a tickers query for an instrument type.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            inst_family: None,
        }
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Request for [`get_index_tickers`](crate::api::market::Market::get_index_tickers).
#[derive(Debug, Clone, Default, Serialize)]
pub struct IndexTickersRequest<'a> {
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    quote_ccy: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
}

impl<'a> IndexTickersRequest<'a> {
    /// Create an empty index-tickers query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the quote currency filter.
    pub fn quote_currency(mut self, quote_ccy: impl Into<Cow<'a, str>>) -> Self {
        self.quote_ccy = Some(quote_ccy.into());
        self
    }

    /// Set the index instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }
}

/// Request for [`get_orderbook`](crate::api::market::Market::get_orderbook).
#[derive(Debug, Clone, Serialize)]
pub struct OrderBookRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sz: Option<u32>,
}

impl<'a> OrderBookRequest<'a> {
    /// Create an order-book query for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            sz: None,
        }
    }

    /// Set the depth, i.e. the number of price levels per side.
    pub fn size(mut self, sz: u32) -> Self {
        self.sz = Some(sz);
        self
    }
}

/// Request for [`get_full_orderbook`](crate::api::market::Market::get_full_orderbook).
#[derive(Debug, Clone, Serialize)]
pub struct FullOrderBookRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sz: Option<u32>,
}

impl<'a> FullOrderBookRequest<'a> {
    /// Create a full order-book query for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            sz: None,
        }
    }

    /// Set the depth per side (OKX maximum: 5,000).
    pub fn size(mut self, sz: u32) -> Self {
        self.sz = Some(sz);
        self
    }
}

/// Request for [`get_sbe_orderbook`](crate::api::market::Market::get_sbe_orderbook).
#[derive(Debug, Clone, Serialize)]
pub struct SbeOrderBookRequest {
    #[serde(rename = "instIdCode")]
    inst_id_code: i64,
    source: u8,
}

impl SbeOrderBookRequest {
    /// Create an SBE order-book query for an instrument ID code.
    ///
    /// The official API currently supports only source `0` (the normal order
    /// book), so it is fixed by this constructor rather than exposed as an
    /// unrestricted value.
    pub fn new(inst_id_code: i64) -> Self {
        Self {
            inst_id_code,
            source: 0,
        }
    }
}

/// Request for [`get_candlesticks`](crate::api::market::Market::get_candlesticks).
#[derive(Debug, Clone, Serialize)]
pub struct CandlesRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> CandlesRequest<'a> {
    /// Create a candlestick query for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            bar: None,
            limit: None,
        }
    }

    /// Set the bar size, e.g. `1m`, `1H`, or `1D`.
    pub fn bar(mut self, bar: impl Into<Cow<'a, str>>) -> Self {
        self.bar = Some(bar.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request for [`get_trades`](crate::api::market::Market::get_trades).
#[derive(Debug, Clone, Serialize)]
pub struct TradesRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> TradesRequest<'a> {
    /// Create a recent-trades query for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            limit: None,
        }
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request for [`get_option_instrument_family_trades`](crate::api::market::Market::get_option_instrument_family_trades).
#[derive(Debug, Clone, Serialize)]
pub struct InstFamilyRequest<'a> {
    #[serde(rename = "instFamily")]
    inst_family: Cow<'a, str>,
}

impl<'a> InstFamilyRequest<'a> {
    /// Create a query for an instrument family.
    pub fn new(inst_family: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_family: inst_family.into(),
        }
    }
}

/// Request for [`get_index_components`](crate::api::market::Market::get_index_components).
#[derive(Debug, Clone, Serialize)]
pub struct IndexRequest<'a> {
    index: Cow<'a, str>,
}

impl<'a> IndexRequest<'a> {
    /// Create a query for an index symbol.
    pub fn new(index: impl Into<Cow<'a, str>>) -> Self {
        Self {
            index: index.into(),
        }
    }
}

/// Query parameters for historical/index/mark-price candlestick endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct CandlesticksRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> CandlesticksRequest<'a> {
    /// Create a candlestick query for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        }
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the bar size, e.g. `1m`, `1H`, or `1D`.
    pub fn bar(mut self, bar: impl Into<Cow<'a, str>>) -> Self {
        self.bar = Some(bar.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Request for [`get_spread_ticker`](crate::api::market::Market::get_spread_ticker).
#[derive(Debug, Clone, Serialize)]
pub struct SpreadIdRequest<'a> {
    #[serde(rename = "sprdId")]
    spread_id: Cow<'a, str>,
}

impl<'a> SpreadIdRequest<'a> {
    /// Create a query for one spread ID.
    pub fn new(spread_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            spread_id: spread_id.into(),
        }
    }
}

/// Query parameters for spread candlestick endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct SpreadCandlesticksRequest<'a> {
    #[serde(rename = "sprdId")]
    spread_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> SpreadCandlesticksRequest<'a> {
    /// Create a candlestick query for a spread.
    pub fn new(spread_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            spread_id: spread_id.into(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        }
    }

    /// Return records earlier than this timestamp cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records newer than this timestamp cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the bar size, e.g. `1m`, `1H`, or `1D`.
    pub fn bar(mut self, bar: impl Into<Cow<'a, str>>) -> Self {
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
pub struct HistoryTradesRequest<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    trade_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> HistoryTradesRequest<'a> {
    /// Create a historical trades query for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            trade_type: None,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Set the OKX trade type filter.
    pub fn trade_type(mut self, trade_type: impl Into<Cow<'a, str>>) -> Self {
        self.trade_type = Some(trade_type.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
