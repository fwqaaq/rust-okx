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
    /// Order type.
    ///
    /// Documented values: `market`, `limit`, `post_only`, `fok`, `ioc`,
    /// `optimal_limit_ioc`, `mmp`, `mmp_and_post_only`, `op_fok`.
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
    /// Liquidity role for the most recent fill: `T` (taker) or `M` (maker).
    #[serde(default)]
    pub exec_type: String,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    ///
    /// Documented values: `live`, `partially_filled`, `filled`, `canceled`.
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
    /// `partial_liquidation`, `delivery`, `ddh`.
    #[serde(default)]
    pub category: String,
    /// Whether this is a reduce-only order: `"true"` or `"false"`.
    #[serde(default)]
    pub reduce_only: String,
    /// Source that triggered the cancellation.
    #[serde(default)]
    pub cancel_source: String,
    /// Human-readable reason for the cancellation.
    #[serde(default)]
    pub cancel_source_reason: String,
    /// Quick-margin type: `manual`, `auto_borrow`, or `auto_repay`.
    #[serde(default)]
    pub quick_mgn_type: String,
    /// Client-supplied algo order ID that triggered this order.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// OKX-assigned algo order ID that triggered this order.
    #[serde(default)]
    pub algo_id: String,
    /// Source of the last amendment.
    #[serde(default)]
    pub amend_source: String,
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
    /// Self-trade prevention group ID.
    #[serde(default)]
    pub stp_id: String,
    /// Self-trade prevention mode: `cancel_maker`, `cancel_taker`, or `cancel_both`.
    #[serde(default)]
    pub stp_mode: String,
    /// Quote currency used for the trade (event contracts only).
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Settlement outcome (event contracts only).
    #[serde(default)]
    pub outcome: String,
    /// Whether this order accessed the Enhanced Liquidity Provider (ELP) pool.
    #[serde(default)]
    pub is_elp_taker_access: bool,
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

/// Private `fills` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-fills-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FillUpdate {
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Instrument type, e.g., `SPOT`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Trade ID assigned by OKX.
    #[serde(default)]
    pub trade_id: String,
    /// OKX-assigned order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Bill ID associated with this fill.
    #[serde(default)]
    pub bill_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Fill side: `buy` or `sell`.
    #[serde(default)]
    pub side: String,
    /// Position side: `long`, `short`, or `net`.
    #[serde(default)]
    pub pos_side: String,
    /// Liquidity role: `T` (taker) or `M` (maker).
    #[serde(default)]
    pub exec_type: String,
    /// Fee amount; negative means deducted.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee rate applied to this fill.
    #[serde(default)]
    pub fee_rate: NumberString,
    /// Fill profit and loss (for closing fills).
    #[serde(default)]
    pub fill_pnl: NumberString,
    /// Implied volatility at the fill price (options only).
    #[serde(default)]
    pub fill_px_vol: NumberString,
    /// USD-denominated option price at fill (options only).
    #[serde(default)]
    pub fill_px_usd: NumberString,
    /// Mark implied volatility at fill time (options only).
    #[serde(default)]
    pub fill_mark_vol: NumberString,
    /// Forward price at fill time (options only).
    #[serde(default)]
    pub fill_fwd_px: NumberString,
    /// Mark price at fill time.
    #[serde(default)]
    pub fill_mark_px: NumberString,
    /// Index price at fill time.
    #[serde(default)]
    pub fill_idx_px: NumberString,
    /// Quote currency used for the trade (event contracts only).
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Fill timestamp (Unix milliseconds).
    #[serde(default)]
    pub fill_time: NumberString,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by regular order place/cancel/amend operations.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-place-order>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderOperationResult {
    /// OKX-assigned order ID; empty on failure.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Operation timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Client-supplied request ID, echoed from the operation.
    #[serde(default)]
    pub req_id: String,
    /// Per-order status code; `"0"` on success.
    #[serde(default)]
    pub s_code: String,
    /// Per-order status message; empty on success.
    #[serde(default)]
    pub s_msg: String,
    /// Sub-error code when `s_code` is non-zero; provides more detail.
    #[serde(default)]
    pub sub_code: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Result row returned by `mass-cancel` and `sprd-mass-cancel`.
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
