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
