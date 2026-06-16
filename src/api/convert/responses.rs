use std::fmt;

use serde::{Deserialize, Deserializer};

use crate::model::{NumberString, OrderSide};

/// A currency supported by OKX Convert.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertCurrency {
    /// Currency code, e.g. `BTC`.
    pub ccy: String,

    /// Deprecated minimum amount to convert.
    ///
    /// OKX currently returns an empty string for this field.
    #[serde(default)]
    pub min: NumberString,

    /// Deprecated maximum amount to convert.
    ///
    /// OKX currently returns an empty string for this field.
    #[serde(default)]
    pub max: NumberString,
}

/// Limits and metadata for a currency pair supported by OKX Convert.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertCurrencyPair {
    /// Currency pair, e.g. `BTC-USDT`.
    pub inst_id: String,

    /// Base currency, e.g. `BTC` in `BTC-USDT`.
    pub base_ccy: String,

    /// Maximum amount of base currency that can be converted.
    pub base_ccy_max: NumberString,

    /// Minimum amount of base currency that can be converted.
    pub base_ccy_min: NumberString,

    /// Quote currency, e.g. `USDT` in `BTC-USDT`.
    pub quote_ccy: String,

    /// Maximum amount of quote currency that can be converted.
    pub quote_ccy_max: NumberString,

    /// Minimum amount of quote currency that can be converted.
    pub quote_ccy_min: NumberString,
}

/// A quote returned by the Convert estimate-quote endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertQuote {
    /// Quotation generation time as a Unix timestamp in milliseconds.
    pub quote_time: NumberString,

    /// Quote validity period in milliseconds.
    pub ttl_ms: NumberString,

    /// Client-supplied quote request ID, when supplied.
    #[serde(default)]
    pub cl_q_req_id: String,

    /// Server-assigned quote ID.
    pub quote_id: String,

    /// Base currency, e.g. `BTC` in `BTC-USDT`.
    pub base_ccy: String,

    /// Quote currency, e.g. `USDT` in `BTC-USDT`.
    pub quote_ccy: String,

    /// Trade side based on the base currency.
    pub side: OrderSide,

    /// Original RFQ amount requested by the client.
    pub orig_rfq_sz: NumberString,

    /// Actual RFQ amount accepted for the quote.
    pub rfq_sz: NumberString,

    /// Currency in which the RFQ amount is denominated.
    pub rfq_sz_ccy: String,

    /// Conversion price denominated in the quote currency.
    pub cnvt_px: NumberString,

    /// Quoted amount of the base currency.
    pub base_sz: NumberString,

    /// Quoted amount of the quote currency.
    pub quote_sz: NumberString,
}

/// State of a Convert trade.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConvertTradeState {
    /// The conversion completed successfully.
    FullyFilled,

    /// The conversion was rejected.
    Rejected,

    /// A state not modeled by this crate version.
    Unknown(String),
}

impl ConvertTradeState {
    /// Return the OKX wire representation of this state.
    pub fn as_str(&self) -> &str {
        match self {
            Self::FullyFilled => "fullyFilled",
            Self::Rejected => "rejected",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for ConvertTradeState {
    fn from(value: &str) -> Self {
        match value {
            "fullyFilled" => Self::FullyFilled,
            "rejected" => Self::Rejected,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for ConvertTradeState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ConvertTradeState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self::from(value.as_str()))
    }
}

/// Result returned after executing a Convert trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertTradeResult {
    /// Server-assigned trade ID.
    pub trade_id: String,

    /// Quote ID used to execute the conversion.
    pub quote_id: String,

    /// Client-supplied trade request ID, when supplied.
    #[serde(default)]
    pub cl_t_req_id: String,

    /// Trade execution state.
    pub state: ConvertTradeState,

    /// Currency pair, e.g. `BTC-USDT`.
    pub inst_id: String,

    /// Base currency, e.g. `BTC` in `BTC-USDT`.
    pub base_ccy: String,

    /// Quote currency, e.g. `USDT` in `BTC-USDT`.
    pub quote_ccy: String,

    /// Trade side based on the base currency.
    pub side: OrderSide,

    /// Filled price denominated in the quote currency.
    pub fill_px: NumberString,

    /// Filled amount of the base currency.
    pub fill_base_sz: NumberString,

    /// Filled amount of the quote currency.
    pub fill_quote_sz: NumberString,

    /// Convert trade time as a Unix timestamp in milliseconds.
    pub ts: NumberString,
}

/// A historical Convert trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertHistory {
    /// Server-assigned trade ID.
    pub trade_id: String,

    /// Client-supplied trade request ID, when supplied.
    #[serde(default)]
    pub cl_t_req_id: String,

    /// Trade execution state.
    pub state: ConvertTradeState,

    /// Currency pair, e.g. `BTC-USDT`.
    pub inst_id: String,

    /// Base currency, e.g. `BTC` in `BTC-USDT`.
    pub base_ccy: String,

    /// Quote currency, e.g. `USDT` in `BTC-USDT`.
    pub quote_ccy: String,

    /// Trade side based on the base currency.
    pub side: OrderSide,

    /// Filled price denominated in the quote currency.
    pub fill_px: NumberString,

    /// Filled amount of the base currency.
    pub fill_base_sz: NumberString,

    /// Filled amount of the quote currency.
    pub fill_quote_sz: NumberString,

    /// Convert trade time as a Unix timestamp in milliseconds.
    pub ts: NumberString,
}
