// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Character classes used by CamelCase boundary detection.

/// ASCII character classes used by CamelCase boundary detection.
#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CharType {
    /// ASCII uppercase letter.
    Upper,

    /// ASCII lowercase letter.
    Lower,

    /// ASCII decimal digit.
    Digit,

    /// Any other byte.
    Other,
}

impl CharType {
    /// Classifies an ASCII byte.
    ///
    /// # Parameters
    ///
    /// * `byte` - Byte to classify.
    ///
    /// # Returns
    ///
    /// Returns the character class used by the boundary detector.
    #[inline]
    pub(super) fn of(byte: u8) -> Self {
        if byte.is_ascii_uppercase() {
            Self::Upper
        } else if byte.is_ascii_lowercase() {
            Self::Lower
        } else if byte.is_ascii_digit() {
            Self::Digit
        } else {
            Self::Other
        }
    }
}
