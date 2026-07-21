use std::borrow::Cow;

use serde::Serialize;

/// Query parameters for
/// [`precheck_set_delta_neutral`](crate::api::account::Account::precheck_set_delta_neutral).
#[derive(Debug, Clone, Serialize)]
pub struct PrecheckSetDeltaNeutralRequest<'a> {
    #[serde(rename = "stgyType")]
    stgy_type: Cow<'a, str>,
}

impl<'a> PrecheckSetDeltaNeutralRequest<'a> {
    /// Create a delta-neutral strategy precheck.
    ///
    /// OKX currently accepts `0` for general mode and `1` for delta-neutral mode.
    pub fn new(stgy_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            stgy_type: stgy_type.into(),
        }
    }
}

/// Request for
/// [`set_settle_currency`](crate::api::account::Account::set_settle_currency).
#[derive(Debug, Clone, Serialize)]
pub struct SetSettleCurrencyRequest<'a> {
    #[serde(rename = "settleCcy")]
    settle_ccy: Cow<'a, str>,
}

impl<'a> SetSettleCurrencyRequest<'a> {
    /// Create a settlement-currency update for USD-margined contracts.
    pub fn new(settle_ccy: impl Into<Cow<'a, str>>) -> Self {
        Self {
            settle_ccy: settle_ccy.into(),
        }
    }
}

/// Request for [`set_fee_type`](crate::api::account::Account::set_fee_type).
#[derive(Debug, Clone, Serialize)]
pub struct SetFeeTypeRequest<'a> {
    #[serde(rename = "feeType")]
    fee_type: Cow<'a, str>,
}

impl<'a> SetFeeTypeRequest<'a> {
    /// Create a spot fee-type update.
    ///
    /// OKX currently accepts `0` for fees in the obtained currency and `1`
    /// for fees in the quote currency.
    pub fn new(fee_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            fee_type: fee_type.into(),
        }
    }
}
