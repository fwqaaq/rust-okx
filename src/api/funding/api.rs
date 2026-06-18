use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::internal::*;
use super::requests::*;
use super::responses::*;

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
