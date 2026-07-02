use std::fmt;

use serde::{Deserialize, Deserializer};

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

/// Mapping for an account bill type and its subtypes.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BillSubtypeMapping {
    /// Bill type.
    #[serde(rename = "type", default)]
    pub bill_type: String,
    /// Bill type description. Empty means the type is not enabled.
    #[serde(default)]
    pub type_desc: String,
    /// Sub-type details.
    #[serde(default)]
    pub sub_type_details: Vec<BillSubtypeDetail>,
}

/// Account bill subtype details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BillSubtypeDetail {
    /// Bill subtype.
    #[serde(default)]
    pub sub_type: String,
    /// Bill subtype description. Empty means the subtype is not enabled.
    #[serde(default)]
    pub sub_type_desc: String,
}

/// Status returned after applying for a historical bills archive.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BillsHistoryArchiveStatus {
    /// A download link already exists.
    LinkAvailable,
    /// The file is being generated.
    Generating,
    /// A value not modeled by this crate version.
    Unknown(String),
}

impl BillsHistoryArchiveStatus {
    /// Return the OKX wire representation of this status.
    pub fn as_str(&self) -> &str {
        match self {
            Self::LinkAvailable => "true",
            Self::Generating => "false",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for BillsHistoryArchiveStatus {
    fn from(value: &str) -> Self {
        match value {
            "true" => Self::LinkAvailable,
            "false" => Self::Generating,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for BillsHistoryArchiveStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for BillsHistoryArchiveStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self::from(value.as_str()))
    }
}

/// Result returned after applying for historical account-bills archive generation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ApplyBillsHistoryArchiveResult {
    /// Whether the download link already exists or is being generated.
    #[serde(rename = "result")]
    pub status: BillsHistoryArchiveStatus,
    /// First request time received by OKX, as Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
}

/// Download-link state for historical account-bills archive files.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BillsHistoryArchiveFileState {
    /// The download link is ready.
    Finished,
    /// The archive file is still being generated.
    Ongoing,
    /// File generation failed; apply again.
    Failed,
    /// A value not modeled by this crate version.
    Unknown(String),
}

impl BillsHistoryArchiveFileState {
    /// Return the OKX wire representation of this state.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Finished => "finished",
            Self::Ongoing => "ongoing",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}

impl From<&str> for BillsHistoryArchiveFileState {
    fn from(value: &str) -> Self {
        match value {
            "finished" => Self::Finished,
            "ongoing" => Self::Ongoing,
            "failed" => Self::Failed,
            other => Self::Unknown(other.to_owned()),
        }
    }
}

impl fmt::Display for BillsHistoryArchiveFileState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for BillsHistoryArchiveFileState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Ok(Self::from(value.as_str()))
    }
}

/// Historical account-bills archive download-link information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BillsHistoryArchiveFile {
    /// Download file link.
    #[serde(default)]
    pub file_href: String,
    /// Download-link status.
    pub state: BillsHistoryArchiveFileState,
    /// First request time received by OKX, as Unix milliseconds.
    #[serde(default)]
    pub ts: NumberString,
}
