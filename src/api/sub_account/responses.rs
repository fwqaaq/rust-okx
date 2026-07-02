use serde::Deserialize;

use crate::NumberString;

/// A sub-account entry (list, create, entrust-list, set-transfer-out).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountEntry {
    /// Sub-account name.
    #[serde(default)]
    pub sub_acct: String,
    /// Sub-account label.
    #[serde(default)]
    pub label: String,
    /// UID of the sub-account.
    #[serde(default)]
    pub uid: String,
    /// Creation timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
    /// Whether the sub-account is enabled.
    #[serde(default)]
    pub enable: bool,
    /// Whether transfers out are permitted.
    #[serde(default)]
    pub can_trans_out: bool,
    /// Whether Google Authenticator is enabled.
    #[serde(default)]
    pub g_auth: bool,
    /// Mobile number bound to the sub-account.
    #[serde(default)]
    pub mobile: String,
    /// Sub-account type (`"1"` = standard).
    #[serde(default, rename = "type")]
    pub sub_type: String,
    /// Sub-account level.
    #[serde(default)]
    pub sub_acct_lv: String,
    /// First-level sub-account name.
    #[serde(default)]
    pub first_lv_sub_acct: String,
    /// Whether DMA is enabled.
    #[serde(default)]
    pub if_dma: bool,
    /// List of frozen functions.
    #[serde(default)]
    pub frozen_func: Vec<String>,
}

/// An API key for a sub-account (create, list, modify, delete responses).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountApiKey {
    /// Sub-account name.
    #[serde(default)]
    pub sub_acct: String,
    /// API key label.
    #[serde(default)]
    pub label: String,
    /// API key value.
    #[serde(default)]
    pub api_key: String,
    /// Secret key (only returned on creation).
    #[serde(default)]
    pub secret_key: String,
    /// Passphrase (only returned on creation).
    #[serde(default)]
    pub passphrase: String,
    /// Permissions (comma-separated, e.g. `"read_only,trade"`).
    #[serde(default)]
    pub perm: String,
    /// Allowed IP addresses (comma-separated).
    #[serde(default)]
    pub ip: String,
    /// Creation timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Per-currency balance detail within a sub-account trading balance.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountTradingBalanceDetail {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Available equity.
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Cash balance.
    #[serde(default)]
    pub cash_bal: NumberString,
    /// Equity of the currency.
    #[serde(default)]
    pub eq: NumberString,
    /// Equity in USD.
    #[serde(default)]
    pub eq_usd: NumberString,
    /// Discount equity in USD.
    #[serde(default)]
    pub dis_eq: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
    /// Quantity of pending orders frozen.
    #[serde(default)]
    pub ord_frozen: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Interest.
    #[serde(default)]
    pub interest: NumberString,
    /// Liabilities.
    #[serde(default)]
    pub liab: NumberString,
    /// Cross-mode liabilities.
    #[serde(default)]
    pub cross_liab: NumberString,
    /// Isolated equity.
    #[serde(default)]
    pub iso_eq: NumberString,
    /// Isolated liabilities.
    #[serde(default)]
    pub iso_liab: NumberString,
    /// Update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Full trading-account balance for a sub-account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountTradingBalance {
    /// Per-currency details.
    #[serde(default)]
    pub details: Vec<SubAccountTradingBalanceDetail>,
    /// Adjusted equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Available equity in USD (Multi-currency margin / Portfolio margin).
    #[serde(default)]
    pub avail_eq: NumberString,
    /// Total equity in USD.
    #[serde(default)]
    pub total_eq: NumberString,
    /// Initial margin requirement in USD.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement in USD.
    #[serde(default)]
    pub mmr: NumberString,
    /// Margin ratio.
    #[serde(default)]
    pub mgn_ratio: NumberString,
    /// Isolated equity in USD.
    #[serde(default)]
    pub iso_eq: NumberString,
    /// Quantity of pending orders frozen (USD).
    #[serde(default)]
    pub ord_froz: NumberString,
    /// Notional value in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Unrealized profit and loss in USD.
    #[serde(default)]
    pub upl: NumberString,
    /// Update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Funding-account balance for a sub-account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountFundingBalance {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Total balance.
    #[serde(default)]
    pub bal: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
}

/// Maximum withdrawal amount for a sub-account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountMaxWithdrawal {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Maximum withdrawal amount.
    #[serde(default)]
    pub max_wd: NumberString,
    /// Maximum withdrawal amount (cross margin with borrowing).
    #[serde(default)]
    pub max_wd_ex: NumberString,
    /// Maximum spot-offset withdrawal amount.
    #[serde(default)]
    pub spot_offset_max_wd: NumberString,
    /// Maximum spot-offset withdrawal (cross margin with borrowing).
    #[serde(default)]
    pub spot_offset_max_wd_ex: NumberString,
}

/// An asset bill for a sub-account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountBill {
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Bill type.
    #[serde(default, rename = "type")]
    pub bill_type: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Transfer amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Sub-account name.
    #[serde(default)]
    pub sub_acct: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// An asset bill for a managed sub-account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ManagedSubAccountBill {
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Bill type.
    #[serde(default, rename = "type")]
    pub bill_type: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Transfer amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Sub-account name.
    #[serde(default)]
    pub sub_acct: String,
    /// Sub-account UID.
    #[serde(default)]
    pub sub_uid: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of an asset transfer between sub-accounts.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountTransferResult {
    /// Transfer ID.
    #[serde(default)]
    pub trans_id: String,
}

/// Result of setting transfer-out permission for a sub-account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetTransferOutResult {
    /// Sub-account name.
    #[serde(default)]
    pub sub_acct: String,
    /// Whether transfers out are now permitted.
    #[serde(default)]
    pub can_trans_out: bool,
}

/// An entrusted sub-account entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EntrustSubAccount {
    /// Sub-account name.
    #[serde(default)]
    pub sub_acct: String,
}
