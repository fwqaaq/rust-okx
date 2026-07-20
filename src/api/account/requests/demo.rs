use std::borrow::Cow;

use serde::Serialize;

/// Direction of a demo-account balance adjustment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoBalanceAdjustmentType {
    /// Add the requested amounts to the demo balances.
    Increase,
    /// Deduct the requested amounts from the demo balances.
    Reduce,
}

/// One currency adjustment in a demo-account balance request.
#[derive(Debug, Clone, Serialize)]
pub struct DemoBalanceAdjustment<'a> {
    ccy: Cow<'a, str>,
    amt: Cow<'a, str>,
}

impl<'a> DemoBalanceAdjustment<'a> {
    /// Create a currency adjustment with an exact decimal-string amount.
    pub fn new(ccy: impl Into<Cow<'a, str>>, amt: impl Into<Cow<'a, str>>) -> Self {
        Self {
            ccy: ccy.into(),
            amt: amt.into(),
        }
    }
}

/// Request body for [`adjust_demo_account_balance`](crate::api::account::Account::adjust_demo_account_balance).
#[derive(Debug, Clone, Serialize)]
pub struct DemoAdjustBalanceRequest<'a> {
    #[serde(rename = "type")]
    adjustment_type: DemoBalanceAdjustmentType,
    adjustments: Vec<DemoBalanceAdjustment<'a>>,
}

impl<'a> DemoAdjustBalanceRequest<'a> {
    /// Create a request containing one or more currency adjustments.
    pub fn new<I>(adjustment_type: DemoBalanceAdjustmentType, adjustments: I) -> Self
    where
        I: IntoIterator<Item = DemoBalanceAdjustment<'a>>,
    {
        Self {
            adjustment_type,
            adjustments: adjustments.into_iter().collect(),
        }
    }
}
