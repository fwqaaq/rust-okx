use std::borrow::Cow;

use serde::Serialize;

/// Request for [`get_currencies`](crate::api::funding::Funding::get_currencies),
/// [`get_balances`](crate::api::funding::Funding::get_balances),
/// [`get_non_tradable_assets`](crate::api::funding::Funding::get_non_tradable_assets), and
/// [`get_asset_valuation`](crate::api::funding::Funding::get_asset_valuation).
#[derive(Debug, Clone, Default, Serialize)]
pub struct CurrencyRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> CurrencyRequest<'a> {
    /// Create an unfiltered currency query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Request for [`get_deposit_address`](crate::api::funding::Funding::get_deposit_address).
#[derive(Debug, Clone, Serialize)]
pub struct DepositAddressRequest<'a> {
    ccy: Cow<'a, str>,
}

impl<'a> DepositAddressRequest<'a> {
    /// Create a deposit-address query for a currency.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self { ccy: ccy.into() }
    }
}

/// Request for [`transfer_state`](crate::api::funding::Funding::transfer_state).
#[derive(Debug, Clone, Serialize)]
pub struct TransferStateRequest<'a> {
    #[serde(rename = "transId")]
    trans_id: Cow<'a, str>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    transfer_type: Option<Cow<'a, str>>,
}

impl<'a> TransferStateRequest<'a> {
    /// Create a transfer-state query for a transfer ID.
    pub fn new(trans_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            trans_id: trans_id.into(),
            transfer_type: None,
        }
    }

    /// Set the transfer type filter.
    pub fn transfer_type(mut self, transfer_type: impl Into<Cow<'a, str>>) -> Self {
        self.transfer_type = Some(transfer_type.into());
        self
    }
}

/// Request for [`cancel_withdrawal`](crate::api::funding::Funding::cancel_withdrawal).
#[derive(Debug, Clone, Serialize)]
pub struct CancelWithdrawalRequest<'a> {
    #[serde(rename = "wdId")]
    wd_id: Cow<'a, str>,
}

impl<'a> CancelWithdrawalRequest<'a> {
    /// Create a cancel-withdrawal request for a withdrawal ID.
    pub fn new(wd_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            wd_id: wd_id.into(),
        }
    }
}

/// Request body for [`Funding::funds_transfer`](crate::api::funding::Funding::funds_transfer).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundsTransferRequest<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
    #[serde(rename = "from")]
    from_account: Cow<'a, str>,
    to: Cow<'a, str>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    transfer_type: Option<Cow<'a, str>>,
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    sub_account: Option<Cow<'a, str>>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "toInstId", skip_serializing_if = "Option::is_none")]
    to_inst_id: Option<Cow<'a, str>>,
    #[serde(rename = "loanTrans", skip_serializing_if = "Option::is_none")]
    loan_transfer: Option<Cow<'a, str>>,
}

impl<'a> FundsTransferRequest<'a> {
    /// Create a funds-transfer request.
    pub fn new(
        ccy: impl Into<Cow<'a, str>>,
        amt: impl Into<Cow<'a, str>>,
        from_account: impl Into<Cow<'a, str>>,
        to: impl Into<Cow<'a, str>>,
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
    pub fn transfer_type(mut self, transfer_type: impl Into<Cow<'a, str>>) -> Self {
        self.transfer_type = Some(transfer_type.into());
        self
    }

    /// Set sub-account name.
    pub fn sub_account(mut self, sub_account: impl Into<Cow<'a, str>>) -> Self {
        self.sub_account = Some(sub_account.into());
        self
    }

    /// Set source instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set destination instrument ID.
    pub fn to_inst_id(mut self, to_inst_id: impl Into<Cow<'a, str>>) -> Self {
        self.to_inst_id = Some(to_inst_id.into());
        self
    }

    /// Set whether this is a loan transfer.
    pub fn loan_transfer(mut self, loan_transfer: impl Into<Cow<'a, str>>) -> Self {
        self.loan_transfer = Some(loan_transfer.into());
        self
    }
}

/// Request body for [`Funding::withdrawal`](crate::api::funding::Funding::withdrawal).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRequest<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
    dest: Cow<'a, str>,
    #[serde(rename = "toAddr")]
    to_addr: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chain: Option<Cow<'a, str>>,
    #[serde(rename = "areaCode", skip_serializing_if = "Option::is_none")]
    area_code: Option<Cow<'a, str>>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<Cow<'a, str>>,
    #[serde(rename = "toAddrType", skip_serializing_if = "Option::is_none")]
    to_addr_type: Option<Cow<'a, str>>,
}

impl<'a> WithdrawalRequest<'a> {
    /// Create a withdrawal request.
    pub fn new(
        ccy: impl Into<Cow<'a, str>>,
        amt: impl Into<Cow<'a, str>>,
        dest: impl Into<Cow<'a, str>>,
        to_addr: impl Into<Cow<'a, str>>,
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
    pub fn chain(mut self, chain: impl Into<Cow<'a, str>>) -> Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set phone area code for internal withdrawals.
    pub fn area_code(mut self, area_code: impl Into<Cow<'a, str>>) -> Self {
        self.area_code = Some(area_code.into());
        self
    }

    /// Set client withdrawal ID.
    pub fn client_id(mut self, client_id: impl Into<Cow<'a, str>>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Set address type.
    pub fn to_addr_type(mut self, to_addr_type: impl Into<Cow<'a, str>>) -> Self {
        self.to_addr_type = Some(to_addr_type.into());
        self
    }
}

/// Query parameters for [`Funding::get_deposit_history`](crate::api::funding::Funding::get_deposit_history).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    deposit_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<Cow<'a, str>>,
    #[serde(rename = "depId", skip_serializing_if = "Option::is_none")]
    dep_id: Option<Cow<'a, str>>,
    #[serde(rename = "fromWdId", skip_serializing_if = "Option::is_none")]
    from_wd_id: Option<Cow<'a, str>>,
}

impl<'a> DepositHistoryRequest<'a> {
    /// Create an empty deposit-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by deposit type.
    pub fn deposit_type(mut self, deposit_type: impl Into<Cow<'a, str>>) -> Self {
        self.deposit_type = Some(deposit_type.into());
        self
    }

    /// Filter by state.
    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Page after the given ID.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page before the given ID.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set result limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by transaction ID.
    pub fn tx_id(mut self, tx_id: impl Into<Cow<'a, str>>) -> Self {
        self.tx_id = Some(tx_id.into());
        self
    }

    /// Filter by deposit ID.
    pub fn dep_id(mut self, dep_id: impl Into<Cow<'a, str>>) -> Self {
        self.dep_id = Some(dep_id.into());
        self
    }

    /// Filter by source withdrawal ID.
    pub fn from_wd_id(mut self, from_wd_id: impl Into<Cow<'a, str>>) -> Self {
        self.from_wd_id = Some(from_wd_id.into());
        self
    }
}

/// Query parameters for [`Funding::get_withdrawal_history`](crate::api::funding::Funding::get_withdrawal_history).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(rename = "wdId", skip_serializing_if = "Option::is_none")]
    wd_id: Option<Cow<'a, str>>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    client_id: Option<Cow<'a, str>>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<Cow<'a, str>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    withdrawal_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "toAddrType", skip_serializing_if = "Option::is_none")]
    to_addr_type: Option<Cow<'a, str>>,
}

impl<'a> WithdrawalHistoryRequest<'a> {
    /// Create an empty withdrawal-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by withdrawal ID.
    pub fn withdrawal_id(mut self, wd_id: impl Into<Cow<'a, str>>) -> Self {
        self.wd_id = Some(wd_id.into());
        self
    }

    /// Filter by client ID.
    pub fn client_id(mut self, client_id: impl Into<Cow<'a, str>>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Filter by transaction ID.
    pub fn tx_id(mut self, tx_id: impl Into<Cow<'a, str>>) -> Self {
        self.tx_id = Some(tx_id.into());
        self
    }

    /// Filter by withdrawal type.
    pub fn withdrawal_type(mut self, withdrawal_type: impl Into<Cow<'a, str>>) -> Self {
        self.withdrawal_type = Some(withdrawal_type.into());
        self
    }

    /// Filter by state.
    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Page after the given ID.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page before the given ID.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set result limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by destination address type.
    pub fn to_addr_type(mut self, to_addr_type: impl Into<Cow<'a, str>>) -> Self {
        self.to_addr_type = Some(to_addr_type.into());
        self
    }
}

/// Query parameters for [`Funding::get_bills`](crate::api::funding::Funding::get_bills).
#[derive(Debug, Clone, Default, Serialize)]
pub struct FundingBillsRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl<'a> FundingBillsRequest<'a> {
    /// Create an empty funding-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by bill type.
    pub fn bill_type(mut self, bill_type: impl Into<Cow<'a, str>>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }

    /// Page after the given bill ID.
    pub fn after(mut self, after: impl Into<Cow<'a, str>>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Page before the given bill ID.
    pub fn before(mut self, before: impl Into<Cow<'a, str>>) -> Self {
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
pub struct DepositLightningRequest<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<Cow<'a, str>>,
}

impl<'a> DepositLightningRequest<'a> {
    /// Create a Lightning deposit request.
    pub fn new(ccy: impl Into<Cow<'a, str>>, amt: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
            to: None,
        }
    }

    /// Set recipient.
    pub fn to(mut self, to: impl Into<Cow<'a, str>>) -> Self {
        self.to = Some(to.into());
        self
    }
}

/// Request body for [`Funding::withdrawal_lightning`](crate::api::funding::Funding::withdrawal_lightning).
#[derive(Debug, Clone, Serialize)]
pub struct WithdrawalLightningRequest<'a> {
    ccy: Cow<'a, str>,
    invoice: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<Cow<'a, str>>,
}

impl<'a> WithdrawalLightningRequest<'a> {
    /// Create a Lightning withdrawal request.
    pub fn new(ccy: impl Into<Cow<'a, str>>, invoice: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            invoice: invoice.into(),
            memo: None,
        }
    }

    /// Set withdrawal memo.
    pub fn memo(mut self, memo: impl Into<Cow<'a, str>>) -> Self {
        self.memo = Some(memo.into());
        self
    }
}

/// Query parameters for [`Funding::get_deposit_withdraw_status`](crate::api::funding::Funding::get_deposit_withdraw_status).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositWithdrawStatusRequest<'a> {
    #[serde(rename = "wdId", skip_serializing_if = "Option::is_none")]
    wd_id: Option<Cow<'a, str>>,
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    tx_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chain: Option<Cow<'a, str>>,
}

impl<'a> DepositWithdrawStatusRequest<'a> {
    /// Create an empty deposit/withdrawal-status query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by withdrawal ID.
    pub fn withdrawal_id(mut self, wd_id: impl Into<Cow<'a, str>>) -> Self {
        self.wd_id = Some(wd_id.into());
        self
    }

    /// Filter by transaction ID.
    pub fn tx_id(mut self, tx_id: impl Into<Cow<'a, str>>) -> Self {
        self.tx_id = Some(tx_id.into());
        self
    }

    /// Filter by currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Filter by destination address.
    pub fn to(mut self, to: impl Into<Cow<'a, str>>) -> Self {
        self.to = Some(to.into());
        self
    }

    /// Filter by chain.
    pub fn chain(mut self, chain: impl Into<Cow<'a, str>>) -> Self {
        self.chain = Some(chain.into());
        self
    }
}
