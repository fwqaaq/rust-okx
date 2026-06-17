//! Market data channel models (`tickers`, `candles`, `trades`, `books`, etc.).
//!
//! Public channels; no authentication required.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

/// Access element `index` from a JSON array, falling back to `NumberString::default()`.
pub(super) fn array_value(values: &[NumberString], index: usize) -> NumberString {
    values.get(index).cloned().unwrap_or_default()
}

ws_object! {
    /// `tickers` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-tickers-channel>
    TickerUpdate {
        inst_type: String,
        inst_id: String,
        last: NumberString,
        last_sz: NumberString,
        ask_px: NumberString,
        ask_sz: NumberString,
        bid_px: NumberString,
        bid_sz: NumberString,
        open24h: NumberString,
        high24h: NumberString,
        low24h: NumberString,
        vol_ccy24h: NumberString,
        vol24h: NumberString,
        sod_utc0: NumberString,
        sod_utc8: NumberString,
        ts: NumberString
    }
}

/// `candle*` channel row represented by OKX's nine-element array.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-candlesticks-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(from = "Vec<NumberString>")]
#[non_exhaustive]
pub struct CandleUpdate {
    /// Opening time of the candlestick,
    /// Unix timestamp format in milliseconds
    pub ts: NumberString,
    /// Open price
    pub o: NumberString,
    /// highest price
    pub h: NumberString,
    /// Lowest price
    pub l: NumberString,
    /// Cloest price
    pub c: NumberString,
    /// Trading volume, with a unit of `contract`.
    /// If it is a `derivatives` contract, the value is the number of contracts.
    /// If it is `SPOT`/`MARGIN`, the value is the quantity in base currency.
    pub volume: NumberString,
    /// Trading volume, with a unit of `currency`.
    /// If it is a `derivatives` contract, the value is the number of base currency.
    /// If it is `SPOT`/`MARGIN`, the value is the quantity in quote currency.
    pub volume_ccy: NumberString,
    /// Trading volume, the value is the quantity in quote currency
    /// e.g. The unit is `USDT` for `BTC-USDT` and `BTC-USDT-SWAP`
    /// The unit is `USD` for `BTC-USD-SWAP`
    pub volume_quote: NumberString,
    /// The state of candlesticks
    /// 0: K line is uncompleted
    /// 1: K line is completed
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

ws_object! {
    /// `trades` and `trades-all` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-trades-channel>
    TradeUpdate {
        inst_id: String,
        trade_id: String,
        px: NumberString,
        sz: NumberString,
        side: String,
        source: String,
        count: NumberString,
        seq_id: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `option-trades` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-option-trades-channel>
    OptionTradeUpdate {
        inst_id: String,
        inst_family: String,
        trade_id: String,
        px: NumberString,
        sz: NumberString,
        side: String,
        opt_type: String,
        fill_vol: NumberString,
        fwd_px: NumberString,
        idx_px: NumberString,
        mark_px: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// `call-auction-details` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-call-auction-details-channel>
    CallAuctionDetailsUpdate {
        inst_id: String,
        eq_px: NumberString,
        matched_sz: NumberString,
        unmatched_sz: NumberString,
        auction_end_time: NumberString,
        state: String,
        ts: NumberString
    }
}

/// An order book push from `books`, `books5`, or tick-by-tick book channels.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-market-data-ws-order-book-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderBookUpdate {
    /// Order book on sell side
    #[serde(default)]
    pub asks: Vec<BookLevel>,
    #[serde(default)]
    /// Order book on buy side
    pub bids: Vec<BookLevel>,
    /// Checksum, implementation details below
    #[serde(default)]
    pub checksum: i64,
    /// Sequence ID of the last sent message.
    /// Only applicable to books, books-l2-tbt, books50-l2-tbt
    #[serde(default)]
    pub prev_seq_id: i64,
    /// Sequence ID of the current message,
    /// implementation details below
    #[serde(default)]
    pub seq_id: i64,
    /// Order book generation time
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized field
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
