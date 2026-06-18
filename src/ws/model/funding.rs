//! Funding account channel models (`deposit-info`, `withdrawal-info`).
//!
//! Private channels; login required.

use serde::Deserialize;
use serde_json::Value;

use crate::model::NumberString;
use super::ExtraFields;

/// Funding-account `deposit-info` channel row.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#funding-account-websocket-deposit-info-channel>
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositInfoUpdate {
    /// Currency being deposited, e.g., `USDT`.
    #[serde(default)]
    pub ccy: String,
    /// Chain identifier for the deposit, e.g., `USDT-ERC20`.
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
    /// OKX-assigned deposit ID.
    #[serde(default)]
    pub dep_id: String,
    /// Withdrawal ID of the originating transfer (internal transfers only).
    #[serde(default)]
    pub from_wd_id: String,
    /// Deposit state.
    ///
    /// Documented values: `0` waiting for confirmation, `1` deposit credited,
    /// `2` deposit successful, `8` pending review, `11` match the address blacklist,
    /// `12` account or deposit is frozen, `13` sub-account deposit interception,
    /// `14` KYC limit.
    #[serde(default)]
    pub state: String,
    /// Number of on-chain confirmations received so far.
    #[serde(default)]
    pub actual_dep_blk_confirm: NumberString,
    /// Event timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
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
    /// Currency being withdrawn, e.g., `USDT`.
    #[serde(default)]
    pub ccy: String,
    /// Chain identifier for the withdrawal, e.g., `USDT-ERC20`.
    #[serde(default)]
    pub chain: String,
    /// Whether this currency is non-tradable (tradable only via withdrawal/deposit).
    #[serde(default)]
    pub non_tradable_asset: bool,
    /// Withdrawal amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Network fee charged for the withdrawal.
    #[serde(default)]
    pub fee: NumberString,
    /// Fee currency.
    #[serde(default)]
    pub fee_ccy: String,
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
    /// Address tag (for currencies that require a tag/memo, e.g., XRP).
    #[serde(default)]
    pub tag: String,
    /// Payment ID (for currencies that use a payment ID, e.g., Monero).
    #[serde(default)]
    pub pmt_id: String,
    /// Memo (for currencies that use a memo, e.g., EOS).
    #[serde(default)]
    pub memo: String,
    /// Extended address metadata (JSON object; structure varies by chain).
    #[serde(default)]
    pub addr_ext: Value,
    /// On-chain transaction hash; empty before broadcast.
    #[serde(default)]
    pub tx_id: String,
    /// OKX-assigned withdrawal ID.
    #[serde(default)]
    pub wd_id: String,
    /// Withdrawal state.
    ///
    /// Documented values: `-3` canceling, `-2` canceled, `-1` failed,
    /// `0` waiting withdrawal, `1` withdrawing, `2` withdrawal success,
    /// `7` approved, `10` waiting transfer, `4` waiting manual review.
    #[serde(default)]
    pub state: String,
    /// Client-supplied withdrawal ID, if any.
    #[serde(default)]
    pub client_id: String,
    /// Event timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Unrecognized fields retained for forward compatibility.
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}
