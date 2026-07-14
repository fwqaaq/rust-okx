use crate::client::OkxClient;
use crate::error::Error;
use crate::transport::Transport;

use super::endpoints::*;
use super::requests::*;
use super::responses::*;
use crate::model::EmptyRequest;

/// Accessor for the authenticated account endpoints.
///
/// Obtain one via [`OkxClient::account`](crate::OkxClient::account). All methods
/// require credentials; calling them without credentials returns
/// [`RestError::Configuration`](crate::RestError::Configuration).
pub struct Account<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Account<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve the trading-account balance.
    ///
    /// `GET /api/v5/account/balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_balance(
        &self,
        request: BalanceRequest<'_>,
    ) -> Result<Vec<AccountBalance>, Error> {
        self.client.get(BALANCE, &request, true).await
    }

    /// Retrieve open positions.
    ///
    /// `GET /api/v5/account/positions`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_positions(
        &self,
        request: &PositionsRequest<'_>,
    ) -> Result<Vec<Position>, Error> {
        self.client.get(POSITIONS, request, true).await
    }

    /// Retrieve account position risk.
    ///
    /// `GET /api/v5/account/account-position-risk`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_position_risk(
        &self,
        request: &PositionRiskRequest,
    ) -> Result<Vec<PositionRisk>, Error> {
        self.client.get(POSITION_RISK, request, true).await
    }

    /// Retrieve account configuration.
    ///
    /// `GET /api/v5/account/config`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_account_config(&self) -> Result<Vec<AccountConfig>, Error> {
        self.client
            .get(ACCOUNT_CONFIG, &EmptyRequest {}, true)
            .await
    }

    /// Retrieve recent account bills.
    ///
    /// `GET /api/v5/account/bills`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_account_bills(
        &self,
        request: &BillsRequest<'_>,
    ) -> Result<Vec<AccountBill>, Error> {
        self.client.get(BILLS, request, true).await
    }

    /// Retrieve archived account bills.
    ///
    /// `GET /api/v5/account/bills-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_account_bills_archive(
        &self,
        request: &BillsArchiveRequest<'_>,
    ) -> Result<Vec<AccountBill>, Error> {
        self.client.get(BILLS_ARCHIVE, request, true).await
    }

    /// Apply for historical account-bills archive generation.
    ///
    /// `POST /api/v5/account/bills-history-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn apply_bills_history_archive(
        &self,
        request: &ApplyBillsHistoryArchiveRequest<'_>,
    ) -> Result<Vec<ApplyBillsHistoryArchiveResult>, Error> {
        self.client.post(BILLS_HISTORY_ARCHIVE, request, true).await
    }

    /// Retrieve historical account-bills archive download links.
    ///
    /// `GET /api/v5/account/bills-history-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_bills_history_archive(
        &self,
        request: &BillsHistoryArchiveRequest<'_>,
    ) -> Result<Vec<BillsHistoryArchiveFile>, Error> {
        self.client.get(BILLS_HISTORY_ARCHIVE, request, true).await
    }

    /// Retrieve account bill types and subtype mappings.
    ///
    /// `GET /api/v5/account/subtypes`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_bill_subtypes(
        &self,
        request: &BillSubtypesRequest<'_>,
    ) -> Result<Vec<BillSubtypeMapping>, Error> {
        self.client.get(SUBTYPES, request, true).await
    }

    /// Set the account position mode.
    ///
    /// `POST /api/v5/account/set-position-mode`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_position_mode(
        &self,
        request: &SetPositionModeRequest<'_>,
    ) -> Result<Vec<SetPositionModeResult>, Error> {
        self.client.post(SET_POSITION_MODE, request, true).await
    }

    /// Set whether all or custom assets are used as collateral.
    ///
    /// `POST /api/v5/account/set-collateral-assets`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_collateral_assets(
        &self,
        request: &SetCollateralAssetsRequest<'_>,
    ) -> Result<Vec<SetCollateralAssetsResult>, Error> {
        self.client.post(SET_COLLATERAL_ASSETS, request, true).await
    }

    /// Retrieve currencies and their collateral settings.
    ///
    /// `GET /api/v5/account/collateral-assets`. Authenticated.
    ///
    /// See the [OKX API documentation](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-collateral-assets).
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_collateral_assets(
        &self,
        request: &GetCollateralAssetsRequest<'_>,
    ) -> Result<Vec<CollateralAsset>, Error> {
        self.client.get(COLLATERAL_ASSETS, request, true).await
    }

    /// Set leverage for an instrument or currency.
    ///
    /// `POST /api/v5/account/set-leverage`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_leverage(
        &self,
        request: &SetLeverageRequest<'_>,
    ) -> Result<Vec<LeverageInfo>, Error> {
        self.client.post(SET_LEVERAGE, request, true).await
    }

    /// Retrieve leverage settings.
    ///
    /// `GET /api/v5/account/leverage-info`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_leverage(
        &self,
        request: &LeverageRequest<'_>,
    ) -> Result<Vec<LeverageInfo>, Error> {
        self.client.get(GET_LEVERAGE, request, true).await
    }

    /// Estimate account state after adjusting leverage.
    ///
    /// `GET /api/v5/account/adjust-leverage-info`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_adjust_leverage_info(
        &self,
        request: &AdjustLeverageInfoRequest<'_>,
    ) -> Result<Vec<AdjustLeverageInfo>, Error> {
        self.client.get(ADJUST_LEVERAGE_INFO, request, true).await
    }

    /// Retrieve maximum tradable size for an instrument.
    ///
    /// `GET /api/v5/account/max-size`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_max_order_size(
        &self,
        request: &MaxOrderSizeRequest<'_>,
    ) -> Result<Vec<MaxOrderSize>, Error> {
        self.client.get(MAX_ORDER_SIZE, request, true).await
    }

    /// Retrieve maximum available size for an instrument.
    ///
    /// `GET /api/v5/account/max-avail-size`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_max_avail_size(
        &self,
        request: &MaxAvailableSizeRequest<'_>,
    ) -> Result<Vec<MaxAvailableSize>, Error> {
        self.client.get(MAX_AVAILABLE_SIZE, request, true).await
    }

    /// Increase or decrease margin for a position.
    ///
    /// `POST /api/v5/account/position/margin-balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn adjust_margin(
        &self,
        request: &AdjustMarginRequest<'_>,
    ) -> Result<Vec<AdjustMarginResult>, Error> {
        self.client.post(ADJUST_MARGIN, request, true).await
    }

    /// Retrieve trade fee rates.
    ///
    /// `GET /api/v5/account/trade-fee`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_fee_rates(
        &self,
        request: &FeeRatesRequest<'_>,
    ) -> Result<Vec<FeeRate>, Error> {
        self.client.get(FEE_RATES, request, true).await
    }

    /// Retrieve account-available instruments.
    ///
    /// `GET /api/v5/account/instruments`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_account_instruments(
        &self,
        request: &AccountInstrumentsRequest<'_>,
    ) -> Result<Vec<AccountInstrument>, Error> {
        self.client.get(ACCOUNT_INSTRUMENTS, request, true).await
    }

    /// Retrieve the maximum loan amount.
    ///
    /// `GET /api/v5/account/max-loan`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_max_loan(&self, request: &MaxLoanRequest<'_>) -> Result<Vec<MaxLoan>, Error> {
        self.client.get(MAX_LOAN, request, true).await
    }

    /// Retrieve interest-accrued records.
    ///
    /// `GET /api/v5/account/interest-accrued`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_interest_accrued(
        &self,
        request: &InterestAccruedRequest<'_>,
    ) -> Result<Vec<InterestAccrued>, Error> {
        self.client.get(INTEREST_ACCRUED, request, true).await
    }

    /// Retrieve interest rates.
    ///
    /// `GET /api/v5/account/interest-rate`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_interest_rate(
        &self,
        request: BalanceRequest<'_>,
    ) -> Result<Vec<InterestRate>, Error> {
        self.client.get(INTEREST_RATE, &request, true).await
    }

    /// Set the greeks display type.
    ///
    /// `POST /api/v5/account/set-greeks`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_greeks(
        &self,
        request: &SetGreeksRequest<'_>,
    ) -> Result<Vec<SetGreeksResult>, Error> {
        self.client.post(SET_GREEKS, request, true).await
    }

    /// Set isolated margin transfer mode.
    ///
    /// `POST /api/v5/account/set-isolated-mode`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_isolated_mode(
        &self,
        request: &SetIsolatedModeRequest<'_>,
    ) -> Result<Vec<SetIsolatedModeResult>, Error> {
        self.client.post(SET_ISOLATED_MODE, request, true).await
    }

    /// Retrieve maximum withdrawal amounts.
    ///
    /// `GET /api/v5/account/max-withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_max_withdrawal(
        &self,
        request: BalanceRequest<'_>,
    ) -> Result<Vec<MaxWithdrawal>, Error> {
        self.client.get(MAX_WITHDRAWAL, &request, true).await
    }

    /// Retrieve borrowing rate and limit information.
    ///
    /// `GET /api/v5/account/interest-limits`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_interest_limits(
        &self,
        request: &InterestLimitsRequest<'_>,
    ) -> Result<Vec<InterestLimit>, Error> {
        self.client.get(INTEREST_LIMITS, request, true).await
    }

    /// Calculate simulated margin information.
    ///
    /// `POST /api/v5/account/simulated_margin`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_simulated_margin(
        &self,
        request: &SimulatedMarginRequest<'_>,
    ) -> Result<Vec<SimulatedMargin>, Error> {
        self.client.post(SIMULATED_MARGIN, request, true).await
    }

    /// Retrieve greeks.
    ///
    /// `GET /api/v5/account/greeks`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_greeks(&self, request: BalanceRequest<'_>) -> Result<Vec<Greek>, Error> {
        self.client.get(GREEKS, &request, true).await
    }

    /// Retrieve position history.
    ///
    /// `GET /api/v5/account/positions-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_positions_history(
        &self,
        request: &PositionsHistoryRequest<'_>,
    ) -> Result<Vec<PositionHistory>, Error> {
        self.client.get(POSITIONS_HISTORY, request, true).await
    }

    /// Retrieve account position tiers.
    ///
    /// `GET /api/v5/account/position-tiers`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_account_position_tiers(
        &self,
        request: &AccountPositionTiersRequest<'_>,
    ) -> Result<Vec<AccountPositionTier>, Error> {
        self.client.get(ACCOUNT_POSITION_TIERS, request, true).await
    }

    /// Retrieve the account risk state.
    ///
    /// `GET /api/v5/account/risk-state`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_risk_state(&self) -> Result<Vec<RiskState>, Error> {
        self.client.get(RISK_STATE, &EmptyRequest {}, true).await
    }

    /// Set the spot risk offset amount.
    ///
    /// `POST /api/v5/account/set-riskOffset-amt`. Authenticated.
    ///
    /// Only applicable to Portfolio Margin Mode.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_risk_offset_amount(
        &self,
        request: &SetRiskOffsetAmountRequest<'_>,
    ) -> Result<Vec<SetRiskOffsetAmountResult>, Error> {
        self.client
            .post(SET_RISK_OFFSET_AMOUNT, request, true)
            .await
    }

    /// Set account auto-loan mode.
    ///
    /// `POST /api/v5/account/set-auto-loan`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_auto_loan(
        &self,
        request: &SetAutoLoanRequest,
    ) -> Result<Vec<SetAutoLoanResult>, Error> {
        self.client.post(SET_AUTO_LOAN, request, true).await
    }

    /// Set the account level.
    ///
    /// `POST /api/v5/account/set-account-level`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_account_level(
        &self,
        request: &SetAccountLevelRequest<'_>,
    ) -> Result<Vec<SetAccountLevelResult>, Error> {
        self.client.post(SET_ACCOUNT_LEVEL, request, true).await
    }

    /// Activate option trading.
    ///
    /// `POST /api/v5/account/activate-option`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn activate_option(&self) -> Result<Vec<ActivateOptionResult>, Error> {
        self.client
            .post(ACTIVATE_OPTION, &EmptyRequest {}, true)
            .await
    }

    /// Build simulated positions and equity.
    ///
    /// `POST /api/v5/account/position-builder`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn position_builder(
        &self,
        request: &PositionBuilderRequest<'_>,
    ) -> Result<Vec<PositionBuilderResult>, Error> {
        self.client.post(POSITION_BUILDER, request, true).await
    }

    /// Manually borrow or repay spot liabilities.
    ///
    /// `POST /api/v5/account/spot-manual-borrow-repay`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn spot_manual_borrow_repay(
        &self,
        request: &SpotManualBorrowRepayRequest<'_>,
    ) -> Result<Vec<SpotBorrowRepayResult>, Error> {
        self.client
            .post(SPOT_MANUAL_BORROW_REPAY, request, true)
            .await
    }

    /// Set automatic repayment for spot borrow/repay.
    ///
    /// `POST /api/v5/account/set-auto-repay`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_auto_repay(
        &self,
        request: &SetAutoRepayRequest,
    ) -> Result<Vec<SetAutoRepayResult>, Error> {
        self.client.post(SET_AUTO_REPAY, request, true).await
    }

    /// Retrieve spot borrow/repay history.
    ///
    /// `GET /api/v5/account/spot-borrow-repay-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn get_spot_borrow_repay_history(
        &self,
        request: &SpotBorrowRepayHistoryRequest<'_>,
    ) -> Result<Vec<SpotBorrowRepayHistory>, Error> {
        self.client
            .get(SPOT_BORROW_REPAY_HISTORY, request, true)
            .await
    }

    /// Set automatic earn for the account.
    ///
    /// `POST /api/v5/account/set-auto-earn`. Authenticated.
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_auto_earn(
        &self,
        request: &SetAutoEarnRequest<'_>,
    ) -> Result<Vec<SetAutoEarnResult>, Error> {
        self.client.post(SET_AUTO_EARN, request, true).await
    }

    /// Configure Market Maker Protection for an option instrument family.
    ///
    /// `POST /api/v5/account/mmp-config`. Authenticated.
    ///
    /// This endpoint applies to options in Portfolio Margin mode and requires
    /// MMP privilege.
    ///
    /// See the [OKX API documentation](https://www.okx.com/docs-v5/en/#trading-account-rest-api-set-mmp).
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn set_mmp_config(
        &self,
        request: &SetMmpConfigRequest<'_>,
    ) -> Result<Vec<SetMmpConfigResult>, Error> {
        self.client.post(MMP_CONFIG, request, true).await
    }

    /// Reset Market Maker Protection status after MMP is triggered.
    ///
    /// `POST /api/v5/account/mmp-reset`. Authenticated.
    ///
    /// This endpoint applies to options in Portfolio Margin mode and requires
    /// MMP privilege.
    ///
    /// See the [OKX API documentation](https://www.okx.com/docs-v5/en/#trading-account-rest-api-reset-mmp-status).
    ///
    /// # Errors
    ///
    /// Returns [`RestError::Configuration`](crate::RestError::Configuration) if no credentials are set,
    /// [`RestError::Okx`](crate::RestError::Okx) on a non-zero OKX code, or transport/decode errors.
    pub async fn reset_mmp_status(
        &self,
        request: &ResetMmpStatusRequest<'_>,
    ) -> Result<Vec<ResetMmpStatusResult>, Error> {
        self.client.post(MMP_RESET, request, true).await
    }
}
