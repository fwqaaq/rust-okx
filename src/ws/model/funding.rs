//! Funding account channel models (`deposit-info`, `withdrawal-info`).
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use crate::model::NumberString;
use super::ExtraFields;

ws_object! {
    /// Funding-account `deposit-info` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-deposit-info-channel>
    DepositInfoUpdate {
        ccy: String,
        chain: String,
        amt: NumberString,
        from: String,
        area_code_from: String,
        to: String,
        tx_id: String,
        dep_id: String,
        from_wd_id: String,
        state: String,
        actual_dep_blk_confirm: NumberString,
        ts: NumberString
    }
}

ws_object! {
    /// Funding-account `withdrawal-info` channel row.
    ///
    /// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-withdrawal-info-channel>
    WithdrawalInfoUpdate {
        ccy: String,
        chain: String,
        non_tradable_asset: bool,
        amt: NumberString,
        fee: NumberString,
        fee_ccy: String,
        from: String,
        area_code_from: String,
        to: String,
        area_code_to: String,
        tag: String,
        pmt_id: String,
        memo: String,
        addr_ext: Value,
        tx_id: String,
        wd_id: String,
        state: String,
        client_id: String,
        ts: NumberString
    }
}
