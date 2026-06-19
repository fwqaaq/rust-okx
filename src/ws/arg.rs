//! WebSocket subscription arguments.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A WebSocket channel argument used in subscribe/unsubscribe requests and
/// returned in channel acknowledgements/data pushes.
///
/// Use [`Arg::new`] for a channel-only subscription, then add standard
/// instrument filters or custom channel parameters with consuming setters.
///
/// OKX docs: <https://www.okx.com/docs-v5/en/#overview-websocket-subscribe>
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Arg {
    /// OKX channel name, e.g. `tickers`, `books5`, `account`, or `orders`.
    pub channel: String,
    /// Instrument ID, e.g. `BTC-USDT`.
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Instrument type, e.g. `SPOT` or `SWAP`.
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument family, e.g. `BTC-USD`.
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Additional channel parameters not modeled as first-class fields.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

impl Arg {
    /// Create a channel argument.
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            inst_id: None,
            inst_type: None,
            inst_family: None,
            extra: BTreeMap::new(),
        }
    }

    /// Set the instrument ID.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the instrument type.
    pub fn inst_type(mut self, inst_type: impl Into<String>) -> Self {
        self.inst_type = Some(inst_type.into());
        self
    }

    /// Set the instrument family.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }

    /// Set an arbitrary channel parameter.
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }

    /// Set the extraParams parameter.
    pub fn extra_param(self, update_interval: impl Into<String>) -> Self {
        let extra = format!("{{\"updateInterval\":\"{}\"}}", update_interval.into());
        self.param("extraParams", extra)
    }

    /// Set the currency parameter.
    pub fn ccy(self, ccy: impl Into<String>) -> Self {
        self.param("ccy", ccy)
    }

    /// Set the spread ID parameter.
    pub fn sprd_id(self, sprd_id: impl Into<String>) -> Self {
        self.param("sprdId", sprd_id)
    }

    /// Set the algo order ID parameter.
    pub fn algo_id(self, algo_id: impl Into<String>) -> Self {
        self.param("algoId", algo_id)
    }

    /// Set the grid type parameter.
    pub fn grid_type(self, grid_type: impl Into<String>) -> Self {
        self.param("gridType", grid_type)
    }
}
