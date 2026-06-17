//! Reusable validation utilities for typed HTTP request models.
//!
//! This module separates generic validation mechanics from endpoint-specific
//! business rules. Request implementations should describe which constraints
//! apply, while the functions in this module perform the actual checks.

use crate::model::OrderSide;

const CLIENT_REQUEST_ID_MAX_LEN: usize = 32;
/// A validation failure detected before an HTTP request is sent.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum RequestValidationError {
    /// A required string field was empty.
    #[error("request field `{field}` must not be empty")]
    EmptyField {
        /// OKX wire-field name.
        field: &'static str,
    },

    /// A string field exceeded the endpoint's documented maximum length.
    #[error("request field `{field}` must be at most {max} characters")]
    TooLong {
        /// OKX wire-field name.
        field: &'static str,

        /// Maximum allowed character count.
        max: usize,
    },

    /// A string field's length was outside the documented range.
    #[error("request field `{field}` must contain between {min} and {max} characters")]
    LengthOutOfRange {
        /// OKX wire-field name.
        field: &'static str,

        /// Inclusive minimum character count.
        min: usize,

        /// Inclusive maximum character count.
        max: usize,
    },

    /// A field did not match the endpoint's required textual format.
    #[error("request field `{field}` has invalid format: {expected}")]
    InvalidFormat {
        /// OKX wire-field name.
        field: &'static str,

        /// Human-readable description of the expected format.
        expected: &'static str,
    },

    /// A numeric request value was outside the endpoint's documented range.
    #[error("request field `{field}` must be between {min} and {max}")]
    OutOfRange {
        /// OKX wire-field name.
        field: &'static str,

        /// Inclusive lower bound.
        min: u64,

        /// Inclusive upper bound.
        max: u64,
    },

    /// A field is required under a particular condition.
    #[error("request field `{field}` is required when {condition}")]
    RequiredWhen {
        /// OKX wire-field name.
        field: &'static str,

        /// Human-readable description of the condition.
        condition: &'static str,
    },

    /// A field is not applicable under a particular condition.
    #[error("request field `{field}` is not applicable when {condition}")]
    NotApplicable {
        /// OKX wire-field name.
        field: &'static str,

        /// Human-readable description of the condition.
        condition: &'static str,
    },

    /// None of a set of conditionally required fields was provided.
    #[error("at least one of these request fields is required: {fields}")]
    AtLeastOneRequired {
        /// Comma-separated OKX wire-field names.
        fields: &'static str,
    },

    /// More than one mutually exclusive field was provided.
    #[error("request fields are mutually exclusive: {fields}")]
    MutuallyExclusive {
        /// Comma-separated OKX wire-field names.
        fields: &'static str,
    },
}

/// Validation implemented by typed request models.
///
/// Endpoint accessors call this before serializing and sending a request so
/// obvious client-side mistakes fail without consuming an OKX rate-limit slot.
pub trait ValidateRequest {
    /// Validate all constraints represented by this SDK version.
    fn validate(&self) -> Result<(), RequestValidationError>;
}

/// Validate a required string.
///
/// Leading and trailing whitespace is ignored when checking whether the value
/// is empty. The original value is not modified.
pub(crate) fn non_empty(field: &'static str, value: &str) -> Result<(), RequestValidationError> {
    if value.trim().is_empty() {
        return Err(RequestValidationError::EmptyField { field });
    }

    Ok(())
}

/// Validate an optional string.
///
/// `None` is valid, but `Some("")` and whitespace-only values are rejected.
pub(crate) fn optional_non_empty(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), RequestValidationError> {
    if value.is_some_and(|value| value.trim().is_empty()) {
        return Err(RequestValidationError::EmptyField { field });
    }

    Ok(())
}

/// Require an optional string under an endpoint-specific condition.
///
/// A present but empty value produces [`RequestValidationError::EmptyField`].
/// A missing value produces [`RequestValidationError::RequiredWhen`].
pub(crate) fn require_when(
    field: &'static str,
    value: Option<&str>,
    condition: &'static str,
) -> Result<(), RequestValidationError> {
    match value {
        Some(value) => non_empty(field, value),

        None => Err(RequestValidationError::RequiredWhen { field, condition }),
    }
}

/// Reject a field when it is not applicable to the current request.
///
/// This function only checks whether the option is present. Validation of the
/// contained value should be performed separately.
pub(crate) fn reject_when_present<T>(
    field: &'static str,
    value: Option<&T>,
    condition: &'static str,
) -> Result<(), RequestValidationError> {
    if value.is_some() {
        return Err(RequestValidationError::NotApplicable { field, condition });
    }

    Ok(())
}

/// Validate that a string does not exceed a maximum character count.
///
/// This counts Unicode scalar values via [`str::chars`], not bytes. Fields that
/// require ASCII-only values should also perform a separate format check.
pub(crate) fn max_length(
    field: &'static str,
    value: &str,
    max: usize,
) -> Result<(), RequestValidationError> {
    if value.chars().count() > max {
        return Err(RequestValidationError::TooLong { field, max });
    }

    Ok(())
}

/// Validate that a string's character count is within an inclusive range.
///
/// Lengths use [`usize`] because iterator counts and collection lengths in Rust
/// are represented by `usize`.
pub(crate) fn length_range(
    field: &'static str,
    value: &str,
    min: usize,
    max: usize,
) -> Result<(), RequestValidationError> {
    let length = value.chars().count();

    if !(min..=max).contains(&length) {
        return Err(RequestValidationError::LengthOutOfRange { field, min, max });
    }

    Ok(())
}

/// Validate an unsigned API value against an inclusive range.
///
/// API-level numeric values use [`u64`] rather than [`usize`] because their
/// wire representation must not depend on the target platform's pointer size.
pub(crate) fn range_u64(
    field: &'static str,
    value: u64,
    min: u64,
    max: u64,
) -> Result<(), RequestValidationError> {
    if !(min..=max).contains(&value) {
        return Err(RequestValidationError::OutOfRange { field, min, max });
    }

    Ok(())
}

/// Require at least one field in a group to be present.
///
/// Each boolean should indicate whether the corresponding request field was
/// provided.
pub(crate) fn at_least_one(
    fields: &'static str,
    present: &[bool],
) -> Result<(), RequestValidationError> {
    if !present.iter().copied().any(|present| present) {
        return Err(RequestValidationError::AtLeastOneRequired { fields });
    }

    Ok(())
}

/// Require no more than one field in a group to be present.
///
/// This is useful for request parameters such as `ordId` and `clOrdId` when an
/// endpoint treats them as mutually exclusive identifiers.
pub(crate) fn at_most_one(
    fields: &'static str,
    present: &[bool],
) -> Result<(), RequestValidationError> {
    let count = present.iter().copied().filter(|present| *present).count();

    if count > 1 {
        return Err(RequestValidationError::MutuallyExclusive { fields });
    }

    Ok(())
}

/// Require exactly one field in a group to be present.
///
/// This combines [`at_least_one`] and [`at_most_one`].
pub(crate) fn exactly_one(
    fields: &'static str,
    present: &[bool],
) -> Result<(), RequestValidationError> {
    at_least_one(fields, present)?;
    at_most_one(fields, present)
}

/// Validata side field in the Order.
///
/// Only buy or sell, other string will be rejected.
pub(crate) fn validate_side(side: &OrderSide) -> Result<(), RequestValidationError> {
    match side {
        OrderSide::Buy | OrderSide::Sell => Ok(()),
        _ => Err(RequestValidationError::InvalidFormat {
            field: "side",
            expected: "buy or sell",
        }),
    }
}

/// Validata cl_q_req_id field in the ConvertQuoteRequest struct.
///
/// [Specialification](https://www.okx.com/docs-v5/en/#funding-account-rest-api-convert-trade):
/// This field from the quote_id field of ConvertQuote struct.
pub(crate) fn validate_client_request_id(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), RequestValidationError> {
    let Some(value) = value else {
        return Ok(());
    };

    non_empty(field, value)?;
    if value.chars().count() > CLIENT_REQUEST_ID_MAX_LEN {
        return Err(RequestValidationError::TooLong {
            field,
            max: CLIENT_REQUEST_ID_MAX_LEN,
        });
    }
    if !value.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(RequestValidationError::InvalidFormat {
            field,
            expected: "1-32 ASCII alphanumeric characters",
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests;
