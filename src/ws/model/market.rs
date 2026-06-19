//! Market data channel models (`tickers`, `candles`, `trades`, `books`, etc.).
//!
//! Public channels; no authentication required.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

/// Access element `index` from a JSON array, falling back to `NumberString::default()`.
pub(super) fn array_value(values: &[NumberString], index: usize) -> NumberString {
    values.get(index).cloned().unwrap_or_default()
}

/// `tickers` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-tickers-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TickerUpdate {
    /// Instrument type, e.g., `SPOT`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Last traded price.
    #[serde(default)]
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
    /// Trading volume in quote currency over the last 24 hours.
    #[serde(default)]
    pub vol_ccy24h: NumberString,
    /// Trading volume in base currency (or contracts for derivatives) over the last 24 hours.
    #[serde(default)]
    pub vol24h: NumberString,
    /// Open price at the start of day (00:00 UTC).
    #[serde(default)]
    pub sod_utc0: NumberString,
    /// Open price at the start of day (08:00 UTC+8).
    #[serde(default)]
    pub sod_utc8: NumberString,
    /// Ticker push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `candle*` channel row represented by OKX's nine-element array.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-candlesticks-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(from = "Vec<NumberString>")]
#[non_exhaustive]
pub struct CandleUpdate {
    /// Opening time of the candlestick (Unix milliseconds).
    pub ts: NumberString,
    /// Open price.
    pub o: NumberString,
    /// Highest price.
    pub h: NumberString,
    /// Lowest price.
    pub l: NumberString,
    /// Close price.
    pub c: NumberString,
    /// Trading volume in contracts (derivatives) or base currency (SPOT/MARGIN).
    pub volume: NumberString,
    /// Trading volume in base currency (derivatives) or quote currency (SPOT/MARGIN).
    pub volume_ccy: NumberString,
    /// Trading volume in quote currency, e.g., `USDT` for `BTC-USDT` and `BTC-USDT-SWAP`,
    /// `USD` for `BTC-USD-SWAP`.
    pub volume_quote: NumberString,
    /// Candlestick state: `0` incomplete (still forming), `1` completed.
    pub confirm: NumberString,
}

impl From<Vec<NumberString>> for CandleUpdate {
    fn from(values: Vec<NumberString>) -> Self {
        Self {
            ts: array_value(&values, 0),
            o: array_value(&values, 1),
            h: array_value(&values, 2),
            l: array_value(&values, 3),
            c: array_value(&values, 4),
            volume: array_value(&values, 5),
            volume_ccy: array_value(&values, 6),
            volume_quote: array_value(&values, 7),
            confirm: array_value(&values, 8),
        }
    }
}

/// `trades` and `trades-all` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TradeUpdate {
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Trade ID assigned by OKX.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade size.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Trade source.
    ///
    /// Empty string for regular trades. `"1"` indicates a block trade.
    #[serde(default)]
    pub source: String,
    /// Number of trades aggregated into this push (only applicable to the `trades` channel).
    #[serde(default)]
    pub count: NumberString,
    /// Sequence ID of this message; monotonically increasing within a session.
    #[serde(default)]
    pub seq_id: NumberString,
    /// Trade timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `option-trades` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-option-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OptionTradeUpdate {
    /// Instrument ID, e.g., `BTC-USD-240329-40000-C`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument family, e.g., `BTC-USD`.
    #[serde(default)]
    pub inst_family: String,
    /// Trade ID assigned by OKX.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade size (number of contracts).
    #[serde(default)]
    pub sz: NumberString,
    /// Trade side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Option type: `C` (call) or `P` (put).
    #[serde(default)]
    pub opt_type: String,
    /// Implied volatility at the fill price.
    #[serde(default)]
    pub fill_vol: NumberString,
    /// Forward price at the time of the trade.
    #[serde(default)]
    pub fwd_px: NumberString,
    /// Index price at the time of the trade.
    #[serde(default)]
    pub idx_px: NumberString,
    /// Mark price at the time of the trade.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Trade timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `call-auction-details` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-call-auction-details-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CallAuctionDetailsUpdate {
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Equilibrium price — the price at which the maximum volume can be matched.
    #[serde(default)]
    pub eq_px: NumberString,
    /// Total matched volume at the equilibrium price.
    #[serde(default)]
    pub matched_sz: NumberString,
    /// Unmatched volume remaining at the equilibrium price.
    #[serde(default)]
    pub unmatched_sz: NumberString,
    /// Auction end time (Unix milliseconds).
    #[serde(default)]
    pub auction_end_time: NumberString,
    /// Auction state.
    ///
    /// Documented values: `prepareStart`, `parallelTrading`, `callAuction`,
    /// `cancelOrder`, `matchWaiting`, `matched`, `normal`.
    #[serde(default)]
    pub state: String,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// An order book push from `books`, `books5`, or tick-by-tick book channels.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-order-book-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderBookUpdate {
    /// Ask levels sorted from best (lowest) price to worst.
    #[serde(default)]
    pub asks: Vec<BookLevel>,
    /// Bid levels sorted from best (highest) price to worst.
    #[serde(default)]
    pub bids: Vec<BookLevel>,
    /// CRC32 checksum of the top-25 bid/ask levels for integrity verification.
    #[serde(default)]
    pub checksum: i64,
    /// Sequence ID of the previous message; used to detect gaps.
    ///
    /// Only applicable to `books`, `books-l2-tbt`, and `books50-l2-tbt`.
    #[serde(default)]
    pub prev_seq_id: i64,
    /// Sequence ID of the current message; monotonically increasing.
    #[serde(default)]
    pub seq_id: i64,
    /// Order book generation time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// A single four-value WebSocket order-book level.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(from = "Vec<NumberString>")]
#[non_exhaustive]
pub struct BookLevel {
    /// The limit price of this order book level.
    pub price: NumberString,

    /// The total depth or size available at this price level (in coins or contracts).
    pub size: NumberString,

    /// The number of liquidation orders currently resting at this price level.
    pub liquidated_order_count: NumberString,

    /// The total number of individual orders making up the total size at this level.
    pub order_count: NumberString,
}

impl From<Vec<NumberString>> for BookLevel {
    fn from(values: Vec<NumberString>) -> Self {
        Self {
            price: array_value(&values, 0),
            size: array_value(&values, 1),
            liquidated_order_count: array_value(&values, 2),
            order_count: array_value(&values, 3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ticker_and_retains_new_fields() {
        let row: TickerUpdate = serde_json::from_str(
            r#"{"instType":"SPOT","instId":"BTC-USDT","last":"1","ts":"2","futureField":"ok"}"#,
        )
        .unwrap();
        assert_eq!(row.inst_id, "BTC-USDT");
        assert_eq!(row.extra["futureField"], "ok");
    }

    #[test]
    fn parses_market_candle_array() {
        let row: CandleUpdate =
            serde_json::from_str(r#"["1","2","3","4","5","6","7","8","1"]"#).unwrap();
        assert_eq!(row.ts.as_str(), "1");
        assert_eq!(row.confirm.as_str(), "1");
    }
}
