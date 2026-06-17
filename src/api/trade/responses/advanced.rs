use crate::model::NumberString;
use serde::Deserialize;

/// A convertible small balance returned by the easy-convert currency list.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EasyConvertFromCurrency {
    /// Value returned by OKX in the `fromCcy` field.
    #[serde(default)]
    pub from_ccy: String,
    /// Value returned by OKX in the `fromAmt` field.
    #[serde(default)]
    pub from_amt: NumberString,
}

/// Easy-convert currency row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EasyConvertCurrency {
    /// Value returned by OKX in the `fromData` field.
    #[serde(default)]
    pub from_data: Vec<EasyConvertFromCurrency>,
    /// Value returned by OKX in the `toCcy` field.
    #[serde(default)]
    pub to_ccy: Vec<String>,
}

/// Easy-convert execution result row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EasyConvertResult {
    /// Value returned by OKX in the `fromCcy` field.
    #[serde(default)]
    pub from_ccy: String,
    /// Value returned by OKX in the `toCcy` field.
    #[serde(default)]
    pub to_ccy: String,
    /// Value returned by OKX in the `fillFromSz` field.
    #[serde(default)]
    pub fill_from_sz: NumberString,
    /// Value returned by OKX in the `fillToSz` field.
    #[serde(default)]
    pub fill_to_sz: NumberString,
    /// Value returned by OKX in the `status` field.
    #[serde(default)]
    pub status: String,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Easy-convert history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EasyConvertHistory {
    /// Value returned by OKX in the `fromCcy` field.
    #[serde(default)]
    pub from_ccy: String,
    /// Value returned by OKX in the `toCcy` field.
    #[serde(default)]
    pub to_ccy: String,
    /// Value returned by OKX in the `fillFromSz` field.
    #[serde(default)]
    pub fill_from_sz: NumberString,
    /// Value returned by OKX in the `fillToSz` field.
    #[serde(default)]
    pub fill_to_sz: NumberString,
    /// Value returned by OKX in the `acct` field.
    #[serde(default)]
    pub acct: String,
    /// Value returned by OKX in the `status` field.
    #[serde(default)]
    pub status: String,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Debt-side information returned by one-click-repay currency-list endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OneClickRepayDebt {
    /// Value returned by OKX in the `debtCcy` field.
    #[serde(default)]
    pub debt_ccy: String,
    /// Value returned by OKX in the `debtAmt` field.
    #[serde(default)]
    pub debt_amt: NumberString,
}

/// Repayment asset information returned by one-click-repay currency-list endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OneClickRepayAsset {
    /// Value returned by OKX in the `repayCcy` field.
    #[serde(default)]
    pub repay_ccy: String,
    /// Value returned by OKX in the `repayAmt` field.
    #[serde(default)]
    pub repay_amt: NumberString,
}

/// One-click-repay currency row (legacy and v2).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OneClickRepayCurrency {
    /// Value returned by OKX in the `debtData` field.
    #[serde(default)]
    pub debt_data: Vec<OneClickRepayDebt>,
    /// Value returned by OKX in the `debtType` field.
    #[serde(default)]
    pub debt_type: String,
    /// Value returned by OKX in the `repayData` field.
    #[serde(default)]
    pub repay_data: Vec<OneClickRepayAsset>,
}

/// One-click-repay execution result row (legacy and v2).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OneClickRepayResult {
    /// Value returned by OKX in the `debtCcy` field.
    #[serde(default)]
    pub debt_ccy: String,
    /// Value returned by OKX in the `repayCcy` field.
    #[serde(default)]
    pub repay_ccy: String,
    /// Value returned by OKX in the `repayCcyList` field.
    #[serde(default)]
    pub repay_ccy_list: Vec<String>,
    /// Value returned by OKX in the `fillDebtSz` field.
    #[serde(default)]
    pub fill_debt_sz: NumberString,
    /// Value returned by OKX in the `fillRepaySz` field.
    #[serde(default)]
    pub fill_repay_sz: NumberString,
    /// Value returned by OKX in the `status` field.
    #[serde(default)]
    pub status: String,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Order generated while fulfilling a v2 one-click repayment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OneClickRepayOrderInfo {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `ordType` field.
    #[serde(default)]
    pub ord_type: String,
    /// Value returned by OKX in the `side` field.
    #[serde(default)]
    pub side: String,
    /// Value returned by OKX in the `px` field.
    #[serde(default)]
    pub px: NumberString,
    /// Value returned by OKX in the `fillPx` field.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Value returned by OKX in the `fillSz` field.
    #[serde(default)]
    pub fill_sz: NumberString,
    /// Value returned by OKX in the `cTime` field.
    #[serde(default)]
    pub c_time: NumberString,
}

/// One-click-repay history row (legacy and v2).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OneClickRepayHistory {
    /// Value returned by OKX in the `debtCcy` field.
    #[serde(default)]
    pub debt_ccy: String,
    /// Value returned by OKX in the `repayCcy` field.
    #[serde(default)]
    pub repay_ccy: String,
    /// Value returned by OKX in the `repayCcyList` field.
    #[serde(default)]
    pub repay_ccy_list: Vec<String>,
    /// Value returned by OKX in the `fillDebtSz` field.
    #[serde(default)]
    pub fill_debt_sz: NumberString,
    /// Value returned by OKX in the `fillRepaySz` field.
    #[serde(default)]
    pub fill_repay_sz: NumberString,
    /// Value returned by OKX in the `status` field.
    #[serde(default)]
    pub status: String,
    /// Value returned by OKX in the `ordIdInfo` field.
    #[serde(default)]
    pub ord_id_info: Vec<OneClickRepayOrderInfo>,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_easy_convert_currency_list() {
        let row: EasyConvertCurrency = serde_json::from_str(
            r#"{"fromData":[{"fromCcy":"ADA","fromAmt":"6.5"}],"toCcy":["USDT"]}"#,
        )
        .unwrap();
        assert_eq!(row.from_data[0].from_ccy, "ADA");
        assert_eq!(row.to_ccy, vec!["USDT".to_owned()]);
    }

    #[test]
    fn decodes_one_click_repay_history() {
        let row: OneClickRepayHistory = serde_json::from_str(
            r#"{"debtCcy":"USDT","repayCcy":"BTC","fillDebtSz":"10","fillRepaySz":"0.001","status":"filled","uTime":"1"}"#,
        ).unwrap();
        assert_eq!(row.debt_ccy, "USDT");
        assert_eq!(row.status, "filled");
    }
}
