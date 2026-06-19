use serde::Deserialize;

use crate::{
    NumberString,
    model::{OrderSide, OrderState, OrderType, PositionSide, TradeMode},
};

mod advanced;
mod algo;

pub use advanced::*;
pub use algo::*;

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

/// Details of an existing order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Order {
    /// Instrument ID.
    pub inst_id: String,
    /// OKX order ID.
    pub ord_id: String,
    /// Client-supplied order ID, if any.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order price.
    #[serde(default)]
    pub px: NumberString,
    /// Order size.
    #[serde(default)]
    pub sz: NumberString,
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
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    pub state: OrderState,
    /// Creation time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
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
    /// Fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Fill size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Fill side.
    pub side: OrderSide,
    /// Order type, when returned by OKX.
    #[serde(default)]
    pub ord_type: Option<OrderType>,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Fee amount.
    #[serde(default)]
    pub fee: NumberString,
    /// Fill timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Historical fill returned by `GET /api/v5/trade/fills-history`.
///
/// OKX documents this response separately from recent fills and does not return
/// `ordType` here, so this type intentionally does not reuse [`Fill`].
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
}
