use std::borrow::Cow;

use serde::Serialize;

use crate::model::InstType;

/// A simulated position used by position-builder and simulated-margin requests.
#[derive(Debug, Clone, Serialize)]
pub struct SimulatedPosition<'a> {
    #[serde(rename = "instId")]
    inst_id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pos: Option<Cow<'a, str>>,
    #[serde(rename = "avgPx", skip_serializing_if = "Option::is_none")]
    avg_px: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lever: Option<Cow<'a, str>>,
}

impl<'a> SimulatedPosition<'a> {
    /// Create a simulated position for an instrument.
    pub fn new(inst_id: impl Into<Cow<'a, str>>) -> Self {
        Self {
            inst_id: inst_id.into(),
            pos: None,
            avg_px: None,
            lever: None,
        }
    }

    /// Set the simulated position size.
    pub fn position(mut self, pos: impl Into<Cow<'a, str>>) -> Self {
        self.pos = Some(pos.into());
        self
    }

    /// Set the simulated average price.
    pub fn average_price(mut self, avg_px: impl Into<Cow<'a, str>>) -> Self {
        self.avg_px = Some(avg_px.into());
        self
    }

    /// Set the simulated leverage.
    pub fn leverage(mut self, lever: impl Into<Cow<'a, str>>) -> Self {
        self.lever = Some(lever.into());
        self
    }
}

/// A simulated asset used by position-builder requests.
#[derive(Debug, Clone, Serialize)]
pub struct SimulatedAsset<'a> {
    ccy: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eq: Option<Cow<'a, str>>,
}

impl<'a> SimulatedAsset<'a> {
    /// Create a simulated asset for a currency.
    pub fn new(ccy: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            eq: None,
        }
    }

    /// Set the simulated equity.
    pub fn equity(mut self, eq: impl Into<Cow<'a, str>>) -> Self {
        self.eq = Some(eq.into());
        self
    }
}

/// Request body for simulated margin calculation.
#[derive(Debug, Clone, Default, Serialize)]
pub struct SimulatedMarginRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "inclRealPos", skip_serializing_if = "Option::is_none")]
    include_real_positions: Option<bool>,
    #[serde(rename = "spotOffsetType", skip_serializing_if = "Option::is_none")]
    spot_offset_type: Option<Cow<'a, str>>,
    #[serde(rename = "simPos", skip_serializing_if = "Option::is_none")]
    simulated_positions: Option<Vec<SimulatedPosition<'a>>>,
}

impl<'a> SimulatedMarginRequest<'a> {
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
    pub fn spot_offset_type(mut self, spot_offset_type: impl Into<Cow<'a, str>>) -> Self {
        self.spot_offset_type = Some(spot_offset_type.into());
        self
    }

    /// Set simulated positions.
    pub fn simulated_positions(mut self, simulated_positions: Vec<SimulatedPosition<'a>>) -> Self {
        self.simulated_positions = Some(simulated_positions);
        self
    }
}

/// Query parameters for account position tiers.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AccountPositionTiersRequest<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    inst_type: Option<InstType>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<Cow<'a, str>>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<Cow<'a, str>>,
}

impl<'a> AccountPositionTiersRequest<'a> {
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
    pub fn underlying(mut self, underlying: impl Into<Cow<'a, str>>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<Cow<'a, str>>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Request body for position builder.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PositionBuilderRequest<'a> {
    #[serde(rename = "acctLv", skip_serializing_if = "Option::is_none")]
    acct_lv: Option<Cow<'a, str>>,
    #[serde(rename = "inclRealPosAndEq", skip_serializing_if = "Option::is_none")]
    include_real_positions_and_equity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lever: Option<Cow<'a, str>>,
    #[serde(rename = "greeksType", skip_serializing_if = "Option::is_none")]
    greeks_type: Option<Cow<'a, str>>,
    #[serde(rename = "simPos", skip_serializing_if = "Option::is_none")]
    simulated_positions: Option<Vec<SimulatedPosition<'a>>>,
    #[serde(rename = "simAsset", skip_serializing_if = "Option::is_none")]
    simulated_assets: Option<Vec<SimulatedAsset<'a>>>,
    #[serde(rename = "idxVol", skip_serializing_if = "Option::is_none")]
    index_volatility: Option<Cow<'a, str>>,
}

impl<'a> PositionBuilderRequest<'a> {
    /// Create an empty position-builder request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the account level.
    pub fn account_level(mut self, acct_lv: impl Into<Cow<'a, str>>) -> Self {
        self.acct_lv = Some(acct_lv.into());
        self
    }

    /// Set whether real positions and equity are included.
    pub fn include_real_positions_and_equity(mut self, include: bool) -> Self {
        self.include_real_positions_and_equity = Some(include);
        self
    }

    /// Set leverage.
    pub fn leverage(mut self, lever: impl Into<Cow<'a, str>>) -> Self {
        self.lever = Some(lever.into());
        self
    }

    /// Set greeks display type.
    pub fn greeks_type(mut self, greeks_type: impl Into<Cow<'a, str>>) -> Self {
        self.greeks_type = Some(greeks_type.into());
        self
    }

    /// Set simulated positions.
    pub fn simulated_positions(mut self, simulated_positions: Vec<SimulatedPosition<'a>>) -> Self {
        self.simulated_positions = Some(simulated_positions);
        self
    }

    /// Set simulated assets.
    pub fn simulated_assets(mut self, simulated_assets: Vec<SimulatedAsset<'a>>) -> Self {
        self.simulated_assets = Some(simulated_assets);
        self
    }

    /// Set index volatility.
    pub fn index_volatility(mut self, index_volatility: impl Into<Cow<'a, str>>) -> Self {
        self.index_volatility = Some(index_volatility.into());
        self
    }
}
