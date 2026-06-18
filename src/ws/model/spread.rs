//! Spread trading channel models (`sprd-orders`, `sprd-trades`) and spread-operation result rows.
//!
//! Mixed public and private channels.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

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
    /// Instrument ID of the spread.
    #[serde(default)]
    pub inst_id: String,
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
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,
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
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Fill price of this trade.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size of this trade.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Trade side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Order state after this fill.
    ///
    /// Documented values: `live`, `partially_filled`, `filled`, `canceled`.
    #[serde(default)]
    pub state: String,
    /// Liquidity role of this fill: `T` (taker) or `M` (maker).
    #[serde(default)]
    pub exec_type: String,
    /// Per-leg execution details for this trade.
    #[serde(default)]
    pub legs: Vec<SpreadTradeLeg>,
    /// Trade timestamp (Unix milliseconds).
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
}
