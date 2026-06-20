//! Trade channel models (`orders`, `fills`) and order-operation result rows.
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use super::ExtraFields;
use crate::model::NumberString;

/// Private `orders` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-order-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderUpdate {
    /// Instrument type, e.g., `SPOT`, `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Target currency for quantity (spot currency-trade only).
    ///
    /// `base_ccy` means the order size is in base currency;
    /// `quote_ccy` means the order size is in quote currency.
    #[serde(default)]
    pub tgt_ccy: String,
    /// Margin currency (for cross-margin orders; empty otherwise).
    #[serde(default)]
    pub ccy: String,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Order price; empty for market orders.
    #[serde(default)]
    pub px: NumberString,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Estimated notional value of the order in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Order type.
    ///
    /// Documented values: `market`, `limit`, `post_only`, `fok`, `ioc`,
    /// `optimal_limit_ioc`, `mmp`, `mmp_and_post_only`, `op_fok`, `elp`.
    #[serde(default)]
    pub ord_type: String,
    /// Order side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Trade mode: `cross`, `isolated`, or `cash`.
    #[serde(default)]
    pub td_mode: String,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Fill price of the most recent fill in this push.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Trade ID of the most recent fill.
    #[serde(default)]
    pub trade_id: String,
    /// Fill size of the most recent fill.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Fill timestamp of the most recent fill (Unix milliseconds).
    #[serde(default)]
    pub fill_time: NumberString,
    /// Fill PnL of the most recent fill.
    #[serde(default)]
    pub fill_pnl: NumberString,
    /// Fee charged for the most recent fill.
    #[serde(default)]
    pub fill_fee: NumberString,
    /// Fee currency for the most recent fill.
    #[serde(default)]
    pub fill_fee_ccy: String,
    /// Filled notional value in USD for the most recent fill.
    #[serde(default)]
    pub fill_notional_usd: NumberString,
    /// Implied volatility at fill (options only).
    #[serde(default)]
    pub fill_px_vol: NumberString,
    /// USD-denominated option price at fill (options only).
    #[serde(default)]
    pub fill_px_usd: NumberString,
    /// Mark implied volatility at fill (options only).
    #[serde(default)]
    pub fill_mark_vol: NumberString,
    /// Forward price at fill (options only).
    #[serde(default)]
    pub fill_fwd_px: NumberString,
    /// Mark price at fill (FUTURES, SWAP, OPTION).
    #[serde(default)]
    pub fill_mark_px: NumberString,
    /// Index price at fill.
    #[serde(default)]
    pub fill_idx_px: NumberString,
    /// Liquidity role for the most recent fill: `T` (taker) or `M` (maker).
    #[serde(default)]
    pub exec_type: String,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    ///
    /// Documented values: `live`, `partially_filled`, `filled`, `canceled`, `mmp_canceled`.
    #[serde(default)]
    pub state: String,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Client-supplied attached algo order ID.
    #[serde(default)]
    pub attach_algo_cl_ord_id: String,
    /// Take-profit trigger price.
    #[serde(default)]
    pub tp_trigger_px: NumberString,
    /// Take-profit trigger price type: `last`, `index`, or `mark`.
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Take-profit order price; `-1` means market order.
    #[serde(default)]
    pub tp_ord_px: NumberString,
    /// Stop-loss trigger price.
    #[serde(default)]
    pub sl_trigger_px: NumberString,
    /// Stop-loss trigger price type: `last`, `index`, or `mark`.
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Stop-loss order price; `-1` means market order.
    #[serde(default)]
    pub sl_ord_px: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee amount; negative means deducted, positive means maker rebate.
    #[serde(default)]
    pub fee: NumberString,
    /// Rebate currency.
    #[serde(default)]
    pub rebate_ccy: String,
    /// Rebate amount (maker rebate).
    #[serde(default)]
    pub rebate: NumberString,
    /// Profit and loss for closing orders.
    #[serde(default)]
    pub pnl: NumberString,
    /// Order source (internal OKX field).
    #[serde(default)]
    pub source: String,
    /// Order category.
    ///
    /// Documented values: `normal`, `twap`, `adl`, `full_liquidation`,
    /// `partial_liquidation`, `delivery`, `ddh`, `auto_conversion`.
    #[serde(default)]
    pub category: String,
    /// Whether this is a reduce-only order: `"true"` or `"false"`.
    #[serde(default)]
    pub reduce_only: String,
    /// Whether this is a TP limit order: `"true"` or `"false"`.
    #[serde(default)]
    pub is_tp_limit: String,
    /// Source that triggered the cancellation.
    #[serde(default)]
    pub cancel_source: String,
    /// Client-supplied algo order ID that triggered this order.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// OKX-assigned algo order ID that triggered this order.
    #[serde(default)]
    pub algo_id: String,
    /// Source of the last amendment.
    #[serde(default)]
    pub amend_source: String,
    /// Result of the last amendment.
    ///
    /// `-1`: failure, `0`: success, `1`: auto-canceled, `2`: auto-amended (options only).
    #[serde(default)]
    pub amend_result: String,
    /// Client-supplied request ID, echoed from the operation that caused this push.
    #[serde(default)]
    pub req_id: String,
    /// Error code; `"0"` on success.
    #[serde(default)]
    pub code: String,
    /// Error message; empty on success.
    #[serde(default)]
    pub msg: String,
    /// Price type for options: `px` (price), `pxVol` (IV), or `pxUsd` (USD price).
    #[serde(default)]
    pub px_type: String,
    /// USD-denominated option order price.
    #[serde(default)]
    pub px_usd: NumberString,
    /// Implied-volatility option order price.
    #[serde(default)]
    pub px_vol: NumberString,
    /// Linked algo order details (JSON object; empty when not linked).
    #[serde(default)]
    pub linked_algo_ord: Value,
    /// Attached algo orders (TP/SL orders attached to this order).
    #[serde(default)]
    pub attach_algo_ords: Vec<Value>,
    /// Self-trade prevention mode: `cancel_maker`, `cancel_taker`, or `cancel_both`.
    #[serde(default)]
    pub stp_mode: String,
    /// Quote currency used for the trade (event contracts only).
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Last price at the time of the push.
    #[serde(default)]
    pub last_px: NumberString,
    /// Settlement outcome (event contracts only): `yes` or `no`.
    #[serde(default)]
    pub outcome: String,
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

/// Private `fills` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-fills-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FillUpdate {
    /// Instrument ID, e.g., `BTC-USDT-SWAP`.
    #[serde(default)]
    pub inst_id: String,
    /// Filled quantity; aggregated when `count > 1`.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Last filled price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Trade direction: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Filled time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// The last trade ID in the trades aggregation.
    #[serde(default)]
    pub trade_id: String,
    /// Liquidity role: `T` (taker) or `M` (maker).
    #[serde(default)]
    pub exec_type: String,
    /// Number of trades aggregated into this push.
    #[serde(default)]
    pub count: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `order` and `batch-orders`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-place-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlaceOrderResult {
    /// OKX-assigned order ID; empty on failure.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Timestamp when request processing finished (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Per-order status code; `"0"` on success.
    #[serde(default)]
    pub s_code: String,
    /// Per-order status message; empty on success.
    #[serde(default)]
    pub s_msg: String,
    /// Sub-code when `s_code` is non-zero; `""` on success.
    #[serde(default)]
    pub sub_code: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `cancel-order` and `batch-cancel-orders`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-cancel-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelOrderResult {
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Timestamp when request processing finished (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
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

/// Result row returned by `amend-order` and `batch-amend-orders`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-amend-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AmendOrderResult {
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Timestamp when request processing finished (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Client-supplied request ID, echoed from the amend request.
    #[serde(default)]
    pub req_id: String,
    /// Per-order status code; `"0"` on success.
    #[serde(default)]
    pub s_code: String,
    /// Per-order status message; empty on success.
    #[serde(default)]
    pub s_msg: String,
    /// Sub-code when `s_code` is non-zero; `""` on success.
    #[serde(default)]
    pub sub_code: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `mass-cancel`.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-mass-cancel-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MassCancelOperationResult {
    /// `true` if the mass-cancel was accepted by OKX.
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
    fn parses_order_update() {
        let row: OrderUpdate = serde_json::from_str(
            r#"{
            "accFillSz":"0.001","algoClOrdId":"","algoId":"","amendResult":"","amendSource":"",
            "avgPx":"31527.1","cancelSource":"","category":"normal","ccy":"","clOrdId":"",
            "code":"0","cTime":"1654084334977","execType":"M","fee":"-0.02522168","feeCcy":"USDT",
            "fillFee":"-0.02522168","fillFeeCcy":"USDT","fillNotionalUsd":"31.50818374",
            "fillPx":"31527.1","fillSz":"0.001","fillPnl":"0.01","fillTime":"1654084353263",
            "fillPxVol":"","fillPxUsd":"","fillMarkVol":"","fillFwdPx":"","fillMarkPx":"",
            "fillIdxPx":"","instId":"BTC-USDT","instType":"SPOT","lever":"0","msg":"",
            "notionalUsd":"31.50818374","ordId":"452197707845865472","ordType":"limit","pnl":"0",
            "posSide":"","px":"31527.1","pxUsd":"","pxVol":"","pxType":"","quickMgnType":"",
            "rebate":"0","rebateCcy":"BTC","reduceOnly":"false","reqId":"","side":"sell",
            "attachAlgoClOrdId":"","slOrdPx":"","slTriggerPx":"","slTriggerPxType":"last",
            "source":"","state":"filled","stpId":"","stpMode":"","sz":"0.001","tag":"",
            "tdMode":"cash","tgtCcy":"","tpOrdPx":"","tpTriggerPx":"","tpTriggerPxType":"last",
            "attachAlgoOrds":[],"tradeId":"242589207","tradeQuoteCcy":"USDT","lastPx":"38892.2",
            "uTime":"1654084353264","isTpLimit":"false","linkedAlgoOrd":{"algoId":""}
        }"#,
        )
        .unwrap();
        assert_eq!(row.ord_id, "452197707845865472");
        assert_eq!(row.state, "filled");
        assert_eq!(row.fill_notional_usd.as_str(), "31.50818374");
        assert_eq!(row.last_px.as_str(), "38892.2");
        assert_eq!(row.is_tp_limit, "false");
        // deprecated fields fall through to extra
        assert!(row.extra.contains_key("stpId"));
        assert!(row.extra.contains_key("quickMgnType"));
    }

    #[test]
    fn parses_fill_update() {
        let row: FillUpdate = serde_json::from_str(
            r#"{
            "instId":"BTC-USDT-SWAP","fillSz":"100","fillPx":"70000","side":"buy",
            "ts":"1705449605015","ordId":"680800019749904384","clOrdId":"1234567890",
            "tradeId":"12345","execType":"T","count":"10"
        }"#,
        )
        .unwrap();
        assert_eq!(row.inst_id, "BTC-USDT-SWAP");
        assert_eq!(row.exec_type, "T");
        assert_eq!(row.count.as_str(), "10");
    }

    #[test]
    fn parses_place_order_result() {
        let row: PlaceOrderResult = serde_json::from_str(
            r#"{"clOrdId":"","ordId":"12345689","tag":"","ts":"1695190491421","sCode":"0","sMsg":"","subCode":""}"#,
        )
        .unwrap();
        assert_eq!(row.ord_id, "12345689");
        assert_eq!(row.s_code, "0");
        assert_eq!(row.ts.as_str(), "1695190491421");
    }

    #[test]
    fn parses_cancel_order_result() {
        let row: CancelOrderResult = serde_json::from_str(
            r#"{"clOrdId":"","ordId":"2510789768709120","ts":"1695190491421","sCode":"0","sMsg":""}"#,
        )
        .unwrap();
        assert_eq!(row.ord_id, "2510789768709120");
        assert_eq!(row.s_code, "0");
    }

    #[test]
    fn parses_amend_order_result() {
        let row: AmendOrderResult = serde_json::from_str(
            r#"{"clOrdId":"","ordId":"2510789768709120","ts":"1695190491421","reqId":"b12344","sCode":"0","sMsg":"","subCode":""}"#,
        )
        .unwrap();
        assert_eq!(row.ord_id, "2510789768709120");
        assert_eq!(row.req_id, "b12344");
        assert_eq!(row.s_code, "0");
    }
}
