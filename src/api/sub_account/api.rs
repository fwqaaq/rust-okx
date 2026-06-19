use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the sub-account management endpoints.
///
/// Obtain one via [`OkxClient::sub_account`](crate::OkxClient::sub_account).
/// All methods require credentials.
pub struct SubAccount<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> SubAccount<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// List sub-accounts of the master account.
    ///
    /// `GET /api/v5/users/subaccount/list`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_list(
        &self,
        request: &SubAccountListRequest,
    ) -> Result<Vec<SubAccountEntry>, Error> {
        self.client.get(SUBACCOUNT_LIST, request, true).await
    }

    /// Create a new sub-account.
    ///
    /// `POST /api/v5/users/subaccount/create-subaccount`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn create_subaccount(
        &self,
        request: &CreateSubAccountRequest,
    ) -> Result<Vec<SubAccountEntry>, Error> {
        self.client.post(SUBACCOUNT_CREATE, request, true).await
    }

    /// Create an API key for a sub-account.
    ///
    /// `POST /api/v5/users/subaccount/apikey`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn create_subaccount_apikey(
        &self,
        request: &CreateSubAccountApiKeyRequest,
    ) -> Result<Vec<SubAccountApiKey>, Error> {
        self.client.post(SUBACCOUNT_APIKEY, request, true).await
    }

    /// List API keys of a sub-account.
    ///
    /// `GET /api/v5/users/subaccount/apikey`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_apikeys(
        &self,
        request: &SubAccountApiKeysRequest,
    ) -> Result<Vec<SubAccountApiKey>, Error> {
        self.client.get(SUBACCOUNT_APIKEY, request, true).await
    }

    /// Modify an API key of a sub-account.
    ///
    /// `POST /api/v5/users/subaccount/modify-apikey`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn modify_subaccount_apikey(
        &self,
        request: &ModifySubAccountApiKeyRequest,
    ) -> Result<Vec<SubAccountApiKey>, Error> {
        self.client
            .post(SUBACCOUNT_APIKEY_MODIFY, request, true)
            .await
    }

    /// Delete an API key of a sub-account.
    ///
    /// `POST /api/v5/users/subaccount/delete-apikey`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn delete_subaccount_apikey(
        &self,
        request: &DeleteSubAccountApiKeyRequest,
    ) -> Result<Vec<SubAccountApiKey>, Error> {
        self.client
            .post(SUBACCOUNT_APIKEY_DELETE, request, true)
            .await
    }

    /// Retrieve trading-account balances of a sub-account.
    ///
    /// `GET /api/v5/account/subaccount/balances`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_trading_balances(
        &self,
        request: &SubAccountTradingBalancesRequest,
    ) -> Result<Vec<SubAccountTradingBalance>, Error> {
        self.client
            .get(SUBACCOUNT_TRADING_BALANCES, request, true)
            .await
    }

    /// Retrieve funding-account balances of a sub-account.
    ///
    /// `GET /api/v5/asset/subaccount/balances`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_funding_balances(
        &self,
        request: &SubAccountFundingBalancesRequest,
    ) -> Result<Vec<SubAccountFundingBalance>, Error> {
        self.client
            .get(SUBACCOUNT_FUNDING_BALANCES, request, true)
            .await
    }

    /// Retrieve the maximum withdrawal amount for a sub-account.
    ///
    /// `GET /api/v5/account/subaccount/max-withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_max_withdrawal(
        &self,
        request: &SubAccountMaxWithdrawalRequest,
    ) -> Result<Vec<SubAccountMaxWithdrawal>, Error> {
        self.client
            .get(SUBACCOUNT_MAX_WITHDRAWAL, request, true)
            .await
    }

    /// Retrieve asset bills for sub-accounts of the master account.
    ///
    /// `GET /api/v5/asset/subaccount/bills`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_bills(
        &self,
        request: &SubAccountBillsRequest,
    ) -> Result<Vec<SubAccountBill>, Error> {
        self.client.get(SUBACCOUNT_BILLS, request, true).await
    }

    /// Retrieve asset bills for managed sub-accounts.
    ///
    /// `GET /api/v5/asset/subaccount/managed-subaccount-bills`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_subaccount_managed_bills(
        &self,
        request: &ManagedSubAccountBillsRequest,
    ) -> Result<Vec<ManagedSubAccountBill>, Error> {
        self.client
            .get(SUBACCOUNT_MANAGED_BILLS, request, true)
            .await
    }

    /// Transfer assets between funding or trading accounts of sub-accounts.
    ///
    /// `POST /api/v5/asset/subaccount/transfer`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn transfer_between_subaccounts(
        &self,
        request: &SubAccountTransferRequest,
    ) -> Result<Vec<SubAccountTransferResult>, Error> {
        self.client.post(SUBACCOUNT_TRANSFER, request, true).await
    }

    /// Enable or disable transfers out for one or more sub-accounts.
    ///
    /// `POST /api/v5/users/subaccount/set-transfer-out`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn set_subaccount_transfer_out(
        &self,
        request: &SetTransferOutRequest,
    ) -> Result<Vec<SetTransferOutResult>, Error> {
        self.client
            .post(SUBACCOUNT_SET_TRANSFER_OUT, request, true)
            .await
    }

    /// List sub-accounts of an entrusted entity.
    ///
    /// `GET /api/v5/users/entrust-subaccount-list`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_entrust_subaccount_list(&self) -> Result<Vec<EntrustSubAccount>, Error> {
        self.client.get(ENTRUST_SUBACCOUNT_LIST, &(), true).await
    }
}
