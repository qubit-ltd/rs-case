// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the public [`qubit_case::CaseStyle`] API.

use std::str::FromStr;

use qubit_case::CaseStyle;
use qubit_case::LOWER_CAMEL;
use qubit_case::LOWER_HYPHEN;
use qubit_case::LOWER_UNDERSCORE;
use qubit_case::UPPER_CAMEL;
use qubit_case::UPPER_UNDERSCORE;

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
    assert_eq!(LOWER_HYPHEN.to(LOWER_UNDERSCORE, "UPPER-HYPHEN"), "UPPER_HYPHEN",);
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
fn test_validation_contract_matrix() {
    let valid = [
        (LOWER_HYPHEN, "a"),
        (LOWER_HYPHEN, "a-1"),
        (LOWER_UNDERSCORE, "a"),
        (LOWER_UNDERSCORE, "a_1"),
        (UPPER_UNDERSCORE, "A"),
        (UPPER_UNDERSCORE, "A_1"),
        (LOWER_CAMEL, "a"),
        (LOWER_CAMEL, "aBC"),
        (UPPER_CAMEL, "A"),
        (UPPER_CAMEL, "HTTP2Client"),
    ];

    for (source, value) in valid {
        assert!(source.matches(value), "{source}: {value:?}");
        assert_eq!(source.validate(value), Ok(()));
        for target in CaseStyle::values() {
            let converted = source.checked_to(*target, value).unwrap();
            assert_eq!(converted, source.to(*target, value));
            assert!(target.matches(&converted), "{source} -> {target}: {value:?}");
        }
    }

    for source in CaseStyle::values() {
        let specific: &[&str] = match source {
            CaseStyle::LowerHyphen => &["-a", "a-", "a--b", "A", "a_b"],
            CaseStyle::LowerUnderscore => &["_a", "a_", "a__b", "A", "a-b"],
            CaseStyle::UpperUnderscore => &["_A", "A_", "A__B", "a", "A-B"],
            CaseStyle::LowerCamel => &["A", "a_b", "a-b"],
            CaseStyle::UpperCamel => &["a", "A_B", "A-B"],
        };
        for value in ["", "1", "1a", "1A", "a b", "A B", "a$", "A$", "é", "aé", "Aé"]
            .into_iter()
            .chain(specific.iter().copied())
        {
            assert!(!source.matches(value), "{source}: {value:?}");
            let error = source.validate(value).unwrap_err();
            assert_eq!(error.style(), *source);
            assert_eq!(error.value(), value);
            for target in CaseStyle::values() {
                assert_eq!(source.checked_to(*target, value), Err(error.clone()));
            }
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
