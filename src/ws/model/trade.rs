//! Trade channel models (`orders`, `fills`) and order-operation result rows.
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Private `orders` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-order-channel>
    OrderUpdate {
        inst_type: String,
        inst_id: String,
        tgt_ccy: String,
        ccy: String,
        ord_id: String,
        cl_ord_id: String,
        tag: String,
        px: NumberString,
        sz: NumberString,
        ord_type: String,
        side: String,
        pos_side: String,
        td_mode: String,
        acc_fill_sz: NumberString,
        fill_px: NumberString,
        trade_id: String,
        fill_sz: NumberString,
        fill_time: NumberString,
        fill_pnl: NumberString,
        fill_fee: NumberString,
        fill_fee_ccy: String,
        exec_type: String,
        avg_px: NumberString,
        state: String,
        lever: NumberString,
        attach_algo_cl_ord_id: String,
        tp_trigger_px: NumberString,
        tp_trigger_px_type: String,
        tp_ord_px: NumberString,
        sl_trigger_px: NumberString,
        sl_trigger_px_type: String,
        sl_ord_px: NumberString,
        fee_ccy: String,
        fee: NumberString,
        rebate_ccy: String,
        rebate: NumberString,
        pnl: NumberString,
        source: String,
        category: String,
        reduce_only: String,
        cancel_source: String,
        cancel_source_reason: String,
        quick_mgn_type: String,
        algo_cl_ord_id: String,
        algo_id: String,
        amend_source: String,
        req_id: String,
        code: String,
        msg: String,
        px_type: String,
        px_usd: NumberString,
        px_vol: NumberString,
        linked_algo_ord: Value,
        attach_algo_ords: Vec<Value>,
        stp_id: String,
        stp_mode: String,
        trade_quote_ccy: String,
        outcome: String,
        is_elp_taker_access: bool,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
}

ws_object! {
    /// Private `fills` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-fills-channel>
    FillUpdate {
        inst_id: String,
        inst_type: String,
        trade_id: String,
        ord_id: String,
        cl_ord_id: String,
        bill_id: String,
        tag: String,
        fill_px: NumberString,
        fill_sz: NumberString,
        side: String,
        pos_side: String,
        exec_type: String,
        fee: NumberString,
        fee_ccy: String,
        fee_rate: NumberString,
        fill_pnl: NumberString,
        fill_px_vol: NumberString,
        fill_px_usd: NumberString,
        fill_mark_vol: NumberString,
        fill_fwd_px: NumberString,
        fill_mark_px: NumberString,
        fill_idx_px: NumberString,
        trade_quote_ccy: String,
        fill_time: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// Result row returned by regular order/place/cancel/amend operations.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-place-order>
    OrderOperationResult {
        ord_id: String,
        cl_ord_id: String,
        tag: String,
        ts: NumberString,
        req_id: String,
        s_code: String,
        s_msg: String,
        sub_code: String
    }
}

ws_object! {
    /// Result row returned by `mass-cancel` and `sprd-mass-cancel`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#order-book-trading-trade-ws-mass-cancel-order>
    MassCancelOperationResult {
        result: bool
    }
}
