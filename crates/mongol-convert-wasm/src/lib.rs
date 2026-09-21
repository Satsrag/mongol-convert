//! WebAssembly binding for `mongol-convert` via wasm-bindgen — an npm package usable in the browser,
//! Node, Deno/Bun and edge runtimes (Cloudflare Workers, etc.). Strings marshal automatically.
//! `mongol-convert` stays `#![forbid(unsafe_code)]`; wasm-bindgen's glue is confined to this crate.

use mongol_convert::{CodeType, TranslationOptions, Warning};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// Translate `input` from encoding `from` to `to`. `from`/`to` are canonical encoding names
/// ("zvvnmod", "delehi", "menk_shape", "menk_letter", "z52", "utn57").
/// Throws a JS `Error` on an unknown encoding name or an unsupported conversion.
#[wasm_bindgen]
pub fn translate(from: &str, to: &str, input: &str) -> Result<String, JsError> {
    let from = CodeType::from_str(from).map_err(|e| JsError::new(&e.to_string()))?;
    let to = CodeType::from_str(to).map_err(|e| JsError::new(&e.to_string()))?;
    mongol_convert::translate(from, to, input).map_err(|e| JsError::new(&e.to_string()))
}

/// A finished conversion: `text` is what `translate` returns, `warnings` says what the conversion
/// had to do beyond what the input said (empty for most conversions). Today only the `utn57`
/// target raises any, for a hub run it could spell only with an invented ZWJ. `repairs` lists
/// the suffix separators that `translate_with_options` restored, one entry per edit; it is empty
/// unless repair was requested.
#[wasm_bindgen(getter_with_clone)]
pub struct Translation {
    pub text: String,
    pub warnings: Vec<String>,
    pub repairs: Vec<String>,
}

fn finish(translation: mongol_convert::Translation) -> Translation {
    let (repairs, warnings): (Vec<_>, Vec<_>) = translation
        .warnings
        .iter()
        .partition(|w| matches!(w, Warning::RepairedSuffixSeparator { .. }));
    Translation {
        text: translation.text,
        warnings: warnings.iter().map(|w| w.to_string()).collect(),
        repairs: repairs.iter().map(|w| w.to_string()).collect(),
    }
}

/// Like `translate`, and also reports the conversion's warnings. Throws on the same errors.
#[wasm_bindgen]
pub fn translate_with_warnings(from: &str, to: &str, input: &str) -> Result<Translation, JsError> {
    let from = CodeType::from_str(from).map_err(|e| JsError::new(&e.to_string()))?;
    let to = CodeType::from_str(to).map_err(|e| JsError::new(&e.to_string()))?;
    let translation = mongol_convert::translate_with_warnings(from, to, input)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(finish(translation))
}

/// Like `translate_with_warnings`, with `repair_suffix_separators` set, also restores NNBSP before
/// known suffixes in `menk_letter` / `delehi` input first (a space that lost its NNBSP would
/// otherwise shape as an independent word). Throws if repair is requested for any other source.
#[wasm_bindgen]
pub fn translate_with_options(
    from: &str,
    to: &str,
    input: &str,
    repair_suffix_separators: bool,
) -> Result<Translation, JsError> {
    translate_with_all_options(from, to, input, repair_suffix_separators, true)
}

/// Like `translate_with_options`, with all currently supported input-normalization switches.
#[wasm_bindgen]
pub fn translate_with_all_options(
    from: &str,
    to: &str,
    input: &str,
    repair_suffix_separators: bool,
    restore_menk_shape_emoji: bool,
) -> Result<Translation, JsError> {
    let from = CodeType::from_str(from).map_err(|e| JsError::new(&e.to_string()))?;
    let to = CodeType::from_str(to).map_err(|e| JsError::new(&e.to_string()))?;
    let options = TranslationOptions {
        repair_suffix_separators,
        restore_menk_shape_emoji,
    };
    let translation = mongol_convert::translate_with_options(from, to, input, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(finish(translation))
}

/// Library version.
#[wasm_bindgen]
pub fn version() -> String {
    mongol_convert::version().to_string()
}

/// Restore legacy SoftBank/iOS emoji that collide with MenkShape PUA.
#[wasm_bindgen]
pub fn restore_menk_shape_emoji(input: &str) -> String {
    mongol_convert::restore_menk_shape_emoji(input)
}
