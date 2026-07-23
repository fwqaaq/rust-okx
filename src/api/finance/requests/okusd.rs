use std::borrow::Cow;

use serde::Serialize;

/// OKUSD redemption settlement mode.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum OkusdRedeemType {
    /// Fast redemption with real-time settlement.
    #[serde(rename = "1")]
    Fast,
    /// Standard redemption with D+5 or D+6 settlement.
    #[serde(rename = "2")]
    Standard,
}

/// Request to subscribe USDT for OKUSD.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkusdSubscribeRequest<'a> {
    amt: Cow<'a, str>,
    cl_ord_id: Cow<'a, str>,
}

impl<'a> OkusdSubscribeRequest<'a> {
    /// Set the USDT amount and unique client order ID.
    pub fn new(
        amt: impl Into<Cow<'a, str>>,
        cl_ord_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            amt: amt.into(),
            cl_ord_id: cl_ord_id.into(),
        }
    }
}

/// Request to redeem OKUSD for USDT.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OkusdRedeemRequest<'a> {
    amt: Cow<'a, str>,
    redeem_type: OkusdRedeemType,
    cl_ord_id: Cow<'a, str>,
}

impl<'a> OkusdRedeemRequest<'a> {
    /// Set the OKUSD amount, settlement mode, and unique client order ID.
    pub fn new(
        amt: impl Into<Cow<'a, str>>,
        redeem_type: OkusdRedeemType,
        cl_ord_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            amt: amt.into(),
            redeem_type,
            cl_ord_id: cl_ord_id.into(),
        }
    }
}
