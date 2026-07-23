use serde::Deserialize;

use crate::model::NumberString;

/// One instrument leg in a listed spread.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Leg direction.
    #[serde(default)]
    pub side: String,
}

/// Public Nitro Spread definition.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Spread {
    /// Spread ID.
    #[serde(default)]
    pub sprd_id: String,
    /// Spread type.
    #[serde(default)]
    pub sprd_type: String,
    /// Current spread state.
    #[serde(default)]
    pub state: String,
    /// Base currency.
    #[serde(default)]
    pub base_ccy: String,
    /// Currency used for order size.
    #[serde(default)]
    pub sz_ccy: String,
    /// Currency used to price the spread.
    #[serde(default)]
    pub quote_ccy: String,
    /// Price tick size.
    #[serde(default)]
    pub tick_sz: NumberString,
    /// Minimum order size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Order size increment.
    #[serde(default)]
    pub lot_sz: NumberString,
    /// Listing timestamp.
    #[serde(default)]
    pub list_time: NumberString,
    /// Expiration timestamp.
    #[serde(default)]
    pub exp_time: NumberString,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Spread legs.
    #[serde(default)]
    pub legs: Vec<SpreadLeg>,
}

/// Public Nitro Spread order book.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadOrderBook {
    /// Sell-side levels as price, quantity, and order count.
    #[serde(default)]
    pub asks: Vec<[NumberString; 3]>,
    /// Buy-side levels as price, quantity, and order count.
    #[serde(default)]
    pub bids: Vec<[NumberString; 3]>,
    /// Order book generation timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Public Nitro Spread trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadPublicTrade {
    /// Spread ID.
    #[serde(default)]
    pub sprd_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade quantity.
    #[serde(default)]
    pub sz: NumberString,
    /// Taker side.
    #[serde(default)]
    pub side: String,
    /// Trade timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of placing or canceling one spread order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadOrderResult {
    /// System order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Per-order result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-order result message.
    #[serde(default)]
    pub s_msg: String,
}

/// Result of mass-canceling spread orders.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadBooleanResult {
    /// Whether the request was accepted.
    #[serde(default)]
    pub result: bool,
}

/// Result of amending one spread order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadAmendOrderResult {
    /// System order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Client amendment request ID.
    #[serde(default)]
    pub req_id: String,
    /// Per-order result code.
    #[serde(default)]
    pub s_code: String,
    /// Per-order result message.
    #[serde(default)]
    pub s_msg: String,
}

/// Nitro Spread order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadOrder {
    /// Spread ID.
    #[serde(default)]
    pub sprd_id: String,
    /// System order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
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
    /// Order type.
    #[serde(default)]
    pub ord_type: String,
    /// Order side.
    #[serde(default)]
    pub side: String,
    /// Size of the most recent fill.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Price of the most recent fill.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Most recent trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Accumulated filled size.
    #[serde(default)]
    pub acc_fill_sz: NumberString,
    /// Pending fill size.
    #[serde(default)]
    pub pending_fill_sz: NumberString,
    /// Pending settlement size.
    #[serde(default)]
    pub pending_settle_sz: NumberString,
    /// Canceled size.
    #[serde(default)]
    pub canceled_sz: NumberString,
    /// Average fill price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Cancellation source.
    #[serde(default)]
    pub cancel_source: String,
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
}

/// One underlying leg of a private spread trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadTradeLeg {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Leg execution price.
    #[serde(default)]
    pub px: NumberString,
    /// Leg size in the spread size unit.
    #[serde(default)]
    pub sz: NumberString,
    /// Leg size in contracts.
    #[serde(default)]
    pub sz_cont: NumberString,
    /// Leg direction.
    #[serde(default)]
    pub side: String,
    /// Filled profit and loss.
    #[serde(default)]
    pub fill_pnl: NumberString,
    /// Trading fee or rebate.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Leg trade ID.
    #[serde(default)]
    pub trade_id: String,
}

/// Private Nitro Spread trade.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadTrade {
    /// Spread ID.
    #[serde(default)]
    pub sprd_id: String,
    /// Spread trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// System order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
    /// Spread fill price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Spread fill size.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Order side.
    #[serde(default)]
    pub side: String,
    /// Trade state.
    #[serde(default)]
    pub state: String,
    /// Execution type.
    #[serde(default)]
    pub exec_type: String,
    /// Trade timestamp.
    #[serde(default)]
    pub ts: NumberString,
    /// Underlying trade legs.
    #[serde(default)]
    pub legs: Vec<SpreadTradeLeg>,
    /// Per-trade result code.
    #[serde(default)]
    pub code: String,
    /// Per-trade result message.
    #[serde(default)]
    pub msg: String,
}

/// Response from configuring spread cancel-all-after.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SpreadCancelAllAfter {
    /// Protection trigger timestamp.
    #[serde(default)]
    pub trigger_time: NumberString,
    /// Timestamp when the request was received.
    #[serde(default)]
    pub ts: NumberString,
}
