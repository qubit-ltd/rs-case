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

mod case_style;
mod case_style_error;
mod case_style_validation_error;
mod internal;

pub use case_style_error::CaseStyleError;
pub use case_style_validation_error::CaseStyleValidationError;

pub use case_style::{
    CaseStyle,
    LOWER_CAMEL,
    LOWER_HYPHEN,
    LOWER_UNDERSCORE,
    UPPER_CAMEL,
    UPPER_UNDERSCORE,
};
