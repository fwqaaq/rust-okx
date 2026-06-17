//! Shared data types: the [`NumberString`] wrapper, the response envelope, and
//! the common string enums used across the API modules.

use std::fmt;
use std::str::FromStr;

use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize};

mod validation;

pub use validation::{RequestValidationError, ValidateRequest};
pub(crate) use validation::{
    at_least_one, at_most_one, exactly_one, length_range, max_length, non_empty,
    optional_non_empty, range_u64, reject_when_present, require_when, validate_client_request_id,
    validate_side,
};

/// The OKX response envelope: `{ "code": "...", "msg": "...", "data": [...] }`.
///
/// Internal — the client unwraps it and returns `data` (or an [`Error::Api`]).
///
/// [`Error::Api`]: crate::Error::Api
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct OkxResponse<D> {
    pub code: String,
    pub msg: String,
    pub data: D,
}

/// A numeric value returned by OKX as a JSON string.
///
/// OKX encodes all prices, sizes, and balances as strings to avoid floating
/// point precision loss. `NumberString` preserves the exact wire representation
/// and lets the caller decide how to interpret it:
///
/// ```
/// use rust_okx::NumberString;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let px = NumberString::from("42000.1");
/// assert_eq!(px.as_str(), "42000.1");
/// let as_f64: f64 = px.parse()?;
/// assert_eq!(as_f64, 42000.1);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NumberString(String);

impl NumberString {
    /// Borrow the raw string value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` if the value is the empty string (OKX uses `""` for
    /// "not applicable" fields).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Parse the value into any [`FromStr`] type, e.g. `f64`, `i64`, or
    /// `rust_decimal::Decimal`.
    pub fn parse<T: FromStr>(&self) -> Result<T, T::Err> {
        self.0.parse()
    }

    /// Consume the wrapper and return the inner [`String`].
    pub fn into_string(self) -> String {
        self.0
    }

    /// Parse the value as a [`rust_decimal::Decimal`].
    #[cfg(feature = "rust-decimal")]
    pub fn to_decimal(&self) -> Result<rust_decimal::Decimal, rust_decimal::Error> {
        self.0.parse()
    }
}

impl From<String> for NumberString {
    fn from(s: String) -> Self {
        NumberString(s)
    }
}

impl From<&str> for NumberString {
    fn from(s: &str) -> Self {
        NumberString(s.to_owned())
    }
}

impl AsRef<str> for NumberString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NumberString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A flexible, untyped request-parameter builder for unsupported or newly
/// introduced OKX fields.
///
/// Prefer endpoint-specific request types whenever one exists. Adding the same
/// key more than once replaces its previous value while preserving insertion
/// order, preventing duplicate JSON object keys.
#[derive(Debug, Clone, Default)]
pub struct RawRequestParams {
    fields: Vec<(String, ParamValue)>,
}

impl RawRequestParams {
    /// Create an empty parameter set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or replace a string parameter.
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.set(key.into(), ParamValue::String(value.into()));
        self
    }

    /// Add or replace a boolean parameter.
    pub fn bool_param(mut self, key: impl Into<String>, value: bool) -> Self {
        self.set(key.into(), ParamValue::Bool(value));
        self
    }

    /// Add or replace an array-of-strings parameter.
    pub fn string_list<I, S>(mut self, key: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.set(
            key.into(),
            ParamValue::StringList(values.into_iter().map(Into::into).collect()),
        );
        self
    }

    /// Return true when no parameters are set.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    fn set(&mut self, key: String, value: ParamValue) {
        if let Some((_, existing)) = self.fields.iter_mut().find(|(name, _)| name == &key) {
            *existing = value;
        } else {
            self.fields.push((key, value));
        }
    }
}

impl Serialize for RawRequestParams {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.fields.len()))?;
        for (key, value) in &self.fields {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

/// Backward-compatible name for [`RawRequestParams`].
///
/// New endpoint implementations should expose typed request structs instead of
/// accepting this alias directly.
pub type RequestParams = RawRequestParams;

#[derive(Debug, Clone)]
enum ParamValue {
    String(String),
    Bool(bool),
    StringList(Vec<String>),
}

impl Serialize for ParamValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String(value) => serializer.serialize_str(value),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::StringList(values) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    seq.serialize_element(value)?;
                }
                seq.end()
            }
        }
    }
}

/// A broad OKX response row for low-frequency endpoints.
///
/// OKX edge endpoints often return sparse, feature-dependent objects. Fields
/// default when absent so deserialization remains forward-compatible.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RestRow {
    /// Instrument type.
    #[serde(default, rename = "instType")]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default, rename = "instId")]
    pub inst_id: String,
    /// Instrument family.
    #[serde(default, rename = "instFamily")]
    pub inst_family: String,
    /// Currency code.
    #[serde(default)]
    pub ccy: String,
    /// Order ID.
    #[serde(default, rename = "ordId")]
    pub ord_id: String,
    /// Client order ID.
    #[serde(default, rename = "clOrdId")]
    pub cl_ord_id: String,
    /// Algo order ID.
    #[serde(default, rename = "algoId")]
    pub algo_id: String,
    /// Client algo order ID.
    #[serde(default, rename = "algoClOrdId")]
    pub algo_cl_ord_id: String,
    /// Quote ID.
    #[serde(default, rename = "quoteId")]
    pub quote_id: String,
    /// Request ID.
    #[serde(default, rename = "reqId")]
    pub req_id: String,
    /// Product ID.
    #[serde(default, rename = "productId")]
    pub product_id: String,
    /// Operation type.
    #[serde(default, rename = "type")]
    pub row_type: String,
    /// State or status.
    #[serde(default)]
    pub state: String,
    /// Status.
    #[serde(default)]
    pub status: String,
    /// Side.
    #[serde(default)]
    pub side: String,
    /// Amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Size.
    #[serde(default)]
    pub sz: NumberString,
    /// Price.
    #[serde(default)]
    pub px: NumberString,
    /// Rate.
    #[serde(default)]
    pub rate: NumberString,
    /// Balance.
    #[serde(default)]
    pub bal: NumberString,
    /// Available balance.
    #[serde(default)]
    pub avail_bal: NumberString,
    /// Timestamp.
    #[serde(default)]
    pub ts: NumberString,
    /// Success code.
    #[serde(default, rename = "sCode")]
    pub s_code: String,
    /// Success message.
    #[serde(default, rename = "sMsg")]
    pub s_msg: String,
}

/// Defines a string-backed enum that round-trips through the OKX wire format and
/// tolerates unknown values via an `Unknown(String)` fallback variant. This
/// keeps response deserialization non-breaking when OKX adds new values.
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident = $wire:literal ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        $vis enum $name {
            $( $(#[$vmeta])* $variant, )*
            /// A value not modeled by this version of the crate; the raw string
            /// is preserved.
            Unknown(String),
        }

        impl $name {
            /// The OKX wire representation of this value.
            pub fn as_str(&self) -> &str {
                match self {
                    $( $name::$variant => $wire, )*
                    $name::Unknown(s) => s.as_str(),
                }
            }
        }

        impl ::core::convert::From<&str> for $name {
            fn from(s: &str) -> Self {
                match s {
                    $( $wire => $name::$variant, )*
                    other => $name::Unknown(other.to_owned()),
                }
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, ser: S) -> ::core::result::Result<S::Ok, S::Error> {
                ser.serialize_str(self.as_str())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(de: D) -> ::core::result::Result<Self, D::Error> {
                let s = <::std::string::String as ::serde::Deserialize>::deserialize(de)?;
                ::core::result::Result::Ok($name::from(s.as_str()))
            }
        }
    };
}

string_enum! {
    /// Instrument type.
    pub enum InstType {
        /// Spot.
        Spot = "SPOT",
        /// Margin.
        Margin = "MARGIN",
        /// Perpetual Futures.
        Swap = "SWAP",
        /// Expiry Futures.
        Futures = "FUTURES",
        /// Option.
        Option = "OPTION",
        /// Event Contracts
        Events = "EVENTS",
    }
}

string_enum! {
    /// Order side.
    pub enum OrderSide {
        /// Buy.
        Buy = "buy",
        /// Sell.
        Sell = "sell",
    }
}

string_enum! {
    /// Order type.
    pub enum OrderType {
        /// Market order.
        Market = "market",
        /// Limit order.
        Limit = "limit",
        /// Post-only order.
        PostOnly = "post_only",
        /// Fill-or-kill order.
        Fok = "fok",
        /// Immediate-or-cancel order.
        Ioc = "ioc",
        /// Market order with immediate-or-cancel (futures/swap).
        OptimalLimitIoc = "optimal_limit_ioc",
    }
}

string_enum! {
    /// Trade (margin) mode.
    pub enum TradeMode {
        /// Non-margin (cash).
        Cash = "cash",
        /// Cross margin.
        Cross = "cross",
        /// Isolated margin.
        Isolated = "isolated",
    }
}

string_enum! {
    /// Position side.
    pub enum PositionSide {
        /// Long position.
        Long = "long",
        /// Short position.
        Short = "short",
        /// Net position.
        Net = "net",
    }
}

string_enum! {
    /// Order lifecycle state.
    pub enum OrderState {
        /// Resting on the book.
        Live = "live",
        /// Partially filled.
        PartiallyFilled = "partially_filled",
        /// Fully filled.
        Filled = "filled",
        /// Canceled.
        Canceled = "canceled",
        /// Canceled by market maker protection.
        MmpCanceled = "mmp_canceled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_string_parses_and_preserves() {
        let n = NumberString::from("1.005");
        assert_eq!(n.as_str(), "1.005");
        assert_eq!(n.parse::<f64>().unwrap(), 1.005);
        assert_eq!(n.into_string(), "1.005");
    }

    #[test]
    fn known_enum_value_round_trips() {
        let v: InstType = serde_json::from_str("\"SWAP\"").unwrap();
        assert_eq!(v, InstType::Swap);
        assert_eq!(serde_json::to_string(&v).unwrap(), "\"SWAP\"");
    }

    #[test]
    fn unknown_enum_value_is_preserved_not_an_error() {
        let v: InstType = serde_json::from_str("\"FUTURE_THING\"").unwrap();
        assert_eq!(v, InstType::Unknown("FUTURE_THING".to_owned()));
        // And serializes back to the original wire value.
        assert_eq!(serde_json::to_string(&v).unwrap(), "\"FUTURE_THING\"");
    }

    #[test]
    fn raw_request_params_replace_duplicate_keys() {
        let params = RawRequestParams::new()
            .param("ccy", "BTC")
            .param("ccy", "ETH");

        assert_eq!(
            serde_json::to_value(params).unwrap(),
            serde_json::json!({"ccy": "ETH"})
        );
    }
}
