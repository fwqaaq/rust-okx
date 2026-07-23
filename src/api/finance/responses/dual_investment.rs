use serde::Deserialize;

use crate::model::NumberString;

/// Available dual-investment currency pair.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentCurrencyPair {
    /// Base currency.
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency.
    #[serde(default)]
    pub quote_ccy: String,
    /// Option type (`C` or `P`).
    #[serde(default)]
    pub opt_type: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
}

/// Dual-investment product.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentProduct {
    /// Absolute yield.
    #[serde(default)]
    pub abs_yield: NumberString,
    /// Annualized yield.
    #[serde(default)]
    pub annualized_yield: NumberString,
    /// Base currency.
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency.
    #[serde(default)]
    pub quote_ccy: String,
    /// Expiry time.
    #[serde(default)]
    pub exp_time: NumberString,
    /// Interest accrual start time.
    #[serde(default)]
    pub interest_accrual_time: NumberString,
    /// Product launch time.
    #[serde(default)]
    pub list_time: NumberString,
    /// Maximum trade size.
    #[serde(default)]
    pub max_size: NumberString,
    /// Minimum trade size.
    #[serde(default)]
    pub min_size: NumberString,
    /// Investment currency.
    #[serde(default)]
    pub notional_ccy: String,
    /// Option type.
    #[serde(default)]
    pub opt_type: String,
    /// Product ID.
    #[serde(default)]
    pub product_id: String,
    /// Quote time.
    #[serde(default)]
    pub quote_time: NumberString,
    /// Latest early-redemption time.
    #[serde(default)]
    pub redeem_end_time: NumberString,
    /// Earliest early-redemption time.
    #[serde(default)]
    pub redeem_start_time: NumberString,
    /// Trade step size.
    #[serde(default)]
    pub step_sz: NumberString,
    /// Trade end time.
    #[serde(default)]
    pub trade_end_time: NumberString,
    /// Strike price.
    #[serde(default)]
    pub strike: NumberString,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
}

/// Live dual-investment quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentQuote {
    /// Absolute yield.
    #[serde(default)]
    pub abs_yield: NumberString,
    /// Annualized yield.
    #[serde(default)]
    pub annualized_yield: NumberString,
    /// Interest accrual start time.
    #[serde(default)]
    pub interest_accrual_time: NumberString,
    /// Investment size.
    #[serde(default)]
    pub notional_sz: NumberString,
    /// Investment currency.
    #[serde(default)]
    pub notional_ccy: String,
    /// Product ID.
    #[serde(default)]
    pub product_id: String,
    /// Quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Quote expiry time.
    #[serde(default)]
    pub valid_until: NumberString,
    /// Index price.
    #[serde(default)]
    pub idx_px: NumberString,
}

/// Dual-investment trade result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentTrade {
    /// Quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Order state.
    #[serde(default)]
    pub state: String,
}

/// Early-redemption quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentRedeemQuote {
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Redeem quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Redeem currency.
    #[serde(default)]
    pub redeem_ccy: String,
    /// Redeem size.
    #[serde(default)]
    pub redeem_sz: NumberString,
    /// Term rate.
    #[serde(default)]
    pub term_rate: NumberString,
    /// Quote expiry time.
    #[serde(default)]
    pub valid_until: NumberString,
}

/// Dual-investment order state.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentOrderState {
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Order state.
    #[serde(default)]
    pub state: String,
}

/// Dual-investment order-history row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DualInvestmentOrder {
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Product ID.
    #[serde(default)]
    pub product_id: String,
    /// Base currency.
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency.
    #[serde(default)]
    pub quote_ccy: String,
    /// Underlying.
    #[serde(default)]
    pub uly: String,
    /// Strike price.
    #[serde(default)]
    pub strike: NumberString,
    /// Notional size.
    #[serde(default)]
    pub notional_sz: NumberString,
    /// Notional currency.
    #[serde(default)]
    pub notional_ccy: String,
    /// Absolute yield rate.
    #[serde(default)]
    pub abs_yield: NumberString,
    /// Annualized yield rate.
    #[serde(default)]
    pub annualized_yield: NumberString,
    /// Yield size.
    #[serde(default)]
    pub yield_sz: NumberString,
    /// Yield currency.
    #[serde(default)]
    pub yield_ccy: String,
    /// Settlement size.
    #[serde(default)]
    pub settle_sz: NumberString,
    /// Settlement currency.
    #[serde(default)]
    pub settle_ccy: String,
    /// Settlement price.
    #[serde(default)]
    pub settle_px: NumberString,
    /// Settlement time.
    #[serde(default)]
    pub settle_time: NumberString,
    /// Expiry time.
    #[serde(default)]
    pub exp_time: NumberString,
    /// Earliest early-redemption time.
    #[serde(default)]
    pub redeem_start_time: NumberString,
    /// Latest early-redemption time.
    #[serde(default)]
    pub redeem_end_time: NumberString,
    /// Creation time.
    #[serde(default)]
    pub c_time: NumberString,
    /// Update time.
    #[serde(default)]
    pub u_time: NumberString,
}
