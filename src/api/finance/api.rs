use crate::client::OkxClient;
use crate::error::Error;
use crate::model::ValidateRequest;
use crate::transport::Transport;

use super::endpoints::*;
use super::internal::{AmountBody, DaysQuery, NoParams, SetLendingRateBody, optional_ccy};
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
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn get_saving_balance(&self, ccy: Option<&str>) -> Result<Vec<SavingBalance>, Error> {
        let query = optional_ccy(ccy);
        self.client.get(SAVINGS_BALANCE, &query, true).await
    }

    /// Purchase or redeem savings.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn purchase_redemption(
        &self,
        request: &SavingsPurchaseRedemptionRequest,
    ) -> Result<Vec<SavingsPurchaseRedemptionResult>, Error> {
        request.validate()?;
        self.client
            .post(SAVINGS_PURCHASE_REDEMPT, request, true)
            .await
    }

    /// Set the savings lending rate.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn set_lending_rate(
        &self,
        ccy: &str,
        rate: &str,
    ) -> Result<Vec<SetLendingRateResult>, Error> {
        let body = SetLendingRateBody { ccy, rate };
        self.client
            .post(SAVINGS_SET_LENDING_RATE, &body, true)
            .await
    }

    /// Retrieve lending history.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn get_lending_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<LendingHistory>, Error> {
        request.validate()?;
        self.client
            .get(SAVINGS_LENDING_HISTORY, request, true)
            .await
    }

    /// Retrieve public borrow history.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn get_public_borrow_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<PublicBorrowHistory>, Error> {
        request.validate()?;
        self.client
            .get(SAVINGS_PUBLIC_BORROW_HISTORY, request, false)
            .await
    }

    /// Retrieve public borrow info.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn get_public_borrow_info(
        &self,
        ccy: Option<&str>,
    ) -> Result<Vec<PublicBorrowInfo>, Error> {
        let query = optional_ccy(ccy);
        self.client
            .get(SAVINGS_PUBLIC_BORROW_INFO, &query, false)
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
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn get_offers(
        &self,
        request: &StakingDefiOffersRequest,
    ) -> Result<Vec<StakingDefiOffer>, Error> {
        request.validate()?;
        self.client.get(STAKING_DEFI_OFFERS, request, true).await
    }

    /// Purchase a Staking/DeFi product.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn purchase(
        &self,
        request: &StakingDefiPurchaseRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        request.validate()?;
        self.client.post(STAKING_DEFI_PURCHASE, request, true).await
    }

    /// Redeem a Staking/DeFi order.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn redeem(
        &self,
        request: &StakingDefiRedeemRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        request.validate()?;
        self.client.post(STAKING_DEFI_REDEEM, request, true).await
    }

    /// Cancel a Staking/DeFi order.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn cancel(
        &self,
        request: &StakingDefiCancelRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        request.validate()?;
        self.client.post(STAKING_DEFI_CANCEL, request, true).await
    }

    /// Retrieve active Staking/DeFi orders.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn get_active_orders(
        &self,
        request: &StakingDefiActiveOrdersRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        request.validate()?;
        self.client
            .get(STAKING_DEFI_ACTIVE_ORDERS, request, true)
            .await
    }

    /// Retrieve Staking/DeFi order history.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn get_orders_history(
        &self,
        request: &StakingDefiOrderHistoryRequest,
    ) -> Result<Vec<StakingDefiOrder>, Error> {
        request.validate()?;
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
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn product_info(&self) -> Result<Vec<StakingProductInfo>, Error> {
        self.client.get(ETH_PRODUCT_INFO, &NoParams {}, true).await
    }

    /// Purchase ETH staking.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn purchase(&self, amt: &str) -> Result<Vec<StakingOrder>, Error> {
        let body = AmountBody { amt };
        self.client.post(ETH_PURCHASE, &body, true).await
    }

    /// Redeem ETH staking.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn redeem(&self, amt: &str) -> Result<Vec<StakingOrder>, Error> {
        let body = AmountBody { amt };
        self.client.post(ETH_REDEEM, &body, true).await
    }

    /// Retrieve ETH staking balance.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn balance(&self) -> Result<Vec<StakingBalance>, Error> {
        self.client.get(ETH_BALANCE, &NoParams {}, true).await
    }

    /// Retrieve ETH staking purchase/redeem history.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn purchase_redeem_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<StakingHistory>, Error> {
        request.validate()?;
        self.client.get(ETH_HISTORY, request, true).await
    }

    /// Retrieve ETH staking APY history.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn apy_history(&self, days: &str) -> Result<Vec<StakingApyHistory>, Error> {
        let query = DaysQuery { days };
        self.client.get(ETH_APY_HISTORY, &query, false).await
    }
}

/// Accessor for SOL staking endpoints.
pub struct SolStaking<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> SolStaking<'_, T> {
    /// Retrieve SOL staking product info.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn product_info(&self) -> Result<StakingProductInfo, Error> {
        self.client.get(SOL_PRODUCT_INFO, &NoParams {}, true).await
    }

    /// Purchase SOL staking.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn purchase(&self, amt: &str) -> Result<Vec<StakingOrder>, Error> {
        let body = AmountBody { amt };
        self.client.post(SOL_PURCHASE, &body, true).await
    }

    /// Redeem SOL staking.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn redeem(&self, amt: &str) -> Result<Vec<StakingOrder>, Error> {
        let body = AmountBody { amt };
        self.client.post(SOL_REDEEM, &body, true).await
    }

    /// Retrieve SOL staking balance.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn balance(&self) -> Result<Vec<StakingBalance>, Error> {
        self.client.get(SOL_BALANCE, &NoParams {}, true).await
    }

    /// Retrieve SOL staking purchase/redeem history.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn purchase_redeem_history(
        &self,
        request: &FinanceHistoryRequest,
    ) -> Result<Vec<StakingHistory>, Error> {
        request.validate()?;
        self.client.get(SOL_HISTORY, request, true).await
    }

    /// Retrieve SOL staking APY history.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn apy_history(&self, days: &str) -> Result<Vec<StakingApyHistory>, Error> {
        let query = DaysQuery { days };
        self.client.get(SOL_APY_HISTORY, &query, false).await
    }
}

/// Accessor for Flexible Loan endpoints.
pub struct FlexibleLoan<'a, T> {
    client: &'a OkxClient<T>,
}

impl<T: Transport> FlexibleLoan<'_, T> {
    /// Retrieve borrowable currencies.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn borrow_currencies(&self) -> Result<Vec<FlexibleLoanCurrency>, Error> {
        self.client
            .get(FLEX_BORROW_CURRENCIES, &NoParams {}, true)
            .await
    }

    /// Retrieve collateral assets.
    ///
    /// # Errors
    ///
    /// Returns API, transport, or decode errors.
    pub async fn collateral_assets(
        &self,
        request: &FlexibleLoanCollateralAssetsRequest,
    ) -> Result<Vec<FlexibleLoanCollateralAsset>, Error> {
        request.validate()?;
        self.client
            .get(FLEX_COLLATERAL_ASSETS, request, false)
            .await
    }

    /// Estimate maximum flexible-loan amount.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn max_loan(
        &self,
        request: &FlexibleLoanMaxLoanRequest,
    ) -> Result<Vec<FlexibleLoanMaxLoan>, Error> {
        request.validate()?;
        self.client.post(FLEX_MAX_LOAN, request, true).await
    }

    /// Retrieve maximum collateral redeem amount.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn max_collateral_redeem_amount(
        &self,
        request: &FlexibleLoanMaxRedeemRequest,
    ) -> Result<Vec<FlexibleLoanMaxRedeem>, Error> {
        request.validate()?;
        self.client.get(FLEX_MAX_REDEEM, request, true).await
    }

    /// Adjust flexible-loan collateral.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn adjust_collateral(
        &self,
        request: &FlexibleLoanAdjustCollateralRequest,
    ) -> Result<Vec<FlexibleLoanOrder>, Error> {
        request.validate()?;
        self.client
            .post(FLEX_ADJUST_COLLATERAL, request, true)
            .await
    }

    /// Retrieve flexible-loan info.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn loan_info(
        &self,
        request: &FlexibleLoanInfoRequest,
    ) -> Result<Vec<FlexibleLoanInfo>, Error> {
        request.validate()?;
        self.client.get(FLEX_LOAN_INFO, request, true).await
    }

    /// Retrieve flexible-loan history.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn loan_history(
        &self,
        request: &FlexibleLoanHistoryRequest,
    ) -> Result<Vec<FlexibleLoanHistory>, Error> {
        request.validate()?;
        self.client.get(FLEX_LOAN_HISTORY, request, true).await
    }

    /// Retrieve flexible-loan accrued interest.
    ///
    /// # Errors
    ///
    /// Returns authentication, API, transport, or decode errors.
    pub async fn interest_accrued(
        &self,
        request: &FlexibleLoanInterestAccruedRequest,
    ) -> Result<Vec<FlexibleLoanInterest>, Error> {
        request.validate()?;
        self.client.get(FLEX_INTEREST_ACCRUED, request, true).await
    }
}
