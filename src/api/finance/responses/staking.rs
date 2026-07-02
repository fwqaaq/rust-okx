use crate::model::NumberString;
use serde::Deserialize;

/// Currency-level investment data returned by the Staking/DeFi offers endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiInvestData {
    /// Investment currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Available balance to invest.
    #[serde(default)]
    pub bal: NumberString,
    /// Minimum subscription amount.
    #[serde(default)]
    pub min_amt: NumberString,
    /// Maximum available subscription amount.
    #[serde(default)]
    pub max_amt: NumberString,
}

/// Earning data returned by the Staking/DeFi offers endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiEarningData {
    /// Earning currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Earning type.
    ///
    /// `0`: Estimated earning, `1`: Cumulative earning.
    #[serde(default)]
    pub earning_type: String,
}

/// Staking/DeFi offer row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiOffer {
    /// Currency type, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `productId` field.
    #[serde(default)]
    pub product_id: String,
    /// Protocol name.
    #[serde(default)]
    pub protocol: String,
    /// Protocol type.
    ///
    /// `defi`: on-chain earn.
    #[serde(default)]
    pub protocol_type: String,
    /// Protocol term.
    ///
    /// Returns the days of a fixed term, or `0` for a flexible product.
    #[serde(default)]
    pub term: String,
    /// Estimated annualization. `0.07` represents 7%.
    #[serde(default)]
    pub apy: NumberString,
    /// Whether the protocol supports early redemption.
    #[serde(default)]
    pub early_redeem: bool,
    /// Current target currency information available for investment.
    #[serde(default)]
    pub invest_data: Vec<StakingDefiInvestData>,
    /// Earning data.
    #[serde(default)]
    pub earning_data: Vec<StakingDefiEarningData>,
    /// Product state.
    ///
    /// `purchasable`: Purchasable, `sold_out`: Sold out,
    /// `Stop`: Suspension of subscription.
    #[serde(default)]
    pub state: String,
    /// Redemption period, format `[min time, max time]` where `H`: Hour, `D`: Day.
    ///
    /// e.g. `["1H","24H"]` or `["14D","14D"]`.
    #[serde(default)]
    pub redeem_period: Vec<String>,
    /// Fast redemption daily limit.
    ///
    /// Returns `''` if fast redemption is not supported.
    #[serde(default)]
    pub fast_redemption_daily_limit: NumberString,
}

/// Currency-level investment data returned by the Staking/DeFi order endpoints
/// (active orders and order history).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiOrderInvestData {
    /// Investment currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Invested amount.
    #[serde(default)]
    pub amt: NumberString,
}

/// Earning data returned by the Staking/DeFi order endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiOrderEarningData {
    /// Earning currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Earning type.
    ///
    /// `0`: Estimated earning, `1`: Cumulative earning.
    #[serde(default)]
    pub earning_type: String,
    /// Earning amount. Returned by the active-orders endpoint.
    #[serde(default)]
    pub earnings: NumberString,
    /// Cumulative earning of redeemed orders. Returned by the order-history
    /// endpoint; only valid when the order is in a redemption state.
    #[serde(default)]
    pub realized_earnings: NumberString,
}

/// Fast redemption data returned by the Staking/DeFi active-orders endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiFastRedemptionData {
    /// Currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Redeeming amount.
    #[serde(default)]
    pub redeeming_amt: NumberString,
}

/// Staking/DeFi order row, returned by both the active-orders and order-history
/// endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiOrder {
    /// Currency, e.g. `BTC`.
    #[serde(default)]
    pub ccy: String,
    /// Order ID.
    #[serde(default)]
    pub ord_id: String,
    /// Product ID.
    #[serde(default)]
    pub product_id: String,
    /// Order state.
    ///
    /// Active orders: `8`: Pending, `13`: Cancelling, `9`: Onchain,
    /// `1`: Earning, `2`: Redeeming.
    /// Order history: `3`: Completed (including canceled and redeemed).
    #[serde(default)]
    pub state: String,
    /// Protocol.
    #[serde(default)]
    pub protocol: String,
    /// Protocol type.
    ///
    /// `defi`: on-chain earn.
    #[serde(default)]
    pub protocol_type: String,
    /// Protocol term.
    ///
    /// Returns the days of a fixed term, or `0` for a flexible product.
    #[serde(default)]
    pub term: String,
    /// Estimated APY. `0.07` represents 7%. Retained to 4 decimal places
    /// (truncated).
    #[serde(default)]
    pub apy: NumberString,
    /// Investment data.
    #[serde(default)]
    pub invest_data: Vec<StakingDefiOrderInvestData>,
    /// Earning data.
    #[serde(default)]
    pub earning_data: Vec<StakingDefiOrderEarningData>,
    /// Fast redemption data. Only returned by the active-orders endpoint.
    #[serde(default)]
    pub fast_redemption_data: Vec<StakingDefiFastRedemptionData>,
    /// Order purchased time, Unix timestamp in milliseconds, e.g.
    /// `1597026383085`.
    #[serde(default)]
    pub purchased_time: NumberString,
    /// Estimated redemption settlement time. Only returned by the active-orders
    /// endpoint.
    #[serde(default)]
    pub est_settlement_time: NumberString,
    /// Deadline for cancellation of a redemption application. Only returned by
    /// the active-orders endpoint.
    #[serde(default)]
    pub cancel_redemption_deadline: NumberString,
    /// Order redeemed time. Only returned by the order-history endpoint.
    #[serde(default)]
    pub redeemed_time: NumberString,
    /// Order tag.
    #[serde(default)]
    pub tag: String,
}

/// Staking product-info row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingProductInfo {
    /// Fast redemption daily limit.
    /// The master account and sub-accounts share the same limit.
    #[serde(default)]
    pub fast_redemption_daily_limit: NumberString,
    /// The latest rate for checking crypto.
    #[serde(default)]
    pub rate: NumberString,
    /// Redemption days of crypto.
    #[serde(default)]
    pub redempt_days: NumberString,
    /// Minimum subscription amount of crypto.
    #[serde(default)]
    pub min_amt: NumberString,
    /// urrently fast redemption max available amount
    /// Only for `SOL_PRODUCT_INFO`
    #[serde(default)]
    pub fast_redemption_avail: Option<NumberString>,
}

/// Staking order row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingOrder {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `state` field.
    #[serde(default)]
    pub state: String,
    /// Value returned by OKX in the `type` field.
    #[serde(default, rename = "type")]
    pub order_type: String,
    /// Value returned by OKX in the `cTime` field.
    #[serde(default)]
    pub c_time: NumberString,
}

/// Cancel redeem ETH
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelRedeem {
    /// Value returned by OKX in the `ordId` field
    #[serde(default)]
    pub ord_id: String,
}

/// Staking balance row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingBalance {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `latestInterestAccrual` field.
    #[serde(default)]
    pub latest_interest_accrual: NumberString,
    /// Value returned by OKX in the `totalInterestAccrual` field.
    #[serde(default)]
    pub total_interest_accrual: NumberString,
    /// Value returned by OKX in the `ts` field. Present for ETH balance;
    /// absent for SOL balance.
    #[serde(default)]
    pub ts: NumberString,
}

/// Staking purchase/redeem history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingHistory {
    /// Value returned by OKX in the `type` field.
    #[serde(default, rename = "type")]
    pub event_type: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `redeemingAmt` field.
    #[serde(default)]
    pub redeeming_amt: NumberString,
    /// Value returned by OKX in the `status` field.
    #[serde(default)]
    pub status: String,
    /// Value returned by OKX in the `ordId` field. Present for ETH history;
    /// absent for SOL history.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `requestTime` field.
    #[serde(default)]
    pub request_time: NumberString,
    /// Value returned by OKX in the `completedTime` field.
    #[serde(default)]
    pub completed_time: NumberString,
    /// Value returned by OKX in the `estCompletedTime` field.
    #[serde(default)]
    pub est_completed_time: NumberString,
}

/// Staking APY-history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingApyHistory {
    /// Value returned by OKX in the `rate` field.
    #[serde(default)]
    pub rate: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}
