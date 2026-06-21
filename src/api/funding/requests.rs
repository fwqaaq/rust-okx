use serde::Serialize;

/// Request for [`get_currencies`](crate::api::funding::Funding::get_currencies),
/// [`get_balances`](crate::api::funding::Funding::get_balances),
/// [`get_non_tradable_assets`](crate::api::funding::Funding::get_non_tradable_assets), and
/// [`get_asset_valuation`](crate::api::funding::Funding::get_asset_valuation).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CurrencyRequest<'a> {
    /// Currency filter, e.g. `Some("BTC")`. `None` returns all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<&'a str>,
}

/// Request for [`get_deposit_address`](crate::api::funding::Funding::get_deposit_address).
#[derive(Debug, Clone, Serialize)]
pub struct DepositAddressRequest<'a> {
    /// Currency, e.g. `"USDT"`.
    pub ccy: &'a str,
}

/// Request for [`transfer_state`](crate::api::funding::Funding::transfer_state).
#[derive(Debug, Clone, Serialize)]
pub struct TransferStateRequest<'a> {
    /// Transfer ID.
    #[serde(rename = "transId")]
    pub trans_id: &'a str,
    /// Transfer type (optional).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<&'a str>,
}

/// Request for [`cancel_withdrawal`](crate::api::funding::Funding::cancel_withdrawal).
#[derive(Debug, Clone, Serialize)]
pub struct CancelWithdrawalRequest<'a> {
    /// Withdrawal ID.
    #[serde(rename = "wdId")]
    pub wd_id: &'a str,
}

/// Request body for [`Funding::funds_transfer`](crate::api::funding::Funding::funds_transfer).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundsTransferRequest {
    ccy: String,
    amt: String,
    #[serde(rename = "from")]
    from_account: String,
    to: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    transfer_type: Option<String>,
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    sub_account: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "toInstId", skip_serializing_if = "Option::is_none")]
    to_inst_id: Option<String>,
    #[serde(rename = "loanTrans", skip_serializing_if = "Option::is_none")]
    loan_transfer: Option<String>,
}

impl FundsTransferRequest {
    /// Create a funds-transfer request.
    pub fn new(
        ccy: impl Into<String>,
        amt: impl Into<String>,
        from_account: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            from_account: from_account.into(),
            to: to.into(),
            transfer_type: None,
            sub_account: None,
            inst_id: None,
            to_inst_id: None,
            loan_transfer: None,
        }
    }

    /// Set transfer type, e.g. `0` for transfer within account.
    pub fn transfer_type(mut self, transfer_type: impl Into<String>) -> Self {
        self.transfer_type = Some(transfer_type.into());
        self
    }

    /// Set sub-account name.
    pub fn sub_account(mut self, sub_account: impl Into<String>) -> Self {
        self.sub_account = Some(sub_account.into());
        self
    }

    /// Set source instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set destination instrument ID.
    pub fn to_inst_id(mut self, to_inst_id: impl Into<String>) -> Self {
        self.to_inst_id = Some(to_inst_id.into());
        self
    }

    /// Set whether this is a loan transfer.
    pub fn loan_transfer(mut self, loan_transfer: impl Into<String>) -> Self {
        self.loan_transfer = Some(loan_transfer.into());
        self
    }
}

/// Request body for [`Funding::withdrawal`](crate::api::funding::Funding::withdrawal).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequest {
    ccy: String,
    amt: String,
    dest: String,
    #[serde(rename = "toAddr")]
    to_addr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    chain: Option<String>,
    #[serde(rename = "areaCode", skip_serializing_if = "Option::is_none")]
    area_code: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(rename = "toAddrType", skip_serializing_if = "Option::is_none")]
    to_addr_type: Option<String>,
}

impl WithdrawalRequest {
    /// Create a withdrawal request.
    pub fn new(
        ccy: impl Into<String>,
        amt: impl Into<String>,
        dest: impl Into<String>,
        to_addr: impl Into<String>,
    ) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            dest: dest.into(),
            to_addr: to_addr.into(),
            chain: None,
            area_code: None,
            client_id: None,
            to_addr_type: None,
        }
    }

    /// Set withdrawal chain.
    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set phone area code for internal withdrawals.
    pub fn area_code(mut self, area_code: impl Into<String>) -> Self {
        self.area_code = Some(area_code.into());
        self
    }

    /// Set client withdrawal ID.
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Set address type.
    pub fn to_addr_type(mut self, to_addr_type: impl Into<String>) -> Self {
        self.to_addr_type = Some(to_addr_type.into());
        self
    }
}

/// Query parameters for [`Funding::get_deposit_history`](crate::api::funding::Funding::get_deposit_history).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    deposit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<String>,
    #[serde(rename = "depId", skip_serializing_if = "Option::is_none")]
    dep_id: Option<String>,
    #[serde(rename = "fromWdId", skip_serializing_if = "Option::is_none")]
    from_wd_id: Option<String>,
}

impl DepositHistoryRequest {
    /// Create an empty deposit-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by deposit type.
    pub fn deposit_type(mut self, deposit_type: impl Into<String>) -> Self {
        self.deposit_type = Some(deposit_type.into());
        self
    }

    /// Filter by state.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Page after the given ID.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page before the given ID.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set result limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by transaction ID.
    pub fn tx_id(mut self, tx_id: impl Into<String>) -> Self {
        self.tx_id = Some(tx_id.into());
        self
    }

    /// Filter by deposit ID.
    pub fn dep_id(mut self, dep_id: impl Into<String>) -> Self {
        self.dep_id = Some(dep_id.into());
        self
    }

    /// Filter by source withdrawal ID.
    pub fn from_wd_id(mut self, from_wd_id: impl Into<String>) -> Self {
        self.from_wd_id = Some(from_wd_id.into());
        self
    }
}

/// Query parameters for [`Funding::get_withdrawal_history`](crate::api::funding::Funding::get_withdrawal_history).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "wdId", skip_serializing_if = "Option::is_none")]
    wd_id: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    withdrawal_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "toAddrType", skip_serializing_if = "Option::is_none")]
    to_addr_type: Option<String>,
}

impl WithdrawalHistoryRequest {
    /// Create an empty withdrawal-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by withdrawal ID.
    pub fn withdrawal_id(mut self, wd_id: impl Into<String>) -> Self {
        self.wd_id = Some(wd_id.into());
        self
    }

    /// Filter by client ID.
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Filter by transaction ID.
    pub fn tx_id(mut self, tx_id: impl Into<String>) -> Self {
        self.tx_id = Some(tx_id.into());
        self
    }

    /// Filter by withdrawal type.
    pub fn withdrawal_type(mut self, withdrawal_type: impl Into<String>) -> Self {
        self.withdrawal_type = Some(withdrawal_type.into());
        self
    }

    /// Filter by state.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Page after the given ID.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page before the given ID.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set result limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by destination address type.
    pub fn to_addr_type(mut self, to_addr_type: impl Into<String>) -> Self {
        self.to_addr_type = Some(to_addr_type.into());
        self
    }
}

/// Query parameters for [`Funding::get_bills`](crate::api::funding::Funding::get_bills).
#[derive(Debug, Clone, Default, Serialize)]
pub struct FundingBillsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FundingBillsRequest {
    /// Create an empty funding-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by bill type.
    pub fn bill_type(mut self, bill_type: impl Into<String>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }

    /// Page after the given bill ID.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page before the given bill ID.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set result limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for [`Funding::get_deposit_lightning`](crate::api::funding::Funding::get_deposit_lightning).
#[derive(Debug, Clone, Serialize)]
pub struct DepositLightningRequest {
    ccy: String,
    amt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
}

impl DepositLightningRequest {
    /// Create a Lightning deposit request.
    pub fn new(ccy: impl Into<String>, amt: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            to: None,
        }
    }

    /// Set recipient.
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }
}

/// Request body for [`Funding::withdrawal_lightning`](crate::api::funding::Funding::withdrawal_lightning).
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawalLightningRequest {
    ccy: String,
    invoice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<String>,
}

impl WithdrawalLightningRequest {
    /// Create a Lightning withdrawal request.
    pub fn new(ccy: impl Into<String>, invoice: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            invoice: invoice.into(),
            memo: None,
        }
    }

    /// Set withdrawal memo.
    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }
}

/// Query parameters for [`Funding::get_deposit_withdraw_status`](crate::api::funding::Funding::get_deposit_withdraw_status).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositWithdrawStatusRequest {
    #[serde(rename = "wdId", skip_serializing_if = "Option::is_none")]
    wd_id: Option<String>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chain: Option<String>,
}

impl DepositWithdrawStatusRequest {
    /// Create an empty deposit/withdrawal-status query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by withdrawal ID.
    pub fn withdrawal_id(mut self, wd_id: impl Into<String>) -> Self {
        self.wd_id = Some(wd_id.into());
        self
    }

    /// Filter by transaction ID.
    pub fn tx_id(mut self, tx_id: impl Into<String>) -> Self {
        self.tx_id = Some(tx_id.into());
        self
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by destination address.
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Filter by chain.
    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }
}
