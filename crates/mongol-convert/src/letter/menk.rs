//! Port of `letter/from/menk/MenkLetterTranslateRuleFrom.java` and
//! `letter/to/menk/MenkTranslateRuleTo.java`. Menk-letter <-> Zvvnmod.
//!
//! The to-rule is the most stateful in the library: a per-word `ue031` flag rewrites a trailing
//! U+180D U+1822 across fragments, plus a chagh/hundii nature fallback over three tables.

use crate::code_mapper::StaticMap;
use crate::error::MongolConvertError;
use crate::letter::rule::{
    ends_with_boundary, LetterTranslateRuleFrom, LetterTranslateRuleTo, WORD_CONNECTOR,
};
use crate::tables::from_menk_letter::{
    FROM_MENK_LETTER, FROM_MENK_LETTER_CHAGH, FROM_MENK_LETTER_HUNDII, FROM_MENK_LETTER_SAARMAG,
    FROM_MENK_LETTER_W_WITH_EHSHIG,
};
use crate::tables::to_menk_letter::{TO_MENK_LETTER, TO_MENK_LETTER_CHAGH, TO_MENK_LETTER_HUNDII};
use crate::unicode::{mgl, zvvnmod};
use crate::word::nature::Nature;
use crate::word::shape_word::ShapeWord;

fn is_word_connector(c: char) -> bool {
    c == '\u{202f}'
}

// ---------- FROM: Menk-letter -> Zvvnmod ----------

static F_MAP: StaticMap = StaticMap::new(FROM_MENK_LETTER);
static F_CHAGH: StaticMap = StaticMap::new(FROM_MENK_LETTER_CHAGH);
static F_HUNDII: StaticMap = StaticMap::new(FROM_MENK_LETTER_HUNDII);
static F_SAARMAG: StaticMap = StaticMap::new(FROM_MENK_LETTER_SAARMAG);
static F_W_EHSHIG: StaticMap = StaticMap::new(FROM_MENK_LETTER_W_WITH_EHSHIG);

// FromMenkLetterCodeMapper.doubleIEhishig — NOTE: lacks U+1822 (unlike Delehi's set).
const DOUBLE_I_EHSHIG: [char; 4] = ['\u{1820}', '\u{1821}', '\u{1823}', '\u{1824}'];

pub(crate) struct MenkLetterFrom;

impl LetterTranslateRuleFrom for MenkLetterFrom {
    fn get_mapper_code(
        &self,
        pre: &[char],
        suf: &[char],
        key: &str,
        nature: Nature,
    ) -> Option<&'static str> {
        if key.trim_matches(' ') == "\u{180a}" {
            return Some("\u{180a}");
        }
        if let Some(r) = resolve_devsger_i(pre, key) {
            return Some(r);
        }
        if let Some(r) = resolve_w(pre, key) {
            return Some(r);
        }
        if let Some(r) = resolve_t(suf, key) {
            return Some(r);
        }
        if let Some(r) = resolve_g(suf, key, nature) {
            return Some(r);
        }
        if let Some(r) = F_MAP.get(key) {
            return Some(r);
        }
        match nature {
            Nature::Chagh => F_CHAGH.get(key),
            Nature::Hundii => F_HUNDII.get(key),
            Nature::Saarmag => F_SAARMAG.get(key),
        }
    }

    fn contains(&self, key: &str) -> bool {
        F_MAP.contains_key(key) || F_CHAGH.contains_key(key)
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

fn resolve_devsger_i(pre: &[char], s: &str) -> Option<&'static str> {
    if s != "\u{1836}" && s != "\u{1822}" {
        return None;
    }
    let c = *pre.last()?;
    if s == "\u{1822}" && (c == '\u{1822}' || c == '\u{1836}') && pre.len() > 2 {
        let pre2 = pre[pre.len() - 2];
        if DOUBLE_I_EHSHIG.contains(&pre2) {
            return Some("");
        }
    }
    if DOUBLE_I_EHSHIG.contains(&c) {
        return Some("\u{e006}\u{e006}");
    }
    None
}

fn resolve_w(pre: &[char], s: &str) -> Option<&'static str> {
    if !s.contains('\u{1838}') {
        return None;
    }
    let c = *pre.last()?;
    if mgl::is_ehshig(c) {
        return F_W_EHSHIG.get(s);
    }
    None
}

fn resolve_t(suf: &[char], s: &str) -> Option<&'static str> {
    if s == "\u{1832}" && !suf.is_empty() && mgl::is_giiguulegch(suf[0]) {
        return Some("\u{e043}");
    }
    None
}

fn resolve_g(suf: &[char], s: &str, nature: Nature) -> Option<&'static str> {
    if s == "\u{182d}" && !suf.is_empty() && mgl::is_giiguulegch(suf[0]) {
        return Some(if nature == Nature::Chagh {
            "\u{e005}\u{e005}"
        } else {
            "\u{e031}"
        });
    }
    None
}

// ---------- TO: Zvvnmod -> Menk-letter ----------

static T_MAP: StaticMap = StaticMap::new(TO_MENK_LETTER);
static T_HUNDII: StaticMap = StaticMap::new(TO_MENK_LETTER_HUNDII);
static T_CHAGH: StaticMap = StaticMap::new(TO_MENK_LETTER_CHAGH);

pub(crate) struct MenkLetterTo;

impl LetterTranslateRuleTo for MenkLetterTo {
    fn get_mapper_code(&self, builder: &mut String, word: &ShapeWord) -> Result<(), MongolConvertError> {
        let mut s = String::new();
        let mut ue031 = false;
        for fragment in &word.fragments {
            let key = fragment.get_key();
            let mut s1: Option<String> = get_menk(&s, &key, word.nature).map(str::to_string);

            if key == "\u{e031}" {
                ue031 = true;
                if s.ends_with("\u{180d}\u{1822}") {
                    let mut cs: Vec<char> = s.chars().collect();
                    cs.truncate(cs.len() - 2);
                    s = cs.into_iter().collect();
                    s.push('\u{1822}');
                }
            }
            if ue031 {
                if let Some(v) = &s1 {
                    if v.contains("\u{180d}\u{1822}") {
                        ue031 = false;
                        s1 = Some(v.replace("\u{180d}\u{1822}", "\u{1822}"));
                    }
                }
            }

            let s1 = match s1 {
                Some(v) => v,
                None => return Err(MongolConvertError::NotFoundInMapper(key)),
            };
            s = concat_and_202f(&s, &s1);
        }
        if s.starts_with(WORD_CONNECTOR) && ends_with_boundary(builder) {
            builder.pop();
        }
        builder.push_str(&s);
        Ok(())
    }

    fn contains(&self, key: &str) -> bool {
        T_CHAGH.contains_key(key) || T_MAP.contains_key(key)
    }

    fn is_translate_code_point(&self, c: char) -> bool {
        zvvnmod::is_zvvnmod_code(c)
    }
}

// Java MenkTranslateRuleTo.get: resolveUe00c, then resolveSingleGiiAndUe011, then the
// chagh/hundii table with a fallback to the base table when the nature table is empty/absent.
fn get_menk(pre: &str, s: &str, nature: Nature) -> Option<&'static str> {
    if let Some(r) = resolve_ue00c_menk(pre, s, nature) {
        return Some(r);
    }
    if let Some(r) = resolve_single_gii_and_ue011(pre, s) {
        return Some(r);
    }
    let m = if nature == Nature::Chagh {
        T_CHAGH.get(s)
    } else {
        T_HUNDII.get(s)
    };
    match m {
        Some(v) if !v.is_empty() => Some(v),
        _ => T_MAP.get(s),
    }
}

fn resolve_ue00c_menk(pre: &str, s: &str, nature: Nature) -> Option<&'static str> {
    if s != "\u{e00c}" {
        return None;
    }
    let chagh_or_hundii = if nature == Nature::Chagh {
        "\u{1820}"
    } else {
        "\u{1821}"
    };
    match pre.chars().last() {
        None => Some(chagh_or_hundii),
        Some(c) if mgl::is_traditional_ehshig(c) => Some("\u{1828}"),
        Some(_) => Some(chagh_or_hundii),
    }
}

fn resolve_single_gii_and_ue011(pre: &str, s: &str) -> Option<&'static str> {
    if s != "\u{e011}" || pre.is_empty() {
        return None;
    }
    if pre.chars().count() == 1 {
        let ch = pre.chars().next().unwrap();
        if mgl::is_giiguulegch(ch) {
            return Some(if ('\u{1832}'..='\u{1834}').contains(&ch) {
                "\u{1824}"
            } else {
                "\u{1824}\u{180b}"
            });
        }
    }
    None
}

fn concat_and_202f(s0: &str, s1: &str) -> String {
    if s1.starts_with(WORD_CONNECTOR) && ends_with_boundary(s0) {
        let mut trimmed = s0.to_owned();
        trimmed.pop(); // char-aware: the boundary is a 1-byte space or a 3-byte NNBSP
        format!("{trimmed}{s1}")
    } else {
        format!("{s0}{s1}")
    }
}
