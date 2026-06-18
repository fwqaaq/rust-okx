//! Spread trading channel models (`sprd-orders`, `sprd-trades`) and spread-operation result rows.
//!
//! Mixed public and private channels.

use serde::Deserialize;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Private `sprd-orders` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-orders-channel>
    SpreadOrderUpdate {
        sprd_id: String,
        inst_id: String,
        ord_id: String,
        cl_ord_id: String,
        tag: String,
        px: NumberString,
        sz: NumberString,
        ord_type: String,
        side: String,
        fill_sz: NumberString,
        fill_px: NumberString,
        trade_id: String,
        acc_fill_sz: NumberString,
        pending_fill_sz: NumberString,
        pending_settle_sz: NumberString,
        canceled_sz: NumberString,
        avg_px: NumberString,
        state: String,
        cancel_source: String,
        req_id: String,
        code: String,
        msg: String,
        c_time: NumberString,
        u_time: NumberString,
        p_time: NumberString
    }
}

ws_object! {
    /// Leg execution nested in a private spread trade.
    SpreadTradeLeg {
        inst_id: String,
        px: NumberString,
        sz: NumberString,
        side: String,
        fee: NumberString,
        sz_cont: NumberString,
        fee_ccy: String,
        trade_id: String
    }
}

ws_object! {
    /// Private/public spread-trade channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-sprd-trades-channel>
    SpreadTradeUpdate {
        sprd_id: String,
        trade_id: String,
        ord_id: String,
        cl_ord_id: String,
        tag: String,
        px: NumberString,
        sz: NumberString,
        fill_px: NumberString,
        fill_sz: NumberString,
        side: String,
        state: String,
        exec_type: String,
        legs: Vec<SpreadTradeLeg>,
        ts: NumberString
    }
}

ws_object! {
    /// Result row returned by `sprd-order`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-place-order>
    SpreadPlaceOrderResult {
        cl_ord_id: String,
        ord_id: String,
        tag: String,
        s_code: String,
        s_msg: String
    }
}

ws_object! {
    /// Result row returned by `sprd-amend-order`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-amend-order>
    SpreadAmendOrderResult {
        cl_ord_id: String,
        ord_id: String,
        req_id: String,
        s_code: String,
        s_msg: String
    }
}

ws_object! {
    /// Result row returned by `sprd-cancel-order`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-order>
    SpreadCancelOrderResult {
        cl_ord_id: String,
        ord_id: String,
        s_code: String,
        s_msg: String
    }
}

ws_object! {
    /// Result row returned by `sprd-mass-cancel`.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#spread-trading-websocket-trade-api-ws-cancel-all-orders>
    SpreadMassCancelResult {
        result: bool
    }
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
