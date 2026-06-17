use crate::model::NumberString;
use serde::Deserialize;

/// Flexible-loan currency row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanCurrency {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `minLoan` field.
    #[serde(default)]
    pub min_loan: NumberString,
    /// Value returned by OKX in the `maxLoan` field.
    #[serde(default)]
    pub max_loan: NumberString,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
}

/// Flexible-loan collateral asset row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanCollateralAsset {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `minCollateral` field.
    #[serde(default)]
    pub min_collateral: NumberString,
    /// Value returned by OKX in the `maxCollateral` field.
    #[serde(default)]
    pub max_collateral: NumberString,
    /// Value returned by OKX in the `discountRate` field.
    #[serde(default)]
    pub discount_rate: NumberString,
}

/// Flexible-loan maximum-loan row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanMaxLoan {
    /// Value returned by OKX in the `borrowCcy` field.
    #[serde(default)]
    pub borrow_ccy: String,
    /// Value returned by OKX in the `maxLoan` field.
    #[serde(default)]
    pub max_loan: NumberString,
}

/// Flexible-loan maximum-redeem row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanMaxRedeem {
    /// Value returned by OKX in the `collateralCcy` field.
    #[serde(default)]
    pub collateral_ccy: String,
    /// Value returned by OKX in the `maxRedeem` field.
    #[serde(default)]
    pub max_redeem: NumberString,
}

/// Flexible-loan order row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanOrder {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `borrowCcy` field.
    #[serde(default)]
    pub borrow_ccy: String,
    /// Value returned by OKX in the `borrowAmt` field.
    #[serde(default)]
    pub borrow_amt: NumberString,
    /// Value returned by OKX in the `collateralCcy` field.
    #[serde(default)]
    pub collateral_ccy: String,
    /// Value returned by OKX in the `collateralAmt` field.
    #[serde(default)]
    pub collateral_amt: NumberString,
    /// Value returned by OKX in the `state` field.
    #[serde(default)]
    pub state: String,
    /// Value returned by OKX in the `cTime` field.
    #[serde(default)]
    pub c_time: NumberString,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Flexible-loan account information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanInfo {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `loanToValue` field.
    #[serde(default)]
    pub loan_to_value: NumberString,
    /// Value returned by OKX in the `liquidationLtv` field.
    #[serde(default)]
    pub liquidation_ltv: NumberString,
    /// Value returned by OKX in the `marginCallLtv` field.
    #[serde(default)]
    pub margin_call_ltv: NumberString,
    /// Value returned by OKX in the `accruedInterest` field.
    #[serde(default)]
    pub accrued_interest: NumberString,
}

/// Flexible-loan history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanHistory {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `type` field.
    #[serde(default, rename = "type")]
    pub event_type: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Flexible-loan interest row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FlexibleLoanInterest {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `interest` field.
    #[serde(default)]
    pub interest: NumberString,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}
