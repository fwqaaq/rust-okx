use serde::Deserialize;

use crate::{NumberString, model::InstType};

mod edge;

pub use edge::*;

/// A tradable instrument.
///
/// Only commonly used fields are modeled; the struct is `#[non_exhaustive]` and
/// unknown JSON fields are ignored, so OKX additions are non-breaking.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Instrument {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID, e.g. `BTC-USDT`.
    pub inst_id: String,
    /// Underlying, e.g. `BTC-USD` (derivatives only).
    #[serde(default)]
    pub uly: String,
    /// Instrument family, e.g. `BTC-USD` (derivatives only).
    #[serde(default)]
    pub inst_family: String,
    /// Base currency, e.g. `BTC` (spot/margin only).
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency, e.g. `USDT` (spot/margin only).
    #[serde(default)]
    pub quote_ccy: String,
    /// Settlement currency (derivatives only).
    #[serde(default)]
    pub settle_ccy: String,
    /// Lot size (order size increment).
    #[serde(default)]
    pub lot_sz: NumberString,
    /// Tick size (price increment).
    #[serde(default)]
    pub tick_sz: NumberString,
    /// Minimum order size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Instrument lifecycle state, e.g. `live`, `suspend`.
    #[serde(default)]
    pub state: String,
}

/// OKX system time.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SystemTime {
    /// Current OKX system timestamp in Unix milliseconds.
    pub ts: NumberString,
}

/// Open interest for an instrument.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OpenInterest {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Open interest in contracts.
    #[serde(default)]
    pub oi: NumberString,
    /// Open interest in coin/currency units.
    #[serde(default)]
    pub oi_ccy: NumberString,
    /// Open interest in USD.
    #[serde(default)]
    pub oi_usd: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Current funding-rate information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRate {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Current funding rate.
    #[serde(default)]
    pub funding_rate: NumberString,
    /// Next estimated funding rate.
    #[serde(default)]
    pub next_funding_rate: NumberString,
    /// Funding time (Unix milliseconds).
    #[serde(default)]
    pub funding_time: NumberString,
    /// Next funding time (Unix milliseconds).
    #[serde(default)]
    pub next_funding_time: NumberString,
}

/// Historical funding-rate row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRateHistory {
    /// Instrument ID.
    pub inst_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Funding rate formula type.
    #[serde(default)]
    pub formula_type: String,
    /// Funding rate.
    #[serde(default)]
    pub funding_rate: NumberString,
    /// Realized funding rate.
    #[serde(default)]
    pub realized_rate: NumberString,
    /// Funding time (Unix milliseconds).
    #[serde(default)]
    pub funding_time: NumberString,
    /// Funding method.
    #[serde(default)]
    pub method: String,
}

/// Price-limit information for an instrument.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PriceLimit {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Highest buy price.
    #[serde(default)]
    pub buy_lmt: NumberString,
    /// Lowest sell price.
    #[serde(default)]
    pub sell_lmt: NumberString,
    /// Whether the price limit is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Mark-price information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MarkPrice {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Delivery/exercise history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeliveryExercise {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Delivery/exercise price.
    #[serde(default)]
    pub px: NumberString,
    /// Delivery/exercise type.
    #[serde(rename = "type", default)]
    pub exercise_type: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Public position-tier information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionTier {
    /// Instrument type.
    pub inst_type: InstType,
    /// Trade mode.
    #[serde(default)]
    pub td_mode: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Tier.
    #[serde(default)]
    pub tier: String,
    /// Minimum size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Maximum size.
    #[serde(default)]
    pub max_sz: NumberString,
    /// Initial margin rate.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin rate.
    #[serde(default)]
    pub mmr: NumberString,
}

/// Insurance-fund snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InsuranceFund {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Fund type.
    #[serde(rename = "type", default)]
    pub fund_type: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Balance amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Contract/coin conversion result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertContractCoin {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Converted size.
    #[serde(default)]
    pub sz: NumberString,
    /// Conversion price.
    #[serde(default)]
    pub px: NumberString,
    /// Conversion unit.
    #[serde(default)]
    pub unit: String,
}
