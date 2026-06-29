// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Case Style Errors
//!
//! Defines errors returned when parsing case style names.

use std::fmt;

/// Error returned when a case style name is unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseStyleError {
    name: String,
}

impl CaseStyleError {
    /// Creates an unknown case style error.
    ///
    /// # Parameters
    ///
    /// * `name` - Original style name that could not be parsed.
    ///
    /// # Returns
    ///
    /// Returns a new error carrying the original name.
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the unknown naming style name.
    ///
    /// # Returns
    ///
    /// Returns the original name passed to `CaseStyle::of` or `FromStr`.
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for CaseStyleError {
    /// Formats the unknown naming style error.
    ///
    /// # Parameters
    ///
    /// * `f` - Formatter receiving the message.
    ///
    /// # Returns
    ///
    /// Returns the formatter result.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown case style: '{}'.", self.name)
    }
}

impl std::error::Error for CaseStyleError {}
