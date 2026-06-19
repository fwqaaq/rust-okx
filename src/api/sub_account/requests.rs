use serde::Serialize;

use crate::model::{
    RequestValidationError, ValidateRequest, at_least_one, non_empty, one_of, range_u64,
};

/// Query parameters for [`SubAccount::get_subaccount_list`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl SubAccountListRequest {
    /// Start with no filters applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by enabled/disabled status.
    pub fn enable(mut self, enable: bool) -> Self {
        self.enable = Some(enable);
        self
    }

    /// Filter to a specific sub-account by name.
    pub fn sub_acct(mut self, sub_acct: impl Into<String>) -> Self {
        self.sub_acct = Some(sub_acct.into());
        self
    }

    /// Return results with UID older than this value.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return results with UID newer than this value.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Maximum number of results (default 100, max 100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_apikeys`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountApiKeysRequest {
    sub_acct: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

impl SubAccountApiKeysRequest {
    /// List all API keys for `sub_acct`.
    pub fn new(sub_acct: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            api_key: None,
        }
    }

    /// Retrieve a specific API key.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_trading_balances`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTradingBalancesRequest {
    sub_acct: String,
}

impl SubAccountTradingBalancesRequest {
    /// Query trading-account balances for `sub_acct`.
    pub fn new(sub_acct: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
        }
    }
}

/// Query parameters for [`SubAccount::get_subaccount_funding_balances`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountFundingBalancesRequest {
    sub_acct: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl SubAccountFundingBalancesRequest {
    /// Query funding-account balances for `sub_acct`.
    pub fn new(sub_acct: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            ccy: None,
        }
    }

    /// Filter by currency.
    pub fn ccy(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_max_withdrawal`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountMaxWithdrawalRequest {
    sub_acct: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl SubAccountMaxWithdrawalRequest {
    /// Query maximum withdrawal for `sub_acct`.
    pub fn new(sub_acct: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            ccy: None,
        }
    }

    /// Filter to a specific currency.
    pub fn ccy(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Request body for [`SubAccount::create_subaccount`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountRequest {
    sub_acct: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pwd: Option<String>,
}

impl CreateSubAccountRequest {
    /// Create a sub-account with the given name.
    pub fn new(sub_acct: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            r#type: None,
            label: None,
            pwd: None,
        }
    }

    /// Set the sub-account type.
    pub fn sub_type(mut self, sub_type: impl Into<String>) -> Self {
        self.r#type = Some(sub_type.into());
        self
    }

    /// Set a display label for the sub-account.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the login password.
    pub fn pwd(mut self, pwd: impl Into<String>) -> Self {
        self.pwd = Some(pwd.into());
        self
    }
}

/// Request body for [`SubAccount::create_subaccount_apikey`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountApiKeyRequest {
    sub_acct: String,
    label: String,
    passphrase: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    perm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
}

impl CreateSubAccountApiKeyRequest {
    /// Create an API key for `sub_acct` with a label and passphrase.
    pub fn new(
        sub_acct: impl Into<String>,
        label: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            label: label.into(),
            passphrase: passphrase.into(),
            perm: None,
            ip: None,
        }
    }

    /// Set permissions (comma-separated, e.g. `"read_only,trade"`).
    pub fn perm(mut self, perm: impl Into<String>) -> Self {
        self.perm = Some(perm.into());
        self
    }

    /// Restrict to specific IP addresses (comma-separated).
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

/// Request body for [`SubAccount::modify_subaccount_apikey`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifySubAccountApiKeyRequest {
    sub_acct: String,
    api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
}

impl ModifySubAccountApiKeyRequest {
    /// Modify the API key identified by `api_key` on `sub_acct`.
    pub fn new(sub_acct: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            api_key: api_key.into(),
            label: None,
            perm: None,
            ip: None,
        }
    }

    /// Change the label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Change permissions (comma-separated).
    pub fn perm(mut self, perm: impl Into<String>) -> Self {
        self.perm = Some(perm.into());
        self
    }

    /// Change allowed IP addresses (comma-separated).
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

/// Request body for [`SubAccount::delete_subaccount_apikey`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyRequest {
    sub_acct: String,
    api_key: String,
}

impl DeleteSubAccountApiKeyRequest {
    /// Delete the API key identified by `api_key` from `sub_acct`.
    pub fn new(sub_acct: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            api_key: api_key.into(),
        }
    }
}

/// Request body for [`SubAccount::transfer_between_subaccounts`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferRequest {
    ccy: String,
    amt: String,
    from: String,
    to: String,
    from_sub_account: String,
    to_sub_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    loan_trans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    omit_pos_risk: Option<String>,
}

impl SubAccountTransferRequest {
    /// Transfer `amt` of `ccy` between sub-accounts.
    ///
    /// `from` / `to` are account-type codes: `"6"` = funding account,
    /// `"18"` = trading account.
    pub fn new(
        ccy: impl Into<String>,
        amt: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
        from_sub_account: impl Into<String>,
        to_sub_account: impl Into<String>,
    ) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            from: from.into(),
            to: to.into(),
            from_sub_account: from_sub_account.into(),
            to_sub_account: to_sub_account.into(),
            loan_trans: None,
            omit_pos_risk: None,
        }
    }

    /// Whether to transfer with borrowing.
    pub fn loan_trans(mut self, loan_trans: bool) -> Self {
        self.loan_trans = Some(loan_trans);
        self
    }

    /// Whether to ignore position risk when transferring.
    pub fn omit_pos_risk(mut self, omit_pos_risk: impl Into<String>) -> Self {
        self.omit_pos_risk = Some(omit_pos_risk.into());
        self
    }
}

/// Request body for [`SubAccount::set_subaccount_transfer_out`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTransferOutRequest {
    sub_acct: String,
    // True is default value.
    can_trans_out: bool,
}

impl SetTransferOutRequest {
    /// Enable or disable transfers out for `sub_acct`.
    pub fn new(sub_acct: impl Into<String>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            can_trans_out: true,
        }
    }

    ///Filter by can_trans_out
    pub fn can_trans_out(mut self, can_trans_out: bool) -> Self {
        self.can_trans_out = can_trans_out;
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_bills`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountBillsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl SubAccountBillsRequest {
    /// Start with no filters applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn ccy(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by bill type.
    pub fn bill_type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Filter by sub-account name.
    pub fn sub_acct(mut self, sub_acct: impl Into<String>) -> Self {
        self.sub_acct = Some(sub_acct.into());
        self
    }

    /// Return results older than this bill ID.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return results newer than this bill ID.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Maximum number of results (default 100, max 100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_managed_bills`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSubAccountBillsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl ManagedSubAccountBillsRequest {
    /// Start with no filters applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn ccy(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by bill type.
    pub fn bill_type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Filter by sub-account name.
    pub fn sub_acct(mut self, sub_acct: impl Into<String>) -> Self {
        self.sub_acct = Some(sub_acct.into());
        self
    }

    /// Filter by sub-account UID.
    pub fn sub_uid(mut self, sub_uid: impl Into<String>) -> Self {
        self.sub_uid = Some(sub_uid.into());
        self
    }

    /// Return results older than this bill ID.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return results newer than this bill ID.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Maximum number of results (default 100, max 100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`SubAccount::get_entrust_subaccount_list`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntrustSubAccountListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<String>,
}

// ── ValidateRequest impls ────────────────────────────────────────────────────

impl ValidateRequest for SubAccountListRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}

impl ValidateRequest for SubAccountApiKeysRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)
    }
}

impl ValidateRequest for SubAccountTradingBalancesRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)
    }
}

impl ValidateRequest for SubAccountFundingBalancesRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)
    }
}

impl ValidateRequest for SubAccountMaxWithdrawalRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)
    }
}

impl ValidateRequest for CreateSubAccountRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)
    }
}

impl ValidateRequest for CreateSubAccountApiKeyRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)?;
        non_empty("label", &self.label)?;
        non_empty("passphrase", &self.passphrase)
    }
}

impl ValidateRequest for ModifySubAccountApiKeyRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)?;
        non_empty("apiKey", &self.api_key)?;
        at_least_one(
            "label, perm, ip",
            &[self.label.is_some(), self.perm.is_some(), self.ip.is_some()],
        )
    }
}

impl ValidateRequest for DeleteSubAccountApiKeyRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)?;
        non_empty("apiKey", &self.api_key)
    }
}

impl ValidateRequest for SubAccountTransferRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("ccy", &self.ccy)?;
        non_empty("amt", &self.amt)?;
        non_empty("fromSubAccount", &self.from_sub_account)?;
        non_empty("toSubAccount", &self.to_sub_account)?;
        one_of("from", &self.from, &["6", "18"], r#""6" or "18""#)?;
        one_of("to", &self.to, &["6", "18"], r#""6" or "18""#)
    }
}

impl ValidateRequest for SetTransferOutRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("subAcct", &self.sub_acct)
    }
}

impl ValidateRequest for SubAccountBillsRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}

impl ValidateRequest for ManagedSubAccountBillsRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        if let Some(limit) = self.limit {
            range_u64("limit", u64::from(limit), 1, 100)?;
        }
        Ok(())
    }
}
