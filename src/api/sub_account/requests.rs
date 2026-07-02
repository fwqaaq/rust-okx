use std::borrow::Cow;

use serde::Serialize;

/// Account type used in sub-account asset transfers.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum SubAccountType {
    /// Funding account (wire value `"6"`).
    #[serde(rename = "6")]
    Funding,
    /// Trading account (wire value `"18"`).
    #[serde(rename = "18")]
    Trading,
}

/// Query parameters for [`SubAccount::get_subaccount_list`](crate::api::sub_account::SubAccount::get_subaccount_list).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountListRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> SubAccountListRequest<'a> {
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
    pub fn sub_acct(mut self, sub_acct: impl Into<Cow<'a, str>>) -> Self {
        self.sub_acct = Some(sub_acct.into());
        self
    }

    /// Return results with UID older than this value.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return results with UID newer than this value.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Maximum number of results (default 100, max 100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_apikeys`](crate::api::sub_account::SubAccount::get_subaccount_apikeys).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountApiKeysRequest<'a> {
    sub_acct: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<Cow<'a, str>>,
}

impl<'a> SubAccountApiKeysRequest<'a> {
    /// List all API keys for `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            api_key: None,
        }
    }

    /// Retrieve a specific API key.
    pub fn api_key(mut self, api_key: impl Into<Cow<'a, str>>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_trading_balances`](crate::api::sub_account::SubAccount::get_subaccount_trading_balances).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTradingBalancesRequest<'a> {
    sub_acct: Cow<'a, str>,
}

impl<'a> SubAccountTradingBalancesRequest<'a> {
    /// Query trading-account balances for `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
        }
    }
}

/// Query parameters for [`SubAccount::get_subaccount_funding_balances`](crate::api::sub_account::SubAccount::get_subaccount_funding_balances).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountFundingBalancesRequest<'a> {
    sub_acct: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> SubAccountFundingBalancesRequest<'a> {
    /// Query funding-account balances for `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            ccy: None,
        }
    }

    /// Filter by currency.
    pub fn ccy(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_max_withdrawal`](crate::api::sub_account::SubAccount::get_subaccount_max_withdrawal).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountMaxWithdrawalRequest<'a> {
    sub_acct: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> SubAccountMaxWithdrawalRequest<'a> {
    /// Query maximum withdrawal for `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            ccy: None,
        }
    }

    /// Filter to a specific currency.
    pub fn ccy(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Request body for [`SubAccount::create_subaccount`](crate::api::sub_account::SubAccount::create_subaccount).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountRequest<'a> {
    sub_acct: Cow<'a, str>,
    r#type: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
    /// Password is security-sensitive; kept as owned `String`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pwd: Option<String>,
}

impl<'a> CreateSubAccountRequest<'a> {
    /// Create a sub-account with the given name and type.
    ///
    /// `sub_type` is required by OKX: `"1"` = standard sub-account,
    /// `"5"` = custody trading (Copper), `"12"` = custody trading (Komainu).
    pub fn new(sub_acct: impl Into<Cow<'a, str>>, sub_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            r#type: sub_type.into(),
            label: None,
            pwd: None,
        }
    }

    /// Set a display label for the sub-account.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the login password.
    pub fn pwd(mut self, pwd: impl Into<String>) -> Self {
        self.pwd = Some(pwd.into());
        self
    }
}

/// Request body for [`SubAccount::create_subaccount_apikey`](crate::api::sub_account::SubAccount::create_subaccount_apikey).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountApiKeyRequest<'a> {
    sub_acct: Cow<'a, str>,
    label: Cow<'a, str>,
    /// Passphrase is security-sensitive; kept as owned `String`.
    passphrase: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    perm: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<Cow<'a, str>>,
}

impl<'a> CreateSubAccountApiKeyRequest<'a> {
    /// Create an API key for `sub_acct` with a label and passphrase.
    pub fn new(
        sub_acct: impl Into<Cow<'a, str>>,
        label: impl Into<Cow<'a, str>>,
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
    pub fn perm(mut self, perm: impl Into<Cow<'a, str>>) -> Self {
        self.perm = Some(perm.into());
        self
    }

    /// Restrict to specific IP addresses (comma-separated).
    pub fn ip(mut self, ip: impl Into<Cow<'a, str>>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

/// Request body for [`SubAccount::modify_subaccount_apikey`](crate::api::sub_account::SubAccount::modify_subaccount_apikey).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifySubAccountApiKeyRequest<'a> {
    sub_acct: Cow<'a, str>,
    api_key: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perm: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<Cow<'a, str>>,
}

impl<'a> ModifySubAccountApiKeyRequest<'a> {
    /// Modify the API key identified by `api_key` on `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>, api_key: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            api_key: api_key.into(),
            label: None,
            perm: None,
            ip: None,
        }
    }

    /// Change the label.
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Change permissions (comma-separated).
    pub fn perm(mut self, perm: impl Into<Cow<'a, str>>) -> Self {
        self.perm = Some(perm.into());
        self
    }

    /// Change allowed IP addresses (comma-separated).
    pub fn ip(mut self, ip: impl Into<Cow<'a, str>>) -> Self {
        self.ip = Some(ip.into());
        self
    }
}

/// Request body for [`SubAccount::delete_subaccount_apikey`](crate::api::sub_account::SubAccount::delete_subaccount_apikey).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubAccountApiKeyRequest<'a> {
    sub_acct: Cow<'a, str>,
    api_key: Cow<'a, str>,
}

impl<'a> DeleteSubAccountApiKeyRequest<'a> {
    /// Delete the API key identified by `api_key` from `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>, api_key: impl Into<Cow<'a, str>>) -> Self {
        Self {
            sub_acct: sub_acct.into(),
            api_key: api_key.into(),
        }
    }
}

/// Request body for [`SubAccount::transfer_between_subaccounts`](crate::api::sub_account::SubAccount::transfer_between_subaccounts).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferRequest<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
    from: SubAccountType,
    to: SubAccountType,
    from_sub_account: Cow<'a, str>,
    to_sub_account: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loan_trans: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    omit_pos_risk: Option<bool>,
}

impl<'a> SubAccountTransferRequest<'a> {
    /// Transfer `amt` of `ccy` between sub-accounts.
    ///
    /// `from` / `to` select the account type on each side:
    /// [`SubAccountType::Funding`] or [`SubAccountType::Trading`].
    pub fn new(
        ccy: impl Into<Cow<'a, str>>,
        amt: impl Into<Cow<'a, str>>,
        from: SubAccountType,
        to: SubAccountType,
        from_sub_account: impl Into<Cow<'a, str>>,
        to_sub_account: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            from,
            to,
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
    pub fn omit_pos_risk(mut self, omit_pos_risk: bool) -> Self {
        self.omit_pos_risk = Some(omit_pos_risk);
        self
    }
}

/// Request body for [`SubAccount::set_subaccount_transfer_out`](crate::api::sub_account::SubAccount::set_subaccount_transfer_out).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTransferOutRequest<'a> {
    sub_acct: Cow<'a, str>,
    // True is default value.
    can_trans_out: bool,
}

impl<'a> SetTransferOutRequest<'a> {
    /// Enable or disable transfers out for `sub_acct`.
    pub fn new(sub_acct: impl Into<Cow<'a, str>>) -> Self {
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

/// Query parameters for [`SubAccount::get_subaccount_bills`](crate::api::sub_account::SubAccount::get_subaccount_bills).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountBillsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> SubAccountBillsRequest<'a> {
    /// Start with no filters applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn ccy(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by bill type.
    pub fn bill_type(mut self, r#type: impl Into<Cow<'a, str>>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Filter by sub-account name.
    pub fn sub_acct(mut self, sub_acct: impl Into<Cow<'a, str>>) -> Self {
        self.sub_acct = Some(sub_acct.into());
        self
    }

    /// Return results older than this bill ID.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return results newer than this bill ID.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Maximum number of results (default 100, max 100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`SubAccount::get_subaccount_managed_bills`](crate::api::sub_account::SubAccount::get_subaccount_managed_bills).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedSubAccountBillsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_uid: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> ManagedSubAccountBillsRequest<'a> {
    /// Start with no filters applied.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn ccy(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by bill type.
    pub fn bill_type(mut self, r#type: impl Into<Cow<'a, str>>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    /// Filter by sub-account name.
    pub fn sub_acct(mut self, sub_acct: impl Into<Cow<'a, str>>) -> Self {
        self.sub_acct = Some(sub_acct.into());
        self
    }

    /// Filter by sub-account UID.
    pub fn sub_uid(mut self, sub_uid: impl Into<Cow<'a, str>>) -> Self {
        self.sub_uid = Some(sub_uid.into());
        self
    }

    /// Return results older than this bill ID.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return results newer than this bill ID.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Maximum number of results (default 100, max 100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`SubAccount::get_entrust_subaccount_list`](crate::api::sub_account::SubAccount::get_entrust_subaccount_list).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntrustSubAccountListRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_acct: Option<Cow<'a, str>>,
}
