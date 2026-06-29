// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Case Style
//!
//! Defines ASCII identifier naming styles and conversion helpers.

use crate::CaseStyleError;

use std::str::FromStr;

/// XML hyphenated variable naming style, such as `lower-hyphen`.
pub const LOWER_HYPHEN: CaseStyle = CaseStyle::LowerHyphen;

/// C++/Python variable naming style, such as `lower_underscore`.
pub const LOWER_UNDERSCORE: CaseStyle = CaseStyle::LowerUnderscore;

/// Java variable naming style, such as `lowerCamel`.
pub const LOWER_CAMEL: CaseStyle = CaseStyle::LowerCamel;

/// Java and C++ class naming style, such as `UpperCamel`.
pub const UPPER_CAMEL: CaseStyle = CaseStyle::UpperCamel;

/// Java and C++ constant naming style, such as `UPPER_UNDERSCORE`.
pub const UPPER_UNDERSCORE: CaseStyle = CaseStyle::UpperUnderscore;

const VALUES: [CaseStyle; 5] = [
    LOWER_HYPHEN,
    LOWER_UNDERSCORE,
    LOWER_CAMEL,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
];

/// Naming styles supported by this crate.
///
/// The conversion rules are intended for ASCII identifiers. Non-ASCII input is
/// accepted on a best-effort basis, but its exact conversion behavior is not a
/// stable contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaseStyle {
    /// Hyphen separated lowercase words, such as `lower-hyphen`.
    LowerHyphen,

    /// Underscore separated lowercase words, such as `lower_underscore`.
    LowerUnderscore,

    /// Lower camel case words, such as `lowerCamel`.
    LowerCamel,

    /// Upper camel case words, such as `UpperCamel`.
    UpperCamel,

    /// Underscore separated uppercase words, such as `UPPER_UNDERSCORE`.
    UpperUnderscore,
}

impl CaseStyle {
    /// XML hyphenated variable naming style, such as `lower-hyphen`.
    pub const LOWER_HYPHEN: Self = Self::LowerHyphen;

    /// C++/Python variable naming style, such as `lower_underscore`.
    pub const LOWER_UNDERSCORE: Self = Self::LowerUnderscore;

    /// Java variable naming style, such as `lowerCamel`.
    pub const LOWER_CAMEL: Self = Self::LowerCamel;

    /// Java and C++ class naming style, such as `UpperCamel`.
    pub const UPPER_CAMEL: Self = Self::UpperCamel;

    /// Java and C++ constant naming style, such as `UPPER_UNDERSCORE`.
    pub const UPPER_UNDERSCORE: Self = Self::UpperUnderscore;

    /// Returns all supported naming styles in the reference implementation
    /// order.
    ///
    /// # Returns
    ///
    /// Returns a static slice containing all naming styles.
    #[inline]
    pub const fn values() -> &'static [Self; 5] {
        &VALUES
    }

    /// Parses a naming style from its canonical name.
    ///
    /// The comparison is case-insensitive, and hyphen and underscore are
    /// treated as equivalent separators.
    ///
    /// # Parameters
    ///
    /// * `name` - Naming style name to parse.
    ///
    /// # Returns
    ///
    /// Returns the parsed naming style, or `CaseStyleError` carrying the
    /// original name when no style matches.
    pub fn of(name: &str) -> Result<Self, CaseStyleError> {
        let normalized_name = name.replace('_', "-").to_ascii_lowercase();
        for style in Self::values() {
            if style.name() == normalized_name {
                return Ok(*style);
            }
        }
        Err(CaseStyleError::new(name))
    }

    /// Returns the canonical name of this naming style.
    ///
    /// # Returns
    ///
    /// Returns the lower-hyphen style name used by the JavaScript reference
    /// implementation.
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            Self::LowerHyphen => "lower-hyphen",
            Self::LowerUnderscore => "lower-underscore",
            Self::LowerCamel => "lower-camel",
            Self::UpperCamel => "upper-camel",
            Self::UpperUnderscore => "upper-underscore",
        }
    }

    /// Returns the word separator inserted by this naming style.
    ///
    /// # Returns
    ///
    /// Returns `"-"` for lower hyphen, `"_"` for underscore styles, and `""`
    /// for camel case styles.
    #[inline]
    pub const fn word_separator(self) -> &'static str {
        match self {
            Self::LowerHyphen => "-",
            Self::LowerUnderscore | Self::UpperUnderscore => "_",
            Self::LowerCamel | Self::UpperCamel => "",
        }
    }

    /// Converts a string from this style to the target style.
    ///
    /// # Parameters
    ///
    /// * `target` - Target naming style.
    /// * `value` - Source string. It should be an ASCII identifier in this
    ///   style; if it is not, conversion is best-effort.
    ///
    /// # Returns
    ///
    /// Returns the converted string. Empty input is returned as an empty
    /// string.
    pub fn to(self, target: Self, value: &str) -> String {
        if value.is_empty() || target == self {
            return value.to_string();
        }
        if let Some(converted) = self.quick_convert(target, value) {
            return converted;
        }
        self.convert_by_words(target, value)
    }

    /// Tests whether a string strictly matches this naming style.
    ///
    /// # Parameters
    ///
    /// * `value` - String to test.
    ///
    /// # Returns
    ///
    /// Returns `true` if `value` is non-empty and follows this style's ASCII
    /// identifier rules; otherwise returns `false`.
    pub fn matches(self, value: &str) -> bool {
        if value.is_empty() {
            return false;
        }
        match self {
            Self::LowerHyphen => {
                matches_separated(value, b'-', is_ascii_lower_or_digit)
            }
            Self::LowerUnderscore => {
                matches_separated(value, b'_', is_ascii_lower_or_digit)
            }
            Self::LowerCamel => matches_camel(value, is_ascii_lower),
            Self::UpperCamel => matches_camel(value, is_ascii_upper),
            Self::UpperUnderscore => {
                matches_separated(value, b'_', is_ascii_upper_or_digit)
            }
        }
    }

    /// Performs optimized direct conversions for separator-only style pairs.
    ///
    /// # Parameters
    ///
    /// * `target` - Target naming style.
    /// * `value` - Source string to convert.
    ///
    /// # Returns
    ///
    /// Returns `Some` converted string for optimized pairs, or `None` when the
    /// generic word conversion should be used.
    fn quick_convert(self, target: Self, value: &str) -> Option<String> {
        match (self, target) {
            (Self::LowerHyphen, Self::LowerUnderscore) => {
                Some(value.replace('-', "_"))
            }
            (Self::LowerHyphen, Self::UpperUnderscore) => {
                Some(value.replace('-', "_").to_ascii_uppercase())
            }
            (Self::LowerUnderscore, Self::LowerHyphen) => {
                Some(value.replace('_', "-"))
            }
            (Self::LowerUnderscore, Self::UpperUnderscore) => {
                Some(value.to_ascii_uppercase())
            }
            (Self::UpperUnderscore, Self::LowerHyphen) => {
                Some(value.replace('_', "-").to_ascii_lowercase())
            }
            (Self::UpperUnderscore, Self::LowerUnderscore) => {
                Some(value.to_ascii_lowercase())
            }
            _ => None,
        }
    }

    /// Converts a string by splitting it into source words and normalizing
    /// them.
    ///
    /// # Parameters
    ///
    /// * `target` - Target naming style.
    /// * `value` - Source string to convert.
    ///
    /// # Returns
    ///
    /// Returns a best-effort converted string.
    fn convert_by_words(self, target: Self, value: &str) -> String {
        let mut out = String::with_capacity(
            value.len() + 4 * target.word_separator().len(),
        );
        let mut word_start = 0;
        let mut search_start = 0;
        let mut has_boundary = false;
        while let Some(boundary) = self.find_boundary(value, search_start) {
            if word_start == boundary {
                search_start = boundary + self.word_separator().len().max(1);
                word_start = search_start.min(value.len());
                continue;
            }
            let word = &value[word_start..boundary];
            if word_start == 0 {
                out.push_str(&target.normalize_first_word(word));
            } else {
                out.push_str(&target.normalize_word(word));
            }
            out.push_str(target.word_separator());
            has_boundary = true;
            word_start = boundary + self.word_separator().len();
            search_start = boundary + self.word_separator().len().max(1);
        }
        if !has_boundary {
            target.normalize_first_word(value)
        } else {
            let word = &value[word_start..];
            out.push_str(&target.normalize_word(word));
            out
        }
    }

    /// Finds the next source word boundary in `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - Source string to search.
    /// * `start` - Byte index where the search starts.
    ///
    /// # Returns
    ///
    /// Returns the byte index of the next boundary, or `None` when no boundary
    /// exists after `start`.
    fn find_boundary(self, value: &str, start: usize) -> Option<usize> {
        match self {
            Self::LowerHyphen => find_first_byte(value, start, b'-'),
            Self::LowerUnderscore | Self::UpperUnderscore => {
                find_first_byte(value, start, b'_')
            }
            Self::LowerCamel | Self::UpperCamel => {
                find_first_camel_case_boundary(value, start)
            }
        }
    }

    /// Normalizes a non-first word for this naming style.
    ///
    /// # Parameters
    ///
    /// * `word` - Source word to normalize.
    ///
    /// # Returns
    ///
    /// Returns the word normalized for this style.
    fn normalize_word(self, word: &str) -> String {
        match self {
            Self::LowerHyphen | Self::LowerUnderscore => {
                word.to_ascii_lowercase()
            }
            Self::LowerCamel | Self::UpperCamel => {
                first_char_only_to_upper(word)
            }
            Self::UpperUnderscore => word.to_ascii_uppercase(),
        }
    }

    /// Normalizes the first word for this naming style.
    ///
    /// # Parameters
    ///
    /// * `word` - First source word to normalize.
    ///
    /// # Returns
    ///
    /// Returns the first word normalized for this style.
    fn normalize_first_word(self, word: &str) -> String {
        match self {
            Self::LowerCamel => word.to_ascii_lowercase(),
            _ => self.normalize_word(word),
        }
    }
}

impl FromStr for CaseStyle {
    type Err = CaseStyleError;

    /// Parses a naming style from a string.
    ///
    /// # Parameters
    ///
    /// * `s` - Naming style name to parse.
    ///
    /// # Returns
    ///
    /// Returns the parsed style, or `CaseStyleError` when `s` is unknown.
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::of(s)
    }
}

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
fn find_first_byte(value: &str, start: usize, needle: u8) -> Option<usize> {
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
fn find_first_camel_case_boundary(value: &str, start: usize) -> Option<usize> {
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
fn is_camel_case_word_boundary(value: &str, index: usize) -> bool {
    let bytes = value.as_bytes();
    if index == 0 || index >= bytes.len() {
        return false;
    }
    let current_type = char_type(bytes[index]);
    if current_type == CharType::Other {
        return false;
    }
    let previous_type = char_type(bytes[index - 1]);
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
                    .map(char_type)
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

/// ASCII character classes used by CamelCase boundary detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharType {
    /// ASCII uppercase letter.
    Upper,

    /// ASCII lowercase letter.
    Lower,

    /// ASCII decimal digit.
    Digit,

    /// Any other byte.
    Other,
}

/// Classifies an ASCII byte.
///
/// # Parameters
///
/// * `byte` - Byte to classify.
///
/// # Returns
///
/// Returns the character class used by the boundary detector.
fn char_type(byte: u8) -> CharType {
    if is_ascii_upper(byte) {
        CharType::Upper
    } else if is_ascii_lower(byte) {
        CharType::Lower
    } else if is_ascii_digit(byte) {
        CharType::Digit
    } else {
        CharType::Other
    }
}

/// Converts a word to upper camel capitalization.
///
/// # Parameters
///
/// * `word` - Source word to normalize.
///
/// # Returns
///
/// Returns the word with its first ASCII character uppercased and remaining
/// ASCII characters lowercased.
fn first_char_only_to_upper(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut out = String::with_capacity(word.len());
    out.push(first.to_ascii_uppercase());
    out.push_str(&chars.as_str().to_ascii_lowercase());
    out
}

/// Tests whether a separated style string matches all separator rules.
///
/// # Parameters
///
/// * `value` - String to test.
/// * `separator` - Required ASCII separator byte.
/// * `is_word_byte` - Predicate for valid non-separator bytes.
///
/// # Returns
///
/// Returns `true` if `value` has no leading, trailing, or repeated separators
/// and every word byte satisfies `is_word_byte`.
fn matches_separated(
    value: &str,
    separator: u8,
    is_word_byte: fn(u8) -> bool,
) -> bool {
    let bytes = value.as_bytes();
    if bytes.first() == Some(&separator) || bytes.last() == Some(&separator) {
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
fn matches_camel(value: &str, is_valid_first: fn(u8) -> bool) -> bool {
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
#[inline]
fn is_ascii_lower(byte: u8) -> bool {
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
#[inline]
fn is_ascii_upper(byte: u8) -> bool {
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
#[inline]
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
#[inline]
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
#[inline]
fn is_ascii_lower_or_digit(byte: u8) -> bool {
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
#[inline]
fn is_ascii_upper_or_digit(byte: u8) -> bool {
    is_ascii_upper(byte) || is_ascii_digit(byte)
}
