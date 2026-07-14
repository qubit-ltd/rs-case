# Qubit Case

[![Rust CI](https://github.com/qubit-ltd/rs-case/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-case/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-case/coverage-badge.json)](https://qubit-ltd.github.io/rs-case/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-case.svg?color=blue)](https://crates.io/crates/qubit-case)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Naming style detection and conversion helpers for Rust.

## Overview

Qubit Case converts ASCII identifiers between common naming styles used
by Java, C++, Python, XML, JSON, and Rust tooling through a compact Rust enum
API.

## Design Goals

- **Small API surface**: expose one core `CaseStyle` enum, five convenience
  constants, and focused parsing and validation errors.
- **Consistent behavior**: provide stable conversion results for supported ASCII
  identifier formats.
- **Strict matching**: validate whether a string conforms to a style.
- **Best-effort conversion**: convert inputs even when they are not strictly
  valid for the source style.
- **No runtime dependencies**: keep the library lightweight.

## Supported Styles

- `LOWER_HYPHEN`: `lower-hyphen`
- `LOWER_UNDERSCORE`: `lower_underscore`
- `LOWER_CAMEL`: `lowerCamel`
- `UPPER_CAMEL`: `UpperCamel`
- `UPPER_UNDERSCORE`: `UPPER_UNDERSCORE`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-case = "0.2"
```

## Quick Start

```rust
use qubit_case::{
    LOWER_CAMEL,
    LOWER_HYPHEN,
    UPPER_UNDERSCORE,
};

let value = LOWER_HYPHEN.to(LOWER_CAMEL, "hello-world");
assert_eq!(value, "helloWorld");

LOWER_CAMEL
    .validate("http2Client")
    .expect("the value should be lower camel case");

let constant = LOWER_CAMEL
    .checked_to(UPPER_UNDERSCORE, "http2Client")
    .expect("the source value should be valid");
assert_eq!(constant, "HTTP_2_CLIENT");

assert!(UPPER_UNDERSCORE.matches("HTTP_2_CLIENT"));
assert_eq!(LOWER_CAMEL.to_string(), "lower-camel");
```

## API Reference

### Types

- [`CaseStyle`](https://docs.rs/qubit-case/latest/qubit_case/enum.CaseStyle.html) -
  supported case styles and conversion methods.
- [`CaseStyleError`](https://docs.rs/qubit-case/latest/qubit_case/struct.CaseStyleError.html) -
  error returned when parsing an unknown style name.
- [`CaseStyleValidationError`](https://docs.rs/qubit-case/latest/qubit_case/struct.CaseStyleValidationError.html) -
  error returned when a value does not match the expected style.

### Constants

- `LOWER_HYPHEN`
- `LOWER_UNDERSCORE`
- `LOWER_CAMEL`
- `UPPER_CAMEL`
- `UPPER_UNDERSCORE`

### Methods

- `CaseStyle::values()` returns all styles in reference order.
- `CaseStyle::of(name)` parses a style name case-insensitively, treating
  hyphen and underscore as equivalent.
- `CaseStyle::name()` returns the canonical lower-hyphen style name.
- `CaseStyle::word_separator()` returns the style separator.
- `CaseStyle::to(target, value)` performs permissive best-effort conversion
  without validating the source or guaranteeing that invalid input matches the
  target style.
- `CaseStyle::validate(value)` validates an identifier and reports a mismatch.
- `CaseStyle::checked_to(target, value)` validates the source before converting
  it.
- `CaseStyle::matches(value)` checks whether an identifier strictly follows a
  style.

### Trait Implementations

- `Display` formats a style using its canonical lower-hyphen name.
- `FromStr` parses the names accepted by `CaseStyle::of`.

## Testing & Code Coverage

This project tests successful and checked conversions, validation errors, style
parsing and display, style matching, CamelCase acronym boundaries, digit
boundaries, empty input, Unicode safety, and best-effort conversion behavior.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Run CI checks (format, clippy, test, coverage, audit)
./ci-check.sh
```

### Coverage Metrics

See [COVERAGE.md](COVERAGE.md) for detailed coverage statistics.

## Dependencies

This crate has no runtime dependencies.

## License

Copyright (c) 2025 - 2026. Haixing Hu.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines.
- Maintain comprehensive test coverage.
- Document all public APIs with examples when they clarify usage.
- Run `./ci-check.sh` before submitting PRs.

## Author

**Haixing Hu**

## Related Projects

More Rust libraries from Qubit are published under the
[qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-case](https://github.com/qubit-ltd/rs-case)
