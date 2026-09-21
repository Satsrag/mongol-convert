//! Port of `letter/from/delehi/DelehiTranslateRuleFrom.java` and
//! `letter/to/delehi/DelehiTranslateRuleTo.java`. Delehi (standard Unicode) <-> Zvvnmod.

use crate::code_mapper::StaticMap;
use crate::error::MongolConvertError;
use crate::letter::rule::{
    ends_with_boundary, LetterTranslateRuleFrom, LetterTranslateRuleTo, WORD_CONNECTOR,
};
use crate::tables::from_delehi::{
    FROM_DELEHI, FROM_DELEHI_CHAGH, FROM_DELEHI_HUNDII, FROM_DELEHI_SAARMAG,
};
use crate::tables::to_delehi::TO_DELEHI;
use crate::unicode::{mgl, zvvnmod};
use crate::word::nature::Nature;
use crate::word::shape_word::ShapeWord;

// DelehiCodeBlock.isWordConnector
fn is_word_connector(c: char) -> bool {
    c == '\u{202f}'
}

// ---------- FROM: Delehi -> Zvvnmod ----------

static FROM_MAP: StaticMap = StaticMap::new(FROM_DELEHI);
static CHAGH: StaticMap = StaticMap::new(FROM_DELEHI_CHAGH);
static HUNDII: StaticMap = StaticMap::new(FROM_DELEHI_HUNDII);
static SAARMAG: StaticMap = StaticMap::new(FROM_DELEHI_SAARMAG);

// FromDelehiCodeMapper.doubleIEhishig
const DOUBLE_I_EHSHIG: [char; 5] = ['\u{1820}', '\u{1821}', '\u{1822}', '\u{1823}', '\u{1824}'];

pub(crate) struct DelehiFrom;

impl LetterTranslateRuleFrom for DelehiFrom {
    fn get_mapper_code(
        &self,
        pre: &[char],
        _suf: &[char],
        key: &str,
        nature: Nature,
    ) -> Option<&'static str> {
        if key.trim_matches(' ') == "\u{180a}" {
            return Some("\u{180a}");
        }
        if let Some(r) = resolve_devsger_i(pre, key) {
            return Some(r);
        }
        if let Some(r) = FROM_MAP.get(key) {
            return Some(r);
        }
        match nature {
            Nature::Chagh => CHAGH.get(key),
            Nature::Hundii => HUNDII.get(key),
            Nature::Saarmag => SAARMAG.get(key),
        }
    }

    fn contains(&self, key: &str) -> bool {
        FROM_MAP.contains_key(key) || CHAGH.contains_key(key)
    }

    fn get_code_nature(&self, c: char) -> Nature {
        mgl::get_code_nature(c)
    }

    fn is_translate_code_point(&self, c: char) -> bool {
        mgl::is_normal_letter(c)
            || mgl::is_free_variation_selector(c)
            || mgl::is_vowel_separator(c)
            || is_word_connector(c)
            || mgl::other_mongolian_code(c)
    }

    fn is_word_code_point(&self, c: char) -> bool {
        mgl::is_normal_letter(c)
            || mgl::is_free_variation_selector(c)
            || mgl::is_vowel_separator(c)
            || mgl::other_mongolian_code(c)
    }
}

// resolveDevsgerI: medial "i" (U+1822) after a double-i ehshig vowel -> "".
fn resolve_devsger_i(pre: &[char], key: &str) -> Option<&'static str> {
    if key != "\u{1822}" || pre.is_empty() {
        return None;
    }
    let c = *pre.last().unwrap();
    if DOUBLE_I_EHSHIG.contains(&c) {
        Some("\u{e006}\u{e006}")
    } else {
        None
    }
}

// ---------- TO: Zvvnmod -> Delehi ----------

static TO_MAP: StaticMap = StaticMap::new(TO_DELEHI);

pub(crate) struct DelehiTo;

impl LetterTranslateRuleTo for DelehiTo {
    fn get_mapper_code(&self, builder: &mut String, word: &ShapeWord) -> Result<(), MongolConvertError> {
        let mut s = String::new();
        for fragment in &word.fragments {
            match get_delehi(&s, &fragment.get_key()) {
                Some(v) => s = v,
                None => return Err(MongolConvertError::NotFoundInMapper(fragment.get_key())),
            }
        }
        // Cross-word merge: leading NNBSP collapses a trailing space already in the builder.
        if s.starts_with(WORD_CONNECTOR) && ends_with_boundary(builder) {
            builder.pop();
        }
        builder.push_str(&s);
        Ok(())
    }

    fn contains(&self, key: &str) -> bool {
        TO_MAP.contains_key(key)
    }

    fn is_translate_code_point(&self, c: char) -> bool {
        zvvnmod::is_zvvnmod_code(c)
    }
}

// Java DelehiTranslateRuleTo.get: resolveUe00c first, else concatAnd202f(pre, mapper.get(key)).
// Returns the new accumulated output string, or None if the key is unmapped.
fn get_delehi(pre_letter_codes: &str, key: &str) -> Option<String> {
    // resolveUe00c: tail "n"/"a" disambiguation for U+E00C.
    if key == "\u{e00c}" {
        let tail = match pre_letter_codes.chars().last() {
            None => '\u{1820}',
            Some(c) if mgl::is_traditional_ehshig(c) => '\u{1828}',
            Some(_) => '\u{1820}',
        };
        return Some(format!("{pre_letter_codes}{tail}"));
    }
    let mapped = TO_MAP.get(key)?;
    Some(concat_and_202f(pre_letter_codes, mapped))
}

// concatAnd202f: a leading NNBSP in s1 collapses a trailing space at the end of s0.
fn concat_and_202f(s0: &str, s1: &str) -> String {
    if s1.starts_with(WORD_CONNECTOR) && ends_with_boundary(s0) {
        let mut trimmed = s0.to_owned();
        trimmed.pop(); // char-aware: the boundary is a 1-byte space or a 3-byte NNBSP
        format!("{trimmed}{s1}")
    } else {
        format!("{s0}{s1}")
    }
}
