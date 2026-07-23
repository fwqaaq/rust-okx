use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
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
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) without credentials, [`RestError::Okx`](crate::RestError::Okx) on a
    /// non-zero OKX code, or transport/decode errors.
    pub async fn get_currencies(
        &self,
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<Currency>, Error> {
        self.client.get(CURRENCIES, request, true).await
    }

    /// Retrieve funding-account balances.
    ///
    /// `GET /api/v5/asset/balances`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_balances(
        &self,
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<FundingBalance>, Error> {
        self.client.get(BALANCES, request, true).await
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
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<NonTradableAsset>, Error> {
        self.client.get(NON_TRADABLE_ASSETS, request, true).await
    }

    /// Retrieve deposit addresses for a currency.
    ///
    /// `GET /api/v5/asset/deposit-address`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_deposit_address(
        &self,
        request: &DepositAddressRequest<'_>,
    ) -> Result<Vec<DepositAddress>, Error> {
        self.client.get(DEPOSIT_ADDRESS, request, true).await
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
        request: &FundsTransferRequest<'_>,
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
        request: &TransferStateRequest<'_>,
    ) -> Result<Vec<TransferState>, Error> {
        self.client.get(TRANSFER_STATE, request, true).await
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
        request: &WithdrawalRequest<'_>,
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
        request: &DepositHistoryRequest<'_>,
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
        request: &FundingBillsRequest<'_>,
    ) -> Result<Vec<FundingBill>, Error> {
        self.client.get(BILLS, request, true).await
    }

    /// Retrieve all-time asset bills history.
    ///
    /// `GET /api/v5/asset/bills-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_bills_history(
        &self,
        request: &FundingBillsHistoryRequest<'_>,
    ) -> Result<Vec<FundingBill>, Error> {
        self.client.get(BILLS_HISTORY, request, true).await
    }

    /// Retrieve OKX's public exchange list.
    ///
    /// `GET /api/v5/asset/exchange-list`. Public.
    ///
    /// # Errors
    ///
    /// Returns an error if transport or response decoding fails, or OKX returns an error code.
    pub async fn get_exchange_list(&self) -> Result<Vec<Exchange>, Error> {
        self.client.get(EXCHANGE_LIST, &(), false).await
    }

    /// Apply for a monthly statement from the past year.
    ///
    /// `POST /api/v5/asset/monthly-statement`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn apply_monthly_statement(
        &self,
        request: &ApplyMonthlyStatementRequest,
    ) -> Result<Vec<MonthlyStatementApplication>, Error> {
        self.client.post(MONTHLY_STATEMENT, request, true).await
    }

    /// Retrieve a generated monthly statement from the past year.
    ///
    /// `GET /api/v5/asset/monthly-statement`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_currencies`](Self::get_currencies).
    pub async fn get_monthly_statement(
        &self,
        request: &MonthlyStatementRequest,
    ) -> Result<Vec<MonthlyStatement>, Error> {
        self.client.get(MONTHLY_STATEMENT, request, true).await
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
        request: &DepositLightningRequest<'_>,
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
        request: &WithdrawalLightningRequest<'_>,
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
    pub async fn cancel_withdrawal(
        &self,
        request: &CancelWithdrawalRequest<'_>,
    ) -> Result<Vec<WithdrawalResult>, Error> {
        self.client.post(CANCEL_WITHDRAWAL, request, true).await
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
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<AssetValuation>, Error> {
        self.client.get(ASSET_VALUATION, request, true).await
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
        request: &DepositWithdrawStatusRequest<'_>,
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
        request: &WithdrawalHistoryRequest<'_>,
    ) -> Result<Vec<WithdrawalRecord>, Error> {
        self.client.get(WITHDRAWAL_HISTORY, request, true).await
    }
}
