use serde::Deserialize;

use crate::model::NumberString;

/// Currencies supported by the trading-statistics endpoints.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TradingDataSupportCoins {
    /// Currencies supported by derivatives statistics.
    #[serde(default)]
    pub contract: Vec<String>,
    /// Currencies supported by option statistics.
    #[serde(default)]
    pub option: Vec<String>,
    /// Currencies supported by spot statistics.
    #[serde(default)]
    pub spot: Vec<String>,
}

/// Contract open-interest history row: `[ts, oi, oiCcy, oiUsd]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "ContractOpenInterestHistoryRaw")]
#[non_exhaustive]
pub struct ContractOpenInterestHistory {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Open interest in contracts.
    pub oi: NumberString,
    /// Open interest in cryptocurrency.
    pub oi_ccy: NumberString,
    /// Open interest in USD.
    pub oi_usd: NumberString,
}

type ContractOpenInterestHistoryRaw = (NumberString, NumberString, NumberString, NumberString);

impl From<ContractOpenInterestHistoryRaw> for ContractOpenInterestHistory {
    fn from(raw: ContractOpenInterestHistoryRaw) -> Self {
        Self {
            ts: raw.0,
            oi: raw.1,
            oi_ccy: raw.2,
            oi_usd: raw.3,
        }
    }
}

/// Taker-volume history row: `[ts, sellVol, buyVol]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "TakerVolumeRaw")]
#[non_exhaustive]
pub struct TakerVolume {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Taker sell volume.
    pub sell_vol: NumberString,
    /// Taker buy volume.
    pub buy_vol: NumberString,
}

type TakerVolumeRaw = (NumberString, NumberString, NumberString);

impl From<TakerVolumeRaw> for TakerVolume {
    fn from(raw: TakerVolumeRaw) -> Self {
        Self {
            ts: raw.0,
            sell_vol: raw.1,
            buy_vol: raw.2,
        }
    }
}

/// Two-column ratio row: `[ts, ratio]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "RatioPointRaw")]
#[non_exhaustive]
pub struct RatioPoint {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Documented ratio value.
    pub ratio: NumberString,
}

type RatioPointRaw = (NumberString, NumberString);

impl From<RatioPointRaw> for RatioPoint {
    fn from(raw: RatioPointRaw) -> Self {
        Self {
            ts: raw.0,
            ratio: raw.1,
        }
    }
}

/// Contract open-interest and volume row: `[ts, oi, vol]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "OpenInterestVolumeRaw")]
#[non_exhaustive]
pub struct OpenInterestVolume {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Open interest.
    pub oi: NumberString,
    /// Trading volume.
    pub vol: NumberString,
}

type OpenInterestVolumeRaw = (NumberString, NumberString, NumberString);

impl From<OpenInterestVolumeRaw> for OpenInterestVolume {
    fn from(raw: OpenInterestVolumeRaw) -> Self {
        Self {
            ts: raw.0,
            oi: raw.1,
            vol: raw.2,
        }
    }
}

/// Option put/call ratio row: `[ts, oiRatio, volRatio]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "PutCallRatioRaw")]
#[non_exhaustive]
pub struct PutCallRatio {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Put/call open-interest ratio.
    pub oi_ratio: NumberString,
    /// Put/call trading-volume ratio.
    pub vol_ratio: NumberString,
}

type PutCallRatioRaw = (NumberString, NumberString, NumberString);

impl From<PutCallRatioRaw> for PutCallRatio {
    fn from(raw: PutCallRatioRaw) -> Self {
        Self {
            ts: raw.0,
            oi_ratio: raw.1,
            vol_ratio: raw.2,
        }
    }
}

/// Option open-interest and volume by expiry:
/// `[ts, expTime, callOI, putOI, callVol, putVol]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "OptionExpiryVolumeRaw")]
#[non_exhaustive]
pub struct OptionExpiryVolume {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Contract expiry date in `YYYYMMDD` format.
    pub exp_time: String,
    /// Total call open interest.
    pub call_oi: NumberString,
    /// Total put open interest.
    pub put_oi: NumberString,
    /// Total call trading volume.
    pub call_vol: NumberString,
    /// Total put trading volume.
    pub put_vol: NumberString,
}

type OptionExpiryVolumeRaw = (
    NumberString,
    String,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
);

impl From<OptionExpiryVolumeRaw> for OptionExpiryVolume {
    fn from(raw: OptionExpiryVolumeRaw) -> Self {
        Self {
            ts: raw.0,
            exp_time: raw.1,
            call_oi: raw.2,
            put_oi: raw.3,
            call_vol: raw.4,
            put_vol: raw.5,
        }
    }
}

/// Option open-interest and volume by strike:
/// `[ts, strike, callOI, putOI, callVol, putVol]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "OptionStrikeVolumeRaw")]
#[non_exhaustive]
pub struct OptionStrikeVolume {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Option strike price.
    pub strike: NumberString,
    /// Total call open interest.
    pub call_oi: NumberString,
    /// Total put open interest.
    pub put_oi: NumberString,
    /// Total call trading volume.
    pub call_vol: NumberString,
    /// Total put trading volume.
    pub put_vol: NumberString,
}

type OptionStrikeVolumeRaw = (
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
);

impl From<OptionStrikeVolumeRaw> for OptionStrikeVolume {
    fn from(raw: OptionStrikeVolumeRaw) -> Self {
        Self {
            ts: raw.0,
            strike: raw.1,
            call_oi: raw.2,
            put_oi: raw.3,
            call_vol: raw.4,
            put_vol: raw.5,
        }
    }
}

/// Current option taker-flow values:
/// `[ts, callBuyVol, callSellVol, putBuyVol, putSellVol, callBlockVol, putBlockVol]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "OptionTakerFlowRaw")]
#[non_exhaustive]
pub struct OptionTakerFlow {
    /// Timestamp in Unix milliseconds.
    pub ts: NumberString,
    /// Call-option taker buy volume.
    pub call_buy_vol: NumberString,
    /// Call-option taker sell volume.
    pub call_sell_vol: NumberString,
    /// Put-option taker buy volume.
    pub put_buy_vol: NumberString,
    /// Put-option taker sell volume.
    pub put_sell_vol: NumberString,
    /// Call block volume.
    pub call_block_vol: NumberString,
    /// Put block volume.
    pub put_block_vol: NumberString,
}

type OptionTakerFlowRaw = (
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
    NumberString,
);

impl From<OptionTakerFlowRaw> for OptionTakerFlow {
    fn from(raw: OptionTakerFlowRaw) -> Self {
        Self {
            ts: raw.0,
            call_buy_vol: raw.1,
            call_sell_vol: raw.2,
            put_buy_vol: raw.3,
            put_sell_vol: raw.4,
            call_block_vol: raw.5,
            put_block_vol: raw.6,
        }
    }
}
