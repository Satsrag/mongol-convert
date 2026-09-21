//! Port of `shape/ShapeTranslator.java` — the greedy longest-match segmenter shared by every
//! shape rule (both from-shape and to-shape directions).
//!
//! Algorithm: append the U+E666 sentinel; greedily extend the current fragment while its key stays
//! in the mapper; on a miss, backtrack one char, commit the longest match, and reprocess the char
//! (Java's `i--`). Non-translate chars flush the pending fragment+word and pass through. The final
//! `pop` strips the appended sentinel.

use crate::error::MongolConvertError;
use crate::shape::rule::ShapeTranslateRule;
use crate::strings;
use crate::word::char_type::CharType;
use crate::word::shape_word::{ShapeWord, ShapeWordFragment};

pub(crate) struct ShapeTranslator {
    rule: &'static dyn ShapeTranslateRule,
}

impl ShapeTranslator {
    pub fn new(rule: &'static dyn ShapeTranslateRule) -> Self {
        Self { rule }
    }

    pub fn translate(&self, s: &str) -> Result<String, MongolConvertError> {
        if strings::is_blank(s) {
            return Ok(String::new());
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars.push('\u{e666}'); // sentinel: forces a final flush, stripped at the end

        let mut word = ShapeWord::new();
        let mut fragment = ShapeWordFragment::new();
        let mut builder = String::new();
        fragment.set_head(Some(CharType::Other));

        let mut i = 0usize;
        while i < chars.len() {
            let c = chars[i];
            if self.rule.is_translate_code_point(c) {
                fragment.push(c);
                // chars[i+1] is always in bounds: the last char is the sentinel (never a translate
                // code point), so this branch is never entered when i == len-1.
                fragment.set_tail(self.rule.get_char_type(chars[i + 1]));
                if self.rule.contains(&fragment) {
                    i += 1;
                    continue; // greedy extend
                }
                fragment.pop()?;
                fragment.set_tail(self.rule.get_char_type(c));
                if fragment.is_blank() {
                    // Passthrough (decision #3): `c` is in this encoding's range but unmappable even
                    // alone — emit it verbatim instead of erroring. Flush the pending word first so
                    // output stays in order.
                    if word.is_not_blank() {
                        self.translate_word(&mut builder, &word)?;
                        word = ShapeWord::new();
                    }
                    builder.push(c);
                    fragment = ShapeWordFragment::new();
                    fragment.set_head(Some(CharType::Other));
                    i += 1;
                    continue;
                }
                word.add(fragment);
                fragment = ShapeWordFragment::new();
                // i >= 1 here: at i == 0 the fragment becomes blank above and we return first.
                fragment.set_head(self.rule.get_char_type(chars[i - 1]));
                continue; // reprocess c (Java's i-- then loop i++)
            } else {
                if fragment.is_not_blank() {
                    fragment.set_tail(Some(CharType::Other));
                    word.add(fragment);
                    fragment = ShapeWordFragment::new();
                    fragment.set_head(Some(CharType::Other));
                }
                if word.is_not_blank() {
                    self.translate_word(&mut builder, &word)?;
                    word = ShapeWord::new();
                }
                builder.push(c);
            }
            i += 1;
        }
        builder.pop(); // strip the sentinel
        Ok(builder)
    }

    fn translate_word(&self, builder: &mut String, word: &ShapeWord) -> Result<(), MongolConvertError> {
        let mut pre: Vec<char> = Vec::new();
        for fragment in &word.fragments {
            match self.rule.get_mapper_code(&pre, fragment) {
                Some(s) => builder.push_str(s),
                None => return Err(MongolConvertError::NotFoundInMapper(fragment.get_key())),
            }
            pre.extend(fragment.content());
        }
        Ok(())
    }
}
