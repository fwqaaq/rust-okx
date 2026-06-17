use crate::model::{NumberString, deserialize_vec_or_empty_string};
use serde::Deserialize;

/// Option summary row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OptionSummary {
    /// Value returned by OKX in the `instType` field.
    #[serde(default)]
    pub inst_type: String,
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `uly` field.
    #[serde(default)]
    pub uly: String,
    /// Value returned by OKX in the `instFamily` field.
    #[serde(default)]
    pub inst_family: String,
    /// Value returned by OKX in the `delta` field.
    #[serde(default)]
    pub delta: NumberString,
    /// Value returned by OKX in the `gamma` field.
    #[serde(default)]
    pub gamma: NumberString,
    /// Value returned by OKX in the `vega` field.
    #[serde(default)]
    pub vega: NumberString,
    /// Value returned by OKX in the `theta` field.
    #[serde(default)]
    pub theta: NumberString,
    /// Value returned by OKX in the `deltaBs` field.
    #[serde(default)]
    pub delta_bs: NumberString,
    /// Value returned by OKX in the `gammaBs` field.
    #[serde(default)]
    pub gamma_bs: NumberString,
    /// Value returned by OKX in the `vegaBs` field.
    #[serde(default)]
    pub vega_bs: NumberString,
    /// Value returned by OKX in the `thetaBs` field.
    #[serde(default)]
    pub theta_bs: NumberString,
    /// Value returned by OKX in the `lever` field.
    #[serde(default)]
    pub lever: NumberString,
    /// Value returned by OKX in the `markVol` field.
    #[serde(default)]
    pub mark_vol: NumberString,
    /// Value returned by OKX in the `realVol` field.
    #[serde(default)]
    pub real_vol: NumberString,
    /// Value returned by OKX in the `volLv` field.
    #[serde(default)]
    pub vol_lv: NumberString,
    /// Value returned by OKX in the `bidVol` field.
    #[serde(default)]
    pub bid_vol: NumberString,
    /// Value returned by OKX in the `askVol` field.
    #[serde(default)]
    pub ask_vol: NumberString,
    /// Value returned by OKX in the `fwdPx` field.
    #[serde(default)]
    pub fwd_px: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Estimated delivery or exercise price row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EstimatedPrice {
    /// Value returned by OKX in the `instType` field.
    #[serde(default)]
    pub inst_type: String,
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `settlePx` field.
    #[serde(default)]
    pub settle_px: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Discount tier returned for interest-free quota calculations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiscountLevel {
    /// Value returned by OKX in the `discountRate` field.
    #[serde(default)]
    pub discount_rate: NumberString,
    /// Value returned by OKX in the `minAmt` field.
    #[serde(default)]
    pub min_amt: NumberString,
    /// Value returned by OKX in the `maxAmt` field.
    #[serde(default)]
    pub max_amt: NumberString,
}

/// Discount-rate and interest-free quota row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiscountRateInterestFreeQuota {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `discountLv` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub discount_lv: Vec<DiscountLevel>,
}

/// Per-currency base rate and quota.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoanQuotaBasic {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
    /// Value returned by OKX in the `quota` field.
    #[serde(default)]
    pub quota: NumberString,
}

/// User-level quota coefficient.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoanQuotaLevel {
    /// Value returned by OKX in the `level` field.
    #[serde(default)]
    pub level: String,
    /// Value returned by OKX in the `loanQuotaCoef` field.
    #[serde(default)]
    pub loan_quota_coef: NumberString,
    /// Value returned by OKX in the `irDiscount` field.
    #[serde(default)]
    pub ir_discount: NumberString,
}

/// Currency with a customized interest rate.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoanQuotaConfiguredCurrency {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
}

/// Customized absolute loan quota.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LoanQuotaConfig {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `stgyType` field.
    #[serde(default)]
    pub stgy_type: String,
    /// Value returned by OKX in the `quota` field.
    #[serde(default)]
    pub quota: NumberString,
    /// Value returned by OKX in the `level` field.
    #[serde(default)]
    pub level: String,
}

/// Interest-rate and loan-quota response row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InterestRateLoanQuota {
    /// Value returned by OKX in the `basic` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub basic: Vec<LoanQuotaBasic>,
    /// Value returned by OKX in the `vip` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub vip: Vec<LoanQuotaLevel>,
    /// Value returned by OKX in the `regular` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub regular: Vec<LoanQuotaLevel>,
    /// Value returned by OKX in the `configCcyList` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub config_ccy_list: Vec<LoanQuotaConfiguredCurrency>,
    /// Value returned by OKX in the `config` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub config: Vec<LoanQuotaConfig>,
}

/// VIP interest-rate loan quota row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct VipInterestRateLoanQuota {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `level` field.
    #[serde(default)]
    pub level: String,
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
    /// Value returned by OKX in the `quota` field.
    #[serde(default)]
    pub quota: NumberString,
}

/// A single price range and tick-size definition.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TickBandDetail {
    /// Value returned by OKX in the `minPx` field.
    #[serde(default)]
    pub min_px: NumberString,
    /// Value returned by OKX in the `maxPx` field.
    #[serde(default)]
    pub max_px: NumberString,
    /// Value returned by OKX in the `tickSz` field.
    #[serde(default)]
    pub tick_sz: NumberString,
}

/// Instrument tick-band row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InstrumentTickBand {
    /// Value returned by OKX in the `instType` field.
    #[serde(default)]
    pub inst_type: String,
    /// Value returned by OKX in the `instFamily` field.
    #[serde(default)]
    pub inst_family: String,
    /// Value returned by OKX in the `tickBand` field.
    #[serde(default, deserialize_with = "deserialize_vec_or_empty_string")]
    pub tick_band: Vec<TickBandDetail>,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Public option-trade row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicOptionTrade {
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `tradeId` field.
    #[serde(default)]
    pub trade_id: String,
    /// Value returned by OKX in the `px` field.
    #[serde(default)]
    pub px: NumberString,
    /// Value returned by OKX in the `sz` field.
    #[serde(default)]
    pub sz: NumberString,
    /// Value returned by OKX in the `side` field.
    #[serde(default)]
    pub side: String,
    /// Value returned by OKX in the `fillVol` field.
    #[serde(default)]
    pub fill_vol: NumberString,
    /// Value returned by OKX in the `fwdPx` field.
    #[serde(default)]
    pub fwd_px: NumberString,
    /// Value returned by OKX in the `idxPx` field.
    #[serde(default)]
    pub idx_px: NumberString,
    /// Value returned by OKX in the `markPx` field.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Market-data history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MarketDataHistory {
    /// Value returned by OKX in the `module` field.
    #[serde(default)]
    pub module: String,
    /// Value returned by OKX in the `instType` field.
    #[serde(default)]
    pub inst_type: String,
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `dateAggrType` field.
    #[serde(default)]
    pub date_aggr_type: String,
    /// Value returned by OKX in the `value` field.
    #[serde(default)]
    pub value: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_summary_ignores_future_fields() {
        let row: OptionSummary = serde_json::from_str(
            r#"{"instType":"OPTION","instId":"BTC-USD-1-C","delta":"0.5","futureField":"ok"}"#,
        )
        .unwrap();
        assert_eq!(row.inst_type, "OPTION");
        assert_eq!(row.inst_id, "BTC-USD-1-C");
    }
}
