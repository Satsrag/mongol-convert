//! Top-level routing. Port of `service/TranslateService.java` (no Spring DI).
//!
//! Hub-and-spoke through Zvvnmod: decode `from` to the hub (unless it is the hub), then encode the
//! hub to `to` (unless it is the hub). Short-circuit exactly as Java: identity or blank input is
//! returned unchanged. Oyun stays Unsupported in both directions. UTN #57 goes both ways, handed
//! to the pure-Rust `zvvnmod-utn57` crate in process rather than through the letter/shape tables.

use crate::code_type::{CodeSeries, CodeType};
use crate::dispatch::{letter_from_rule, letter_to_rule, shape_from_rule, shape_to_rule};
use crate::error::MecoError;
use crate::letter::from_translator::LetterFromTranslator;
use crate::letter::rule::WORD_CONNECTOR;
use crate::letter::to_translator::LetterToTranslator;
use crate::shape::punctuation_gap;
use crate::shape::softbank_emoji;
use crate::shape::translator::ShapeTranslator;
use crate::strings;
use crate::unicode::zvvnmod::is_zvvnmod_code;
use crate::utn57_shape;
use std::borrow::Cow;
use std::fmt;

/// Something a conversion did beyond what its input said, including optional heuristic repairs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Warning {
    /// The UTN #57 encoder spelled a hub run with a ZWJ the hub did not carry.
    ///
    /// The hub is positional, and a run that begins or ends with a joined-form glyph — a medial at
    /// the start, a medial or initial at the end — can only be joined to nothing by inventing a
    /// joiner. When the source meant a whole word, that is a gap in the hub's inventory: the glyph
    /// the source needed in that position does not exist there, the way `G i O f` did not until
    /// `E096` (Satsrag/mongol-convert#32). The reason is the encoder's own message and names the run's
    /// codes, so the missing glyph can be read off it.
    Utn57(String),
    /// An opt-in heuristic replaced a space before a recognised suffix with NNBSP.
    /// The offset is a UTF-8 byte offset in the original input, before any repairs or conversion.
    RepairedSuffixSeparator { byte_offset: usize, original: char },
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Warning::Utn57(reason) => write!(f, "UTN #57: {reason}"),
            Warning::RepairedSuffixSeparator { byte_offset, original } => write!(
                f,
                "repaired possible suffix separator at input byte {byte_offset}: U+{:04X} -> U+202F",
                *original as u32
            ),
        }
    }
}

/// A finished conversion: the text, and whatever the conversion had to do beyond what the input
/// said. See [`translate_with_warnings`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    /// The converted text, after any explicitly enabled input repairs.
    pub text: String,
    /// Input repairs in source order, followed by conversion warnings.
    pub warnings: Vec<Warning>,
}

/// Optional processing before conversion. Default settings preserve the input behavior of
/// [`translate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranslationOptions {
    /// Replace single spaces/NBSP before a small allowlist of detached suffixes with NNBSP.
    /// Supported for MenkLetter and Delehi sources, including same-encoding conversions.
    /// This is a heuristic: it neither analyses grammar nor splits concatenated words. Each
    /// replacement is reported as [`Warning::RepairedSuffixSeparator`].
    pub repair_suffix_separators: bool,
    /// Restore legacy SoftBank/iOS emoji back to MenkShape PUA before decoding MenkShape input.
    /// This is enabled by default because chat apps can rewrite MenkShape PUA into modern emoji.
    pub restore_menk_shape_emoji: bool,
}

impl Default for TranslationOptions {
    fn default() -> Self {
        Self {
            repair_suffix_separators: false,
            restore_menk_shape_emoji: true,
        }
    }
}

/// Convert with optional input repair. Repairs run before source decoding, even when `from == to`.
/// With default options this is identical to [`translate_with_warnings`].
pub fn translate_with_options(
    from: CodeType,
    to: CodeType,
    input: &str,
    options: &TranslationOptions,
) -> Result<Translation, MecoError> {
    let (input, mut warnings) = if options.repair_suffix_separators {
        crate::repair::suffix_separators(from, input)?
    } else {
        (Cow::Borrowed(input), Vec::new())
    };
    let mut result = translate_inner(from, to, &input, options)?;
    warnings.append(&mut result.warnings);
    result.warnings = warnings;
    Ok(result)
}

impl Translation {
    fn plain(text: String) -> Self {
        Self {
            text,
            warnings: Vec::new(),
        }
    }
}

/// Convert `input` from one Mongolian encoding to another. UTF-8 in/out.
///
/// The text-only form of [`translate_with_warnings`]: a conversion that had to go beyond its input
/// still succeeds here, silently.
pub fn translate(from: CodeType, to: CodeType, input: &str) -> Result<String, MecoError> {
    translate_with_warnings(from, to, input).map(|translation| translation.text)
}

/// Convert `input` from one Mongolian encoding to another, and say what the conversion had to do
/// beyond what the input said. UTF-8 in/out.
///
/// The text is the one [`translate`] returns. The warnings are [`Warning`]s; today only the
/// UTN #57 target raises any, for a hub run it could spell only with an invented ZWJ.
pub fn translate_with_warnings(
    from: CodeType,
    to: CodeType,
    input: &str,
) -> Result<Translation, MecoError> {
    translate_inner(from, to, input, &TranslationOptions::default())
}

fn translate_inner(
    from: CodeType,
    to: CodeType,
    input: &str,
    options: &TranslationOptions,
) -> Result<Translation, MecoError> {
    if strings::is_blank(input) {
        return Ok(Translation::plain(input.to_string()));
    }
    if from == to {
        return Ok(Translation::plain(
            normalize_menk_shape_source(from, input, options).into_owned(),
        ));
    }
    let hub = if from == CodeType::Zvvnmod {
        input.to_string()
    } else {
        translate_from(from, input, options)?
    };
    if to == CodeType::Zvvnmod {
        return Ok(Translation::plain(hub));
    }
    translate_to(to, &hub)
}

/// The nirugu, as the Zvvnmod hub spells it.
///
/// ZVVNMOD's own inventory has a code for it and the UTN #57 crate uses that one, so the hub does
/// too: it is the hub's own spelling, the way every other hub code is. The legacy tables were
/// dumped from Java, which knew the nirugu only as Unicode's `U+180A` — MenkShape maps that to
/// `E23E`, the Unicode encodings keep it as it is — so each side is handed the spelling it knows,
/// the way the suffix boundary already is below. Left untranslated, `E0E5` reached MenkShape
/// output and rendered as a missing glyph (Satsrag/mongol-convert#29).
const HUB_NIRUGU: &str = "\u{E0E5}";

/// The nirugu as the legacy tables and the Unicode encodings spell it.
const UNICODE_NIRUGU: &str = "\u{180A}";

/// The hub's word-initial G + O-final ligature, `E096` (`G i O f`).
///
/// ZVVNMOD's font never had this glyph. Every other bowed consonant carries both a word-initial
/// and a medial ligature with a final O; G has only the medial `G m O f`, `E09C`, so Menksoft —
/// and the Java tables dumped from it — wrote that at the start of a word too. The hub is
/// positional and the UTN #57 encoder reads it that way: a medial glyph with nothing to its left
/// can only be spelled with an invented ZWJ (Satsrag/mongol-convert#32). So the hub promotes a
/// word-initial `E09C` to `E096`, which `zvvnmod-utn57` knows as `G_O_ISOL`, and demotes it again
/// for the legacy tables, whose ink for `E09C` is exactly the initial ligature's.
const HUB_G_O_ISOL: char = '\u{E096}';

/// The medial ligature the legacy tables spell the whole word with.
const LEGACY_G_O_FINA: char = '\u{E09C}';

/// Promote every `E09C` that starts a word to `E096`.
///
/// "Starts a word" is what the UTN #57 encoder will see: nothing to its left that joins — no hub
/// shape, no nirugu, no ZWJ. FVS marks and the legacy controls `E140..=E144` are transparent, as
/// they are for the encoder, which drops the latter and passes the former through.
fn promote_word_initial_g_o(hub: &str) -> String {
    let chars: Vec<char> = hub.chars().collect();
    let mut out = String::with_capacity(hub.len());
    for (index, &c) in chars.iter().enumerate() {
        if c == LEGACY_G_O_FINA && !joined_on_the_left(&chars[..index]) {
            out.push(HUB_G_O_ISOL);
        } else {
            out.push(c);
        }
    }
    out
}

fn joined_on_the_left(before: &[char]) -> bool {
    before
        .iter()
        .rev()
        .find(|c| !is_transparent_mark(**c))
        .is_some_and(|c| joins_to_the_right(*c))
}

fn is_transparent_mark(c: char) -> bool {
    matches!(c, '\u{180B}'..='\u{180D}' | '\u{E140}'..='\u{E144}')
}

fn joins_to_the_right(c: char) -> bool {
    // The Java set does not know the hub's own codes, E0E5 and E096.
    is_zvvnmod_code(c) || matches!(c, HUB_G_O_ISOL | '\u{E0E5}' | '\u{180A}' | '\u{200D}')
}

fn normalize_menk_shape_source<'a>(
    ct: CodeType,
    s: &'a str,
    options: &TranslationOptions,
) -> Cow<'a, str> {
    if ct == CodeType::MenkShape && options.restore_menk_shape_emoji {
        softbank_emoji::restore_menk_shape(s)
    } else {
        Cow::Borrowed(s)
    }
}

fn translate_from(
    ct: CodeType,
    s: &str,
    options: &TranslationOptions,
) -> Result<String, MecoError> {
    if ct == CodeType::Oyun {
        return Err(MecoError::Unsupported(ct));
    }
    if ct == CodeType::Utn57Shape {
        // The written-unit spelling of a UTN #57 text: read it as that text.
        let utn57 = utn57_shape::decode(s)?;
        return translate_from(CodeType::Utn57, &utn57, options);
    }
    if ct == CodeType::Utn57 {
        // Already hub-spelled: the UTN #57 crate reads and writes E0E5 itself.
        return zvvnmod_utn57::convert_utn57_to_zvvnmod(s)
            .map_err(|error| MecoError::Utn57(error.to_string()));
    }
    let hub = match ct.code_series() {
        // Menksoft's punctuation gap is spacing, not content: it comes back out before the text
        // reaches the hub, so both sides agree on what the word is.
        CodeSeries::Shape => {
            // The punctuation gap is spacing, not content: it comes back out before the text
            // reaches the hub, so both sides agree on what the word is.
            let source = normalize_menk_shape_source(ct, s, options);
            let plain = match punctuation_gap::of(ct) {
                Some(gap) => gap.strip(&source),
                None => source.into_owned(),
            };
            ShapeTranslator::new(shape_from_rule(ct)?).translate(&plain)?
        }
        CodeSeries::Letter => LetterFromTranslator::new(letter_from_rule(ct)?).translate(s)?,
    };
    Ok(promote_word_initial_g_o(
        &hub.replace(UNICODE_NIRUGU, HUB_NIRUGU),
    ))
}

fn translate_to(ct: CodeType, s: &str) -> Result<Translation, MecoError> {
    if ct == CodeType::Oyun {
        return Err(MecoError::Unsupported(ct));
    }
    if ct == CodeType::Utn57Shape {
        // The UTN #57 conversion, then its shape; the warnings are that conversion's.
        let utn57 = translate_to(CodeType::Utn57, s)?;
        return Ok(Translation {
            text: utn57_shape::encode(&utn57.text)?,
            warnings: utn57.warnings,
        });
    }
    // A hub written by hand, or by an older release, may still spell the nirugu the Unicode way.
    // Both readings are accepted; only the hub spelling is ever produced.
    let hub = s.replace(UNICODE_NIRUGU, HUB_NIRUGU);
    if ct == CodeType::Utn57 {
        let conversion = zvvnmod_utn57::convert_zvvnmod_to_utn57_with_warnings(&hub)
            .map_err(|error| MecoError::Utn57(error.to_string()))?;
        return Ok(Translation {
            text: conversion.text,
            warnings: conversion
                .warnings
                .iter()
                .map(|warning| Warning::Utn57(warning.to_string()))
                .collect(),
        });
    }
    // The legacy tables predate E096; they write its ink for E09C.
    let legacy = hub
        .replace(HUB_NIRUGU, UNICODE_NIRUGU)
        .replace(HUB_G_O_ISOL, &LEGACY_G_O_FINA.to_string());
    let text = match ct.code_series() {
        // The shape encodings have no NNBSP of their own: they spell a suffix boundary with an
        // ordinary space and always have, so the hub's connector is flattened back for them.
        CodeSeries::Shape => {
            let flattened = legacy.replace(WORD_CONNECTOR, " ");
            let shaped = ShapeTranslator::new(shape_to_rule(ct)?).translate(&flattened)?;
            // The shape encodings' punctuation glyphs have no side bearing of their own, so the
            // gap is written in with the space each of them uses.
            match punctuation_gap::of(ct) {
                Some(gap) => gap.insert(&shaped),
                None => shaped,
            }
        }
        CodeSeries::Letter => LetterToTranslator::new(letter_to_rule(ct)?).translate(&legacy)?,
    };
    Ok(Translation::plain(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hub_g_o_isol_is_the_utn57_crate_inventory_code() {
        assert_eq!(u32::from(HUB_G_O_ISOL), zvvnmod_utn57::G_O_ISOL.0);
        assert_eq!(u32::from(LEGACY_G_O_FINA), zvvnmod_utn57::G_O_FINA.0);
    }

    #[test]
    fn only_a_word_initial_g_o_is_promoted() {
        let cases = [
            ("\u{E09C}", "\u{E096}"),
            (" \u{E09C} ", " \u{E096} "),
            ("\u{1802}\u{E09C}", "\u{1802}\u{E096}"),
            ("\u{202F}\u{E09C}", "\u{202F}\u{E096}"),
            ("\u{E00C}\u{202F}\u{E09C}", "\u{E00C}\u{202F}\u{E096}"),
            // Joined on the left: a hub shape, the nirugu either way, a ZWJ.
            ("\u{E000}\u{E005}\u{E09C}", "\u{E000}\u{E005}\u{E09C}"),
            ("\u{E0E5}\u{E09C}", "\u{E0E5}\u{E09C}"),
            ("\u{180A}\u{E09C}", "\u{180A}\u{E09C}"),
            ("\u{200D}\u{E09C}", "\u{200D}\u{E09C}"),
            // FVS and the legacy controls are transparent: the shape before them still joins.
            ("\u{E006}\u{E140}\u{E09C}", "\u{E006}\u{E140}\u{E09C}"),
            ("\u{E006}\u{180B}\u{E09C}", "\u{E006}\u{180B}\u{E09C}"),
            ("\u{180B}\u{E09C}", "\u{180B}\u{E096}"),
            // Already promoted, or something else entirely.
            ("\u{E096}", "\u{E096}"),
            ("\u{E093}", "\u{E093}"),
        ];
        for (hub, expected) in cases {
            assert_eq!(promote_word_initial_g_o(hub), expected, "{hub:?}");
        }
    }

    #[test]
    fn the_hub_nirugu_is_the_utn57_crate_inventory_code() {
        let hub = HUB_NIRUGU.chars().next().unwrap();
        assert_eq!(u32::from(hub), zvvnmod_utn57::NIRUGU.0);
        assert_eq!(UNICODE_NIRUGU, "\u{180A}");
    }
}
