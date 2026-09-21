//! Shape word + fragment model. Port of `word/ShapeWordFragment.java` and `word/ShapeWord.java`.
//!
//! Load-bearing subtlety (per the design risks): `get_key()` is the raw content (UNpadded, used by
//! the *to-shape* rules), while `get_locate_key()` is space-padded by head/tail `CharType` (used by
//! the *from-shape* rules). Conflating them silently mis-maps.

use crate::error::MongolConvertError;
use crate::word::char_type::CharType;
use crate::word::nature::Nature;

#[derive(Default)]
pub struct ShapeWordFragment {
    head: Option<CharType>,
    tail: Option<CharType>,
    content: Vec<char>,
}

impl ShapeWordFragment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, c: char) {
        self.content.push(c);
    }

    /// Java `pop`: errors with `NOTHING_TO_POP` if empty.
    pub fn pop(&mut self) -> Result<(), MongolConvertError> {
        if self.content.is_empty() {
            return Err(MongolConvertError::NothingToPop);
        }
        self.content.pop();
        Ok(())
    }

    pub fn is_blank(&self) -> bool {
        self.content.is_empty()
    }

    pub fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }

    pub fn set_head(&mut self, h: Option<CharType>) {
        self.head = h;
    }

    pub fn set_tail(&mut self, t: Option<CharType>) {
        self.tail = t;
    }

    pub fn content(&self) -> &[char] {
        &self.content
    }

    /// Raw content joined, no padding. Java `getKey`.
    pub fn get_key(&self) -> String {
        self.content.iter().collect()
    }

    /// Content with a leading/trailing U+0020 when head/tail is not Mongolian. Java `getLocateKey`.
    /// Only the from-shape rules call this, and the translator always sets head+tail beforehand
    /// (Java would throw `UNICODE_TYPE_NOT_BE_NULL` on null — that state is unreachable here).
    pub fn get_locate_key(&self) -> String {
        if self.content.is_empty() {
            return String::new();
        }
        debug_assert!(
            self.head.is_some() && self.tail.is_some(),
            "head/tail must be set before get_locate_key"
        );
        let mut s = String::with_capacity(self.content.len() + 2);
        if self.head != Some(CharType::Mongolian) {
            s.push('\u{20}');
        }
        s.extend(&self.content);
        if self.tail != Some(CharType::Mongolian) {
            s.push('\u{20}');
        }
        s
    }
}

pub struct ShapeWord {
    pub nature: Nature,
    pub fragments: Vec<ShapeWordFragment>,
}

impl ShapeWord {
    pub fn new() -> Self {
        // Java ShapeWord defaults to CHAGH (vs LetterWord's SAARMAG).
        Self { nature: Nature::Chagh, fragments: Vec::new() }
    }

    /// Java `add`: latches nature to HUNDII for three magic fragment keys, then appends.
    pub fn add(&mut self, fragment: ShapeWordFragment) {
        let k = fragment.get_key();
        if k == "\u{e006}\u{e00d}" || k == "\u{e031}" || k == "\u{e006}\u{e006}\u{e00d}" {
            self.nature = Nature::Hundii;
        }
        self.fragments.push(fragment);
    }

    pub fn is_blank(&self) -> bool {
        self.fragments.is_empty()
    }

    pub fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_vs_locate_key_padding() {
        let mut f = ShapeWordFragment::new();
        f.push('\u{e000}');
        f.set_head(Some(CharType::Other));
        f.set_tail(Some(CharType::Other));
        assert_eq!(f.get_key(), "\u{e000}");
        assert_eq!(f.get_locate_key(), " \u{e000} "); // both sides padded (non-Mongolian)

        f.set_head(Some(CharType::Mongolian));
        f.set_tail(Some(CharType::Mongolian));
        assert_eq!(f.get_locate_key(), "\u{e000}"); // no padding when both Mongolian
        assert_eq!(f.get_key(), "\u{e000}"); // get_key never pads
    }

    #[test]
    fn pop_empty_errs() {
        let mut f = ShapeWordFragment::new();
        assert_eq!(f.pop(), Err(MongolConvertError::NothingToPop));
    }
}
