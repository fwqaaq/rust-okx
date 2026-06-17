//! Trading bot channel models (`grid-orders-*`, `grid-positions`, `recurring-buy`, etc.).
//!
//! Private channels; login required.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Spot/contract grid-order channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-orders-channel>
    GridOrderUpdate {
        inst_id: String,
        algo_id: String,
        algo_cl_ord_id: String,
        inst_type: String,
        algo_ord_type: String,
        state: String,
        direction: String,
        run_type: String,
        sz: NumberString,
        investment: NumberString,
        total_pnl: NumberString,
        pnl_ratio: NumberString,
        stop_type: String,
        cancel_type: String,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
}

/// Backward-compatible name for grid-order updates.
pub type TradingBotUpdate = GridOrderUpdate;

ws_object! {
    /// `grid-positions` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-positions-channel>
    GridPositionUpdate {
        algo_id: String,
        inst_id: String,
        inst_type: String,
        mgn_mode: String,
        pos_side: String,
        pos: NumberString,
        avg_px: NumberString,
        upl: NumberString,
        upl_ratio: NumberString,
        lever: NumberString,
        liq_px: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// `grid-sub-orders` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-grid-sub-orders-channel>
    GridSubOrderUpdate {
        algo_id: String,
        inst_id: String,
        ord_id: String,
        cl_ord_id: String,
        side: String,
        px: NumberString,
        sz: NumberString,
        state: String,
        avg_px: NumberString,
        acc_fill_sz: NumberString,
        fee: NumberString,
        fee_ccy: String,
        pnl: NumberString,
        c_time: NumberString,
        u_time: NumberString
    }
}

ws_object! {
    /// Currency allocation nested in [`RecurringBuyOrderUpdate`].
    RecurringBuyAllocation {
        ccy: String,
        ratio: NumberString
    }
}

ws_object! {
    /// `algo-recurring-buy` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#trading-bot-websocket-recurring-buy-orders-channel>
    RecurringBuyOrderUpdate {
        algo_id: String,
        algo_cl_ord_id: String,
        inst_type: String,
        algo_ord_type: String,
        state: String,
        stgy_name: String,
        recurring_list: Vec<RecurringBuyAllocation>,
        period: String,
        recurring_day: String,
        recurring_hour: String,
        recurring_time: String,
        time_zone: String,
        amt: NumberString,
        investment_amt: NumberString,
        investment_ccy: String,
        total_investment: NumberString,
        total_pnl: NumberString,
        total_ann_rate: NumberString,
        pnl_ratio: NumberString,
        mkt_cap: NumberString,
        cycles: NumberString,
        tag: String,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
}

ws_object! {
    /// `copytrading-notification` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#copy-trading-websocket-copy-trading-notification-channel>
    CopyTradingNotification {
        notification_type: String,
        notification_msg: String,
        inst_type: String,
        inst_id: String,
        sub_pos_id: String,
        unique_code: String,
        ts: NumberString
    }
}
