use std::borrow::Cow;

use serde::Serialize;

use crate::model::{OrderSide, PositionSide, TradeMode};

/// Source-side details for one position-transfer leg.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionFrom<'a> {
    pos_id: Cow<'a, str>,
    side: OrderSide,
    sz: Cow<'a, str>,
}

impl<'a> MovePositionFrom<'a> {
    /// Create the source side of a position-transfer leg.
    pub fn new(
        pos_id: impl Into<Cow<'a, str>>,
        side: OrderSide,
        sz: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            pos_id: pos_id.into(),
            side,
            sz: sz.into(),
        }
    }
}

/// Destination-side settings for one position-transfer leg.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionTo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    td_mode: Option<TradeMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos_side: Option<PositionSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<Cow<'a, str>>,
}

impl<'a> MovePositionTo<'a> {
    /// Create destination settings with OKX defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the destination trading mode.
    pub fn trade_mode(mut self, td_mode: TradeMode) -> Self {
        self.td_mode = Some(td_mode);
        self
    }

    /// Set the destination position side.
    pub fn position_side(mut self, pos_side: PositionSide) -> Self {
        self.pos_side = Some(pos_side);
        self
    }

    /// Set the destination margin currency.
    pub fn currency(mut self, ccy: impl Into<Cow<'a, str>>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }
}

/// One leg in a position-transfer request.
#[derive(Debug, Clone, Serialize)]
pub struct MovePositionLeg<'a> {
    from: MovePositionFrom<'a>,
    to: MovePositionTo<'a>,
}

impl<'a> MovePositionLeg<'a> {
    /// Create a position-transfer leg.
    pub fn new(from: MovePositionFrom<'a>, to: MovePositionTo<'a>) -> Self {
        Self { from, to }
    }
}

/// Request body for moving positions between master and sub-accounts.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsRequest<'a> {
    from_acct: Cow<'a, str>,
    to_acct: Cow<'a, str>,
    legs: Vec<MovePositionLeg<'a>>,
    client_id: Cow<'a, str>,
}

impl<'a> MovePositionsRequest<'a> {
    /// Create a position-transfer request.
    pub fn new(
        from_acct: impl Into<Cow<'a, str>>,
        to_acct: impl Into<Cow<'a, str>>,
        legs: Vec<MovePositionLeg<'a>>,
        client_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            from_acct: from_acct.into(),
            to_acct: to_acct.into(),
            legs,
            client_id: client_id.into(),
        }
    }
}

/// Query parameters for retrieving position-transfer history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsHistoryRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_td_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    begin_ts: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_ts: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<Cow<'a, str>>,
}

impl<'a> MovePositionsHistoryRequest<'a> {
    /// Create an unfiltered position-transfer history query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by an OKX-generated block trade ID.
    pub fn block_trade_id(mut self, block_td_id: impl Into<Cow<'a, str>>) -> Self {
        self.block_td_id = Some(block_td_id.into());
        self
    }

    /// Filter by a client-supplied transfer ID.
    pub fn client_id(mut self, client_id: impl Into<Cow<'a, str>>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Set the inclusive beginning timestamp in Unix milliseconds.
    pub fn begin_timestamp(mut self, begin_ts: impl Into<Cow<'a, str>>) -> Self {
        self.begin_ts = Some(begin_ts.into());
        self
    }

    /// Set the inclusive ending timestamp in Unix milliseconds.
    pub fn end_timestamp(mut self, end_ts: impl Into<Cow<'a, str>>) -> Self {
        self.end_ts = Some(end_ts.into());
        self
    }

    /// Set the maximum number of rows to return. OKX allows up to 100.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter by transfer state, such as `filled` or `pending`.
    pub fn state(mut self, state: impl Into<Cow<'a, str>>) -> Self {
        self.state = Some(state.into());
        self
    }
}

/// A simulated position for a position-builder trend graph.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionBuilderGraphPosition<'a> {
    inst_id: Cow<'a, str>,
    pos: Cow<'a, str>,
    avg_px: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lever: Option<Cow<'a, str>>,
}

impl<'a> PositionBuilderGraphPosition<'a> {
    /// Create a simulated graph position with all documented required fields.
    pub fn new(
        inst_id: impl Into<Cow<'a, str>>,
        pos: impl Into<Cow<'a, str>>,
        avg_px: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            inst_id: inst_id.into(),
            pos: pos.into(),
            avg_px: avg_px.into(),
            lever: None,
        }
    }

    /// Set leverage for a multi-currency margin simulation.
    pub fn leverage(mut self, lever: impl Into<Cow<'a, str>>) -> Self {
        self.lever = Some(lever.into());
        self
    }
}

/// A simulated asset for a position-builder trend graph.
#[derive(Debug, Clone, Serialize)]
pub struct PositionBuilderGraphAsset<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
}

impl<'a> PositionBuilderGraphAsset<'a> {
    /// Create a simulated asset with its currency amount.
    pub fn new(ccy: impl Into<Cow<'a, str>>, amt: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
        }
    }
}

/// MMR configuration for a position-builder trend graph.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionBuilderGraphMmrConfig<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    acct_lv: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lever: Option<Cow<'a, str>>,
}

impl<'a> PositionBuilderGraphMmrConfig<'a> {
    /// Create an MMR graph configuration with OKX defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the simulated account level (`3` or `4`).
    pub fn account_level(mut self, acct_lv: impl Into<Cow<'a, str>>) -> Self {
        self.acct_lv = Some(acct_lv.into());
        self
    }

    /// Set cross-margin leverage for multi-currency margin mode.
    pub fn leverage(mut self, lever: impl Into<Cow<'a, str>>) -> Self {
        self.lever = Some(lever.into());
        self
    }
}

/// Request body for an MMR position-builder trend graph.
#[derive(Debug, Clone, Serialize)]
pub struct PositionBuilderGraphRequest<'a> {
    #[serde(rename = "inclRealPosAndEq", skip_serializing_if = "Option::is_none")]
    include_real_positions_and_equity: Option<bool>,
    #[serde(rename = "simPos", skip_serializing_if = "Option::is_none")]
    simulated_positions: Option<Vec<PositionBuilderGraphPosition<'a>>>,
    #[serde(rename = "simAsset", skip_serializing_if = "Option::is_none")]
    simulated_assets: Option<Vec<PositionBuilderGraphAsset<'a>>>,
    #[serde(rename = "greeksType", skip_serializing_if = "Option::is_none")]
    greeks_type: Option<Cow<'a, str>>,
    #[serde(rename = "type")]
    graph_type: Cow<'a, str>,
    #[serde(rename = "mmrConfig")]
    mmr_config: PositionBuilderGraphMmrConfig<'a>,
}

impl<'a> PositionBuilderGraphRequest<'a> {
    /// Create an MMR trend-graph request.
    pub fn new(mmr_config: PositionBuilderGraphMmrConfig<'a>) -> Self {
        Self {
            include_real_positions_and_equity: None,
            simulated_positions: None,
            simulated_assets: None,
            greeks_type: None,
            graph_type: Cow::Borrowed("mmr"),
            mmr_config,
        }
    }

    /// Set whether existing positions and equity are included.
    pub fn include_real_positions_and_equity(mut self, include: bool) -> Self {
        self.include_real_positions_and_equity = Some(include);
        self
    }

    /// Set simulated positions.
    pub fn simulated_positions(
        mut self,
        simulated_positions: Vec<PositionBuilderGraphPosition<'a>>,
    ) -> Self {
        self.simulated_positions = Some(simulated_positions);
        self
    }

    /// Set simulated assets.
    pub fn simulated_assets(
        mut self,
        simulated_assets: Vec<PositionBuilderGraphAsset<'a>>,
    ) -> Self {
        self.simulated_assets = Some(simulated_assets);
        self
    }

    /// Set the option greeks display type.
    pub fn greeks_type(mut self, greeks_type: impl Into<Cow<'a, str>>) -> Self {
        self.greeks_type = Some(greeks_type.into());
        self
    }
}
