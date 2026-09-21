#![forbid(unsafe_code)]
// TODO(remove before binding stage): silence dead_code while the spine is wired up incrementally.
// Every item is exercised by unit tests; once the router/translators consume them this comes off.
#![allow(dead_code)]
//! # mongol-convert
//!
//! Pure-Rust core of the Mongolian Encoding Converter (`mongol-convert`), ported from the
//! original Java library `com.zvvnmod.mongolconvert`.
//!
//! Design & decisions: `docs/superpowers/specs/2026-06-18-meco-rust-port-design.md`.
//!
//! Ground rules baked into this crate:
//! - Generated Java tables are the historical behavior baseline; Rust-owned Zvvnmod rules may
//!   intentionally override them when the project adopts corrected semantics.
//! - Conversions route through the intermediate **Zvvnmod** encoding.
//! - The build has no I/O or framework and is pure compute on every platform, including
//!   `wasm32-unknown-unknown`. UTN #57 target conversion is linked in through the pure-Rust
//!   `zvvnmod-utn57` crate; no external command or interpreter is started.
//!
//! Status: scaffolding — the shared spine (encoding types, errors, string helpers) is in place.
//! Translation routing and the shape/letter subsystems are added in later steps.

mod code_mapper;
mod code_type;
mod dispatch;
mod error;
mod letter;
mod repair;
mod router;
mod shape;
mod strings;
mod tables;
mod unicode;
mod utn57_shape;
mod word;

pub use code_type::{CodeSeries, CodeType};
pub use error::MongolConvertError;
pub use router::{
    translate, translate_with_options, translate_with_warnings, Translation, TranslationOptions,
    Warning,
};

/// Restore legacy SoftBank/iOS emoji that collide with MenkShape PUA.
///
/// This is a normalization helper for paste paths. It does not decode MenkShape; it only maps
/// modern Unicode emoji back to the corresponding MenkShape private-use code points when that
/// collision is known.
pub fn restore_menk_shape_emoji(input: &str) -> String {
    shape::softbank_emoji::restore_menk_shape(input).into_owned()
}

/// Crate version (currently just the Cargo package version). A table-provenance tag
/// (the Java commit the generated tables come from) will be appended once tables exist.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
