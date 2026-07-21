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
