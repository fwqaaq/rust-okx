use crate::model::RequestParams;

/// Generic history query for finance endpoints.
pub type FinanceHistoryRequest = RequestParams;

/// Savings purchase/redemption request.
pub type SavingsPurchaseRedemptionRequest = RequestParams;

/// Staking/DeFi offers request.
pub type StakingDefiOffersRequest = RequestParams;

/// Staking/DeFi purchase request.
pub type StakingDefiPurchaseRequest = RequestParams;

/// Staking/DeFi redeem request.
pub type StakingDefiRedeemRequest = RequestParams;

/// Staking/DeFi cancel request.
pub type StakingDefiCancelRequest = RequestParams;

/// Staking/DeFi orders request.
pub type StakingDefiOrdersRequest = RequestParams;

/// Flexible-loan max-loan request.
pub type FlexibleLoanMaxLoanRequest = RequestParams;

/// Flexible-loan collateral adjustment request.
pub type FlexibleLoanAdjustCollateralRequest = RequestParams;
