use serde::Serialize;

use crate::model::{InstType, PositionSide, RequestParams, TradeMode};

/// Query parameters for VIP interest-accrued records.
pub type VipInterestAccruedRequest = RequestParams;

/// Query parameters for VIP interest-deducted records.
pub type VipInterestDeductedRequest = RequestParams;

/// Query parameters for VIP-loan order list.
pub type VipLoanOrderListRequest = RequestParams;

/// Query parameters for VIP-loan order detail.
pub type VipLoanOrderDetailRequest = RequestParams;

/// Query parameters for fixed-loan borrowing limits.
pub type FixedLoanBorrowingLimitRequest = RequestParams;

/// Request body for a fixed-loan borrowing quote.
pub type FixedLoanBorrowingQuoteRequest = RequestParams;

/// Request body for placing a fixed-loan borrowing order.
pub type FixedLoanBorrowingOrderRequest = RequestParams;

/// Request body for amending a fixed-loan borrowing order.
pub type FixedLoanAmendBorrowingOrderRequest = RequestParams;

/// Request body for manually reborrowing a fixed-loan order.
pub type FixedLoanManualReborrowRequest = RequestParams;

/// Request body for repaying a fixed-loan borrowing order.
pub type FixedLoanRepayBorrowingOrderRequest = RequestParams;

/// Query parameters for fixed-loan borrowing order history/list.
pub type FixedLoanBorrowingOrdersListRequest = RequestParams;

/// Request body for spot manual borrow/repay.
pub type SpotManualBorrowRepayRequest = RequestParams;

/// Request body for setting spot auto repay.
pub type SetAutoRepayRequest = RequestParams;

/// Query parameters for spot borrow/repay history.
pub type SpotBorrowRepayHistoryRequest = RequestParams;

/// Request body for setting auto earn.
pub type SetAutoEarnRequest = RequestParams;

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
