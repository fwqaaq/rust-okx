use serde::Deserialize;

use crate::{
    NumberString,
    model::{OrderSide, OrderState, OrderType, PositionSide, TradeMode},
};

mod advanced;
mod algo;
mod risk_controls;

pub use advanced::*;
pub use algo::*;
pub use risk_controls::*;

/// The result of placing an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlaceOrderResult {
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Per-order status code (`"0"` on success).
    pub s_code: String,
    /// Per-order status message.
    #[serde(default)]
    pub s_msg: String,
    /// Per-order sub status code.
    #[serde(default)]
    pub sub_code: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// The result of cancelling an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelOrderResult {
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Per-order status code (`"0"` on success).
    pub s_code: String,
    /// Per-order status message.
    #[serde(default)]
    pub s_msg: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// The result of amending an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AmendOrderResult {
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Request ID, if supplied.
    #[serde(default)]
    pub req_id: String,
    /// Per-order status code (`"0"` on success).
    pub s_code: String,
    /// Per-order status message.
    #[serde(default)]
    pub s_msg: String,
    /// Per-order sub status code.
    #[serde(default)]
    pub sub_code: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// The result of closing a position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ClosePositionResult {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Client order ID, if supplied.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Tag, if supplied.
    #[serde(default)]
    pub tag: String,
}

/// An attached SL/TP algo order nested inside an [`Order`] (`attachAlgoOrds`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderAttachAlgoOrder {
    /// Value returned by OKX in the `attachAlgoId` field.
    #[serde(default)]
    pub attach_algo_id: String,
    /// Value returned by OKX in the `attachAlgoClOrdId` field.
    #[serde(default)]
    pub attach_algo_cl_ord_id: String,
    /// Value returned by OKX in the `tpOrdKind` field.
    #[serde(default)]
    pub tp_ord_kind: String,
    /// Value returned by OKX in the `tpTriggerPx` field.
    #[serde(default)]
    pub tp_trigger_px: NumberString,
    /// Value returned by OKX in the `tpTriggerRatio` field.
    #[serde(default)]
    pub tp_trigger_ratio: NumberString,
    /// Value returned by OKX in the `tpTriggerPxType` field.
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Value returned by OKX in the `tpOrdPx` field.
    #[serde(default)]
    pub tp_ord_px: NumberString,
    /// Value returned by OKX in the `slTriggerPx` field.
    #[serde(default)]
    pub sl_trigger_px: NumberString,
    /// Value returned by OKX in the `slTriggerRatio` field.
    #[serde(default)]
    pub sl_trigger_ratio: NumberString,
    /// Value returned by OKX in the `slTriggerPxType` field.
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Value returned by OKX in the `slOrdPx` field.
    #[serde(default)]
    pub sl_ord_px: NumberString,
    /// Value returned by OKX in the `sz` field.
    #[serde(default)]
    pub sz: NumberString,
    /// Value returned by OKX in the `amendPxOnTriggerType` field.
    #[serde(default)]
    pub amend_px_on_trigger_type: String,
    /// Value returned by OKX in the `callbackRatio` field.
    #[serde(default)]
    pub callback_ratio: NumberString,
    /// Value returned by OKX in the `callbackSpread` field.
    #[serde(default)]
    pub callback_spread: NumberString,
    /// Value returned by OKX in the `activePx` field.
    #[serde(default)]
    pub active_px: NumberString,
    /// Value returned by OKX in the `failCode` field.
    #[serde(default)]
    pub fail_code: String,
    /// Value returned by OKX in the `failReason` field.
    #[serde(default)]
    pub fail_reason: String,
}

/// A linked algo order nested inside an [`Order`] (`linkedAlgoOrd`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderLinkedAlgoOrder {
    /// Value returned by OKX in the `algoId` field.
    #[serde(default)]
    pub algo_id: String,
}

/// Details of an existing order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Order {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Order quantity unit setting for spot market orders (`tgtCcy`).
    #[serde(default)]
    pub tgt_ccy: String,
    /// Margin currency.
    #[serde(default)]
    pub ccy: String,
    /// OKX order ID.
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
    /// Order price in USD (options).
    #[serde(default)]
    pub px_usd: NumberString,
    /// Implied volatility of the order price (options).
    #[serde(default)]
    pub px_vol: NumberString,
    /// Price type (`px`/`pxVol`/`pxUsd`).
    #[serde(default)]
    pub px_type: String,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
    /// Profit and loss.
    #[serde(default)]
    pub pnl: NumberString,
    /// Order type.
    pub ord_type: OrderType,
    /// Order side.
    pub side: OrderSide,
    /// Position side.
    pub pos_side: PositionSide,
    /// Trade mode.
    pub td_mode: TradeMode,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Last fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Last trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Last filled size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Last fill time (Unix milliseconds).
    #[serde(default)]
    pub fill_time: NumberString,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    pub state: OrderState,
    /// Self-trade prevention ID.
    #[serde(default)]
    pub stp_id: String,
    /// Self-trade prevention mode.
    #[serde(default)]
    pub stp_mode: String,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Client-supplied ID of the attached SL/TP order.
    #[serde(default)]
    pub attach_algo_cl_ord_id: String,
    /// Take-profit trigger price.
    #[serde(default)]
    pub tp_trigger_px: NumberString,
    /// Take-profit trigger price type.
    #[serde(default)]
    pub tp_trigger_px_type: String,
    /// Take-profit order price.
    #[serde(default)]
    pub tp_ord_px: NumberString,
    /// Stop-loss trigger price.
    #[serde(default)]
    pub sl_trigger_px: NumberString,
    /// Stop-loss trigger price type.
    #[serde(default)]
    pub sl_trigger_px_type: String,
    /// Stop-loss order price.
    #[serde(default)]
    pub sl_ord_px: NumberString,
    /// Attached SL/TP algo orders.
    #[serde(default)]
    pub attach_algo_ords: Vec<OrderAttachAlgoOrder>,
    /// Linked algo order, if any.
    #[serde(default)]
    pub linked_algo_ord: Option<OrderLinkedAlgoOrder>,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee amount.
    #[serde(default)]
    pub fee: NumberString,
    /// Rebate currency.
    #[serde(default)]
    pub rebate_ccy: String,
    /// Rebate amount.
    #[serde(default)]
    pub rebate: NumberString,
    /// Order source.
    #[serde(default)]
    pub source: String,
    /// Order category.
    #[serde(default)]
    pub category: String,
    /// Whether the order is reduce-only (`"true"`/`"false"`).
    #[serde(default)]
    pub reduce_only: String,
    /// Whether the take-profit order is a limit order.
    #[serde(default)]
    pub is_tp_limit: String,
    /// Cancellation source.
    #[serde(default)]
    pub cancel_source: String,
    /// Cancellation source reason.
    #[serde(default)]
    pub cancel_source_reason: String,
    /// Quick margin type.
    #[serde(default)]
    pub quick_mgn_type: String,
    /// Client-supplied algo order ID.
    #[serde(default)]
    pub algo_cl_ord_id: String,
    /// Algo order ID.
    #[serde(default)]
    pub algo_id: String,
    /// Update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Trade quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
    /// Order outcome.
    #[serde(default)]
    pub outcome: String,
}

/// A trade fill.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Fill {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Bill sub-type.
    #[serde(default)]
    pub sub_type: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Index price at fill time.
    #[serde(default)]
    pub fill_idx_px: NumberString,
    /// Fill profit and loss.
    #[serde(default)]
    pub fill_pnl: NumberString,
    /// Implied volatility at fill price.
    #[serde(default)]
    pub fill_px_vol: NumberString,
    /// Fill price in USD.
    #[serde(default)]
    pub fill_px_usd: NumberString,
    /// Mark volatility at fill time.
    #[serde(default)]
    pub fill_mark_vol: NumberString,
    /// Forward price at fill time.
    #[serde(default)]
    pub fill_fwd_px: NumberString,
    /// Mark price at fill time.
    #[serde(default)]
    pub fill_mark_px: NumberString,
    /// Fill side.
    pub side: OrderSide,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Liquidity role, e.g. `T` or `M`.
    #[serde(default)]
    pub exec_type: String,
    /// Order type, when returned by OKX.
    ///
    /// Retained for backwards compatibility; not part of the current documented
    /// response and left as `None` when absent.
    #[serde(default)]
    pub ord_type: Option<OrderType>,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee amount.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee rate.
    #[serde(default)]
    pub fee_rate: NumberString,
    /// Fill timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Fill time (Unix milliseconds).
    #[serde(default)]
    pub fill_time: NumberString,
    /// Trade quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
}

/// Historical fill returned by `GET /api/v5/trade/fills-history`.
///
/// OKX documents this response separately from recent fills, so this type
/// intentionally does not reuse [`Fill`] (which retains a legacy `ordType`
/// field). The documented field set is otherwise identical.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FillHistory {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// OKX order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Bill sub-type.
    #[serde(default)]
    pub sub_type: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Index price at fill time.
    #[serde(default)]
    pub fill_idx_px: NumberString,
    /// Fill profit and loss.
    #[serde(default)]
    pub fill_pnl: NumberString,
    /// Implied volatility at fill price.
    #[serde(default)]
    pub fill_px_vol: NumberString,
    /// Fill price in USD.
    #[serde(default)]
    pub fill_px_usd: NumberString,
    /// Mark volatility at fill time.
    #[serde(default)]
    pub fill_mark_vol: NumberString,
    /// Forward price at fill time.
    #[serde(default)]
    pub fill_fwd_px: NumberString,
    /// Mark price at fill time.
    #[serde(default)]
    pub fill_mark_px: NumberString,
    /// Fill side.
    pub side: OrderSide,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Liquidity role, e.g. `T` or `M`.
    #[serde(default)]
    pub exec_type: String,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee amount.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee rate.
    #[serde(default)]
    pub fee_rate: NumberString,
    /// Fill timestamp (Unix milliseconds).
    #[serde(default)]
    pub fill_time: NumberString,
    /// Trade creation timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Trade quote currency.
    #[serde(default)]
    pub trade_quote_ccy: String,
}
