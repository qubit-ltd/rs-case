// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
fn test_from_str_reports_unknown_style_name() {
    let error = CaseStyle::from_str("unknown-style")
        .expect_err("unknown style should fail");
    assert_eq!(error.name(), "unknown-style");
    assert_eq!(error.to_string(), "Unknown case style: 'unknown-style'.");
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
fn test_to_converts_lower_hyphen_to_all_styles() {
    let source = "hello-world-everybody";

    assert_eq!(LOWER_HYPHEN.to(LOWER_HYPHEN, source), source);
    assert_eq!(
        LOWER_HYPHEN.to(LOWER_UNDERSCORE, source),
        "hello_world_everybody"
    );
    assert_eq!(LOWER_HYPHEN.to(LOWER_CAMEL, source), "helloWorldEverybody");
    assert_eq!(LOWER_HYPHEN.to(UPPER_CAMEL, source), "HelloWorldEverybody");
    assert_eq!(
        LOWER_HYPHEN.to(UPPER_UNDERSCORE, source),
        "HELLO_WORLD_EVERYBODY"
    );
}

#[test]
fn test_to_converts_lower_underscore_to_all_styles() {
    let source = "hello_world_everybody";

    assert_eq!(
        LOWER_UNDERSCORE.to(LOWER_HYPHEN, source),
        "hello-world-everybody"
    );
    assert_eq!(LOWER_UNDERSCORE.to(LOWER_UNDERSCORE, source), source);
    assert_eq!(
        LOWER_UNDERSCORE.to(LOWER_CAMEL, source),
        "helloWorldEverybody"
    );
    assert_eq!(
        LOWER_UNDERSCORE.to(UPPER_CAMEL, source),
        "HelloWorldEverybody"
    );
    assert_eq!(
        LOWER_UNDERSCORE.to(UPPER_UNDERSCORE, source),
        "HELLO_WORLD_EVERYBODY"
    );
}

#[test]
fn test_to_converts_upper_underscore_to_all_styles() {
    let source = "HELLO_WORLD_EVERYBODY";

    assert_eq!(
        UPPER_UNDERSCORE.to(LOWER_HYPHEN, source),
        "hello-world-everybody"
    );
    assert_eq!(
        UPPER_UNDERSCORE.to(LOWER_UNDERSCORE, source),
        "hello_world_everybody"
    );
    assert_eq!(
        UPPER_UNDERSCORE.to(LOWER_CAMEL, source),
        "helloWorldEverybody"
    );
    assert_eq!(
        UPPER_UNDERSCORE.to(UPPER_CAMEL, source),
        "HelloWorldEverybody"
    );
    assert_eq!(UPPER_UNDERSCORE.to(UPPER_UNDERSCORE, source), source);
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
fn test_to_preserves_empty_string() {
    for source in CaseStyle::values() {
        for target in CaseStyle::values() {
            assert_eq!(source.to(*target, ""), "");
        }
    }
}

#[test]
fn test_to_uses_best_effort_for_non_boundary_symbols() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "Hello$World"), "hello$world");
    assert_eq!(UPPER_CAMEL.to(LOWER_CAMEL, "A1$B"), "a1$b");
    assert_eq!(UPPER_CAMEL.to(UPPER_UNDERSCORE, "A1$B"), "A_1$B");
}

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
