use serde::Deserialize;

use crate::model::NumberString;

/// Account bill row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBill {
    /// Bill ID.
    #[serde(default)]
    pub bill_id: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Margin mode.
    #[serde(default)]
    pub mgn_mode: String,
    /// Bill type.
    #[serde(rename = "type", default)]
    pub bill_type: String,
    /// Bill subtype.
    #[serde(default)]
    pub sub_type: String,
    /// Balance change.
    #[serde(default)]
    pub sz: NumberString,
    /// Balance after the change.
    #[serde(default)]
    pub bal: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}
