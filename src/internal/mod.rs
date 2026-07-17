// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Internal helpers for case-style conversion and validation.

mod boundary;
mod char_type;
mod conversion;

pub(crate) use boundary::{
    find_first_byte,
    find_first_camel_case_boundary,
    is_ascii_lower,
    is_ascii_lower_or_digit,
    is_ascii_upper,
    is_ascii_upper_or_digit,
    matches_camel,
    matches_separated,
};
pub(crate) use conversion::{
    push_ascii_case,
    push_first_char_only_to_upper,
    replace_and_change_ascii_case,
};
