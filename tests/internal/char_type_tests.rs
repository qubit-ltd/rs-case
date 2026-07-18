// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Character-class effects on CamelCase splitting, observed through public
//! APIs.

use qubit_case::{
    LOWER_CAMEL,
    LOWER_HYPHEN,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
};

#[test]
fn test_upper_to_lower_transition_starts_new_word() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "HttpClient"), "http-client");
    assert_eq!(LOWER_CAMEL.to(LOWER_HYPHEN, "httpClient"), "http-client");
}

#[test]
fn test_digit_boundaries_split_around_numbers() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "Http2Client"), "http-2-client");
    assert_eq!(
        LOWER_CAMEL.to(UPPER_UNDERSCORE, "http2Client"),
        "HTTP_2_CLIENT"
    );
    assert_eq!(UPPER_CAMEL.to(LOWER_CAMEL, "A1B2"), "a1B2");
}

#[test]
fn test_acronym_before_word_keeps_final_upper_with_following_lower() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "XMLHttp"), "xml-http");
    assert_eq!(UPPER_CAMEL.to(LOWER_CAMEL, "XMLHttp"), "xmlHttp");
}

#[test]
fn test_other_bytes_do_not_create_camel_boundaries() {
    assert_eq!(UPPER_CAMEL.to(LOWER_HYPHEN, "Hello$World"), "hello$world");
    assert_eq!(LOWER_CAMEL.to(LOWER_HYPHEN, "hello$World"), "hello$world");
}
