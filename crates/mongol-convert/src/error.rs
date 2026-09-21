//! Error type. Folds the Java `State` / `TranslateState` / `DelehiState` runtime codes into one
//! Rust enum.
//!
//! Note (design decision #3): a content-level *unmappable code point* is **not** an error here —
//! it is passed through unchanged. So [`MongolConvertError::NotFoundInMapper`] is reserved for internal/
//! diagnostic use. The default build's public `translate` returns `Err` only for structural problems
//! (unsupported encoding, unsupported series, unknown enum string) and for UTN #57 conversion
//! failures reported by the in-process `zvvnmod-utn57` backend.

use crate::code_type::CodeType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MongolConvertError {
    /// No translate rule registered for this code (defensive; should be unreachable for supported types).
    MissTranslateRule(CodeType),
    /// Internal stack underflow during fragment processing.
    NothingToPop,
    /// A key was not found in a mapper table (internal/diagnostic; content path passes through instead).
    NotFoundInMapper(String),
    /// A code's series was neither Letter nor Shape (defensive; unreachable given the enum).
    NotSupportedCodeSeries(CodeType),
    /// A string could not be parsed into a [`CodeType`].
    UnsupportedEnumType(String),
    /// Conversion involving this code is not supported in the active build.
    Unsupported(CodeType),
    /// Suffix separator repair currently accepts MenkLetter and Delehi source text only.
    UnsupportedInputRepair(CodeType),
    /// An in-process UTN #57 conversion (`zvvnmod-utn57` + `mongol-norm`), either direction, failed.
    Utn57(String),
}

impl fmt::Display for MongolConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MongolConvertError::MissTranslateRule(ct) => write!(f, "missing translate rule for {ct:?}"),
            MongolConvertError::NothingToPop => write!(f, "nothing to pop"),
            MongolConvertError::NotFoundInMapper(k) => write!(f, "key not found in mapper: {k:?}"),
            MongolConvertError::NotSupportedCodeSeries(ct) => {
                write!(f, "unsupported code series for {ct:?}")
            }
            MongolConvertError::UnsupportedEnumType(s) => write!(f, "unsupported encoding name: {s:?}"),
            MongolConvertError::Unsupported(ct) => write!(f, "conversion not supported for {ct:?}"),
            MongolConvertError::UnsupportedInputRepair(ct) => write!(
                f,
                "suffix separator repair requires menk_letter or delehi input, got {ct:?}"
            ),
            MongolConvertError::Utn57(reason) => write!(f, "UTN #57 conversion failed: {reason}"),
        }
    }
}

impl std::error::Error for MongolConvertError {}
