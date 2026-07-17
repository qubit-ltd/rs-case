// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Word-boundary detection and identifier validation helpers.

use super::char_type::CharType;

/// Finds the first occurrence of a byte from a start index.
///
/// # Parameters
///
/// * `value` - String to search.
/// * `start` - Byte index where the search starts.
/// * `needle` - ASCII byte to find.
///
/// # Returns
///
/// Returns the byte index of `needle`, or `None` if it is not found.
pub(crate) fn find_first_byte(
    value: &str,
    start: usize,
    needle: u8,
) -> Option<usize> {
    value
        .as_bytes()
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, byte)| (*byte == needle).then_some(index))
}

/// Finds the next CamelCase word boundary.
///
/// # Parameters
///
/// * `value` - String to search.
/// * `start` - Byte index where the search starts.
///
/// # Returns
///
/// Returns the byte index of the next CamelCase boundary, or `None` if no
/// boundary exists.
pub(crate) fn find_first_camel_case_boundary(
    value: &str,
    start: usize,
) -> Option<usize> {
    let start = start.max(1);
    value
        .as_bytes()
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, _)| {
            is_camel_case_word_boundary(value, index).then_some(index)
        })
}

/// Tests whether an index is a CamelCase word boundary.
///
/// # Parameters
///
/// * `value` - String to test.
/// * `index` - Byte index to examine.
///
/// # Returns
///
/// Returns `true` if `index` is a boundary according to the JavaScript
/// reference finite-state rules; otherwise returns `false`.
#[must_use]
fn is_camel_case_word_boundary(value: &str, index: usize) -> bool {
    let bytes = value.as_bytes();
    if index == 0 || index >= bytes.len() {
        return false;
    }
    let current_type = CharType::of(bytes[index]);
    if current_type == CharType::Other {
        return false;
    }
    let previous_type = CharType::of(bytes[index - 1]);
    match previous_type {
        CharType::Lower => {
            current_type == CharType::Upper || current_type == CharType::Digit
        }
        CharType::Upper => {
            if current_type == CharType::Lower {
                false
            } else if current_type == CharType::Digit {
                true
            } else if current_type == CharType::Upper {
                let next = bytes
                    .get(index + 1)
                    .copied()
                    .map(CharType::of)
                    .unwrap_or(CharType::Other);
                next == CharType::Lower
            } else {
                false
            }
        }
        CharType::Digit => current_type == CharType::Upper,
        CharType::Other => false,
    }
}

/// Tests whether a separated style string matches all separator rules.
///
/// # Parameters
///
/// * `value` - String to test.
/// * `separator` - Required ASCII separator byte.
/// * `is_valid_first` - Predicate for the required first byte.
/// * `is_word_byte` - Predicate for valid non-separator bytes.
///
/// # Returns
///
/// Returns `true` if `value` has no leading, trailing, or repeated separators
/// and every word byte satisfies `is_word_byte`.
#[must_use]
pub(crate) fn matches_separated(
    value: &str,
    separator: u8,
    is_valid_first: fn(u8) -> bool,
    is_word_byte: fn(u8) -> bool,
) -> bool {
    let bytes = value.as_bytes();
    let Some(first) = bytes.first() else {
        return false;
    };
    if !is_valid_first(*first) || bytes.last() == Some(&separator) {
        return false;
    }
    let mut last_is_separator = false;
    for byte in bytes {
        if *byte == separator {
            if last_is_separator {
                return false;
            }
            last_is_separator = true;
        } else if is_word_byte(*byte) {
            last_is_separator = false;
        } else {
            return false;
        }
    }
    true
}

/// Tests whether a camel style string matches the first character rule.
///
/// # Parameters
///
/// * `value` - String to test.
/// * `is_valid_first` - Predicate for the required first byte.
///
/// # Returns
///
/// Returns `true` if the first byte satisfies `is_valid_first` and all
/// following bytes are ASCII letters or digits.
///
/// # Panics
///
/// Panics if `value` is empty. Callers must reject empty input first.
#[must_use]
pub(crate) fn matches_camel(
    value: &str,
    is_valid_first: fn(u8) -> bool,
) -> bool {
    let bytes = value.as_bytes();
    if !is_valid_first(bytes[0]) {
        return false;
    }
    bytes.iter().skip(1).all(|byte| is_ascii_alnum(*byte))
}

/// Tests whether a byte is an ASCII lowercase letter.
///
/// # Parameters
///
/// * `byte` - Byte to test.
///
/// # Returns
///
/// Returns `true` for `a` through `z`.
#[must_use]
#[inline(always)]
pub(crate) fn is_ascii_lower(byte: u8) -> bool {
    byte.is_ascii_lowercase()
}

/// Tests whether a byte is an ASCII uppercase letter.
///
/// # Parameters
///
/// * `byte` - Byte to test.
///
/// # Returns
///
/// Returns `true` for `A` through `Z`.
#[must_use]
#[inline(always)]
pub(crate) fn is_ascii_upper(byte: u8) -> bool {
    byte.is_ascii_uppercase()
}

/// Tests whether a byte is an ASCII decimal digit.
///
/// # Parameters
///
/// * `byte` - Byte to test.
///
/// # Returns
///
/// Returns `true` for `0` through `9`.
#[must_use]
#[inline(always)]
fn is_ascii_digit(byte: u8) -> bool {
    byte.is_ascii_digit()
}

/// Tests whether a byte is an ASCII letter or digit.
///
/// # Parameters
///
/// * `byte` - Byte to test.
///
/// # Returns
///
/// Returns `true` for ASCII letters or decimal digits.
#[must_use]
#[inline(always)]
fn is_ascii_alnum(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

/// Tests whether a byte is an ASCII lowercase letter or digit.
///
/// # Parameters
///
/// * `byte` - Byte to test.
///
/// # Returns
///
/// Returns `true` for lowercase ASCII letters or decimal digits.
#[must_use]
#[inline(always)]
pub(crate) fn is_ascii_lower_or_digit(byte: u8) -> bool {
    is_ascii_lower(byte) || is_ascii_digit(byte)
}

/// Tests whether a byte is an ASCII uppercase letter or digit.
///
/// # Parameters
///
/// * `byte` - Byte to test.
///
/// # Returns
///
/// Returns `true` for uppercase ASCII letters or decimal digits.
#[must_use]
#[inline(always)]
pub(crate) fn is_ascii_upper_or_digit(byte: u8) -> bool {
    is_ascii_upper(byte) || is_ascii_digit(byte)
}
