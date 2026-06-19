use serde::Serialize;

use crate::model::{InstType, PositionSide, TradeMode};

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
#[derive(Debug, Clone, Serialize)]
pub struct AccountInstrumentsRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "seriesId", skip_serializing_if = "Option::is_none")]
    series_id: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
}

impl AccountInstrumentsRequest {
    /// Create an empty account-instruments query.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            series_id: None,
            inst_family: None,
            inst_id: None,
        }
    }

    /// Set the series_id filter.
    pub fn series_id(mut self, series_id: impl Into<String>) -> Self {
        self.series_id = Some(series_id.into());
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
