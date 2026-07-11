// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Case Style Validation Errors
//!
//! Defines errors returned when validating values against case styles.

use std::fmt;

use crate::CaseStyle;

/// Error returned when a value does not match an expected case style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseStyleValidationError {
    style: CaseStyle,
    value: String,
}

impl CaseStyleValidationError {
    /// Creates a case style validation error.
    ///
    /// # Parameters
    ///
    /// * `style` - Expected case style.
    /// * `value` - Original value that failed validation.
    ///
    /// # Returns
    ///
    /// Returns a new error carrying the expected style and original value.
    #[inline]
    pub fn new(style: CaseStyle, value: impl Into<String>) -> Self {
        Self {
            style,
            value: value.into(),
        }
    }

    /// Returns the expected case style.
    ///
    /// # Returns
    ///
    /// Returns the style used to validate the value.
    #[inline]
    pub const fn style(&self) -> CaseStyle {
        self.style
    }

    /// Returns the invalid value.
    ///
    /// # Returns
    ///
    /// Returns the original value passed to `CaseStyle::validate` or
    /// `CaseStyle::checked_to`.
    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for CaseStyleValidationError {
    /// Formats the case style validation error.
    ///
    /// # Parameters
    ///
    /// * `f` - Formatter receiving the message.
    ///
    /// # Returns
    ///
    /// Returns the formatter result.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Value '{}' does not match case style '{}'.",
            self.value,
            self.style.name(),
        )
    }
}

impl std::error::Error for CaseStyleValidationError {}
