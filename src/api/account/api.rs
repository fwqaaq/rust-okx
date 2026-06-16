use crate::client::OkxClient;
use crate::error::Error;
use crate::model::InstType;
use crate::transport::Transport;

use super::endpoints::*;
use super::internal::*;
use super::requests::*;
use super::responses::*;

/// Accessor for the authenticated account endpoints.
///
/// Obtain one via [`OkxClient::account`](crate::OkxClient::account). All methods
/// require credentials; calling them without credentials returns
/// [`Error::Configuration`].
pub struct Account<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> Account<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve the trading-account balance.
    ///
    /// `GET /api/v5/account/balance`. Authenticated. Pass `ccy` to filter to one
    /// or more comma-separated currencies (e.g. `Some("BTC,USDT")`). The result
    /// is a single [`AccountBalance`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no credentials are set,
    /// [`Error::Api`] on a non-zero OKX code, or transport/decode errors.
    pub async fn get_balance(&self, ccy: Option<&str>) -> Result<Vec<AccountBalance>, Error> {
        let query = BalanceQuery { ccy };
        self.client.get(BALANCE, &query, true).await
    }

    /// Retrieve open positions.
    ///
    /// `GET /api/v5/account/positions`. Authenticated. Both filters are
    /// optional; omit them to return all positions.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_positions(
        &self,
        inst_type: Option<InstType>,
        inst_id: Option<&str>,
    ) -> Result<Vec<Position>, Error> {
        let query = PositionsQuery {
            inst_type: inst_type.as_ref(),
            inst_id,
        };
        self.client.get(POSITIONS, &query, true).await
    }

    /// Retrieve account position risk.
    ///
    /// `GET /api/v5/account/account-position-risk`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_position_risk(
        &self,
        inst_type: Option<InstType>,
    ) -> Result<Vec<PositionRisk>, Error> {
        let query = PositionRiskQuery {
            inst_type: inst_type.as_ref(),
        };
        self.client.get(POSITION_RISK, &query, true).await
    }

    /// Retrieve account configuration.
    ///
    /// `GET /api/v5/account/config`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_account_config(&self) -> Result<Vec<AccountConfig>, Error> {
        self.client.get(ACCOUNT_CONFIG, &NoQuery, true).await
    }

    /// Retrieve recent account bills.
    ///
    /// `GET /api/v5/account/bills`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_account_bills(
        &self,
        request: &BillsRequest,
    ) -> Result<Vec<AccountBill>, Error> {
        self.client.get(BILLS, request, true).await
    }

    /// Retrieve archived account bills.
    ///
    /// `GET /api/v5/account/bills-archive`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_account_bills_archive(
        &self,
        request: &BillsArchiveRequest,
    ) -> Result<Vec<AccountBill>, Error> {
        self.client.get(BILLS_ARCHIVE, request, true).await
    }

    /// Set the account position mode.
    ///
    /// `POST /api/v5/account/set-position-mode`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_position_mode(
        &self,
        pos_mode: &str,
    ) -> Result<Vec<SetPositionModeResult>, Error> {
        let body = SetPositionModeBody { pos_mode };
        self.client.post(SET_POSITION_MODE, &body, true).await
    }

    /// Set leverage for an instrument or currency.
    ///
    /// `POST /api/v5/account/set-leverage`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_leverage(
        &self,
        request: &SetLeverageRequest,
    ) -> Result<Vec<LeverageInfo>, Error> {
        self.client.post(SET_LEVERAGE, request, true).await
    }

    /// Retrieve leverage settings.
    ///
    /// `GET /api/v5/account/leverage-info`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_leverage(
        &self,
        request: &LeverageRequest,
    ) -> Result<Vec<LeverageInfo>, Error> {
        self.client.get(GET_LEVERAGE, request, true).await
    }

    /// Retrieve maximum tradable size for an instrument.
    ///
    /// `GET /api/v5/account/max-size`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_max_order_size(
        &self,
        request: &MaxOrderSizeRequest,
    ) -> Result<Vec<MaxOrderSize>, Error> {
        self.client.get(MAX_ORDER_SIZE, request, true).await
    }

    /// Retrieve maximum available size for an instrument.
    ///
    /// `GET /api/v5/account/max-avail-size`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_max_avail_size(
        &self,
        request: &MaxAvailableSizeRequest,
    ) -> Result<Vec<MaxAvailableSize>, Error> {
        self.client.get(MAX_AVAILABLE_SIZE, request, true).await
    }

    /// Increase or decrease margin for a position.
    ///
    /// `POST /api/v5/account/position/margin-balance`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn adjust_margin(
        &self,
        request: &AdjustMarginRequest,
    ) -> Result<Vec<AdjustMarginResult>, Error> {
        self.client.post(ADJUST_MARGIN, request, true).await
    }

    /// Retrieve trade fee rates.
    ///
    /// `GET /api/v5/account/trade-fee`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_fee_rates(&self, request: &FeeRatesRequest) -> Result<Vec<FeeRate>, Error> {
        self.client.get(FEE_RATES, request, true).await
    }

    /// Retrieve account-available instruments.
    ///
    /// `GET /api/v5/account/instruments`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_account_instruments(
        &self,
        request: &AccountInstrumentsRequest,
    ) -> Result<Vec<AccountInstrument>, Error> {
        self.client.get(ACCOUNT_INSTRUMENTS, request, true).await
    }

    /// Retrieve the maximum loan amount.
    ///
    /// `GET /api/v5/account/max-loan`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_max_loan(&self, request: &MaxLoanRequest) -> Result<Vec<MaxLoan>, Error> {
        self.client.get(MAX_LOAN, request, true).await
    }

    /// Retrieve interest-accrued records.
    ///
    /// `GET /api/v5/account/interest-accrued`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_interest_accrued(
        &self,
        request: &InterestAccruedRequest,
    ) -> Result<Vec<InterestAccrued>, Error> {
        self.client.get(INTEREST_ACCRUED, request, true).await
    }

    /// Retrieve interest rates.
    ///
    /// `GET /api/v5/account/interest-rate`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_interest_rate(&self, ccy: Option<&str>) -> Result<Vec<InterestRate>, Error> {
        let query = BalanceQuery { ccy };
        self.client.get(INTEREST_RATE, &query, true).await
    }

    /// Set the greeks display type.
    ///
    /// `POST /api/v5/account/set-greeks`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_greeks(&self, greeks_type: &str) -> Result<Vec<SetGreeksResult>, Error> {
        let body = SetGreeksBody { greeks_type };
        self.client.post(SET_GREEKS, &body, true).await
    }

    /// Set isolated margin transfer mode.
    ///
    /// `POST /api/v5/account/set-isolated-mode`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_isolated_mode(
        &self,
        iso_mode: &str,
        mode_type: &str,
    ) -> Result<Vec<SetIsolatedModeResult>, Error> {
        let body = SetIsolatedModeBody {
            iso_mode,
            mode_type,
        };
        self.client.post(SET_ISOLATED_MODE, &body, true).await
    }

    /// Retrieve maximum withdrawal amounts.
    ///
    /// `GET /api/v5/account/max-withdrawal`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_max_withdrawal(&self, ccy: Option<&str>) -> Result<Vec<MaxWithdrawal>, Error> {
        let query = BalanceQuery { ccy };
        self.client.get(MAX_WITHDRAWAL, &query, true).await
    }

    /// Borrow or repay margin.
    ///
    /// `POST /api/v5/account/borrow-repay`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn borrow_repay(
        &self,
        request: &BorrowRepayRequest,
    ) -> Result<Vec<BorrowRepayResult>, Error> {
        self.client.post(BORROW_REPAY, request, true).await
    }

    /// Retrieve borrow/repay history.
    ///
    /// `GET /api/v5/account/borrow-repay-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_borrow_repay_history(
        &self,
        request: &BorrowRepayHistoryRequest,
    ) -> Result<Vec<BorrowRepayHistory>, Error> {
        self.client.get(BORROW_REPAY_HISTORY, request, true).await
    }

    /// Retrieve borrowing rate and limit information.
    ///
    /// `GET /api/v5/account/interest-limits`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_interest_limits(
        &self,
        request: &InterestLimitsRequest,
    ) -> Result<Vec<InterestLimit>, Error> {
        self.client.get(INTEREST_LIMITS, request, true).await
    }

    /// Calculate simulated margin information.
    ///
    /// `POST /api/v5/account/simulated_margin`. Authenticated. This is separate
    /// from [`OkxClientBuilder::demo_trading`](crate::OkxClientBuilder::demo_trading),
    /// which only toggles the OKX simulated-trading header.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_simulated_margin(
        &self,
        request: &SimulatedMarginRequest,
    ) -> Result<Vec<SimulatedMargin>, Error> {
        self.client.post(SIMULATED_MARGIN, request, true).await
    }

    /// Retrieve greeks.
    ///
    /// `GET /api/v5/account/greeks`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_greeks(&self, ccy: Option<&str>) -> Result<Vec<Greek>, Error> {
        let query = BalanceQuery { ccy };
        self.client.get(GREEKS, &query, true).await
    }

    /// Retrieve position history.
    ///
    /// `GET /api/v5/account/positions-history`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_positions_history(
        &self,
        request: &PositionsHistoryRequest,
    ) -> Result<Vec<PositionHistory>, Error> {
        self.client.get(POSITIONS_HISTORY, request, true).await
    }

    /// Retrieve account position tiers.
    ///
    /// `GET /api/v5/account/position-tiers`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_account_position_tiers(
        &self,
        request: &AccountPositionTiersRequest,
    ) -> Result<Vec<AccountPositionTier>, Error> {
        self.client.get(ACCOUNT_POSITION_TIERS, request, true).await
    }

    /// Retrieve the account risk state.
    ///
    /// `GET /api/v5/account/risk-state`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_risk_state(&self) -> Result<Vec<RiskState>, Error> {
        self.client.get(RISK_STATE, &NoQuery, true).await
    }

    /// Set account risk offset type.
    ///
    /// `POST /api/v5/account/set-riskOffset-type`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_risk_offset_type(
        &self,
        risk_offset_type: &str,
    ) -> Result<Vec<SetRiskOffsetTypeResult>, Error> {
        let body = TypeBody {
            value: risk_offset_type,
        };
        self.client.post(SET_RISK_OFFSET_TYPE, &body, true).await
    }

    /// Set account auto-loan mode.
    ///
    /// `POST /api/v5/account/set-auto-loan`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_auto_loan(&self, auto_loan: bool) -> Result<Vec<SetAutoLoanResult>, Error> {
        let body = SetAutoLoanBody { auto_loan };
        self.client.post(SET_AUTO_LOAN, &body, true).await
    }

    /// Set the account level.
    ///
    /// `POST /api/v5/account/set-account-level`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_account_level(
        &self,
        acct_lv: &str,
    ) -> Result<Vec<SetAccountLevelResult>, Error> {
        let body = SetAccountLevelBody { acct_lv };
        self.client.post(SET_ACCOUNT_LEVEL, &body, true).await
    }

    /// Activate option trading.
    ///
    /// `POST /api/v5/account/activate-option`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn activate_option(&self) -> Result<Vec<ActivateOptionResult>, Error> {
        self.client.post(ACTIVATE_OPTION, &EmptyBody {}, true).await
    }

    /// Build simulated positions and equity.
    ///
    /// `POST /api/v5/account/position-builder`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn position_builder(
        &self,
        request: &PositionBuilderRequest,
    ) -> Result<Vec<PositionBuilderResult>, Error> {
        self.client.post(POSITION_BUILDER, request, true).await
    }

    /// Retrieve VIP-loan interest accrued records.
    ///
    /// `GET /api/v5/account/vip-interest-accrued`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_vip_interest_accrued(
        &self,
        request: &VipInterestAccruedRequest,
    ) -> Result<Vec<VipInterestAccrued>, Error> {
        self.client.get(VIP_INTEREST_ACCRUED, request, true).await
    }

    /// Retrieve VIP-loan interest deducted records.
    ///
    /// `GET /api/v5/account/vip-interest-deducted`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_vip_interest_deducted(
        &self,
        request: &VipInterestDeductedRequest,
    ) -> Result<Vec<VipInterestDeducted>, Error> {
        self.client.get(VIP_INTEREST_DEDUCTED, request, true).await
    }

    /// Retrieve VIP-loan orders.
    ///
    /// `GET /api/v5/account/vip-loan-order-list`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_vip_loan_order_list(
        &self,
        request: &VipLoanOrderListRequest,
    ) -> Result<Vec<VipLoanOrder>, Error> {
        self.client.get(VIP_LOAN_ORDER_LIST, request, true).await
    }

    /// Retrieve a VIP-loan order detail.
    ///
    /// `GET /api/v5/account/vip-loan-order-detail`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_vip_loan_order_detail(
        &self,
        request: &VipLoanOrderDetailRequest,
    ) -> Result<Vec<VipLoanOrder>, Error> {
        self.client.get(VIP_LOAN_ORDER_DETAIL, request, true).await
    }

    /// Retrieve fixed-loan borrowing limits.
    ///
    /// `GET /api/v5/account/fixed-loan/borrowing-limit`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_fixed_loan_borrowing_limit(
        &self,
        request: &FixedLoanBorrowingLimitRequest,
    ) -> Result<Vec<FixedLoanBorrowingLimit>, Error> {
        self.client
            .get(FIXED_LOAN_BORROWING_LIMIT, request, true)
            .await
    }

    /// Request a fixed-loan borrowing quote.
    ///
    /// `POST /api/v5/account/fixed-loan/borrowing-quote`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn fixed_loan_borrowing_quote(
        &self,
        request: &FixedLoanBorrowingQuoteRequest,
    ) -> Result<Vec<FixedLoanBorrowingQuote>, Error> {
        self.client
            .post(FIXED_LOAN_BORROWING_QUOTE, request, true)
            .await
    }

    /// Place a fixed-loan borrowing order.
    ///
    /// `POST /api/v5/account/fixed-loan/borrowing-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn fixed_loan_borrowing_order(
        &self,
        request: &FixedLoanBorrowingOrderRequest,
    ) -> Result<Vec<FixedLoanBorrowingOrder>, Error> {
        self.client
            .post(FIXED_LOAN_BORROWING_ORDER, request, true)
            .await
    }

    /// Amend a fixed-loan borrowing order.
    ///
    /// `POST /api/v5/account/fixed-loan/amend-borrowing-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn amend_fixed_loan_borrowing_order(
        &self,
        request: &FixedLoanAmendBorrowingOrderRequest,
    ) -> Result<Vec<FixedLoanBorrowingOrder>, Error> {
        self.client
            .post(FIXED_LOAN_AMEND_BORROWING_ORDER, request, true)
            .await
    }

    /// Manually reborrow a fixed-loan order.
    ///
    /// `POST /api/v5/account/fixed-loan/manual-reborrow`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn fixed_loan_manual_reborrow(
        &self,
        request: &FixedLoanManualReborrowRequest,
    ) -> Result<Vec<FixedLoanBorrowingOrder>, Error> {
        self.client
            .post(FIXED_LOAN_MANUAL_REBORROW, request, true)
            .await
    }

    /// Repay a fixed-loan borrowing order.
    ///
    /// `POST /api/v5/account/fixed-loan/repay-borrowing-order`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn repay_fixed_loan_borrowing_order(
        &self,
        request: &FixedLoanRepayBorrowingOrderRequest,
    ) -> Result<Vec<FixedLoanBorrowingOrder>, Error> {
        self.client
            .post(FIXED_LOAN_REPAY_BORROWING_ORDER, request, true)
            .await
    }

    /// Retrieve fixed-loan borrowing orders.
    ///
    /// `GET /api/v5/account/fixed-loan/borrowing-orders-list`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_fixed_loan_borrowing_orders_list(
        &self,
        request: &FixedLoanBorrowingOrdersListRequest,
    ) -> Result<Vec<FixedLoanBorrowingOrder>, Error> {
        self.client
            .get(FIXED_LOAN_BORROWING_ORDERS_LIST, request, true)
            .await
    }

    /// Manually borrow or repay spot liabilities.
    ///
    /// `POST /api/v5/account/spot-manual-borrow-repay`. Authenticated.
    ///
    /// # Errors
    ///
    /// See [`get_balance`](Self::get_balance).
    pub async fn spot_manual_borrow_repay(
        &self,
        request: &SpotManualBorrowRepayRequest,
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
    /// See [`get_balance`](Self::get_balance).
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
    /// See [`get_balance`](Self::get_balance).
    pub async fn get_spot_borrow_repay_history(
        &self,
        request: &SpotBorrowRepayHistoryRequest,
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
    /// See [`get_balance`](Self::get_balance).
    pub async fn set_auto_earn(
        &self,
        request: &SetAutoEarnRequest,
    ) -> Result<Vec<SetAutoEarnResult>, Error> {
        self.client.post(SET_AUTO_EARN, request, true).await
    }
}
