use serde::Deserialize;

use crate::model::NumberString;

/// Fiat deposit or withdrawal method.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatPaymentMethod {
    /// Fiat currency.
    #[serde(default)]
    pub ccy: String,
    /// Payment method identifier.
    #[serde(default)]
    pub payment_method: String,
    /// Percentage fee rate.
    #[serde(default)]
    pub fee_rate: NumberString,
    /// Minimum fee.
    #[serde(default)]
    pub min_fee: NumberString,
    /// Transaction limits.
    #[serde(default)]
    pub limits: FiatPaymentLimits,
    /// Associated payment accounts.
    #[serde(default)]
    pub accounts: Vec<FiatPaymentAccount>,
}

/// Limits associated with a fiat payment method.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatPaymentLimits {
    /// Daily transaction limit.
    #[serde(default)]
    pub daily_limit: NumberString,
    /// Remaining daily limit.
    #[serde(default)]
    pub daily_limit_remaining: NumberString,
    /// Weekly transaction limit.
    #[serde(default)]
    pub weekly_limit: NumberString,
    /// Remaining weekly limit.
    #[serde(default)]
    pub weekly_limit_remaining: NumberString,
    /// Monthly transaction limit.
    #[serde(default)]
    pub monthly_limit: NumberString,
    /// Remaining monthly limit.
    #[serde(default)]
    pub monthly_limit_remaining: NumberString,
    /// Maximum amount per transaction.
    #[serde(default)]
    pub max_amt: NumberString,
    /// Minimum amount per transaction.
    #[serde(default)]
    pub min_amt: NumberString,
    /// Lifetime transaction limit.
    #[serde(default)]
    pub lifetime_limit: NumberString,
}

/// Bank or payment account associated with a fiat method.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatPaymentAccount {
    /// Payment account ID.
    #[serde(default)]
    pub payment_acct_id: String,
    /// Bank or payment account number.
    #[serde(default)]
    pub acct_num: String,
    /// Recipient name.
    #[serde(default)]
    pub recipient_name: String,
    /// Bank name.
    #[serde(default)]
    pub bank_name: String,
    /// SWIFT, BIC, or bank code.
    #[serde(default)]
    pub bank_code: String,
    /// Account state.
    #[serde(default)]
    pub state: String,
}

/// Fiat deposit or withdrawal order.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatOrder {
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client-supplied ID.
    #[serde(default)]
    pub client_id: String,
    /// Order amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Fiat currency.
    #[serde(default)]
    pub ccy: String,
    /// Transaction fee.
    #[serde(default)]
    pub fee: NumberString,
    /// Payment account ID.
    #[serde(default)]
    pub payment_acct_id: String,
    /// Payment method.
    #[serde(default)]
    pub payment_method: String,
    /// Order state.
    #[serde(default)]
    pub state: String,
    /// Creation time in Unix milliseconds.
    #[serde(default)]
    pub c_time: NumberString,
    /// Update time in Unix milliseconds.
    #[serde(default)]
    pub u_time: NumberString,
}

/// Fiat withdrawal cancellation result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelFiatWithdrawal {
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Cancellation state.
    #[serde(default)]
    pub state: String,
}

/// Currency entry returned by the fiat buy/sell catalog.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatCurrency {
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
}

/// Fiat and crypto currencies available for buy/sell.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatBuySellCurrencies {
    /// Supported fiat currencies.
    #[serde(default)]
    pub fiat_ccy_list: Vec<FiatCurrency>,
    /// Supported crypto currencies.
    #[serde(default)]
    pub crypto_ccy_list: Vec<FiatCurrency>,
}

/// Supported fiat buy/sell pair and limits.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatBuySellPair {
    /// Supported side.
    #[serde(default)]
    pub side: String,
    /// Currency sold.
    #[serde(default)]
    pub from_ccy: String,
    /// Currency bought.
    #[serde(default)]
    pub to_ccy: String,
    /// Maximum single-trade amount.
    #[serde(default)]
    pub single_trade_max: NumberString,
    /// Minimum single-trade amount.
    #[serde(default)]
    pub single_trade_min: NumberString,
    /// Remaining fixed-price daily quota.
    #[serde(default)]
    pub fixed_px_remaining_daily_quota: NumberString,
    /// Fixed-price daily limit.
    #[serde(default)]
    pub fixed_px_daily_limit: NumberString,
    /// Supported payment methods.
    #[serde(default)]
    pub payment_methods: Vec<String>,
}

/// Fiat buy/sell quote.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatBuySellQuote {
    /// Quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Quote side.
    #[serde(default)]
    pub side: String,
    /// Currency sold.
    #[serde(default)]
    pub from_ccy: String,
    /// Currency bought.
    #[serde(default)]
    pub to_ccy: String,
    /// Requested RFQ amount.
    #[serde(default)]
    pub rfq_amt: NumberString,
    /// RFQ amount currency.
    #[serde(default)]
    pub rfq_ccy: String,
    /// Quote price.
    #[serde(default)]
    pub quote_px: NumberString,
    /// Quote-price currency.
    #[serde(default)]
    pub quote_ccy: String,
    /// Quoted amount in the sold currency.
    #[serde(default)]
    pub quote_from_amt: NumberString,
    /// Quoted amount in the bought currency.
    #[serde(default)]
    pub quote_to_amt: NumberString,
    /// Quote generation time in Unix milliseconds.
    #[serde(default)]
    pub quote_time: NumberString,
    /// Quote validity in milliseconds.
    #[serde(default)]
    pub ttl_ms: NumberString,
}

/// Fiat buy/sell trade or history row.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FiatBuySellTrade {
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default)]
    pub cl_ord_id: String,
    /// Quote ID.
    #[serde(default)]
    pub quote_id: String,
    /// Trade state.
    #[serde(default)]
    pub state: String,
    /// Trade side.
    #[serde(default)]
    pub side: String,
    /// Currency sold.
    #[serde(default)]
    pub from_ccy: String,
    /// Currency bought.
    #[serde(default)]
    pub to_ccy: String,
    /// Requested RFQ amount.
    #[serde(default)]
    pub rfq_amt: NumberString,
    /// RFQ amount currency.
    #[serde(default)]
    pub rfq_ccy: String,
    /// Filled price.
    #[serde(default)]
    pub fill_px: NumberString,
    /// Filled-price quote currency.
    #[serde(default)]
    pub fill_quote_ccy: String,
    /// Filled amount in the sold currency.
    #[serde(default)]
    pub fill_from_amt: NumberString,
    /// Filled amount in the bought currency.
    #[serde(default)]
    pub fill_to_amt: NumberString,
    /// Creation time in Unix milliseconds.
    #[serde(default)]
    pub c_time: NumberString,
    /// Update time in Unix milliseconds.
    #[serde(default)]
    pub u_time: NumberString,
}
