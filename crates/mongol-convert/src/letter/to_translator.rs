//! Port of `letter/to/LetterToTranslator.java`. Zvvnmod -> Letter.
//!
//! Reuses the shape word model (`ShapeWord`/`ShapeWordFragment`, keyed unpadded). Greedy match, but
//! on a miss it starts the next fragment with the current char inline (no reprocess index). The rule
//! emits the whole word into `builder` (word-level state lives in the rule, not the translator).

use crate::error::MongolConvertError;
use crate::letter::rule::LetterTranslateRuleTo;
use crate::strings;
use crate::word::shape_word::{ShapeWord, ShapeWordFragment};

pub(crate) struct LetterToTranslator {
    rule: &'static dyn LetterTranslateRuleTo,
}

impl LetterToTranslator {
    pub fn new(rule: &'static dyn LetterTranslateRuleTo) -> Self {
        Self { rule }
    }

    pub fn translate(&self, s: &str) -> Result<String, MongolConvertError> {
        if strings::is_blank(s) {
            return Ok(String::new());
        }
        let mut chars: Vec<char> = s.chars().collect();
        chars.push('\u{e666}');

        let mut builder = String::new();
        let mut fragment = ShapeWordFragment::new();
        let mut word = ShapeWord::new();

        for c in chars {
            if self.rule.is_translate_code_point(c) {
                fragment.push(c);
                if self.rule.contains(&fragment.get_key()) {
                    continue;
                }
                fragment.pop()?;
                if fragment.is_blank() {
                    // Passthrough (decision #3): in-range but unmappable -> emit verbatim.
                    if word.is_not_blank() {
                        self.rule.get_mapper_code(&mut builder, &word)?;
                        word = ShapeWord::new();
                    }
                    builder.push(c);
                    fragment = ShapeWordFragment::new();
                    continue;
                }
                word.add(fragment);
                fragment = ShapeWordFragment::new();
                fragment.push(c);
            } else {
                if fragment.is_not_blank() {
                    word.add(fragment);
                    fragment = ShapeWordFragment::new();
                }
                if word.is_not_blank() {
                    self.rule.get_mapper_code(&mut builder, &word)?;
                    word = ShapeWord::new();
                }
                builder.push(c);
            }
        }
        builder.pop(); // strip sentinel
        Ok(builder)
    }
}
