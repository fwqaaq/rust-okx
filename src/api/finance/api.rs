use crate::client::OkxClient;
use crate::error::Error;
use crate::model::EmptyRequest;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;
/// Accessor for OKX finance endpoint groups.
///
/// Obtain one via [`OkxClient::finance`](crate::OkxClient::finance).
pub struct Finance<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Finance<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Access Savings endpoints.
    pub fn savings(&self) -> Savings<'_, T> {
        Savings {
            client: self.client,
        }
    }

    /// Access Staking/DeFi endpoints.
    pub fn staking_defi(&self) -> StakingDefi<'_, T> {
        StakingDefi {
            client: self.client,
        }
    }

    /// Access ETH staking endpoints.
    pub fn eth_staking(&self) -> EthStaking<'_, T> {
        EthStaking {
            client: self.client,
        }
    }

    /// Access SOL staking endpoints.
    pub fn sol_staking(&self) -> SolStaking<'_, T> {
        SolStaking {
            client: self.client,
        }
    }

    /// Access Flexible Loan endpoints.
    pub fn flexible_loan(&self) -> FlexibleLoan<'_, T> {
        FlexibleLoan {
            client: self.client,
        }
    }
}

/// Accessor for Savings endpoints.
pub struct Savings<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> Savings<'_, T> {
    /// Retrieve savings balances.
    ///
    /// `GET /api/v5/finance/savings/balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero
    /// OKX code, or transport/decode errors.
    pub async fn get_saving_balance(
        &self,
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<SavingBalance>, Error> {
        self.client.get(SAVINGS_BALANCE, request, true).await
    }

    /// Purchase or redeem savings.
    ///
    /// `POST /api/v5/finance/savings/purchase-redempt`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero
    /// OKX code, or transport/decode errors.
    pub async fn purchase_redemption(
        &self,
        request: &SavingsPurchaseRedemptionRequest,
    ) -> Result<Vec<SavingsPurchaseRedemptionResult>, Error> {
        self.client
            .post(SAVINGS_PURCHASE_REDEMPT, request, true)
            .await
    }

    /// Set the savings lending rate.
    ///
    /// `POST /api/v5/finance/savings/set-lending-rate`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero
    /// OKX code, or transport/decode errors.
    pub async fn set_lending_rate(
        &self,
        request: &SetLendingRateRequest<'_>,
    ) -> Result<Vec<SetLendingRateResult>, Error> {
        self.client
            .post(SAVINGS_SET_LENDING_RATE, request, true)
            .await
    }

    /// Retrieve lending history.
    ///
    /// `GET /api/v5/finance/savings/lending-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero
    /// OKX code, or transport/decode errors.
    pub async fn get_lending_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<LendingHistory>, Error> {
        self.client
            .get(SAVINGS_LENDING_HISTORY, request, true)
            .await
    }

    /// Retrieve public borrow history.
    ///
    /// `GET /api/v5/finance/savings/lending-rate-history`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_public_borrow_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<PublicBorrowHistory>, Error> {
        self.client
            .get(SAVINGS_PUBLIC_BORROW_HISTORY, request, false)
            .await
    }

    /// Retrieve public borrow info.
    ///
    /// `GET /api/v5/finance/savings/lending-rate-summary`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_public_borrow_info(
        &self,
        request: &CurrencyRequest<'_>,
    ) -> Result<Vec<PublicBorrowInfo>, Error> {
        self.client
            .get(SAVINGS_PUBLIC_BORROW_INFO, request, false)
            .await
    }
}

/// Accessor for Staking/DeFi endpoints.
pub struct StakingDefi<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> StakingDefi<'_, T> {
    /// Retrieve Staking/DeFi offers.
    ///
    /// `GET /api/v5/finance/staking-defi/offers`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_offers(
        &self,
        request: &StakingDefiOffersRequest,
    ) -> Result<Vec<StakingDefiOffer>, Error> {
        self.client.get(STAKING_DEFI_OFFERS, request, true).await
    }

    /// Purchase a Staking/DeFi product.
    ///
    /// `POST /api/v5/finance/staking-defi/purchase`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn purchase(
        &self,
        request: &StakingDefiPurchaseRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        self.client.post(STAKING_DEFI_PURCHASE, request, true).await
    }

    /// Redeem a Staking/DeFi order.
    ///
    /// `POST /api/v5/finance/staking-defi/redeem`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn redeem(
        &self,
        request: &StakingDefiRedeemRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        self.client.post(STAKING_DEFI_REDEEM, request, true).await
    }

    /// Cancel a Staking/DeFi order.
    ///
    /// `POST /api/v5/finance/staking-defi/cancel`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn cancel(
        &self,
        request: &StakingDefiCancelRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        self.client.post(STAKING_DEFI_CANCEL, request, true).await
    }

    /// Retrieve active Staking/DeFi orders.
    ///
    /// `GET /api/v5/finance/staking-defi/orders-active`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_active_orders(
        &self,
        request: &StakingDefiActiveOrdersRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        self.client
            .get(STAKING_DEFI_ACTIVE_ORDERS, request, true)
            .await
    }

    /// Retrieve Staking/DeFi order history.
    ///
    /// `GET /api/v5/finance/staking-defi/orders-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_orders_history(
        &self,
        request: &StakingDefiOrderHistoryRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        self.client
            .get(STAKING_DEFI_ORDERS_HISTORY, request, true)
            .await
    }
}

/// Accessor for ETH staking endpoints.
pub struct EthStaking<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> EthStaking<'_, T> {
    /// Retrieve ETH staking product info.
    ///
    /// `GET /api/v5/finance/staking-defi/eth/product-info`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn product_info(&self) -> Result<Vec<StakingProductInfo>, Error> {
        self.client
            .get(ETH_PRODUCT_INFO, &EmptyRequest {}, true)
            .await
    }

    /// Purchase ETH staking.
    ///
    /// `POST /api/v5/finance/staking-defi/eth/purchase`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn purchase(&self, request: &AmountRequest<'_>) -> Result<Vec<StakingOrder>, Error> {
        self.client.post(ETH_PURCHASE, request, true).await
    }

    /// Redeem ETH staking.
    ///
    /// `POST /api/v5/finance/staking-defi/eth/redeem`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn redeem(&self, request: &AmountRequest<'_>) -> Result<Vec<StakingOrder>, Error> {
        self.client.post(ETH_REDEEM, request, true).await
    }

    /// Cancel redeem ETH staking.
    ///
    /// `POST /api/v5/finance/staking-defi/eth/cancel-redeem`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn cancel_redeem(
        &self,
        request: &CancelRedeemRequest<'_>,
    ) -> Result<Vec<CancelRedeem>, Error> {
        self.client.post(ETH_CANCEL_REDEEM, request, true).await
    }

    /// Retrieve ETH staking balance.
    ///
    /// `GET /api/v5/finance/staking-defi/eth/balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn balance(&self) -> Result<Vec<StakingBalance>, Error> {
        self.client.get(ETH_BALANCE, &EmptyRequest {}, true).await
    }

    /// Retrieve ETH staking purchase/redeem history.
    ///
    /// `GET /api/v5/finance/staking-defi/eth/purchase-redeem-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn purchase_redeem_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<StakingHistory>, Error> {
        self.client.get(ETH_HISTORY, request, true).await
    }

    /// Retrieve ETH staking APY history.
    ///
    /// `GET /api/v5/finance/staking-defi/eth/apy-history`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn apy_history(
        &self,
        request: &ApyHistoryRequest<'_>,
    ) -> Result<Vec<StakingApyHistory>, Error> {
        self.client.get(ETH_APY_HISTORY, request, false).await
    }
}

/// Accessor for SOL staking endpoints.
pub struct SolStaking<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> SolStaking<'_, T> {
    /// Retrieve SOL staking product info.
    ///
    /// `GET /api/v5/finance/staking-defi/sol/product-info`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn product_info(&self) -> Result<StakingProductInfo, Error> {
        self.client
            .get(SOL_PRODUCT_INFO, &EmptyRequest {}, true)
            .await
    }

    /// Purchase SOL staking.
    ///
    /// `POST /api/v5/finance/staking-defi/sol/purchase`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn purchase(&self, request: &AmountRequest<'_>) -> Result<Vec<StakingOrder>, Error> {
        self.client.post(SOL_PURCHASE, request, true).await
    }

    /// Redeem SOL staking.
    ///
    /// `POST /api/v5/finance/staking-defi/sol/redeem`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn redeem(&self, request: &AmountRequest<'_>) -> Result<Vec<StakingOrder>, Error> {
        self.client.post(SOL_REDEEM, request, true).await
    }

    /// Retrieve SOL staking balance.
    ///
    /// `GET /api/v5/finance/staking-defi/sol/balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn balance(&self) -> Result<Vec<StakingBalance>, Error> {
        self.client.get(SOL_BALANCE, &EmptyRequest {}, true).await
    }

    /// Retrieve SOL staking purchase/redeem history.
    ///
    /// `GET /api/v5/finance/staking-defi/sol/purchase-redeem-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn purchase_redeem_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<StakingHistory>, Error> {
        self.client.get(SOL_HISTORY, request, true).await
    }

    /// Retrieve SOL staking APY history.
    ///
    /// `GET /api/v5/finance/staking-defi/sol/apy-history`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn apy_history(
        &self,
        request: &ApyHistoryRequest<'_>,
    ) -> Result<Vec<StakingApyHistory>, Error> {
        self.client.get(SOL_APY_HISTORY, request, false).await
    }
}

/// Accessor for Flexible Loan endpoints.
pub struct FlexibleLoan<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> FlexibleLoan<'_, T> {
    /// Retrieve borrowable currencies.
    ///
    /// `GET /api/v5/finance/flexible-loan/borrow-currencies`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn borrow_currencies(&self) -> Result<Vec<FlexibleLoanCurrency>, Error> {
        self.client
            .get(FLEX_BORROW_CURRENCIES, &EmptyRequest {}, true)
            .await
    }

    /// Retrieve collateral assets.
    ///
    /// `GET /api/v5/finance/flexible-loan/collateral-assets`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn collateral_assets(
        &self,
        request: &FlexibleLoanCollateralAssetsRequest,
    ) -> Result<Vec<FlexibleLoanCollateralAsset>, Error> {
        self.client
            .get(FLEX_COLLATERAL_ASSETS, request, false)
            .await
    }

    /// Estimate maximum flexible-loan amount.
    ///
    /// `POST /api/v5/finance/flexible-loan/max-loan`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn max_loan(
        &self,
        request: &FlexibleLoanMaxLoanRequest,
    ) -> Result<Vec<FlexibleLoanMaxLoan>, Error> {
        self.client.post(FLEX_MAX_LOAN, request, true).await
    }

    /// Retrieve maximum collateral redeem amount.
    ///
    /// `GET /api/v5/finance/flexible-loan/max-collateral-redeem-amount`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn max_collateral_redeem_amount(
        &self,
        request: &FlexibleLoanMaxRedeemRequest,
    ) -> Result<Vec<FlexibleLoanMaxRedeem>, Error> {
        self.client.get(FLEX_MAX_REDEEM, request, true).await
    }

    /// Adjust flexible-loan collateral.
    ///
    /// `POST /api/v5/finance/flexible-loan/adjust-collateral`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn adjust_collateral(
        &self,
        request: &FlexibleLoanAdjustCollateralRequest,
    ) -> Result<Vec<FlexibleLoanOrder>, Error> {
        self.client
            .post(FLEX_ADJUST_COLLATERAL, request, true)
            .await
    }

    /// Retrieve flexible-loan info.
    ///
    /// `GET /api/v5/finance/flexible-loan/loan-info`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn loan_info(
        &self,
        request: &FlexibleLoanInfoRequest,
    ) -> Result<Vec<FlexibleLoanInfo>, Error> {
        self.client.get(FLEX_LOAN_INFO, request, true).await
    }

    /// Retrieve flexible-loan history.
    ///
    /// `GET /api/v5/finance/flexible-loan/loan-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn loan_history(
        &self,
        request: &FlexibleLoanHistoryRequest,
    ) -> Result<Vec<FlexibleLoanHistory>, Error> {
        self.client.get(FLEX_LOAN_HISTORY, request, true).await
    }

    /// Retrieve flexible-loan accrued interest.
    ///
    /// `GET /api/v5/finance/flexible-loan/interest-accrued`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] without credentials, [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn interest_accrued(
        &self,
        request: &FlexibleLoanInterestAccruedRequest,
    ) -> Result<Vec<FlexibleLoanInterest>, Error> {
        self.client.get(FLEX_INTEREST_ACCRUED, request, true).await
    }
}
