// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for [`qubit_case::CaseStyleError`].

use std::str::FromStr;

use qubit_case::{
    CaseStyle,
    CaseStyleError,
};

#[test]
fn test_from_str_reports_unknown_style_name() {
    let error = CaseStyle::from_str("unknown-style")
        .expect_err("unknown style should fail");
    assert_eq!(error.name(), "unknown-style");
    assert_eq!(error.to_string(), "Unknown case style: 'unknown-style'.");
}

#[test]
fn test_of_reports_unknown_style_name() {
    let error =
        CaseStyle::of("not-a-style").expect_err("unknown style should fail");
    assert_eq!(error.name(), "not-a-style");
    assert_eq!(error.to_string(), "Unknown case style: 'not-a-style'.");
}

#[test]
fn test_new_preserves_original_name() {
    let error = CaseStyleError::new("Custom-Name");
    assert_eq!(error.name(), "Custom-Name");
}

#[test]
fn test_error_implements_standard_traits() {
    fn assert_error<T: std::error::Error + Send + Sync + 'static>() {}

    assert_error::<CaseStyleError>();
    let left = CaseStyleError::new("x");
    let right = CaseStyleError::new("x");
    assert_eq!(left, right);
    assert_eq!(left.clone(), right);
    assert!(format!("{left:?}").contains("CaseStyleError"));
}
