// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Boundary detection and style matching, observed through public APIs.

use qubit_case::{
    LOWER_CAMEL,
    LOWER_HYPHEN,
    LOWER_UNDERSCORE,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
};

#[test]
fn test_matches_accepts_valid_style_examples() {
    assert!(LOWER_HYPHEN.matches("lower-hyphen-123"));
    assert!(LOWER_UNDERSCORE.matches("lower_underscore_123"));
    assert!(LOWER_CAMEL.matches("lowerCamel123"));
    assert!(UPPER_CAMEL.matches("UpperCamel123"));
    assert!(UPPER_UNDERSCORE.matches("UPPER_UNDERSCORE_123"));
}

#[test]
fn test_matches_rejects_invalid_style_examples() {
    assert!(!LOWER_HYPHEN.matches(""));
    assert!(!LOWER_HYPHEN.matches("-lower"));
    assert!(!LOWER_HYPHEN.matches("lower--hyphen"));
    assert!(!LOWER_HYPHEN.matches("lower_Hyphen"));
    assert!(!LOWER_UNDERSCORE.matches("_lower"));
    assert!(!LOWER_UNDERSCORE.matches("lower__underscore"));
    assert!(!LOWER_CAMEL.matches("LowerCamel"));
    assert!(!LOWER_CAMEL.matches("lower-camel"));
    assert!(!UPPER_CAMEL.matches("upperCamel"));
    assert!(!UPPER_CAMEL.matches("Upper_Camel"));
    assert!(!UPPER_UNDERSCORE.matches("upper_underscore"));
    assert!(!UPPER_UNDERSCORE.matches("UPPER__UNDERSCORE"));
}

#[test]
fn test_matches_rejects_digit_as_first_character() {
    assert!(!LOWER_HYPHEN.matches("123-lower"));
    assert!(!LOWER_UNDERSCORE.matches("123_lower"));
    assert!(!UPPER_UNDERSCORE.matches("123_UPPER"));
}

#[test]
fn test_to_converts_lower_camel_to_all_styles_with_numbers() {
    let source = "oAuth2Token";

    assert_eq!(LOWER_CAMEL.to(LOWER_HYPHEN, source), "o-auth-2-token");
    assert_eq!(LOWER_CAMEL.to(LOWER_UNDERSCORE, source), "o_auth_2_token");
    assert_eq!(LOWER_CAMEL.to(LOWER_CAMEL, source), source);
    assert_eq!(LOWER_CAMEL.to(UPPER_CAMEL, source), "OAuth2Token");
    assert_eq!(LOWER_CAMEL.to(UPPER_UNDERSCORE, source), "O_AUTH_2_TOKEN");
}

#[test]
fn test_to_converts_upper_camel_to_all_styles_with_acronyms_and_numbers() {
    let source = "XMLParser2Response";

    assert_eq!(
        UPPER_CAMEL.to(LOWER_HYPHEN, source),
        "xml-parser-2-response"
    );
    assert_eq!(
        UPPER_CAMEL.to(LOWER_UNDERSCORE, source),
        "xml_parser_2_response"
    );
    assert_eq!(UPPER_CAMEL.to(LOWER_CAMEL, source), "xmlParser2Response");
    assert_eq!(UPPER_CAMEL.to(UPPER_CAMEL, source), source);
    assert_eq!(
        UPPER_CAMEL.to(UPPER_UNDERSCORE, source),
        "XML_PARSER_2_RESPONSE"
    );
}

#[test]
fn test_to_uses_best_effort_for_non_boundary_symbols() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "Hello$World"), "hello$world");
    assert_eq!(UPPER_CAMEL.to(LOWER_CAMEL, "A1$B"), "a1$b");
    assert_eq!(UPPER_CAMEL.to(UPPER_UNDERSCORE, "A1$B"), "A_1$B");
}
