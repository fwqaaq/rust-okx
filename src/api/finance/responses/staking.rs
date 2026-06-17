use crate::model::NumberString;
use serde::Deserialize;

/// Currency-level investment data returned by Staking/DeFi endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiInvestData {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `earnings` field.
    #[serde(default)]
    pub earnings: NumberString,
}

/// Staking/DeFi offer row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiOffer {
    /// Value returned by OKX in the `productId` field.
    #[serde(default)]
    pub product_id: String,
    /// Value returned by OKX in the `protocolType` field.
    #[serde(default)]
    pub protocol_type: String,
    /// Value returned by OKX in the `name` field.
    #[serde(default)]
    pub name: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `term` field.
    #[serde(default)]
    pub term: String,
    /// Value returned by OKX in the `apy` field.
    #[serde(default)]
    pub apy: NumberString,
    /// Value returned by OKX in the `state` field.
    #[serde(default)]
    pub state: String,
    /// Value returned by OKX in the `investData` field.
    #[serde(default)]
    pub invest_data: Vec<StakingDefiInvestData>,
}

/// Staking/DeFi order row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingDefiOrder {
    /// Value returned by OKX in the `ordId` field.
    #[serde(default)]
    pub ord_id: String,
    /// Value returned by OKX in the `productId` field.
    #[serde(default)]
    pub product_id: String,
    /// Value returned by OKX in the `protocolType` field.
    #[serde(default)]
    pub protocol_type: String,
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `state` field.
    #[serde(default)]
    pub state: String,
    /// Value returned by OKX in the `term` field.
    #[serde(default)]
    pub term: String,
    /// Value returned by OKX in the `apy` field.
    #[serde(default)]
    pub apy: NumberString,
    /// Value returned by OKX in the `cTime` field.
    #[serde(default)]
    pub c_time: NumberString,
    /// Value returned by OKX in the `uTime` field.
    #[serde(default)]
    pub u_time: NumberString,
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
    /// Value returned by OKX in the `earnings` field.
    #[serde(default)]
    pub earnings: NumberString,
}

/// Staking history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingHistory {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `amt` field.
    #[serde(default)]
    pub amt: NumberString,
    /// Value returned by OKX in the `type` field.
    #[serde(default, rename = "type")]
    pub event_type: String,
    /// Value returned by OKX in the `state` field.
    #[serde(default)]
    pub state: String,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}

/// Staking APY-history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct StakingApyHistory {
    /// Value returned by OKX in the `ccy` field.
    #[serde(default)]
    pub ccy: String,
    /// Value returned by OKX in the `apy` field.
    #[serde(default)]
    pub apy: NumberString,
    /// Value returned by OKX in the `ts` field.
    #[serde(default)]
    pub ts: NumberString,
}
