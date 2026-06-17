use super::*;

#[test]
fn non_empty_accepts_non_empty_value() {
    assert_eq!(non_empty("instId", "BTC-USDT"), Ok(()));
}

#[test]
fn non_empty_rejects_whitespace_only_value() {
    assert_eq!(
        non_empty("instId", "   "),
        Err(RequestValidationError::EmptyField { field: "instId" }),
    );
}

#[test]
fn optional_non_empty_accepts_none() {
    assert_eq!(optional_non_empty("instId", None), Ok(()));
}

#[test]
fn optional_non_empty_accepts_non_empty_value() {
    assert_eq!(optional_non_empty("instId", Some("BTC-USDT")), Ok(()),);
}

#[test]
fn optional_non_empty_rejects_empty_value() {
    assert_eq!(
        optional_non_empty("instId", Some("")),
        Err(RequestValidationError::EmptyField { field: "instId" }),
    );
}

#[test]
fn require_when_accepts_present_value() {
    assert_eq!(
        require_when("seriesId", Some("BTC-ABOVE-DAILY"), "instType is EVENTS",),
        Ok(()),
    );
}

#[test]
fn require_when_rejects_missing_value() {
    assert_eq!(
        require_when("seriesId", None, "instType is EVENTS",),
        Err(RequestValidationError::RequiredWhen {
            field: "seriesId",
            condition: "instType is EVENTS",
        }),
    );
}

#[test]
fn require_when_rejects_empty_value() {
    assert_eq!(
        require_when("seriesId", Some(" "), "instType is EVENTS",),
        Err(RequestValidationError::EmptyField { field: "seriesId" }),
    );
}

#[test]
fn reject_when_present_accepts_none() {
    let value: Option<&String> = None;

    assert_eq!(
        reject_when_present("instFamily", value, "instType is SPOT",),
        Ok(()),
    );
}

#[test]
fn reject_when_present_rejects_some() {
    let value = "BTC-USDT".to_owned();

    assert_eq!(
        reject_when_present("instFamily", Some(&value), "instType is SPOT",),
        Err(RequestValidationError::NotApplicable {
            field: "instFamily",
            condition: "instType is SPOT",
        }),
    );
}

#[test]
fn max_length_accepts_exact_limit() {
    assert_eq!(max_length("clQReqId", "12345678", 8), Ok(()),);
}

#[test]
fn max_length_rejects_value_above_limit() {
    assert_eq!(
        max_length("clQReqId", "123456789", 8),
        Err(RequestValidationError::TooLong {
            field: "clQReqId",
            max: 8,
        }),
    );
}

#[test]
fn length_range_accepts_value_inside_range() {
    assert_eq!(length_range("clQReqId", "abc123", 1, 32), Ok(()),);
}

#[test]
fn length_range_rejects_value_below_range() {
    assert_eq!(
        length_range("clQReqId", "", 1, 32),
        Err(RequestValidationError::LengthOutOfRange {
            field: "clQReqId",
            min: 1,
            max: 32,
        }),
    );
}

#[test]
fn length_range_rejects_value_above_range() {
    let value = "a".repeat(33);

    assert_eq!(
        length_range("clQReqId", &value, 1, 32),
        Err(RequestValidationError::LengthOutOfRange {
            field: "clQReqId",
            min: 1,
            max: 32,
        }),
    );
}

#[test]
fn range_u64_accepts_inclusive_boundaries() {
    assert_eq!(range_u64("limit", 1, 1, 100), Ok(()));
    assert_eq!(range_u64("limit", 100, 1, 100), Ok(()));
}

#[test]
fn range_u64_rejects_value_below_range() {
    assert_eq!(
        range_u64("limit", 0, 1, 100),
        Err(RequestValidationError::OutOfRange {
            field: "limit",
            min: 1,
            max: 100,
        }),
    );
}

#[test]
fn range_u64_rejects_value_above_range() {
    assert_eq!(
        range_u64("limit", 101, 1, 100),
        Err(RequestValidationError::OutOfRange {
            field: "limit",
            min: 1,
            max: 100,
        }),
    );
}

#[test]
fn at_least_one_accepts_one_present_field() {
    assert_eq!(at_least_one("ordId, clOrdId", &[false, true]), Ok(()),);
}

#[test]
fn at_least_one_rejects_all_missing_fields() {
    assert_eq!(
        at_least_one("ordId, clOrdId", &[false, false]),
        Err(RequestValidationError::AtLeastOneRequired {
            fields: "ordId, clOrdId",
        }),
    );
}

#[test]
fn at_most_one_accepts_zero_or_one_field() {
    assert_eq!(at_most_one("ordId, clOrdId", &[false, false]), Ok(()),);

    assert_eq!(at_most_one("ordId, clOrdId", &[true, false]), Ok(()),);
}

#[test]
fn at_most_one_rejects_multiple_fields() {
    assert_eq!(
        at_most_one("ordId, clOrdId", &[true, true]),
        Err(RequestValidationError::MutuallyExclusive {
            fields: "ordId, clOrdId",
        }),
    );
}

#[test]
fn exactly_one_requires_one_field() {
    assert_eq!(exactly_one("ordId, clOrdId", &[true, false]), Ok(()),);

    assert_eq!(
        exactly_one("ordId, clOrdId", &[false, false]),
        Err(RequestValidationError::AtLeastOneRequired {
            fields: "ordId, clOrdId",
        }),
    );

    assert_eq!(
        exactly_one("ordId, clOrdId", &[true, true]),
        Err(RequestValidationError::MutuallyExclusive {
            fields: "ordId, clOrdId",
        }),
    );
}
