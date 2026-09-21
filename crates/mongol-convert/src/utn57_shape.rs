//! `utn57_shape`: UTN #57 text spelled as written units — ᠰᠠᠢᠨ as `SAIIA`.
//!
//! The encoding is `utn57` seen through mongol-norm's shape function. Writing renders each
//! Mongolian word of a UTN #57 text into its written units and concatenates their PascalCase
//! names, structural units included (`Mvs`, `Nirugu`, `Zwj`); reading parses each spelled word —
//! compact `SAIIA` or `+`-joined `S+A+I+I+A` — and asks mongol-norm for the canonical UTN #57
//! spelling of that shape. Everything that is not a word passes through unchanged, as in every
//! other encoding.
//!
//! Design: `docs/superpowers/specs/2026-09-13-utn57-shape-encoding-design.md`.

use crate::error::MongolConvertError;
use std::sync::OnceLock;
use zvvnmod_utn57::mongol_norm::{is_mongolian_word_char, Locale, Shaper};

/// The Hudum shaper, built once for the process from mongol-norm's static tables.
fn shaper() -> &'static Shaper {
    static SHAPER: OnceLock<Shaper> = OnceLock::new();
    SHAPER.get_or_init(|| Shaper::new(Locale::Mng))
}

fn error(reason: impl std::fmt::Display) -> MongolConvertError {
    MongolConvertError::Utn57(format!("utn57_shape: {reason}"))
}

/// UTN #57 text → shape text: every Mongolian word as its written units, the rest verbatim.
pub(crate) fn encode(utn57: &str) -> Result<String, MongolConvertError> {
    fn flush(word: &mut String, out: &mut String) -> Result<(), MongolConvertError> {
        if word.is_empty() {
            return Ok(());
        }
        for unit in shaper().shape(word).map_err(error)? {
            out.push_str(unit.as_str());
        }
        word.clear();
        Ok(())
    }

    let mut out = String::with_capacity(utn57.len());
    let mut word = String::new();
    for c in utn57.chars() {
        if is_mongolian_word_char(c) {
            word.push(c);
        } else {
            flush(&mut word, &mut out)?;
            out.push(c);
        }
    }
    flush(&mut word, &mut out)?;
    Ok(out)
}

/// Shape text → UTN #57 text: every spelled word as its canonical spelling, the rest verbatim.
///
/// A spelled word that is not one — an unknown unit, an ambiguous compact segmentation, a shape
/// the normalize table does not cover — is an error, not passthrough: the encoding's alphabet is
/// ASCII, so keeping the text would hand a wrong word downstream.
pub(crate) fn decode(shape: &str) -> Result<String, MongolConvertError> {
    let mut out = String::with_capacity(shape.len() * 3);
    for piece in split(shape) {
        match piece {
            Piece::Word(word) => {
                let shaper = shaper();
                let units = shaper.parse_written_units(word).map_err(error)?;
                out.push_str(&shaper.normalize_written_units(&units).map_err(error)?);
            }
            Piece::Other(text) => out.push_str(text),
        }
    }
    Ok(out)
}

/// One piece of shape text: a spelled word, or anything else.
#[derive(Debug, PartialEq, Eq)]
enum Piece<'a> {
    Word(&'a str),
    Other(&'a str),
}

/// The characters a spelled word is made of: unit names, digits (`K2`, `B2`) and the `+` joiner.
fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '+'
}

/// Split shape text into spelled words and passthrough.
///
/// A word is a maximal run of ASCII letters, digits and `+` that begins with an uppercase letter —
/// every unit name does, so a run that begins otherwise (`2024`, `hello`) is not a spelling and
/// passes through with whatever surrounds it.
fn split(text: &str) -> Vec<Piece<'_>> {
    let mut pieces = Vec::new();
    let mut other_start: Option<usize> = None;
    let mut index = 0;
    while index < text.len() {
        let rest = &text[index..];
        let run = rest.find(|c: char| !is_word_char(c)).unwrap_or(rest.len());
        if run > 0 && rest.starts_with(|c: char| c.is_ascii_uppercase()) {
            if let Some(start) = other_start.take() {
                pieces.push(Piece::Other(&text[start..index]));
            }
            pieces.push(Piece::Word(&rest[..run]));
            index += run;
        } else {
            other_start.get_or_insert(index);
            index += if run > 0 {
                run
            } else {
                rest.chars().next().map_or(0, char::len_utf8)
            };
        }
    }
    if let Some(start) = other_start {
        pieces.push(Piece::Other(&text[start..]));
    }
    pieces
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ᠰᠠᠢᠨ in its canonical UTN #57 spelling.
    const SAIN: &str = "\u{1830}\u{1820}\u{1822}\u{180D}\u{1822}\u{180D}\u{1820}\u{180C}";
    /// ᠮᠣᠩᠭᠣᠯ᠎ᠤᠨ in its canonical UTN #57 spelling.
    const MONGOL_UN: &str = "\u{182E}\u{1823}\u{1820}\u{182D}\u{180C}\u{1828}\u{180B}\u{1828}\u{180B}\u{1823}\u{182F}\u{180E}\u{1824}\u{180B}\u{1820}\u{180C}";

    #[test]
    fn a_word_is_spelled_as_its_written_units() {
        assert_eq!(encode(SAIN).unwrap(), "SAIIA");
        assert_eq!(encode(MONGOL_UN).unwrap(), "MOAGNNOLMvsOA");
    }

    #[test]
    fn compact_and_joined_input_reach_the_same_canonical_text() {
        assert_eq!(decode("SAIIA").unwrap(), SAIN);
        assert_eq!(decode("S+A+I+I+A").unwrap(), SAIN);
        assert_eq!(decode("MOAGNNOLMvsOA").unwrap(), MONGOL_UN);
    }

    #[test]
    fn text_outside_a_word_passes_through_both_ways() {
        assert_eq!(
            encode(&format!("{SAIN} {SAIN}\u{1802} 2024 hello")).unwrap(),
            "SAIIA SAIIA\u{1802} 2024 hello"
        );
        assert_eq!(
            decode("SAIIA SAIIA\u{1802} 2024 hello").unwrap(),
            format!("{SAIN} {SAIN}\u{1802} 2024 hello")
        );
        assert_eq!(decode("").unwrap(), "");
        assert_eq!(encode("  ").unwrap(), "  ");
    }

    #[test]
    fn an_uppercase_led_run_that_is_not_a_spelling_is_an_error() {
        let error = decode("Hello").unwrap_err();
        assert!(
            error
                .to_string()
                .starts_with("UTN #57 conversion failed: utn57_shape:"),
            "{error}"
        );
        assert!(matches!(decode("SAIIA Hello"), Err(MongolConvertError::Utn57(_))));
    }

    #[test]
    fn words_are_uppercase_led_ascii_runs() {
        assert_eq!(
            split("SAIIA, 2024 hello S+A"),
            vec![
                Piece::Word("SAIIA"),
                Piece::Other(", 2024 hello "),
                Piece::Word("S+A")
            ]
        );
        assert_eq!(split(""), Vec::<Piece<'_>>::new());
        assert_eq!(split("ᠠ"), vec![Piece::Other("ᠠ")]);
    }
}
