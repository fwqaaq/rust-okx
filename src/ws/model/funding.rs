//! Funding account channel models (`deposit-info`, `withdrawal-info`).
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use super::ExtraFields;
use crate::model::NumberString;

/// Funding-account `deposit-info` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-deposit-info-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositInfoUpdate {
    /// User identifier of the message producer.
    #[serde(default)]
    pub uid: String,
    /// Sub-account name; empty string for the master account.
    #[serde(default)]
    pub sub_acct: String,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,
    /// Currency being deposited, e.g., `USDT`.
    #[serde(default)]
    pub ccy: String,
    /// Chain identifier for the deposit, e.g., `USDT-TRC20`.
    #[serde(default)]
    pub chain: String,
    /// Deposit amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Sending address; may be empty for internal transfers.
    #[serde(default)]
    pub from: String,
    /// Country calling code for the sending address (phone-number deposits only).
    #[serde(default)]
    pub area_code_from: String,
    /// Receiving OKX deposit address.
    #[serde(default)]
    pub to: String,
    /// On-chain transaction hash; empty before broadcast.
    #[serde(default)]
    pub tx_id: String,
    /// Event timestamp when the deposit record was created (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Deposit state.
    ///
    /// Documented values: `0` waiting for confirmation, `1` deposit credited,
    /// `2` deposit successful, `8` pending review, `11` address blacklist,
    /// `12` account or deposit frozen, `13` sub-account deposit interception,
    /// `14` KYC limit.
    #[serde(default)]
    pub state: String,
    /// OKX-assigned deposit ID.
    #[serde(default)]
    pub dep_id: String,
    /// Withdrawal ID of the originating internal transfer; empty otherwise.
    #[serde(default)]
    pub from_wd_id: String,
    /// Number of on-chain confirmations received so far.
    #[serde(default)]
    pub actual_dep_blk_confirm: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

/// Funding-account `withdrawal-info` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-withdrawal-info-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WithdrawalInfoUpdate {
    /// User identifier of the message producer.
    #[serde(default)]
    pub uid: String,
    /// Sub-account name; empty string for the master account.
    #[serde(default)]
    pub sub_acct: String,
    /// Push time (Unix milliseconds).
    #[serde(default)]
    pub p_time: NumberString,
    /// Currency being withdrawn, e.g., `USDT`.
    #[serde(default)]
    pub ccy: String,
    /// Chain identifier for the withdrawal, e.g., `USDT-TRC20`.
    #[serde(default)]
    pub chain: String,
    /// Whether this currency is non-tradable.
    #[serde(default)]
    pub non_tradable_asset: bool,
    /// Withdrawal amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Event timestamp when the withdrawal request was submitted (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Source address or account; may be empty for internal transfers.
    #[serde(default)]
    pub from: String,
    /// Country calling code for the source address (phone-number withdrawals only).
    #[serde(default)]
    pub area_code_from: String,
    /// Destination withdrawal address.
    #[serde(default)]
    pub to: String,
    /// Country calling code for the destination address (phone-number withdrawals only).
    #[serde(default)]
    pub area_code_to: String,
    /// Address type: `1` wallet/email/phone/login, `2` UID.
    #[serde(default)]
    pub to_addr_type: String,
    /// Address tag (for currencies that require a tag, e.g., XRP).
    #[serde(default)]
    pub tag: String,
    /// Payment ID (for currencies that use a payment ID, e.g., Monero).
    #[serde(default)]
    pub pmt_id: String,
    /// Memo (for currencies that use a memo, e.g., EOS).
    #[serde(default)]
    pub memo: String,
    /// Withdrawal address attachment (structure varies by chain; `null` when absent).
    #[serde(default)]
    pub addr_ex: Value,
    /// On-chain transaction hash; empty before broadcast or for internal transfers.
    #[serde(default)]
    pub tx_id: String,
    /// Network fee charged for the withdrawal.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
    /// Withdrawal state.
    ///
    /// Documented values: `17` pending Travel Rule, `10` waiting transfer,
    /// `0` waiting withdrawal, `4`/`5`/`6`/`8`/`9`/`12` waiting manual review,
    /// `7` approved, `1` broadcasting to chain, `15` pending validation,
    /// `16` may take up to 24 h, `-3` canceling, `-2` canceled, `-1` failed,
    /// `2` success.
    #[serde(default)]
    pub state: String,
    /// OKX-assigned withdrawal ID.
    #[serde(default)]
    pub wd_id: String,
    /// Client-supplied withdrawal ID, if any.
    #[serde(default)]
    pub client_id: String,
    /// Withdrawal note.
    #[serde(default)]
    pub note: String,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
