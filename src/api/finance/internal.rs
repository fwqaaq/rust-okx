use serde::Serialize;

/// Empty query/body object for endpoints without parameters.
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub(super) struct NoParams {}

/// Optional currency filter shared by several Finance endpoints.
#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct OptionalCurrency<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) ccy: Option<&'a str>,
}

pub(super) fn optional_ccy(ccy: Option<&str>) -> OptionalCurrency<'_> {
    OptionalCurrency { ccy }
}

/// Amount-only request body used by ETH/SOL staking purchase and redemption.
#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct AmountBody<'a> {
    pub(super) amt: &'a str,
}

/// APY-history query.
#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct DaysQuery<'a> {
    pub(super) days: &'a str,
}

/// Savings lending-rate request body.
#[derive(Debug, Clone, Copy, Serialize)]
pub(super) struct SetLendingRateBody<'a> {
    pub(super) ccy: &'a str,
    pub(super) rate: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finance_helpers_serialize_as_objects() {
        assert_eq!(
            serde_json::to_value(NoParams {}).unwrap(),
            serde_json::json!({})
        );
        assert_eq!(
            serde_json::to_value(optional_ccy(None)).unwrap(),
            serde_json::json!({})
        );
        assert_eq!(
            serde_json::to_value(SetLendingRateBody {
                ccy: "USDT",
                rate: "0.01",
            })
            .unwrap(),
            serde_json::json!({"ccy": "USDT", "rate": "0.01"})
        );
    }
}
