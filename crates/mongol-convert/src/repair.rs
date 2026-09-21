//! Optional input repair, before source decoding.
//!
//! This is a spelling heuristic, not a morphological analyser. The small allowlist follows the
//! separated case suffixes in https://unicode.org/L2/L2019/19130-mwg3-8-mong-spec-r.pdf.
//! Forms such as bar (also "tiger") and tür (also "temporarily") are accepted only in the
//! stem contexts where a corpus shows an ordinary space before them is almost always damage.
//! See docs/suffix-separator-repair.md for the Hudum particle mapping audit: shaping support
//! alone does not establish that an ordinary space is a damaged suffix separator.

use crate::{CodeType, MongolConvertError, Warning};
use std::borrow::Cow;

// MenkLetter/Delehi letter spellings, not canonical UTN #57 spellings. No FVS is stripped to
// make a match: a selector may carry information this heuristic does not understand.
const SUFFIXES: &[&str] = &[
    "ᠶᠢᠨ",
    "ᠤᠨ",
    "ᠦᠨ",
    "ᠤ",
    "ᠦ", // genitive
    "ᠶᠢ",
    "ᠢ", // accusative
    "ᠳᠤ",
    "ᠳᠦ",
    "ᠲᠤ",
    "ᠲᠦ",
    "ᠳᠤᠷ",
    "ᠳᠦᠷ",
    "ᠲᠤᠷ",
    "ᠲᠦᠷ", // dative
    "ᠠᠴᠠ",
    "ᠡᠴᠡ", // ablative
    "ᠢᠶᠠᠷ",
    "ᠢᠶᠡᠷ", // instrumental (bar/ber excluded)
    "ᠢᠶᠠᠨ",
    "ᠢᠶᠡᠨ", // reflexive
    "ᠯᠤᠭ\u{180E}ᠠ",
    "ᠯᠦᠭᠡ", // comitative; preserve the internal MVS in luγ-a
    "ᠨᠤᠭᠤᠳ",
    "ᠨᠦᠭᠦᠳ",
    "ᠤᠳ",
    "ᠦᠳ", // plural
    "ᠳᠠᠭᠠᠨ",
    "ᠳᠡᠭᠡᠨ",
    "ᠲᠠᠭᠠᠨ",
    "ᠲᠡᠭᠡᠨ", // reflexive dative
    "ᠶᠤᠭᠠᠨ",
    "ᠶᠦᠭᠡᠨ", // reflexive accusative
    "ᠠᠴᠠᠭᠠᠨ",
    "ᠡᠴᠡᠭᠡᠨ", // reflexive ablative
    "ᠳᠤᠨᠢ",
    "ᠳᠦᠨᠢ",
    "ᠲᠤᠨᠢ",
    "ᠲᠦᠨᠢ", // possessive dative
    "ᠳᠠᠬᠢ",
    "ᠳᠡᠬᠢ",
    "ᠲᠠᠬᠢ",
    "ᠲᠡᠬᠢ",  // locative attributive; additions use suffix_context below
    "ᠲᠡᠬᠡᠨ", // tegen: feminine g and k share ink, and this is the common corpus spelling
    "ᠪᠠᠷ",
    "ᠪᠡᠷ", // instrumental after vowels
    "ᠪᠠᠨ",
    "ᠪᠡᠨ", // reflexive after vowels
    "ᠲᠠᠢ",
    "ᠲᠡᠢ", // comitative
    "ᠨᠠᠷ",
    "ᠨᠡᠷ", // plural
    "ᠳᠤᠭᠠᠷ",
    "ᠳᠦᠭᠡᠷ", // ordinal, not after Mongolian words
];

fn letter(c: char) -> bool {
    matches!(c, '\u{1820}'..='\u{1842}')
}

fn word_char(c: char) -> bool {
    letter(c) || matches!(c, '\u{180B}'..='\u{180F}' | '\u{200C}' | '\u{200D}')
}

fn mongolian_word(s: &str) -> bool {
    s.chars().all(word_char) && s.chars().any(letter)
}

// Final consonants that select the T-initial allomorph (tu, tur, taki, tegen, ...).
const T_SELECTING: &str = "ᠪᠭᠬᠷᠰᠱᠳᠲᠴᠺᠫᠹᠽᠼᠾ";

// Any other token can take a suffix: a number, a Latin or Chinese word, a closing bracket or
// quote around a title (︾ ᠶᠢᠨ), or a unit symbol (60° ᠡᠴᠡ). Sentence punctuation, opening
// brackets, other spaces, line starts and controls cannot.
fn token_end(c: char) -> bool {
    c.is_alphanumeric()
        || matches!(
            c,
            ')' | ']'
                | '}'
                | '"'
                | '\''
                | '\u{2019}'
                | '\u{201D}'
                | '\u{00BB}'
                | '\u{203A}'
                | '\u{300B}'
                | '\u{300D}'
                | '\u{300F}'
                | '\u{3009}'
                | '\u{3011}'
                | '\u{3015}'
                | '\u{FF09}'
                | '\u{FF3D}'
                | '\u{FF5D}'
                | '\u{FE36}'
                | '\u{FE38}'
                | '\u{FE3A}'
                | '\u{FE3C}'
                | '\u{FE3E}'
                | '\u{FE40}'
                | '\u{FE42}'
                | '\u{FE44}'
                | '%'
                | '\u{FF05}'
                | '\u{2030}'
                | '\u{00B0}'
                | '\u{2103}'
                | '\u{2109}'
                | '+'
                | '$'
                | '\u{00A5}'
                | '\u{20AC}'
                | '\u{00A3}'
                | '\u{FFE5}'
                | '#'
                | '\u{2116}'
        )
}

// Rendering after NNBSP depends only on the suffix, so no vowel-harmony check is made. These
// rules only decline stems after which the same letters are usually an independent word.
fn suffix_context(previous: &str, suffix: &str) -> bool {
    let last = previous.chars().rev().find(|&c| letter(c));
    match suffix {
        // After a Mongolian word, dugar is an independent word (ordinals attach to numeral
        // words). After a number or any other token it is the ordinal.
        "ᠳᠤᠭᠠᠷ" | "ᠳᠦᠭᠡᠷ" => false,
        "ᠲᠠᠢ" | "ᠲᠡᠢ" => true,
        // After a vowel or n, ᠲᠦᠷ is the independent word tür.
        _ if suffix.starts_with('ᠲ') => last.is_some_and(|c| T_SELECTING.contains(c)),
        // After a consonant, bar is usually the independent word (tiger, bar). A final ᠶ is a
        // diphthong, and a chachlag stem ends in A/E.
        "ᠪᠠᠷ" | "ᠪᠡᠷ" | "ᠪᠠᠨ" | "ᠪᠡᠨ" => {
            last.is_some_and(|c| c <= '\u{1827}' || c == 'ᠶ')
        }
        _ => true,
    }
}

/// Replace a single ordinary or non-breaking space before an allowlisted suffix. All offsets
/// describe the original UTF-8 input. Existing suffix boundaries, layout and word letters stay
/// untouched. Runs joined without a space are never split.
pub(crate) fn suffix_separators(
    from: CodeType,
    input: &str,
) -> Result<(Cow<'_, str>, Vec<Warning>), MongolConvertError> {
    if !matches!(from, CodeType::MenkLetter | CodeType::Delehi) {
        return Err(MongolConvertError::UnsupportedInputRepair(from));
    }

    let mut out = String::new();
    let mut warnings = Vec::new();
    let mut copied = 0;
    let mut word_start = 0;
    for (offset, c) in input.char_indices() {
        if matches!(c, ' ' | '\u{00A0}') {
            let next = offset + c.len_utf8();
            let previous = &input[word_start..offset];
            let word = mongolian_word(previous);
            let other_token =
                previous.is_empty() && input[..offset].chars().next_back().is_some_and(token_end);
            // Bound the lookahead to the short suffix inventory. This also avoids copying or
            // rescanning arbitrary following words in a large document.
            let suffix = (word || other_token)
                && SUFFIXES.iter().any(|suffix| {
                    input[next..].strip_prefix(suffix).is_some_and(|rest| {
                        (!word || suffix_context(previous, suffix))
                            && rest.chars().next().map_or(true, |c| {
                                c.is_whitespace()
                                    || matches!(
                                        c,
                                        '\u{1800}'
                                            ..='\u{1809}'
                                                | '.'
                                                | ','
                                                | ';'
                                                | ':'
                                                | '!'
                                                | '?'
                                                | ')'
                                                | ']'
                                                | '}'
                                                | '"'
                                                | '\''
                                    )
                            })
                    })
                });
            if suffix {
                out.push_str(&input[copied..offset]);
                out.push('\u{202F}');
                copied = next;
                warnings.push(Warning::RepairedSuffixSeparator {
                    byte_offset: offset,
                    original: c,
                });
            }
        }
        if !word_char(c) {
            word_start = offset + c.len_utf8();
        }
    }
    if warnings.is_empty() {
        Ok((Cow::Borrowed(input), warnings))
    } else {
        out.push_str(&input[copied..]);
        Ok((Cow::Owned(out), warnings))
    }
}
