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

/// Discount tier returned for interest-free quota calculations (legacy format).
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

/// Discount tier returned in the `details` array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiscountDetail {
    /// Value returned by OKX in the `discountRate` field.
    #[serde(default)]
    pub discount_rate: NumberString,
    /// Value returned by OKX in the `liqPenaltyRate` field.
    #[serde(default)]
    pub liq_penalty_rate: NumberString,
    /// Value returned by OKX in the `minAmt` field.
    #[serde(default)]
    pub min_amt: NumberString,
    /// Value returned by OKX in the `maxAmt` field.
    #[serde(default)]
    pub max_amt: NumberString,
    /// Value returned by OKX in the `tier` field.
    #[serde(default)]
    pub tier: String,
    /// Value returned by OKX in the `disCcyEq` field.
    #[serde(default)]
    pub dis_ccy_eq: NumberString,
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
    /// Value returned by OKX in the `discountLv` field (discount level ID).
    #[serde(default)]
    pub discount_lv: String,
    /// Value returned by OKX in the `details` field.
    #[serde(default)]
    pub details: Vec<DiscountDetail>,
    /// Value returned by OKX in the `collateralRestrict` field.
    #[serde(default)]
    pub collateral_restrict: bool,
    /// Value returned by OKX in the `minDiscountRate` field.
    #[serde(default)]
    pub min_discount_rate: NumberString,
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

/// MM Program instrument-type classification row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MmInstrumentType {
    /// Value returned by OKX in the `instId` field.
    #[serde(default)]
    pub inst_id: String,
    /// Value returned by OKX in the `instType` field.
    #[serde(default)]
    pub inst_type: String,
    /// Value returned by OKX in the `pairType` field (MM Program classification type).
    #[serde(default)]
    pub pair_type: String,
}

/// Macro-economic calendar row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EconomicCalendar {
    /// Value returned by OKX in the `calendarId` field.
    #[serde(default)]
    pub calendar_id: String,
    /// Value returned by OKX in the `date` field (Unix milliseconds).
    #[serde(default)]
    pub date: NumberString,
    /// Value returned by OKX in the `region` field.
    #[serde(default)]
    pub region: String,
    /// Value returned by OKX in the `category` field.
    #[serde(default)]
    pub category: String,
    /// Value returned by OKX in the `event` field.
    #[serde(default)]
    pub event: String,
    /// Value returned by OKX in the `refDate` field (Unix milliseconds).
    #[serde(default)]
    pub ref_date: NumberString,
    /// Value returned by OKX in the `actual` field.
    #[serde(default)]
    pub actual: String,
    /// Value returned by OKX in the `previous` field.
    #[serde(default)]
    pub previous: String,
    /// Value returned by OKX in the `forecast` field.
    #[serde(default)]
    pub forecast: String,
    /// Value returned by OKX in the `dateSpan` field.
    #[serde(default)]
    pub date_span: String,
    /// Value returned by OKX in the `importance` field.
    #[serde(default)]
    pub importance: String,
    /// Value returned by OKX in the `uTime` field (Unix milliseconds).
    #[serde(default)]
    pub u_time: NumberString,
    /// Value returned by OKX in the `prevInitial` field.
    #[serde(default)]
    pub prev_initial: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `unit` field.
    #[serde(default)]
    pub unit: String,
}

/// Historical swap-premium row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PremiumHistory {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Premium index.
    #[serde(default)]
    pub premium: NumberString,
    /// Data generation time in Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
}

/// Settlement metadata for an event-contract series.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EventContractSettlement {
    /// Settlement method.
    #[serde(default)]
    pub method: String,
    /// Whether the market may settle before its expiration time.
    #[serde(default)]
    pub close_early: bool,
    /// Settlement source name.
    #[serde(default)]
    pub src_name: String,
    /// Price underlying in OKX instrument format.
    #[serde(default)]
    pub underlying: String,
}

/// Event-contract series metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EventContractSeries {
    /// Series ID.
    #[serde(default)]
    pub series_id: String,
    /// Series frequency.
    #[serde(default)]
    pub freq: String,
    /// Series title.
    #[serde(default)]
    pub title: String,
    /// Series category.
    #[serde(default)]
    pub category: String,
    /// Settlement metadata.
    #[serde(default)]
    pub settlement: EventContractSettlement,
}

/// Prediction-market event metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EventContractEvent {
    /// Series ID.
    #[serde(default)]
    pub series_id: String,
    /// Event ID.
    #[serde(default)]
    pub event_id: String,
    /// Strike-price fixing time in Unix milliseconds.
    #[serde(default)]
    pub fix_time: NumberString,
    /// Event strike time in Unix milliseconds.
    #[serde(default)]
    pub exp_time: NumberString,
    /// Event state.
    #[serde(default)]
    pub state: String,
}

/// Prediction-market instrument metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EventContractMarket {
    /// Series ID.
    #[serde(default)]
    pub series_id: String,
    /// Event ID.
    #[serde(default)]
    pub event_id: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Listing time in Unix milliseconds.
    #[serde(default)]
    pub list_time: NumberString,
    /// Strike-price fixing time in Unix milliseconds.
    #[serde(default)]
    pub fix_time: NumberString,
    /// Strike time in Unix milliseconds.
    #[serde(default)]
    pub exp_time: NumberString,
    /// Market state.
    #[serde(default)]
    pub state: String,
    /// Whether the market has been disputed.
    #[serde(default)]
    pub disputed: bool,
    /// Market outcome.
    #[serde(default)]
    pub outcome: String,
    /// Minimum expiration value that produces a YES outcome.
    #[serde(default)]
    pub floor_strike: NumberString,
    /// Maximum expiration value that produces a YES outcome.
    #[serde(default)]
    pub cap_strike: NumberString,
    /// Final settlement value.
    #[serde(default)]
    pub settle_value: NumberString,
    /// Hit direction for `hit` settlement methods.
    #[serde(default)]
    pub hit_dir: String,
}

/// Public single-leg block-trade row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicBlockTrade {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Trade price.
    #[serde(default)]
    pub px: NumberString,
    /// Trade quantity.
    #[serde(default)]
    pub sz: NumberString,
    /// Trade side.
    #[serde(default)]
    pub side: String,
    /// Implied volatility for options.
    #[serde(default)]
    pub fill_vol: NumberString,
    /// Forward price for options.
    #[serde(default)]
    pub fwd_px: NumberString,
    /// Index price for derivatives.
    #[serde(default)]
    pub idx_px: NumberString,
    /// Mark price for derivatives.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Group RFQ ID.
    #[serde(default)]
    pub group_id: String,
    /// Trade time in Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
}

/// Estimated futures settlement information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EstimatedSettlementInfo {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Next settlement time in Unix milliseconds.
    #[serde(default)]
    pub next_settle_time: NumberString,
    /// Estimated settlement price.
    #[serde(default)]
    pub est_settle_px: NumberString,
    /// Data return time in Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
}

/// Per-instrument futures settlement detail.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SettlementHistoryDetail {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Settlement price.
    #[serde(default)]
    pub settle_px: NumberString,
}

/// Futures settlement-history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SettlementHistory {
    /// Settlement time in Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
    /// Instrument settlement details.
    #[serde(default)]
    pub details: Vec<SettlementHistoryDetail>,
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
