// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for [`qubit_case::CaseStyleValidationError`].

use qubit_case::{
    CaseStyleValidationError,
    LOWER_CAMEL,
    UPPER_UNDERSCORE,
};

#[test]
fn test_validate_reports_style_and_value_for_invalid_input() {
    assert_eq!(LOWER_CAMEL.validate("lowerCamel123"), Ok(()));

    let error = LOWER_CAMEL
        .validate("UpperCamel")
        .expect_err("an uppercase initial should fail lower-camel validation");
    assert_eq!(error.style(), LOWER_CAMEL);
    assert_eq!(error.value(), "UpperCamel");
    assert_eq!(
        error,
        CaseStyleValidationError::new(LOWER_CAMEL, "UpperCamel"),
    );
    assert_eq!(
        error.to_string(),
        "Value 'UpperCamel' does not match case style 'lower-camel'.",
    );
}

#[test]
fn test_checked_to_validates_source_before_conversion() {
    assert_eq!(
        LOWER_CAMEL.checked_to(UPPER_UNDERSCORE, "http2Client"),
        Ok("HTTP_2_CLIENT".to_string()),
    );

    let error = LOWER_CAMEL
        .checked_to(UPPER_UNDERSCORE, "HTTP2Client")
        .expect_err("invalid lower-camel input should not be converted");
    assert_eq!(error.style(), LOWER_CAMEL);
    assert_eq!(error.value(), "HTTP2Client");
}

#[test]
fn test_new_preserves_style_and_value() {
    let error = CaseStyleValidationError::new(UPPER_UNDERSCORE, "bad_value");
    assert_eq!(error.style(), UPPER_UNDERSCORE);
    assert_eq!(error.value(), "bad_value");
    assert_eq!(
        error.to_string(),
        "Value 'bad_value' does not match case style 'upper-underscore'.",
    );
}

#[test]
fn test_error_implements_standard_traits() {
    fn assert_error<T: std::error::Error + Send + Sync + 'static>() {}

    assert_error::<CaseStyleValidationError>();
    let left = CaseStyleValidationError::new(LOWER_CAMEL, "X");
    let right = CaseStyleValidationError::new(LOWER_CAMEL, "X");
    assert_eq!(left, right);
    assert_eq!(left.clone(), right);
    assert!(format!("{left:?}").contains("CaseStyleValidationError"));
}
