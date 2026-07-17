// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String normalization helpers used by case-style conversion.

/// Appends a word using upper camel capitalization.
///
/// # Parameters
///
/// * `out` - Destination string.
/// * `word` - Source word to normalize.
///
/// Appends the word with its first ASCII character uppercased and remaining
/// ASCII characters lowercased.
#[inline]
pub(crate) fn push_first_char_only_to_upper(out: &mut String, word: &str) {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return;
    };
    out.push(first.to_ascii_uppercase());
    push_ascii_case(out, chars.as_str(), false);
}

/// Appends a string after converting its ASCII character case.
///
/// # Parameters
///
/// * `out` - Destination string.
/// * `value` - Source string.
/// * `uppercase` - Whether ASCII characters are uppercased rather than
///   lowercased.
pub(crate) fn push_ascii_case(out: &mut String, value: &str, uppercase: bool) {
    out.extend(value.chars().map(|ch| {
        if uppercase {
            ch.to_ascii_uppercase()
        } else {
            ch.to_ascii_lowercase()
        }
    }));
}

/// Replaces one character and converts ASCII character case in one pass.
///
/// # Parameters
///
/// * `value` - Source string.
/// * `from` - Character to replace.
/// * `to` - Replacement character.
/// * `uppercase` - Whether other ASCII characters are uppercased rather than
///   lowercased.
///
/// # Returns
///
/// Returns a newly allocated string produced in one traversal of `value`.
#[must_use]
pub(crate) fn replace_and_change_ascii_case(
    value: &str,
    from: char,
    to: char,
    uppercase: bool,
) -> String {
    let mut out = String::with_capacity(value.len());
    out.extend(value.chars().map(|ch| {
        if ch == from {
            to
        } else if uppercase {
            ch.to_ascii_uppercase()
        } else {
            ch.to_ascii_lowercase()
        }
    }));
    out
}
