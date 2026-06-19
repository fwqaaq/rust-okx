//! Algo trading channel models (`orders-algo`, `algo-advance`).
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use super::ExtraFields;
use crate::model::NumberString;

/// Private `orders-algo` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-algo-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AlgoOrderUpdate {
    /// Instrument type, e.g., `SPOT`, `MARGIN`, `SWAP`, `FUTURES`, `OPTION`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// Margin currency (cross-margin orders only).
    #[serde(default)]
    pub ccy: String,
    /// OKX-assigned regular order ID (populated when the algo fires and places an order).
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// OKX-assigned algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Algo order type.
    ///
    /// Documented values: `conditional`, `oco`, `trigger`, `move_order_stop`,
    /// `chase_order`, `iceberg`, `twap`.
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
    /// Target currency for quantity (spot currency-trade only): `base_ccy` or `quote_ccy`.
    #[serde(default)]
    pub tgt_ccy: String,
    /// Algo order state.
    ///
    /// Documented values: `live`, `pause`, `partially_effective`, `effective`,
    /// `canceled`, `order_failed`, `partially_failed`.
    #[serde(default)]
    pub state: String,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Actual order size when the algo fires.
    #[serde(default)]
    pub actual_sz: NumberString,
    /// Actual order price when the algo fires.
    #[serde(default)]
    pub actual_px: NumberString,
    /// Effective side of the actual order when the algo fires.
    #[serde(default)]
    pub actual_side: String,
    /// Trigger price (for `trigger` and `move_order_stop` types).
    #[serde(default)]
    pub trigger_px: NumberString,
    /// Trigger price type: `last`, `index`, or `mark`.
    #[serde(default)]
    pub trigger_px_type: String,
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
    /// Order price used when the algo fires.
    #[serde(default)]
    pub ord_px: NumberString,
    /// Trailing callback ratio (for `move_order_stop` and `chase_order` types).
    #[serde(default)]
    pub callback_ratio: NumberString,
    /// Trailing callback spread (for `move_order_stop` and `chase_order` types).
    #[serde(default)]
    pub callback_spread: NumberString,
    /// Activated tracking price (for `move_order_stop` and `chase_order` types).
    #[serde(default)]
    pub active_px: NumberString,
    /// Price that activated the trailing move trigger.
    #[serde(default)]
    pub move_trigger_px: NumberString,
    /// Whether this is a reduce-only order: `"true"` or `"false"`.
    #[serde(default)]
    pub reduce_only: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Error code when state is `order_failed`; empty otherwise.
    #[serde(default)]
    pub fail_code: String,
    /// Human-readable reason for failure.
    #[serde(default)]
    pub fail_reason: String,
    /// Amend-price-on-trigger type.
    ///
    /// Documented values: `0` (no amend), `1` (amend to market price at trigger).
    #[serde(default)]
    pub amend_px_on_trigger_type: String,
    /// Attached TP/SL algo orders.
    #[serde(default)]
    pub attach_algo_ords: Vec<Value>,
    /// Algo order creation time (Unix milliseconds).
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

/// Private `algo-advance` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-advance-algo-orders-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AdvancedAlgoOrderUpdate {
    /// Instrument type, e.g., `SPOT`, `SWAP`, `FUTURES`.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID, e.g., `BTC-USDT`.
    #[serde(default)]
    pub inst_id: String,
    /// OKX-assigned algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Advanced algo order type: `iceberg` or `twap`.
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
    /// Total order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Order state: `live`, `pause`, `partially_effective`, `effective`, `canceled`, `order_failed`.
    #[serde(default)]
    pub state: String,
    /// Fraction of the position to close (for close orders).
    #[serde(default)]
    pub close_fraction: NumberString,
    /// Trigger price for conditional activation.
    #[serde(default)]
    pub trigger_px: NumberString,
    /// Trigger price type: `last`, `index`, or `mark`.
    #[serde(default)]
    pub trigger_px_type: String,
    /// Limit price for each child order placed by the algo.
    #[serde(default)]
    pub ord_px: NumberString,
    /// Trailing callback ratio (for `move_order_stop`).
    #[serde(default)]
    pub callback_ratio: NumberString,
    /// Trailing callback spread (for `move_order_stop`).
    #[serde(default)]
    pub callback_spread: NumberString,
    /// Activated tracking price (for `move_order_stop`).
    #[serde(default)]
    pub active_px: NumberString,
    /// Price that activated the trailing move trigger.
    #[serde(default)]
    pub move_trigger_px: NumberString,
    /// Size executed so far.
    #[serde(default)]
    pub actual_sz: NumberString,
    /// Average fill price of executed child orders.
    #[serde(default)]
    pub actual_px: NumberString,
    /// Effective side of executed child orders.
    #[serde(default)]
    pub actual_side: String,
    /// Error code when state is `order_failed`; empty otherwise.
    #[serde(default)]
    pub fail_code: String,
    /// Human-readable reason for failure.
    #[serde(default)]
    pub fail_reason: String,
    /// Algo order creation time (Unix milliseconds).
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
