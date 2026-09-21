//! UniFFI binding for `mongol-convert`: generates idiomatic Swift (iOS) and Kotlin (Android) bindings
//! from this one Rust crate, using proc-macro mode (no `.udl`). `mongol-convert` stays dependency-free
//! and `#![forbid(unsafe_code)]`; UniFFI's generated scaffolding lives here.

uniffi::setup_scaffolding!();

use mongol_convert::CodeType;
use std::str::FromStr;

// Field is `reason`, not `message`: in the generated Kotlin this enum subclasses Throwable, whose
// `message` member would clash (Swift/Python are unaffected).
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum MongolConvertError {
    #[error("{reason}")]
    Translate { reason: String },
}

/// Translate `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names
/// ("zvvnmod", "delehi", "menk_shape", "menk_letter", "z52", "utn57").
/// Throws on an unknown encoding or an unsupported conversion. UTF-8 throughout (Swift `String` / Kotlin `String`).
#[uniffi::export]
pub fn translate(from: String, to: String, input: String) -> Result<String, MongolConvertError> {
    let parse = |s: &str| {
        CodeType::from_str(s).map_err(|e| MongolConvertError::Translate { reason: e.to_string() })
    };
    let from = parse(&from)?;
    let to = parse(&to)?;
    mongol_convert::translate(from, to, &input).map_err(|e| MongolConvertError::Translate { reason: e.to_string() })
}

/// Library version.
#[uniffi::export]
pub fn version() -> String {
    mongol_convert::version().to_string()
}
