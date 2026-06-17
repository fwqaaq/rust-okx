//! Algo trading channel models (`orders-algo`, `algo-advance`).
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Private `orders-algo` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-algo-orders-channel>
    AlgoOrderUpdate {
        inst_type: String,
        inst_id: String,
        ccy: String,
        ord_id: String,
        cl_ord_id: String,
        algo_id: String,
        algo_cl_ord_id: String,
        sz: NumberString,
        ord_type: String,
        side: String,
        pos_side: String,
        td_mode: String,
        tgt_ccy: String,
        state: String,
        lever: NumberString,
        actual_sz: NumberString,
        actual_px: NumberString,
        actual_side: String,
        trigger_px: NumberString,
        trigger_px_type: String,
        tp_trigger_px: NumberString,
        tp_trigger_px_type: String,
        tp_ord_px: NumberString,
        sl_trigger_px: NumberString,
        sl_trigger_px_type: String,
        sl_ord_px: NumberString,
        ord_px: NumberString,
        callback_ratio: NumberString,
        callback_spread: NumberString,
        active_px: NumberString,
        move_trigger_px: NumberString,
        reduce_only: String,
        tag: String,
        fail_code: String,
        fail_reason: String,
        amend_px_on_trigger_type: String,
        attach_algo_ords: Vec<Value>,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
}

ws_object! {
    /// Private `algo-advance` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-algo-trading-ws-advance-algo-orders-channel>
    AdvancedAlgoOrderUpdate {
        inst_type: String,
        inst_id: String,
        algo_id: String,
        algo_cl_ord_id: String,
        ord_type: String,
        side: String,
        pos_side: String,
        td_mode: String,
        sz: NumberString,
        state: String,
        close_fraction: NumberString,
        trigger_px: NumberString,
        trigger_px_type: String,
        ord_px: NumberString,
        callback_ratio: NumberString,
        callback_spread: NumberString,
        active_px: NumberString,
        move_trigger_px: NumberString,
        actual_sz: NumberString,
        actual_px: NumberString,
        actual_side: String,
        fail_code: String,
        fail_reason: String,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
}
