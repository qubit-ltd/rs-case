// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Case
//!
//! Provides naming style detection and conversion helpers for ASCII
//! identifiers.

pub mod case_style;
/// Case style detection, parsing, and conversion utilities.
mod case_style_error;
pub use case_style_error::CaseStyleError;

pub use case_style::{
    CaseStyle,
    LOWER_CAMEL,
    LOWER_HYPHEN,
    LOWER_UNDERSCORE,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
};
