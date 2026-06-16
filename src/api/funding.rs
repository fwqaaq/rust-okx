//! Authenticated funding-account and asset endpoints (`/api/v5/asset/*`).

use serde::{Deserialize, Serialize};

use crate::client::OkxClient;
use crate::error::Error;
use crate::model::NumberString;
use crate::transport::Transport;

const NON_TRADABLE_ASSETS: &str = "/api/v5/asset/non-tradable-assets";
const DEPOSIT_ADDRESS: &str = "/api/v5/asset/deposit-address";
const BALANCES: &str = "/api/v5/asset/balances";
const TRANSFER: &str = "/api/v5/asset/transfer";
const TRANSFER_STATE: &str = "/api/v5/asset/transfer-state";
const WITHDRAWAL: &str = "/api/v5/asset/withdrawal";
const DEPOSIT_HISTORY: &str = "/api/v5/asset/deposit-history";
const CURRENCIES: &str = "/api/v5/asset/currencies";
const PURCHASE_REDEMPT: &str = "/api/v5/asset/purchase_redempt";
const BILLS: &str = "/api/v5/asset/bills";
const DEPOSIT_LIGHTNING: &str = "/api/v5/asset/deposit-lightning";
const WITHDRAWAL_LIGHTNING: &str = "/api/v5/asset/withdrawal-lightning";
const CANCEL_WITHDRAWAL: &str = "/api/v5/asset/cancel-withdrawal";
const CONVERT_DUST_ASSETS: &str = "/api/v5/asset/convert-dust-assets";
const ASSET_VALUATION: &str = "/api/v5/asset/asset-valuation";
const WITHDRAWAL_HISTORY: &str = "/api/v5/asset/withdrawal-history";
const DEPOSIT_WITHDRAW_STATUS: &str = "/api/v5/asset/deposit-withdraw-status";

/// Accessor for authenticated funding-account and asset endpoints.
///
/// Obtain one via [`OkxClient::funding`](crate::OkxClient::funding). All methods
/// require credentials. Mutating endpoints operate on the funding account, not
/// the trading account.
pub struct Funding<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Funding<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve currency metadata and chain settings.
    ///
    /// `GET /api/v5/asset/currencies`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a
    /// non-zero OKX code, or transport/decode errors.
    pub async fn get_currencies(&self, ccy: Option<&str>) -> Result<Vec<Currency>, Error> {
        let query = CcyQuery { ccy };
        self.client.get(CURRENCIES, &query, true).await
    }

    /// Retrieve funding-account balances.
    ///
    /// `GET /api/v5/asset/balances`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_balances(&self, ccy: Option<&str>) -> Result<Vec<FundingBalance>, Error> {
        let query = CcyQuery { ccy };
        self.client.get(BALANCES, &query, true).await
    }

    /// Retrieve non-tradable assets.
    ///
    /// `GET /api/v5/asset/non-tradable-assets`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_non_tradable_assets(
        &self,
        ccy: Option<&str>,
    ) -> Result<Vec<NonTradableAsset>, Error> {
        let query = CcyQuery { ccy };
        self.client.get(NON_TRADABLE_ASSETS, &query, true).await
    }

    /// Retrieve deposit addresses for a currency.
    ///
    /// `GET /api/v5/asset/deposit-address`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_deposit_address(&self, ccy: &str) -> Result<Vec<DepositAddress>, Error> {
        let query = RequiredCcyQuery { ccy };
        self.client.get(DEPOSIT_ADDRESS, &query, true).await
    }

    /// Transfer funds between OKX account types.
    ///
    /// `POST /api/v5/asset/transfer`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn funds_transfer(
        &self,
        request: &FundsTransferRequest,
    ) -> Result<Vec<TransferResult>, Error> {
        self.client.post(TRANSFER, request, true).await
    }

    /// Retrieve the state of a funds transfer.
    ///
    /// `GET /api/v5/asset/transfer-state`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn transfer_state(
        &self,
        trans_id: &str,
        transfer_type: Option<&str>,
    ) -> Result<Vec<TransferState>, Error> {
        let query = TransferStateQuery {
            trans_id,
            transfer_type,
        };
        self.client.get(TRANSFER_STATE, &query, true).await
    }

    /// Withdraw funds from OKX.
    ///
    /// `POST /api/v5/asset/withdrawal`. Authenticated. This is a real asset
    /// movement endpoint; build requests deliberately with
    /// [`WithdrawalRequest::new`].
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn withdrawal(
        &self,
        request: &WithdrawalRequest,
    ) -> Result<Vec<WithdrawalResult>, Error> {
        self.client.post(WITHDRAWAL, request, true).await
    }

    /// Retrieve deposit history.
    ///
    /// `GET /api/v5/asset/deposit-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_deposit_history(
        &self,
        request: &DepositHistoryRequest,
    ) -> Result<Vec<DepositRecord>, Error> {
        self.client.get(DEPOSIT_HISTORY, request, true).await
    }

    /// Subscribe or redeem savings through the legacy purchase/redemption
    /// endpoint.
    ///
    /// `POST /api/v5/asset/purchase_redempt`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn purchase_redempt(
        &self,
        ccy: &str,
        amt: &str,
        side: &str,
        rate: &str,
    ) -> Result<Vec<PurchaseRedemptResult>, Error> {
        let body = PurchaseRedemptBody {
            ccy,
            amt,
            side,
            rate,
        };
        self.client.post(PURCHASE_REDEMPT, &body, true).await
    }

    /// Retrieve funding-account bills.
    ///
    /// `GET /api/v5/asset/bills`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_bills(
        &self,
        request: &FundingBillsRequest,
    ) -> Result<Vec<FundingBill>, Error> {
        self.client.get(BILLS, request, true).await
    }

    /// Create or retrieve a Lightning Network deposit invoice.
    ///
    /// `GET /api/v5/asset/deposit-lightning`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_deposit_lightning(
        &self,
        request: &DepositLightningRequest,
    ) -> Result<Vec<DepositLightning>, Error> {
        self.client.get(DEPOSIT_LIGHTNING, request, true).await
    }

    /// Withdraw through the Lightning Network.
    ///
    /// `POST /api/v5/asset/withdrawal-lightning`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn withdrawal_lightning(
        &self,
        request: &WithdrawalLightningRequest,
    ) -> Result<Vec<WithdrawalLightning>, Error> {
        self.client.post(WITHDRAWAL_LIGHTNING, request, true).await
    }

    /// Cancel a withdrawal.
    ///
    /// `POST /api/v5/asset/cancel-withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn cancel_withdrawal(&self, wd_id: &str) -> Result<Vec<WithdrawalResult>, Error> {
        let body = WithdrawalIdBody { wd_id };
        self.client.post(CANCEL_WITHDRAWAL, &body, true).await
    }

    /// Convert small balances into another asset.
    ///
    /// `POST /api/v5/asset/convert-dust-assets`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn convert_dust_assets(
        &self,
        ccy: &[&str],
    ) -> Result<Vec<ConvertDustAssetsResult>, Error> {
        let body = ConvertDustAssetsBody { ccy };
        self.client.post(CONVERT_DUST_ASSETS, &body, true).await
    }

    /// Retrieve total asset valuation.
    ///
    /// `GET /api/v5/asset/asset-valuation`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_asset_valuation(
        &self,
        ccy: Option<&str>,
    ) -> Result<Vec<AssetValuation>, Error> {
        let query = CcyQuery { ccy };
        self.client.get(ASSET_VALUATION, &query, true).await
    }

    /// Retrieve deposit/withdrawal status.
    ///
    /// `GET /api/v5/asset/deposit-withdraw-status`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_deposit_withdraw_status(
        &self,
        request: &DepositWithdrawStatusRequest,
    ) -> Result<Vec<DepositWithdrawStatus>, Error> {
        self.client
            .get(DEPOSIT_WITHDRAW_STATUS, request, true)
            .await
    }

    /// Retrieve withdrawal history.
    ///
    /// `GET /api/v5/asset/withdrawal-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_withdrawal_history(
        &self,
        request: &WithdrawalHistoryRequest,
    ) -> Result<Vec<WithdrawalRecord>, Error> {
        self.client.get(WITHDRAWAL_HISTORY, request, true).await
    }
}

#[derive(Debug, Serialize)]
struct CcyQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct RequiredCcyQuery<'a> {
    ccy: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TransferStateQuery<'a> {
    #[serde(rename = "transId")]
    trans_id: &'a str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    transfer_type: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct PurchaseRedemptBody<'a> {
    ccy: &'a str,
    amt: &'a str,
    side: &'a str,
    rate: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct WithdrawalIdBody<'a> {
    #[serde(rename = "wdId")]
    wd_id: &'a str,
}

#[derive(Debug, Serialize)]
struct ConvertDustAssetsBody<'a> {
    ccy: &'a [&'a str],
}

/// Request body for [`Funding::funds_transfer`].
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

/// Request body for [`Funding::withdrawal`].
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

/// Query parameters for [`Funding::get_deposit_history`].
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

/// Query parameters for [`Funding::get_withdrawal_history`].
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

/// Query parameters for [`Funding::get_bills`].
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

/// Query parameters for [`Funding::get_deposit_lightning`].
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

/// Request body for [`Funding::withdrawal_lightning`].
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

/// Query parameters for [`Funding::get_deposit_withdraw_status`].
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
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Transfer amount.
    #[serde(default)]
    pub amt: NumberString,
}

/// Funds-transfer state.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TransferState {
    /// Transfer ID.
    #[serde(default, rename = "transId")]
    pub trans_id: String,
    /// Transfer state.
    #[serde(default)]
    pub state: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Transfer amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Source account.
    #[serde(default, rename = "from")]
    pub from_account: String,
    /// Destination account.
    #[serde(default)]
    pub to: String,
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
    /// Balance change.
    #[serde(default)]
    pub bal_chg: NumberString,
    /// Balance after change.
    #[serde(default)]
    pub bal: NumberString,
    /// Bill type.
    #[serde(default, rename = "type")]
    pub bill_type: String,
    /// Timestamp.
    #[serde(default)]
    pub ts: NumberString,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funds_transfer_request_omits_unset_optional_fields() {
        let value =
            serde_json::to_value(FundsTransferRequest::new("USDT", "1", "6", "18")).unwrap();

        assert_eq!(value["ccy"], "USDT");
        assert_eq!(value["amt"], "1");
        assert_eq!(value["from"], "6");
        assert_eq!(value["to"], "18");
        assert!(value.get("subAcct").is_none());
        assert!(value.get("instId").is_none());
        assert!(value.get("loanTrans").is_none());
    }

    #[test]
    fn withdrawal_request_omits_unset_optional_fields() {
        let value =
            serde_json::to_value(WithdrawalRequest::new("USDT", "1", "3", "example")).unwrap();

        assert_eq!(value["ccy"], "USDT");
        assert_eq!(value["amt"], "1");
        assert_eq!(value["dest"], "3");
        assert_eq!(value["toAddr"], "example");
        assert!(value.get("chain").is_none());
        assert!(value.get("areaCode").is_none());
        assert!(value.get("toAddrType").is_none());
    }

    #[test]
    fn history_requests_omit_unset_optional_fields() {
        let deposit = serde_urlencoded::to_string(DepositHistoryRequest::new().limit(5)).unwrap();
        assert_eq!(deposit, "limit=5");

        let withdrawal =
            serde_urlencoded::to_string(WithdrawalHistoryRequest::new().to_addr_type("1")).unwrap();
        assert_eq!(withdrawal, "toAddrType=1");
    }

    #[test]
    fn deposit_withdraw_status_request_omits_unset_optional_fields() {
        let query = serde_urlencoded::to_string(
            DepositWithdrawStatusRequest::new()
                .currency("USDT")
                .chain("USDT-TRC20"),
        )
        .unwrap();

        assert_eq!(query, "ccy=USDT&chain=USDT-TRC20");
    }
}
