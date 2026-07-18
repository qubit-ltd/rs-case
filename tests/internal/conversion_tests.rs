// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Conversion helpers, observed through public
//! [`CaseStyle::to`](qubit_case::CaseStyle::to).

use qubit_case::{
    LOWER_CAMEL,
    LOWER_HYPHEN,
    LOWER_UNDERSCORE,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
};

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
