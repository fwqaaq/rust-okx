//! Spread trading channel models (`sprd-orders`, `sprd-trades`) and spread-operation result rows.
//!
//! Mixed public and private channels.

use serde::Deserialize;

use super::ExtraFields;
use crate::model::NumberString;

/// Private `sprd-orders` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadOrderUpdate {
    /// Spread ID, e.g., `BTC-USDT_BTC-USDT-SWAP`.
    #[serde(default)]
    pub sprd_id: String,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Order size (number of contracts).
    #[serde(default)]
    pub sz: NumberString,
    /// Order type, e.g., `limit`, `post_only`, `ioc`, `fok`.
    #[serde(default)]
    pub ord_type: String,
    /// Order side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Fill size for the most recent fill of this push.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Fill price for the most recent fill.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Trade ID of the most recent fill.
    #[serde(default)]
    pub trade_id: String,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Size pending to be filled.
    #[serde(default)]
    pub pending_fill_sz: NumberString,
    /// Size pending to be settled.
    #[serde(default)]
    pub pending_settle_sz: NumberString,
    /// Canceled size.
    #[serde(default)]
    pub canceled_sz: NumberString,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    ///
    /// Documented values: `live`, `partially_filled`, `filled`, `canceled`.
    #[serde(default)]
    pub state: String,
    /// Source that triggered the cancellation.
    #[serde(default)]
    pub cancel_source: String,
    /// Client-supplied request ID, echoed from the original operation request.
    #[serde(default)]
    pub req_id: String,
    /// Result of the last amendment: `-1` failure, `0` success, `""` no amendment.
    #[serde(default)]
    pub amend_result: String,
    /// Error code; `"0"` on success.
    #[serde(default)]
    pub code: String,
    /// Error message; empty on success.
    #[serde(default)]
    pub msg: String,
    /// Order creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Leg execution nested in a spread trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadTradeLeg {
    /// Instrument ID of this leg, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Leg fill price.
    #[serde(default)]
    pub px: NumberString,
    /// Leg fill size in base currency.
    #[serde(default)]
    pub sz: NumberString,
    /// Leg side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Fill PnL for this leg (for closing fills; `""` otherwise).
    #[serde(default)]
    pub fill_pnl: NumberString,
    /// Fee charged for this leg.
    #[serde(default)]
    pub fee: NumberString,
    /// Leg fill size in contracts.
    #[serde(default)]
    pub sz_cont: NumberString,
    /// Fee currency for this leg.
    #[serde(default)]
    pub fee_ccy: String,
    /// Trade ID of this leg fill.
    #[serde(default)]
    pub trade_id: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Private/public spread-trade channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadTradeUpdate {
    /// Spread ID, e.g., `BTC-USDT_BTC-USDT-SWAP`.
    #[serde(default)]
    pub sprd_id: String,
    /// Trade ID assigned by OKX.
    #[serde(default)]
    pub trade_id: String,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Fill price of this trade.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size of this trade.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Trade side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Trade state: `filled` or `rejected`.
    #[serde(default)]
    pub state: String,
    /// Liquidity role of this fill: `T` (taker) or `M` (maker).
    #[serde(default)]
    pub exec_type: String,
    /// Trade timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Per-leg execution details for this trade.
    #[serde(default)]
    pub legs: Vec<SpreadTradeLeg>,
    /// Error code; `"0"` on success.
    #[serde(default)]
    pub code: String,
    /// Error message; empty on success.
    #[serde(default)]
    pub msg: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Public `sprd-public-trades` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-public-trades-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadPublicTradeUpdate {
    /// Spread ID, e.g., `BTC-USDT_BTC-USDT-SWAP`.
    #[serde(default)]
    pub sprd_id: String,
    /// Trade ID assigned by OKX.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade size.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade direction: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Filled time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// `sprd-tickers` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-tickers-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadTickerUpdate {
    /// Spread ID, e.g., `BTC-USDT_BTC-USDT-SWAP`.
    #[serde(default)]
    pub sprd_id: String,
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
    /// Open price over the past 24 hours.
    #[serde(default)]
    pub open24h: NumberString,
    /// Highest price over the past 24 hours.
    #[serde(default)]
    pub high24h: NumberString,
    /// Lowest price over the past 24 hours.
    #[serde(default)]
    pub low24h: NumberString,
    /// 24-hour trading volume in base currency (spot-USDT spreads) or USD (coin-margined spreads).
    #[serde(default)]
    pub vol24h: NumberString,
    /// Ticker data generation time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `sprd-order`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-place-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadPlaceOrderResult {
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// OKX-assigned order ID; empty on failure.
    #[serde(default)]
    pub ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Per-order status code; `"0"` on success.
    #[serde(default)]
    pub s_code: String,
    /// Per-order status message; empty on success.
    #[serde(default)]
    pub s_msg: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `sprd-amend-order`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-amend-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadAmendOrderResult {
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied request ID, echoed from the amend request.
    #[serde(default)]
    pub req_id: String,
    /// Per-order status code; `"0"` on success.
    #[serde(default)]
    pub s_code: String,
    /// Per-order status message; empty on success.
    #[serde(default)]
    pub s_msg: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `sprd-cancel-order`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadCancelOrderResult {
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Per-order status code; `"0"` on success.
    #[serde(default)]
    pub s_code: String,
    /// Per-order status message; empty on success.
    #[serde(default)]
    pub s_msg: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `sprd-mass-cancel`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-all-orders>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadMassCancelResult {
    /// `true` if the mass-cancel was accepted.
    #[serde(default)]
    pub result: bool,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_operation_result() {
        let row: SpreadAmendOrderResult = serde_json::from_str(
            r#"{"ordId":"1","clOrdId":"c","reqId":"r","sCode":"0","sMsg":""}"#,
        )
        .unwrap();
        assert_eq!(row.s_code, "0");
    }

    #[test]
    fn parses_spread_order_update() {
        let row: SpreadOrderUpdate = serde_json::from_str(
            r#"{
            "sprdId":"BTC-USDT_BTC-USDT-SWAP","ordId":"312269865356374016","clOrdId":"b1",
            "tag":"","px":"999","sz":"3","ordType":"limit","side":"buy","fillSz":"0",
            "fillPx":"","tradeId":"","accFillSz":"0","pendingFillSz":"2","pendingSettleSz":"1",
            "canceledSz":"1","state":"live","avgPx":"0","cancelSource":"","uTime":"1597026383085",
            "cTime":"1597026383085","code":"0","msg":"","reqId":"","amendResult":"",
            "instId":"ignored","pTime":"ignored"
        }"#,
        )
        .unwrap();
        assert_eq!(row.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(row.amend_result, "");
        assert!(row.extra.contains_key("instId"));
        assert!(row.extra.contains_key("pTime"));
    }

    #[test]
    fn parses_spread_trade_update() {
        let row: SpreadTradeUpdate = serde_json::from_str(
            r#"{
            "sprdId":"BTC-USDT-SWAP_BTC-USDT-200329","tradeId":"123","ordId":"123445",
            "clOrdId":"b16","tag":"","fillPx":"999","fillSz":"3","state":"filled","side":"buy",
            "execType":"M","ts":"1597026383085","legs":[],"code":"","msg":""
        }"#,
        )
        .unwrap();
        assert_eq!(row.sprd_id, "BTC-USDT-SWAP_BTC-USDT-200329");
        assert_eq!(row.exec_type, "M");
        assert_eq!(row.code, "");
        assert_eq!(row.msg, "");
    }

    #[test]
    fn parses_spread_public_trade_update() {
        let row: SpreadPublicTradeUpdate = serde_json::from_str(
            r#"{"sprdId":"BTC-USDT_BTC-USDT-SWAP","tradeId":"2499206329160695808","px":"-10","sz":"0.001","side":"sell","ts":"1726801105519"}"#,
        ).unwrap();
        assert_eq!(row.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(row.px.as_str(), "-10");
        assert_eq!(row.sz.as_str(), "0.001");
    }

    #[test]
    fn parses_spread_ticker_update() {
        let row: SpreadTickerUpdate = serde_json::from_str(
            r#"{
            "sprdId":"BTC-USDT_BTC-USDT-SWAP","last":"4","lastSz":"0.01","askPx":"19.7",
            "askSz":"5.79","bidPx":"5.9","bidSz":"5.79","open24h":"-7","high24h":"19.6",
            "low24h":"-7","vol24h":"9.87","ts":"1715247061026"
        }"#,
        )
        .unwrap();
        assert_eq!(row.sprd_id, "BTC-USDT_BTC-USDT-SWAP");
        assert_eq!(row.last.as_str(), "4");
        assert_eq!(row.ts.as_str(), "1715247061026");
    }
}
