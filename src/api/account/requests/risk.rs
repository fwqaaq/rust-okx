use serde::Serialize;

use crate::model::{
    InstType, RequestValidationError, ValidateRequest, at_least_one, non_empty, optional_non_empty,
    optional_one_of, optional_positive_decimal_string,
};

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

impl SimulatedPosition {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("simPos.instId", &self.inst_id)?;
        optional_non_empty("simPos.pos", self.pos.as_deref())?;
        optional_positive_decimal_string("simPos.avgPx", self.avg_px.as_deref())?;
        optional_positive_decimal_string("simPos.lever", self.lever.as_deref())?;
        Ok(())
    }
}

impl SimulatedAsset {
    fn validate(&self) -> Result<(), RequestValidationError> {
        non_empty("simAsset.ccy", &self.ccy)?;
        optional_non_empty("simAsset.eq", self.eq.as_deref())
    }
}

impl ValidateRequest for SimulatedMarginRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        if matches!(self.inst_type, Some(InstType::Unknown(_))) {
            return Err(RequestValidationError::InvalidFormat {
                field: "instType",
                expected: "SPOT, MARGIN, SWAP, FUTURES, OPTION, or EVENTS",
            });
        }
        optional_one_of(
            "spotOffsetType",
            self.spot_offset_type.as_deref(),
            &["1", "2", "3"],
            "1, 2, or 3",
        )?;
        if self.include_real_positions == Some(false) && self.simulated_positions.is_none() {
            return Err(RequestValidationError::RequiredWhen {
                field: "simPos",
                condition: "inclRealPos is false",
            });
        }
        if let Some(positions) = &self.simulated_positions {
            if positions.is_empty() {
                return Err(RequestValidationError::EmptyField { field: "simPos" });
            }
            for position in positions {
                position.validate()?;
            }
        }
        Ok(())
    }
}

impl ValidateRequest for AccountPositionTiersRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        match &self.inst_type {
            Some(InstType::Swap | InstType::Futures | InstType::Option) => {}
            Some(_) => {
                return Err(RequestValidationError::InvalidFormat {
                    field: "instType",
                    expected: "SWAP, FUTURES, or OPTION",
                });
            }
            None => {
                return Err(RequestValidationError::RequiredWhen {
                    field: "instType",
                    condition: "querying account position tiers",
                });
            }
        }
        optional_non_empty("uly", self.underlying.as_deref())?;
        optional_non_empty("instFamily", self.inst_family.as_deref())?;
        at_least_one(
            "uly, instFamily",
            &[self.underlying.is_some(), self.inst_family.is_some()],
        )
    }
}

impl ValidateRequest for PositionBuilderRequest {
    fn validate(&self) -> Result<(), RequestValidationError> {
        optional_one_of(
            "acctLv",
            self.acct_lv.as_deref(),
            &["1", "2", "3", "4"],
            "1, 2, 3, or 4",
        )?;
        optional_positive_decimal_string("lever", self.lever.as_deref())?;
        optional_one_of(
            "greeksType",
            self.greeks_type.as_deref(),
            &["PA", "BS"],
            "PA or BS",
        )?;
        optional_positive_decimal_string("idxVol", self.index_volatility.as_deref())?;
        if let Some(positions) = &self.simulated_positions {
            if positions.is_empty() {
                return Err(RequestValidationError::EmptyField { field: "simPos" });
            }
            for position in positions {
                position.validate()?;
            }
        }
        if let Some(assets) = &self.simulated_assets {
            if assets.is_empty() {
                return Err(RequestValidationError::EmptyField { field: "simAsset" });
            }
            for asset in assets {
                asset.validate()?;
            }
        }
        if self.include_real_positions_and_equity != Some(true) {
            at_least_one(
                "simPos, simAsset",
                &[
                    self.simulated_positions.is_some(),
                    self.simulated_assets.is_some(),
                ],
            )?;
        }
        Ok(())
    }
}

