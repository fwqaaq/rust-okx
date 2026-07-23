//! Shared data types: the [`NumberString`] wrapper, the response envelope, and
//! the common string enums used across the API modules.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct EmptyRequest {}

/// Deserialize an OKX array field that may be encoded as an empty string when
/// no entries are available.
///
/// A few OKX REST endpoints document these fields as arrays but return `""`
/// for accounts/currencies without data. Treat only the empty string and
/// `null` as an empty vector; non-empty strings remain decode errors.
pub(crate) fn deserialize_vec_or_empty_string<'de, D, T>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum WireValue<T> {
        Sequence(Vec<T>),
        String(String),
        Null(()),
    }

    match WireValue::<T>::deserialize(deserializer)? {
        WireValue::Sequence(values) => Ok(values),
        WireValue::String(value) if value.is_empty() => Ok(Vec::new()),
        WireValue::String(value) => Err(serde::de::Error::custom(format!(
            "expected an array or empty string, got {value:?}"
        ))),
        WireValue::Null(()) => Ok(Vec::new()),
    }
}

/// The OKX response envelope: `{ "code": "...", "msg": "...", "data": [...] }`.
///
/// Internal — the client unwraps it and returns `data` (or a [`RestError::Okx`](crate::RestError::Okx)(crate::RestError::Okx)).
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
        /// Exclusive liquidity provider order.
        Elp = "elp",
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
        /// Spot isolated margin mode.
        SpotIsolated = "spot_isolated",
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
}
