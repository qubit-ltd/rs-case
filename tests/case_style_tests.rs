// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the public [`qubit_case::CaseStyle`] API.

use std::str::FromStr;

use qubit_case::{
    CaseStyle,
    LOWER_CAMEL,
    LOWER_HYPHEN,
    LOWER_UNDERSCORE,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
};

#[test]
fn test_values_returns_all_styles_in_reference_order() {
    assert_eq!(
        CaseStyle::values(),
        &[
            LOWER_HYPHEN,
            LOWER_UNDERSCORE,
            LOWER_CAMEL,
            UPPER_CAMEL,
            UPPER_UNDERSCORE,
        ],
    );
}

#[test]
fn test_of_accepts_case_insensitive_names_with_hyphen_or_underscore() {
    assert_eq!(CaseStyle::of("lower-hyphen"), Ok(LOWER_HYPHEN));
    assert_eq!(CaseStyle::of("LOWER_HYPHEN"), Ok(LOWER_HYPHEN));
    assert_eq!(CaseStyle::of("Lower_Underscore"), Ok(LOWER_UNDERSCORE));
    assert_eq!(CaseStyle::of("lower-camel"), Ok(LOWER_CAMEL));
    assert_eq!(CaseStyle::of("UPPER-CAMEL"), Ok(UPPER_CAMEL));
    assert_eq!(CaseStyle::of("upper_underscore"), Ok(UPPER_UNDERSCORE));
}

#[test]
fn test_name_and_word_separator_return_style_metadata() {
    assert_eq!(LOWER_HYPHEN.name(), "lower-hyphen");
    assert_eq!(LOWER_HYPHEN.word_separator(), "-");
    assert_eq!(LOWER_UNDERSCORE.name(), "lower-underscore");
    assert_eq!(LOWER_UNDERSCORE.word_separator(), "_");
    assert_eq!(LOWER_CAMEL.name(), "lower-camel");
    assert_eq!(LOWER_CAMEL.word_separator(), "");
    assert_eq!(UPPER_CAMEL.name(), "upper-camel");
    assert_eq!(UPPER_CAMEL.word_separator(), "");
    assert_eq!(UPPER_UNDERSCORE.name(), "upper-underscore");
    assert_eq!(UPPER_UNDERSCORE.word_separator(), "_");
}

#[test]
fn test_to_preserves_empty_string() {
    for source in CaseStyle::values() {
        for target in CaseStyle::values() {
            assert_eq!(source.to(*target, ""), "");
        }
    }
}

#[test]
fn test_to_remains_permissive_for_invalid_source_input() {
    assert_eq!(
        LOWER_HYPHEN.to(LOWER_UNDERSCORE, "UPPER-HYPHEN"),
        "UPPER_HYPHEN",
    );
}

#[test]
fn test_checked_to_produces_values_matching_every_target_style() {
    let valid_values = [
        (LOWER_HYPHEN, "http-2-client"),
        (LOWER_UNDERSCORE, "http_2_client"),
        (LOWER_CAMEL, "http2Client"),
        (UPPER_CAMEL, "HTTP2Client"),
        (UPPER_UNDERSCORE, "HTTP_2_CLIENT"),
    ];

    for (source, value) in valid_values {
        for target in CaseStyle::values() {
            let converted = source
                .checked_to(*target, value)
                .expect("the source fixture should be valid");
            assert!(
                target.matches(&converted),
                "{value:?} converted from {} to {} as {converted:?}",
                source.name(),
                target.name(),
            );
        }
    }
}

#[test]
fn test_style_name_round_trips_through_from_str() {
    for style in CaseStyle::values() {
        assert_eq!(CaseStyle::from_str(style.name()), Ok(*style));
    }
}

#[test]
fn test_display_returns_canonical_style_name() {
    for style in CaseStyle::values() {
        assert_eq!(style.to_string(), style.name());
    }
}

#[test]
fn test_to_handles_non_ascii_input_without_panicking() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "ÉclairHTTP"), "Éclair-http",);
}
