//! Authenticated account endpoints (`/api/v5/account/*`).

use serde::{Deserialize, Serialize};

use crate::client::OkxClient;
use crate::error::Error;
use crate::model::{InstType, NumberString, PositionSide, TradeMode};
use crate::transport::Transport;

const BALANCE: &str = "/api/v5/account/balance";
const POSITIONS: &str = "/api/v5/account/positions";
const POSITION_RISK: &str = "/api/v5/account/account-position-risk";
const ACCOUNT_CONFIG: &str = "/api/v5/account/config";
const BILLS: &str = "/api/v5/account/bills";
const BILLS_ARCHIVE: &str = "/api/v5/account/bills-archive";
const SET_POSITION_MODE: &str = "/api/v5/account/set-position-mode";
const SET_LEVERAGE: &str = "/api/v5/account/set-leverage";
const GET_LEVERAGE: &str = "/api/v5/account/leverage-info";
const MAX_ORDER_SIZE: &str = "/api/v5/account/max-size";
const MAX_AVAILABLE_SIZE: &str = "/api/v5/account/max-avail-size";
const ADJUST_MARGIN: &str = "/api/v5/account/position/margin-balance";
const FEE_RATES: &str = "/api/v5/account/trade-fee";
const ACCOUNT_INSTRUMENTS: &str = "/api/v5/account/instruments";
const MAX_LOAN: &str = "/api/v5/account/max-loan";
const INTEREST_ACCRUED: &str = "/api/v5/account/interest-accrued";
const INTEREST_RATE: &str = "/api/v5/account/interest-rate";
const SET_GREEKS: &str = "/api/v5/account/set-greeks";
const SET_ISOLATED_MODE: &str = "/api/v5/account/set-isolated-mode";
const MAX_WITHDRAWAL: &str = "/api/v5/account/max-withdrawal";
const BORROW_REPAY: &str = "/api/v5/account/borrow-repay";
const BORROW_REPAY_HISTORY: &str = "/api/v5/account/borrow-repay-history";
const INTEREST_LIMITS: &str = "/api/v5/account/interest-limits";
const SIMULATED_MARGIN: &str = "/api/v5/account/simulated_margin";
const GREEKS: &str = "/api/v5/account/greeks";
const POSITIONS_HISTORY: &str = "/api/v5/account/positions-history";
const ACCOUNT_POSITION_TIERS: &str = "/api/v5/account/position-tiers";
const RISK_STATE: &str = "/api/v5/account/risk-state";
const SET_RISK_OFFSET_TYPE: &str = "/api/v5/account/set-riskOffset-type";
const SET_AUTO_LOAN: &str = "/api/v5/account/set-auto-loan";
const SET_ACCOUNT_LEVEL: &str = "/api/v5/account/set-account-level";
const ACTIVATE_OPTION: &str = "/api/v5/account/activate-option";
const POSITION_BUILDER: &str = "/api/v5/account/position-builder";

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
}

#[derive(Serialize)]
struct NoQuery;

#[derive(Serialize)]
struct EmptyBody {}

#[derive(Serialize)]
struct BalanceQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<&'a str>,
}

#[derive(Serialize)]
struct PositionsQuery<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<&'a InstType>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<&'a str>,
}

#[derive(Serialize)]
struct PositionRiskQuery<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<&'a InstType>,
}

#[derive(Serialize)]
struct SetPositionModeBody<'a> {
    #[serde(rename = "posMode")]
    pos_mode: &'a str,
}

/// Query parameters for account bills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillsRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(rename = "ctType", skip_serializing_if = "Option::is_none")]
    ct_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    bill_type: Option<String>,
    #[serde(rename = "subType", skip_serializing_if = "Option::is_none")]
    sub_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl BillsRequest {
    /// Create an empty account-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Set the contract type filter.
    pub fn contract_type(mut self, ct_type: impl Into<String>) -> Self {
        self.ct_type = Some(ct_type.into());
        self
    }

    /// Set the bill type filter.
    pub fn bill_type(mut self, bill_type: impl Into<String>) -> Self {
        self.bill_type = Some(bill_type.into());
        self
    }

    /// Set the bill subtype filter.
    pub fn sub_type(mut self, sub_type: impl Into<String>) -> Self {
        self.sub_type = Some(sub_type.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for archived account bills.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BillsArchiveRequest {
    #[serde(flatten)]
    base: BillsRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<String>,
}

impl BillsArchiveRequest {
    /// Create an empty archived-bills query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the common bills filters.
    pub fn filters(mut self, base: BillsRequest) -> Self {
        self.base = base;
        self
    }

    /// Set the begin timestamp.
    pub fn begin(mut self, begin: impl Into<String>) -> Self {
        self.begin = Some(begin.into());
        self
    }

    /// Set the end timestamp.
    pub fn end(mut self, end: impl Into<String>) -> Self {
        self.end = Some(end.into());
        self
    }
}

/// Request body for setting leverage.
#[derive(Debug, Clone, Serialize)]
pub struct SetLeverageRequest {
    lever: String,
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
}

impl SetLeverageRequest {
    /// Create a leverage-setting request.
    pub fn new(lever: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self {
            lever: lever.into(),
            mgn_mode,
            inst_id: None,
            ccy: None,
            pos_side: None,
        }
    }

    /// Set the instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the position side.
    pub fn position_side(mut self, pos_side: PositionSide) -> Self {
        self.pos_side = Some(pos_side);
        self
    }
}

/// Query parameters for retrieving leverage.
#[derive(Debug, Clone, Serialize)]
pub struct LeverageRequest {
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl LeverageRequest {
    /// Create a leverage-info query.
    pub fn new(mgn_mode: TradeMode) -> Self {
        Self {
            mgn_mode,
            inst_id: None,
            ccy: None,
        }
    }

    /// Set the instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// Query parameters for maximum order size.
#[derive(Debug, Clone, Serialize)]
pub struct MaxOrderSizeRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "tdMode")]
    td_mode: TradeMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    px: Option<String>,
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl MaxOrderSizeRequest {
    /// Create a maximum-order-size query.
    pub fn new(inst_id: impl Into<String>, td_mode: TradeMode) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            ccy: None,
            px: None,
            trade_quote_ccy: None,
        }
    }

    /// Set the currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the price.
    pub fn price(mut self, px: impl Into<String>) -> Self {
        self.px = Some(px.into());
        self
    }

    /// Set the trade quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// Query parameters for maximum available size.
#[derive(Debug, Clone, Serialize)]
pub struct MaxAvailableSizeRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "tdMode")]
    td_mode: TradeMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(rename = "unSpotOffset", skip_serializing_if = "Option::is_none")]
    un_spot_offset: Option<bool>,
    #[serde(rename = "quickMgnType", skip_serializing_if = "Option::is_none")]
    quick_mgn_type: Option<String>,
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl MaxAvailableSizeRequest {
    /// Create a maximum-available-size query.
    pub fn new(inst_id: impl Into<String>, td_mode: TradeMode) -> Self {
        Self {
            inst_id: inst_id.into(),
            td_mode,
            ccy: None,
            reduce_only: None,
            un_spot_offset: None,
            quick_mgn_type: None,
            trade_quote_ccy: None,
        }
    }

    /// Set the currency.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the reduce-only filter.
    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Set the spot offset flag.
    pub fn un_spot_offset(mut self, un_spot_offset: bool) -> Self {
        self.un_spot_offset = Some(un_spot_offset);
        self
    }

    /// Set the quick margin type.
    pub fn quick_margin_type(mut self, quick_mgn_type: impl Into<String>) -> Self {
        self.quick_mgn_type = Some(quick_mgn_type.into());
        self
    }

    /// Set the trade quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// Request body for adjusting position margin.
#[derive(Debug, Clone, Serialize)]
pub struct AdjustMarginRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "posSide")]
    pos_side: PositionSide,
    #[serde(rename = "type")]
    action: String,
    amt: String,
    #[serde(rename = "loanTrans", skip_serializing_if = "Option::is_none")]
    loan_trans: Option<bool>,
}

impl AdjustMarginRequest {
    /// Create a margin-adjustment request.
    pub fn new(
        inst_id: impl Into<String>,
        pos_side: PositionSide,
        action: impl Into<String>,
        amt: impl Into<String>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            pos_side,
            action: action.into(),
            amt: amt.into(),
            loan_trans: None,
        }
    }

    /// Set whether to allow loan transfer.
    pub fn loan_transfer(mut self, loan_trans: bool) -> Self {
        self.loan_trans = Some(loan_trans);
        self
    }
}

/// Query parameters for trade fee rates.
#[derive(Debug, Clone, Serialize)]
pub struct FeeRatesRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl FeeRatesRequest {
    /// Create a fee-rates query.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            inst_id: None,
            underlying: None,
            category: None,
            inst_family: None,
        }
    }

    /// Set the instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the underlying.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the fee category.
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set the instrument family.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for account instruments.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AccountInstrumentsRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
}

impl AccountInstrumentsRequest {
    /// Create an empty account-instruments query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }
}

/// Query parameters for maximum loan.
#[derive(Debug, Clone, Serialize)]
pub struct MaxLoanRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "mgnMode")]
    mgn_mode: TradeMode,
    #[serde(rename = "mgnCcy", skip_serializing_if = "Option::is_none")]
    mgn_ccy: Option<String>,
    #[serde(rename = "tradeQuoteCcy", skip_serializing_if = "Option::is_none")]
    trade_quote_ccy: Option<String>,
}

impl MaxLoanRequest {
    /// Create a maximum-loan query.
    pub fn new(inst_id: impl Into<String>, mgn_mode: TradeMode) -> Self {
        Self {
            inst_id: inst_id.into(),
            mgn_mode,
            mgn_ccy: None,
            trade_quote_ccy: None,
        }
    }

    /// Set the margin currency.
    pub fn margin_currency(mut self, mgn_ccy: impl Into<String>) -> Self {
        self.mgn_ccy = Some(mgn_ccy.into());
        self
    }

    /// Set the trade quote currency.
    pub fn trade_quote_currency(mut self, trade_quote_ccy: impl Into<String>) -> Self {
        self.trade_quote_ccy = Some(trade_quote_ccy.into());
        self
    }
}

/// Query parameters for interest-accrued records.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestAccruedRequest {
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl InterestAccruedRequest {
    /// Create an empty interest-accrued query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Serialize)]
struct SetGreeksBody<'a> {
    #[serde(rename = "greeksType")]
    greeks_type: &'a str,
}

#[derive(Serialize)]
struct SetIsolatedModeBody<'a> {
    #[serde(rename = "isoMode")]
    iso_mode: &'a str,
    #[serde(rename = "type")]
    mode_type: &'a str,
}

/// Request body for borrow/repay.
#[derive(Debug, Clone, Serialize)]
pub struct BorrowRepayRequest {
    ccy: String,
    side: String,
    amt: String,
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    ord_id: Option<String>,
}

impl BorrowRepayRequest {
    /// Create a borrow/repay request.
    pub fn new(ccy: impl Into<String>, side: impl Into<String>, amt: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            side: side.into(),
            amt: amt.into(),
            ord_id: None,
        }
    }

    /// Set the related order ID.
    pub fn order_id(mut self, ord_id: impl Into<String>) -> Self {
        self.ord_id = Some(ord_id.into());
        self
    }
}

/// Query parameters for borrow/repay history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BorrowRepayHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl BorrowRepayHistoryRequest {
    /// Create an empty borrow/repay-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for interest limits.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InterestLimitsRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    limit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
}

impl InterestLimitsRequest {
    /// Create an empty interest-limits query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the OKX interest-limit type.
    pub fn limit_type(mut self, limit_type: impl Into<String>) -> Self {
        self.limit_type = Some(limit_type.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// A simulated position used by position-builder and simulated-margin requests.
#[derive(Debug, Clone, Serialize)]
pub struct SimulatedPosition {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos: Option<String>,
    #[serde(rename = "avgPx", skip_serializing_if = "Option::is_none")]
    avg_px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lever: Option<String>,
}

impl SimulatedPosition {
    /// Create a simulated position for an instrument.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            pos: None,
            avg_px: None,
            lever: None,
        }
    }

    /// Set the simulated position size.
    pub fn position(mut self, pos: impl Into<String>) -> Self {
        self.pos = Some(pos.into());
        self
    }

    /// Set the simulated average price.
    pub fn average_price(mut self, avg_px: impl Into<String>) -> Self {
        self.avg_px = Some(avg_px.into());
        self
    }

    /// Set the simulated leverage.
    pub fn leverage(mut self, lever: impl Into<String>) -> Self {
        self.lever = Some(lever.into());
        self
    }
}

/// A simulated asset used by position-builder requests.
#[derive(Debug, Clone, Serialize)]
pub struct SimulatedAsset {
    ccy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<String>,
}

impl SimulatedAsset {
    /// Create a simulated asset for a currency.
    pub fn new(ccy: impl Into<String>) -> Self {
        Self {
            ccy: ccy.into(),
            eq: None,
        }
    }

    /// Set the simulated equity.
    pub fn equity(mut self, eq: impl Into<String>) -> Self {
        self.eq = Some(eq.into());
        self
    }
}

/// Request body for simulated margin calculation.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SimulatedMarginRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "inclRealPos", skip_serializing_if = "Option::is_none")]
    include_real_positions: Option<bool>,
    #[serde(rename = "spotOffsetType", skip_serializing_if = "Option::is_none")]
    spot_offset_type: Option<String>,
    #[serde(rename = "simPos", skip_serializing_if = "Option::is_none")]
    simulated_positions: Option<Vec<SimulatedPosition>>,
}

impl SimulatedMarginRequest {
    /// Create an empty simulated-margin request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set whether real positions and equity are included.
    pub fn include_real_positions(mut self, include_real_positions: bool) -> Self {
        self.include_real_positions = Some(include_real_positions);
        self
    }

    /// Set the spot offset type.
    pub fn spot_offset_type(mut self, spot_offset_type: impl Into<String>) -> Self {
        self.spot_offset_type = Some(spot_offset_type.into());
        self
    }

    /// Set simulated positions.
    pub fn simulated_positions(mut self, simulated_positions: Vec<SimulatedPosition>) -> Self {
        self.simulated_positions = Some(simulated_positions);
        self
    }
}

/// Query parameters for account position tiers.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AccountPositionTiersRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl AccountPositionTiersRequest {
    /// Create an empty account position-tiers query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

#[derive(Serialize)]
struct TypeBody<'a> {
    #[serde(rename = "type")]
    value: &'a str,
}

#[derive(Serialize)]
struct SetAutoLoanBody {
    #[serde(rename = "autoLoan")]
    auto_loan: bool,
}

#[derive(Serialize)]
struct SetAccountLevelBody<'a> {
    #[serde(rename = "acctLv")]
    acct_lv: &'a str,
}

/// Request body for position builder.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PositionBuilderRequest {
    #[serde(rename = "acctLv", skip_serializing_if = "Option::is_none")]
    acct_lv: Option<String>,
    #[serde(rename = "inclRealPosAndEq", skip_serializing_if = "Option::is_none")]
    include_real_positions_and_equity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lever: Option<String>,
    #[serde(rename = "greeksType", skip_serializing_if = "Option::is_none")]
    greeks_type: Option<String>,
    #[serde(rename = "simPos", skip_serializing_if = "Option::is_none")]
    simulated_positions: Option<Vec<SimulatedPosition>>,
    #[serde(rename = "simAsset", skip_serializing_if = "Option::is_none")]
    simulated_assets: Option<Vec<SimulatedAsset>>,
    #[serde(rename = "idxVol", skip_serializing_if = "Option::is_none")]
    index_volatility: Option<String>,
}

impl PositionBuilderRequest {
    /// Create an empty position-builder request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account level.
    pub fn account_level(mut self, acct_lv: impl Into<String>) -> Self {
        self.acct_lv = Some(acct_lv.into());
        self
    }

    /// Set whether real positions and equity are included.
    pub fn include_real_positions_and_equity(mut self, include: bool) -> Self {
        self.include_real_positions_and_equity = Some(include);
        self
    }

    /// Set leverage.
    pub fn leverage(mut self, lever: impl Into<String>) -> Self {
        self.lever = Some(lever.into());
        self
    }

    /// Set greeks display type.
    pub fn greeks_type(mut self, greeks_type: impl Into<String>) -> Self {
        self.greeks_type = Some(greeks_type.into());
        self
    }

    /// Set simulated positions.
    pub fn simulated_positions(mut self, simulated_positions: Vec<SimulatedPosition>) -> Self {
        self.simulated_positions = Some(simulated_positions);
        self
    }

    /// Set simulated assets.
    pub fn simulated_assets(mut self, simulated_assets: Vec<SimulatedAsset>) -> Self {
        self.simulated_assets = Some(simulated_assets);
        self
    }

    /// Set index volatility.
    pub fn index_volatility(mut self, index_volatility: impl Into<String>) -> Self {
        self.index_volatility = Some(index_volatility.into());
        self
    }
}

/// Query parameters for position history.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PositionsHistoryRequest {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "mgnMode", skip_serializing_if = "Option::is_none")]
    mgn_mode: Option<TradeMode>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    close_type: Option<String>,
    #[serde(rename = "posId", skip_serializing_if = "Option::is_none")]
    pos_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl PositionsHistoryRequest {
    /// Create an empty position-history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the instrument type filter.
    pub fn inst_type(mut self, inst_type: InstType) -> Self {
        self.inst_type = Some(inst_type);
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the margin mode filter.
    pub fn margin_mode(mut self, mgn_mode: TradeMode) -> Self {
        self.mgn_mode = Some(mgn_mode);
        self
    }

    /// Set the OKX close type filter.
    pub fn close_type(mut self, close_type: impl Into<String>) -> Self {
        self.close_type = Some(close_type.into());
        self
    }

    /// Set the position ID filter.
    pub fn position_id(mut self, pos_id: impl Into<String>) -> Self {
        self.pos_id = Some(pos_id.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// The trading-account balance summary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBalance {
    /// Total equity in USD.
    #[serde(default)]
    pub total_eq: NumberString,
    /// Adjusted / effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Per-currency balance details.
    #[serde(default)]
    pub details: Vec<BalanceDetail>,
    /// Last update time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Balance details for a single currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceDetail {
    /// Currency, e.g. `USDT`.
    pub ccy: String,
    /// Equity of the currency.
    #[serde(default)]
    pub eq: NumberString,
    /// Cash balance.
    #[serde(default)]
    pub cash_bal: NumberString,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Frozen balance.
    #[serde(default)]
    pub frozen_bal: NumberString,
}

/// An open position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Position {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Position side.
    pub pos_side: PositionSide,
    /// Margin mode.
    pub mgn_mode: TradeMode,
    /// Quantity of positions.
    #[serde(default)]
    pub pos: NumberString,
    /// Average open price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: NumberString,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: NumberString,
}

/// Account position-risk snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionRisk {
    /// Adjusted/effective equity in USD.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Balance data included in the risk snapshot.
    #[serde(default)]
    pub bal_data: Vec<BalanceDetail>,
    /// Position data included in the risk snapshot.
    #[serde(default)]
    pub pos_data: Vec<Position>,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Account configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountConfig {
    /// Account ID.
    #[serde(default)]
    pub uid: String,
    /// Account level.
    #[serde(default)]
    pub acct_lv: String,
    /// Position mode.
    #[serde(default)]
    pub pos_mode: String,
    /// Greeks display type.
    #[serde(default)]
    pub greeks_type: String,
    /// Whether auto-borrow is enabled. OKX returns this as a JSON boolean.
    #[serde(default)]
    pub auto_loan: bool,
}

/// Account bill row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBill {
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Bill type.
    #[serde(rename = "type", default)]
    pub bill_type: String,
    /// Bill subtype.
    #[serde(default)]
    pub sub_type: String,
    /// Balance change.
    #[serde(default)]
    pub sz: NumberString,
    /// Balance after the change.
    #[serde(default)]
    pub bal: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of setting position mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetPositionModeResult {
    /// Position mode.
    #[serde(default)]
    pub pos_mode: String,
}

/// Leverage information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeverageInfo {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode.
    pub mgn_mode: TradeMode,
    /// Position side.
    pub pos_side: PositionSide,
    /// Leverage.
    #[serde(default)]
    pub lever: NumberString,
}

/// Maximum order size information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxOrderSize {
    /// Instrument ID.
    pub inst_id: String,
    /// Maximum buy size.
    #[serde(default)]
    pub max_buy: NumberString,
    /// Maximum sell size.
    #[serde(default)]
    pub max_sell: NumberString,
}

/// Maximum available size information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxAvailableSize {
    /// Instrument ID.
    pub inst_id: String,
    /// Available buy size.
    #[serde(default)]
    pub avail_buy: NumberString,
    /// Available sell size.
    #[serde(default)]
    pub avail_sell: NumberString,
}

/// Trade fee-rate information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FeeRate {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Fee category.
    #[serde(default)]
    pub category: String,
    /// Maker fee rate.
    #[serde(default)]
    pub maker: NumberString,
    /// Taker fee rate.
    #[serde(default)]
    pub taker: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Maximum withdrawal amount for a currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxWithdrawal {
    /// Currency.
    pub ccy: String,
    /// Maximum withdrawal amount.
    #[serde(default)]
    pub max_wd: NumberString,
    /// Maximum withdrawal amount excluding borrowed amount.
    #[serde(default)]
    pub max_wd_ex: NumberString,
}

/// Historical position row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionHistory {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Margin mode.
    pub mgn_mode: TradeMode,
    /// Close type.
    #[serde(rename = "type", default)]
    pub close_type: String,
    /// Realized PnL.
    #[serde(default)]
    pub realized_pnl: NumberString,
    /// Created time (Unix milliseconds).
    #[serde(default)]
    pub c_time: NumberString,
    /// Updated time (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
}

/// Account risk state.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RiskState {
    /// Whether the account is currently at risk, as represented by OKX.
    #[serde(default)]
    pub at_risk: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Result of adding or reducing margin on a position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AdjustMarginResult {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position side.
    #[serde(default)]
    pub pos_side: String,
    /// Adjustment amount.
    #[serde(default)]
    pub amt: NumberString,
    /// OKX adjustment type.
    #[serde(rename = "type", default)]
    pub adjustment_type: String,
}

/// Account-level instrument configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountInstrument {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Base currency.
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency.
    #[serde(default)]
    pub quote_ccy: String,
    /// Settlement currency.
    #[serde(default)]
    pub settle_ccy: String,
}

/// Maximum loan amount available for an instrument or currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxLoan {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Margin currency.
    #[serde(default)]
    pub mgn_ccy: String,
    /// Maximum loan amount.
    #[serde(default)]
    pub max_loan: NumberString,
}

/// Interest accrued by account borrowing.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestAccrued {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Accrued interest.
    #[serde(default)]
    pub interest: NumberString,
    /// Interest rate.
    #[serde(default)]
    pub interest_rate: NumberString,
    /// Liability.
    #[serde(default)]
    pub liab: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Account borrowing interest rate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestRate {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Interest rate.
    #[serde(default)]
    pub interest_rate: NumberString,
}

/// Result of updating the greeks display type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetGreeksResult {
    /// Greeks display type.
    #[serde(default)]
    pub greeks_type: String,
}

/// Result of updating isolated margin mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetIsolatedModeResult {
    /// Isolated margin mode.
    #[serde(default)]
    pub iso_mode: String,
    /// OKX isolated-mode scope type.
    #[serde(rename = "type", default)]
    pub mode_type: String,
}

/// Result of a borrow/repay request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BorrowRepayResult {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Borrow or repay side.
    #[serde(default)]
    pub side: String,
    /// Requested amount.
    #[serde(default)]
    pub amt: NumberString,
    /// OKX borrow/repay order ID.
    #[serde(default)]
    pub ord_id: String,
}

/// Borrow/repay history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BorrowRepayHistory {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Borrow or repay side.
    #[serde(default)]
    pub side: String,
    /// Amount.
    #[serde(default)]
    pub amt: NumberString,
    /// OKX borrow/repay order ID.
    #[serde(default)]
    pub ord_id: String,
    /// OKX state value.
    #[serde(default)]
    pub state: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Borrowing interest limit information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestLimit {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Interest rate.
    #[serde(default)]
    pub rate: NumberString,
    /// Loan quota.
    #[serde(default)]
    pub loan_quota: NumberString,
    /// Used loan quota.
    #[serde(default)]
    pub used_loan: NumberString,
}

/// Simulated margin calculation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SimulatedMargin {
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Margin ratio.
    #[serde(default)]
    pub mr: NumberString,
    /// Notional value in USD.
    #[serde(default)]
    pub notional_usd: NumberString,
    /// Per-instrument details returned by OKX.
    #[serde(default)]
    pub details: Vec<SimulatedMarginDetail>,
}

/// Per-instrument detail in a simulated margin response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SimulatedMarginDetail {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position size.
    #[serde(default)]
    pub pos: NumberString,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Unrealized PnL.
    #[serde(default)]
    pub upl: NumberString,
}

/// Account greeks row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Greek {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Black-Scholes delta.
    #[serde(rename = "deltaBS", default)]
    pub delta_bs: NumberString,
    /// Portfolio-adjusted delta.
    #[serde(rename = "deltaPA", default)]
    pub delta_pa: NumberString,
    /// Black-Scholes gamma.
    #[serde(rename = "gammaBS", default)]
    pub gamma_bs: NumberString,
    /// Black-Scholes theta.
    #[serde(rename = "thetaBS", default)]
    pub theta_bs: NumberString,
    /// Black-Scholes vega.
    #[serde(rename = "vegaBS", default)]
    pub vega_bs: NumberString,
}

/// Account position-tier row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountPositionTier {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
    /// Instrument family.
    #[serde(default)]
    pub inst_family: String,
    /// Position type.
    #[serde(default)]
    pub pos_type: String,
    /// Minimum size in the tier.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Maximum size in the tier.
    #[serde(default)]
    pub max_sz: NumberString,
}

/// Result of updating risk offset type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetRiskOffsetTypeResult {
    /// OKX risk offset type.
    #[serde(rename = "type", default)]
    pub risk_offset_type: String,
}

/// Result of updating auto loan.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetAutoLoanResult {
    /// Auto-loan setting as returned by OKX.
    #[serde(default)]
    pub auto_loan: String,
}

/// Result of updating account level.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetAccountLevelResult {
    /// Account level.
    #[serde(default)]
    pub acct_lv: String,
}

/// Result of activating option trading.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ActivateOptionResult {
    /// OKX result marker, when returned.
    #[serde(default)]
    pub result: String,
}

/// Position-builder result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderResult {
    /// Account level used for the calculation.
    #[serde(default)]
    pub acct_lv: String,
    /// Adjusted / effective equity.
    #[serde(default)]
    pub adj_eq: NumberString,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: NumberString,
    /// Margin ratio.
    #[serde(default)]
    pub mr: NumberString,
    /// Simulated or real position data.
    #[serde(default)]
    pub pos_data: Vec<PositionBuilderPosition>,
    /// Simulated or real asset data.
    #[serde(default)]
    pub asset_data: Vec<PositionBuilderAsset>,
}

/// Position row returned by position builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderPosition {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Position size.
    #[serde(default)]
    pub pos: NumberString,
    /// Average price.
    #[serde(default)]
    pub avg_px: NumberString,
    /// Unrealized PnL.
    #[serde(default)]
    pub upl: NumberString,
}

/// Asset row returned by position builder.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionBuilderAsset {
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Equity.
    #[serde(default)]
    pub eq: NumberString,
}

#[cfg(test)]
mod tests {
    use crate::test_util::MockTransport;
    use crate::{Credentials, OkxClient};

    fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
        OkxClient::with_transport(mock)
            .credentials(Credentials::new("key", "secret", "pass"))
            .build()
    }

    #[tokio::test]
    async fn get_balance_signs_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"totalEq":"10000","adjEq":"9500","uTime":"1597026383085","details":[
                {"ccy":"USDT","eq":"10000","cashBal":"10000","availBal":"9000","frozenBal":"1000"}]}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let balances = client.account().get_balance(None).await.unwrap();
        assert_eq!(balances[0].total_eq.as_str(), "10000");
        assert_eq!(balances[0].details[0].ccy, "USDT");
        assert_eq!(balances[0].details[0].avail_bal.as_str(), "9000");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::GET);
        assert!(req.uri.ends_with("/api/v5/account/balance"));
        assert!(req.is_signed(), "authenticated endpoint must be signed");
    }

    #[tokio::test]
    async fn get_positions_passes_filters() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","posId":"1","posSide":"long",
             "mgnMode":"cross","pos":"1","avgPx":"42000","upl":"5","lever":"10","liqPx":"38000"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let positions = client
            .account()
            .get_positions(Some(crate::model::InstType::Swap), Some("BTC-USDT-SWAP"))
            .await
            .unwrap();
        assert_eq!(positions[0].inst_id, "BTC-USDT-SWAP");
        assert_eq!(positions[0].pos_side, crate::model::PositionSide::Long);

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn missing_credentials_is_configuration_error() {
        let mock = MockTransport::new("{}");
        let client = OkxClient::with_transport(mock).build();
        let err = client.account().get_balance(None).await.unwrap_err();
        assert!(matches!(err, crate::Error::Configuration(_)));
    }

    #[tokio::test]
    async fn get_position_risk_signs_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"adjEq":"1000","ts":"1597026383085","balData":[{"ccy":"USDT","eq":"1000"}],
             "posData":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","posSide":"long","mgnMode":"cross","pos":"1"}]}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let risk = client
            .account()
            .get_position_risk(Some(crate::model::InstType::Swap))
            .await
            .unwrap();
        assert_eq!(risk[0].adj_eq.as_str(), "1000");
        assert_eq!(risk[0].pos_data[0].inst_id, "BTC-USDT-SWAP");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_account_config_signs_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"uid":"1","acctLv":"2","posMode":"net_mode","greeksType":"PA","autoLoan":false}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let config = client.account().get_account_config().await.unwrap();
        assert_eq!(config[0].pos_mode, "net_mode");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/account/config"));
        assert_eq!(req.query(), None);
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_account_bills_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"billId":"1","instType":"SPOT","ccy":"USDT","mgnMode":"cash","type":"1","subType":"2",
             "sz":"10","bal":"100","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::BillsRequest::new()
            .inst_type(crate::model::InstType::Spot)
            .currency("USDT")
            .bill_type("1")
            .limit(1);

        let bills = client.account().get_account_bills(&request).await.unwrap();
        assert_eq!(bills[0].bill_id, "1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&ccy=USDT&type=1&limit=1"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_account_bills_archive_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"billId":"2","instType":"SPOT","ccy":"USDT","type":"1","sz":"10","bal":"100","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::BillsArchiveRequest::new()
            .filters(super::BillsRequest::new().currency("USDT"))
            .begin("100")
            .end("200");

        let bills = client
            .account()
            .get_account_bills_archive(&request)
            .await
            .unwrap();
        assert_eq!(bills[0].bill_id, "2");

        let req = mock.captured();
        assert_eq!(req.query(), Some("ccy=USDT&begin=100&end=200"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_position_mode_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[{"posMode":"net_mode"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client
            .account()
            .set_position_mode("net_mode")
            .await
            .unwrap();
        assert_eq!(result[0].pos_mode, "net_mode");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/set-position-mode"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["posMode"], "net_mode");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_leverage_posts_builder_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","mgnMode":"cross","posSide":"long","lever":"10"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::SetLeverageRequest::new("10", crate::model::TradeMode::Cross)
            .inst_id("BTC-USDT-SWAP")
            .position_side(crate::model::PositionSide::Long);

        let result = client.account().set_leverage(&request).await.unwrap();
        assert_eq!(result[0].lever.as_str(), "10");

        let req = mock.captured();
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["lever"], "10");
        assert_eq!(sent["mgnMode"], "cross");
        assert_eq!(sent["instId"], "BTC-USDT-SWAP");
        assert_eq!(sent["posSide"], "long");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_leverage_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","mgnMode":"cross","posSide":"long","lever":"10"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request =
            super::LeverageRequest::new(crate::model::TradeMode::Cross).inst_id("BTC-USDT-SWAP");

        let result = client.account().get_leverage(&request).await.unwrap();
        assert_eq!(result[0].mgn_mode, crate::model::TradeMode::Cross);

        let req = mock.captured();
        assert_eq!(req.query(), Some("mgnMode=cross&instId=BTC-USDT-SWAP"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_max_order_size_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","maxBuy":"1","maxSell":"2"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::MaxOrderSizeRequest::new("BTC-USDT", crate::model::TradeMode::Cash)
            .price("42000");

        let result = client.account().get_max_order_size(&request).await.unwrap();
        assert_eq!(result[0].max_buy.as_str(), "1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT&tdMode=cash&px=42000"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_max_avail_size_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","availBuy":"1","availSell":"2"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request =
            super::MaxAvailableSizeRequest::new("BTC-USDT", crate::model::TradeMode::Cash)
                .reduce_only(false);

        let result = client.account().get_max_avail_size(&request).await.unwrap();
        assert_eq!(result[0].avail_sell.as_str(), "2");

        let req = mock.captured();
        assert_eq!(
            req.query(),
            Some("instId=BTC-USDT&tdMode=cash&reduceOnly=false")
        );
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_fee_rates_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","category":"1","maker":"-0.0001","taker":"0.001","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::FeeRatesRequest::new(crate::model::InstType::Spot).inst_id("BTC-USDT");

        let result = client.account().get_fee_rates(&request).await.unwrap();
        assert_eq!(result[0].maker.as_str(), "-0.0001");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SPOT&instId=BTC-USDT"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_max_withdrawal_queries_currency() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ccy":"USDT","maxWd":"100","maxWdEx":"90"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client
            .account()
            .get_max_withdrawal(Some("USDT"))
            .await
            .unwrap();
        assert_eq!(result[0].max_wd.as_str(), "100");

        let req = mock.captured();
        assert_eq!(req.query(), Some("ccy=USDT"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_positions_history_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","posId":"1","mgnMode":"cross",
             "type":"2","realizedPnl":"5","cTime":"1597026383085","uTime":"1597026383999"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::PositionsHistoryRequest::new()
            .inst_type(crate::model::InstType::Swap)
            .inst_id("BTC-USDT-SWAP")
            .limit(1);

        let result = client
            .account()
            .get_positions_history(&request)
            .await
            .unwrap();
        assert_eq!(result[0].realized_pnl.as_str(), "5");

        let req = mock.captured();
        assert_eq!(
            req.query(),
            Some("instType=SWAP&instId=BTC-USDT-SWAP&limit=1")
        );
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_risk_state_signs_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[{"atRisk":"false","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().get_risk_state().await.unwrap();
        assert_eq!(result[0].at_risk, "false");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/account/risk-state"));
        assert_eq!(req.query(), None);
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn demo_trading_sets_simulated_header() {
        let body = r#"{"code":"0","msg":"","data":[
            {"uid":"1","acctLv":"2","posMode":"net_mode","greeksType":"PA","autoLoan":false}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone())
            .credentials(Credentials::new("key", "secret", "pass"))
            .demo_trading(true)
            .build();

        let config = client.account().get_account_config().await.unwrap();
        assert_eq!(config[0].acct_lv, "2");

        let req = mock.captured();
        assert_eq!(
            req.headers
                .get("x-simulated-trading")
                .and_then(|v| v.to_str().ok()),
            Some("1")
        );
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn adjust_margin_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","posSide":"long","type":"add","amt":"100"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::AdjustMarginRequest::new(
            "BTC-USDT-SWAP",
            crate::model::PositionSide::Long,
            "add",
            "100",
        )
        .loan_transfer(true);

        let result = client.account().adjust_margin(&request).await.unwrap();
        assert_eq!(result[0].amt.as_str(), "100");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/position/margin-balance"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["instId"], "BTC-USDT-SWAP");
        assert_eq!(sent["posSide"], "long");
        assert_eq!(sent["type"], "add");
        assert_eq!(sent["amt"], "100");
        assert_eq!(sent["loanTrans"], true);
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_account_instruments_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","uly":"BTC-USDT","instFamily":"BTC-USDT",
             "baseCcy":"BTC","quoteCcy":"USDT","settleCcy":"USDT"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::AccountInstrumentsRequest::new()
            .inst_type(crate::model::InstType::Swap)
            .inst_id("BTC-USDT-SWAP");

        let result = client
            .account()
            .get_account_instruments(&request)
            .await
            .unwrap();
        assert_eq!(result[0].settle_ccy, "USDT");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_max_loan_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","mgnMode":"cross","mgnCcy":"USDT","maxLoan":"1000"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::MaxLoanRequest::new("BTC-USDT", crate::model::TradeMode::Cross)
            .margin_currency("USDT");

        let result = client.account().get_max_loan(&request).await.unwrap();
        assert_eq!(result[0].max_loan.as_str(), "1000");

        let req = mock.captured();
        assert_eq!(
            req.query(),
            Some("instId=BTC-USDT&mgnMode=cross&mgnCcy=USDT")
        );
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_interest_accrued_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT","ccy":"USDT","mgnMode":"cross","interest":"1",
             "interestRate":"0.0001","liab":"100","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::InterestAccruedRequest::new()
            .inst_id("BTC-USDT")
            .currency("USDT")
            .limit(1);

        let result = client
            .account()
            .get_interest_accrued(&request)
            .await
            .unwrap();
        assert_eq!(result[0].interest_rate.as_str(), "0.0001");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT&ccy=USDT&limit=1"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_interest_rate_queries_currency() {
        let body = r#"{"code":"0","msg":"","data":[{"ccy":"USDT","interestRate":"0.0001"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client
            .account()
            .get_interest_rate(Some("USDT"))
            .await
            .unwrap();
        assert_eq!(result[0].ccy, "USDT");

        let req = mock.captured();
        assert_eq!(req.query(), Some("ccy=USDT"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_greeks_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[{"greeksType":"PA"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().set_greeks("PA").await.unwrap();
        assert_eq!(result[0].greeks_type, "PA");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/set-greeks"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["greeksType"], "PA");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_isolated_mode_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[{"isoMode":"automatic","type":"MARGIN"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client
            .account()
            .set_isolated_mode("automatic", "MARGIN")
            .await
            .unwrap();
        assert_eq!(result[0].iso_mode, "automatic");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/set-isolated-mode"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["isoMode"], "automatic");
        assert_eq!(sent["type"], "MARGIN");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn borrow_repay_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ccy":"USDT","side":"borrow","amt":"100","ordId":"1"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::BorrowRepayRequest::new("USDT", "borrow", "100").order_id("1");

        let result = client.account().borrow_repay(&request).await.unwrap();
        assert_eq!(result[0].ord_id, "1");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/borrow-repay"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["ccy"], "USDT");
        assert_eq!(sent["side"], "borrow");
        assert_eq!(sent["amt"], "100");
        assert_eq!(sent["ordId"], "1");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_borrow_repay_history_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ccy":"USDT","side":"borrow","amt":"100","ordId":"1","state":"2","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::BorrowRepayHistoryRequest::new()
            .currency("USDT")
            .limit(1);

        let result = client
            .account()
            .get_borrow_repay_history(&request)
            .await
            .unwrap();
        assert_eq!(result[0].state, "2");

        let req = mock.captured();
        assert_eq!(req.query(), Some("ccy=USDT&limit=1"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_interest_limits_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ccy":"USDT","rate":"0.0001","loanQuota":"1000","usedLoan":"100"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::InterestLimitsRequest::new()
            .limit_type("1")
            .currency("USDT");

        let result = client
            .account()
            .get_interest_limits(&request)
            .await
            .unwrap();
        assert_eq!(result[0].loan_quota.as_str(), "1000");

        let req = mock.captured();
        assert_eq!(req.query(), Some("type=1&ccy=USDT"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_simulated_margin_posts_body_and_omits_unset_fields() {
        let body = r#"{"code":"0","msg":"","data":[
            {"imr":"10","mmr":"5","mr":"100","notionalUsd":"1000",
             "details":[{"instId":"BTC-USDT-SWAP","pos":"1","imr":"10","mmr":"5","upl":"2"}]}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::SimulatedMarginRequest::new()
            .inst_type(crate::model::InstType::Swap)
            .simulated_positions(vec![
                super::SimulatedPosition::new("BTC-USDT-SWAP")
                    .position("1")
                    .leverage("10"),
            ]);

        let result = client
            .account()
            .get_simulated_margin(&request)
            .await
            .unwrap();
        assert_eq!(result[0].details[0].upl.as_str(), "2");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/simulated_margin"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["instType"], "SWAP");
        assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
        assert_eq!(sent["simPos"][0]["pos"], "1");
        assert_eq!(sent["simPos"][0]["lever"], "10");
        assert!(sent.get("inclRealPos").is_none());
        assert!(sent["simPos"][0].get("avgPx").is_none());
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_greeks_queries_currency() {
        let body = r#"{"code":"0","msg":"","data":[
            {"ccy":"BTC","deltaBS":"1","deltaPA":"0.9","gammaBS":"0.1","thetaBS":"-0.01","vegaBS":"2"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().get_greeks(Some("BTC")).await.unwrap();
        assert_eq!(result[0].delta_pa.as_str(), "0.9");

        let req = mock.captured();
        assert_eq!(req.query(), Some("ccy=BTC"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn get_account_position_tiers_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"OPTION","uly":"BTC-USD","instFamily":"BTC-USD","posType":"1","minSz":"0","maxSz":"100"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::AccountPositionTiersRequest::new()
            .inst_type(crate::model::InstType::Option)
            .underlying("BTC-USD");

        let result = client
            .account()
            .get_account_position_tiers(&request)
            .await
            .unwrap();
        assert_eq!(result[0].max_sz.as_str(), "100");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=OPTION&uly=BTC-USD"));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_risk_offset_type_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[{"type":"1"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().set_risk_offset_type("1").await.unwrap();
        assert_eq!(result[0].risk_offset_type, "1");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/set-riskOffset-type"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["type"], "1");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_auto_loan_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[{"autoLoan":"true"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().set_auto_loan(true).await.unwrap();
        assert_eq!(result[0].auto_loan, "true");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/set-auto-loan"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["autoLoan"], true);
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn set_account_level_posts_body() {
        let body = r#"{"code":"0","msg":"","data":[{"acctLv":"2"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().set_account_level("2").await.unwrap();
        assert_eq!(result[0].acct_lv, "2");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/set-account-level"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["acctLv"], "2");
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn activate_option_posts_empty_body() {
        let body = r#"{"code":"0","msg":"","data":[{"result":"true"}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());

        let result = client.account().activate_option().await.unwrap();
        assert_eq!(result[0].result, "true");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/activate-option"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent, serde_json::json!({}));
        assert!(req.is_signed());
    }

    #[tokio::test]
    async fn position_builder_posts_body_and_omits_unset_fields() {
        let body = r#"{"code":"0","msg":"","data":[
            {"acctLv":"2","adjEq":"1000","imr":"10","mmr":"5","mr":"100",
             "posData":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","pos":"1","avgPx":"42000","upl":"2"}],
             "assetData":[{"ccy":"USDT","eq":"1000"}]}]}"#;
        let mock = MockTransport::new(body);
        let client = signed_client(mock.clone());
        let request = super::PositionBuilderRequest::new()
            .account_level("2")
            .include_real_positions_and_equity(false)
            .simulated_positions(vec![
                super::SimulatedPosition::new("BTC-USDT-SWAP").position("1"),
            ])
            .simulated_assets(vec![super::SimulatedAsset::new("USDT").equity("1000")]);

        let result = client.account().position_builder(&request).await.unwrap();
        assert_eq!(result[0].pos_data[0].inst_id, "BTC-USDT-SWAP");
        assert_eq!(result[0].asset_data[0].eq.as_str(), "1000");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::POST);
        assert!(req.uri.ends_with("/api/v5/account/position-builder"));
        let sent: serde_json::Value = serde_json::from_str(req.body_str()).unwrap();
        assert_eq!(sent["acctLv"], "2");
        assert_eq!(sent["inclRealPosAndEq"], false);
        assert_eq!(sent["simPos"][0]["instId"], "BTC-USDT-SWAP");
        assert_eq!(sent["simAsset"][0]["ccy"], "USDT");
        assert!(sent.get("lever").is_none());
        assert!(sent["simPos"][0].get("avgPx").is_none());
        assert!(req.is_signed());
    }
}
