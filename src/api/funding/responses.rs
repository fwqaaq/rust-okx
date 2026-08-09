use serde::Deserialize;

use crate::model::NumberString;

/// Currency metadata and chain settings.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Currency {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Display name.
    #[serde(default)]
    pub name: String,
    /// Chain name.
    #[serde(default)]
    pub chain: String,
    /// Minimum withdrawal amount.
    #[serde(default, rename = "minWd")]
    pub min_wd: NumberString,
    /// Minimum deposit amount.
    #[serde(default, rename = "minDep")]
    pub min_dep: NumberString,
    /// Withdrawal fee.
    #[serde(default, rename = "minFee")]
    pub min_fee: NumberString,
    /// Whether deposit is enabled.
    #[serde(default, rename = "canDep")]
    pub can_dep: bool,
    /// Whether withdrawal is enabled.
    #[serde(default, rename = "canWd")]
    pub can_wd: bool,
    /// Whether internal transfer is enabled.
    #[serde(default, rename = "canInternal")]
    pub can_internal: bool,
}

/// Funding-account balance for one currency.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingBalance {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Total balance.
    #[serde(default)]
    pub bal: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
}

/// Non-tradable asset row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NonTradableAsset {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Asset amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Asset type.
    #[serde(default, rename = "type")]
    pub asset_type: String,
}

/// Deposit address for one currency/chain.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositAddress {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Chain name.
    #[serde(default)]
    pub chain: String,
    /// Deposit address.
    #[serde(default)]
    pub addr: String,
    /// Address tag, memo, or payment ID when required by the chain.
    #[serde(default)]
    pub tag: String,
    /// Selected account.
    #[serde(default)]
    pub selected: bool,
}

/// Funds-transfer result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TransferResult {
    /// Transfer ID.
    #[serde(default, rename = "transId")]
    pub trans_id: String,
    /// Client-supplied ID.
    #[serde(default, rename = "clientId")]
    pub client_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// The remitting account.
    #[serde(default, rename = "from")]
    pub from_account: String,
    /// Transfer amount.
    #[serde(default)]
    pub amt: NumberString,
    /// The beneficiary account.
    #[serde(default)]
    pub to: String,
}

/// Funds-transfer state.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TransferState {
    /// Transfer ID.
    #[serde(default, rename = "transId")]
    pub trans_id: String,
    /// Client-supplied ID.
    #[serde(default, rename = "clientId")]
    pub client_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Transfer amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Transfer type.
    #[serde(default, rename = "type")]
    pub transfer_type: String,
    /// Source account.
    #[serde(default, rename = "from")]
    pub from_account: String,
    /// Destination account.
    #[serde(default)]
    pub to: String,
    /// Name of the sub-account.
    #[serde(default, rename = "subAcct")]
    pub sub_account: String,
    /// Transfer state.
    #[serde(default)]
    pub state: String,
}

/// Withdrawal result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WithdrawalResult {
    /// Withdrawal ID.
    #[serde(default, rename = "wdId")]
    pub wd_id: String,
    /// Client withdrawal ID.
    #[serde(default, rename = "clientId")]
    pub client_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Withdrawal amount.
    #[serde(default)]
    pub amt: NumberString,
}

/// Deposit history row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositRecord {
    /// Deposit ID.
    #[serde(default, rename = "depId")]
    pub dep_id: String,
    /// Transaction ID.
    #[serde(default, rename = "txId")]
    pub tx_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Chain name.
    #[serde(default)]
    pub chain: String,
    /// Deposit amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Deposit state.
    #[serde(default)]
    pub state: String,
    /// Timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Withdrawal history row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WithdrawalRecord {
    /// Withdrawal ID.
    #[serde(default, rename = "wdId")]
    pub wd_id: String,
    /// Client withdrawal ID.
    #[serde(default, rename = "clientId")]
    pub client_id: String,
    /// Transaction ID.
    #[serde(default, rename = "txId")]
    pub tx_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Chain name.
    #[serde(default)]
    pub chain: String,
    /// Withdrawal amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Fee.
    #[serde(default)]
    pub fee: NumberString,
    /// Withdrawal state.
    #[serde(default)]
    pub state: String,
    /// Timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Funding-account bill row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingBill {
    /// Bill ID.
    #[serde(default, rename = "billId")]
    pub bill_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Client-supplied transfer or withdrawal ID.
    #[serde(default, rename = "clientId")]
    pub client_id: String,
    /// Balance change.
    #[serde(default)]
    pub bal_chg: NumberString,
    /// Balance after change.
    #[serde(default)]
    pub bal: NumberString,
    /// Bill type.
    #[serde(default, rename = "type")]
    pub bill_type: String,
    /// Notes associated with the bill.
    #[serde(default)]
    pub notes: String,
    /// Timestamp.
    #[serde(default)]
    pub ts: NumberString,
}

/// Exchange recognized by OKX's public exchange list.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Exchange {
    /// Exchange name.
    #[serde(default, rename = "exchName")]
    pub exchange_name: String,
    /// Exchange decentralized identifier.
    #[serde(default, rename = "exchId")]
    pub exchange_id: String,
}

/// Result of applying for a monthly statement.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MonthlyStatementApplication {
    /// Download-link generation time as a Unix-millisecond string.
    #[serde(default)]
    pub ts: NumberString,
}

/// State of a requested monthly statement.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MonthlyStatement {
    /// Download file link.
    #[serde(default, rename = "fileHref")]
    pub file_href: String,
    /// Download-link generation time as a Unix-millisecond integer.
    #[serde(default)]
    pub ts: i64,
    /// Download-link status (`finished` or `ongoing`).
    #[serde(default)]
    pub state: String,
}

/// Lightning deposit invoice.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositLightning {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Invoice amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Lightning invoice.
    #[serde(default)]
    pub invoice: String,
    /// Recipient.
    #[serde(default)]
    pub to: String,
}

/// Lightning withdrawal result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WithdrawalLightning {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Withdrawal amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Withdrawal ID.
    #[serde(default, rename = "wdId")]
    pub wd_id: String,
}

/// Total asset valuation.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AssetValuation {
    /// Valuation details by account area.
    #[serde(default)]
    pub details: AssetValuationDetails,
    /// Total balance in the requested valuation currency.
    #[serde(default, rename = "totalBal")]
    pub total_bal: NumberString,
}

/// Asset valuation details by account area.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AssetValuationDetails {
    /// Funding-account valuation.
    #[serde(default)]
    pub funding: NumberString,
    /// Trading-account valuation.
    #[serde(default)]
    pub trading: NumberString,
    /// Earn-account valuation.
    #[serde(default)]
    pub earn: NumberString,
    /// Classic-account valuation.
    #[serde(default)]
    pub classic: NumberString,
}

/// Deposit/withdrawal status row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositWithdrawStatus {
    /// Withdrawal ID.
    #[serde(default, rename = "wdId")]
    pub wd_id: String,
    /// Transaction ID.
    #[serde(default, rename = "txId")]
    pub tx_id: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Chain name.
    #[serde(default)]
    pub chain: String,
    /// Destination address.
    #[serde(default)]
    pub to: String,
    /// State.
    #[serde(default)]
    pub state: String,
}

/// Dust-conversion result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertDustAssetsResult {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Converted amount.
    #[serde(default)]
    pub amt: NumberString,
}

/// Purchase/redemption result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PurchaseRedemptResult {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Operation side.
    #[serde(default)]
    pub side: String,
}
