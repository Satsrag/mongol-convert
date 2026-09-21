//! Letter word + fragment model. Port of `word/LetterWordFragment.java` and `word/LetterWord.java`.
//!
//! Unlike `ShapeWordFragment`, `LetterWordFragment::get_key()` IS space-padded by head/tail (it is
//! the from-direction locate key), and each fragment carries a `Nature` that latches SAARMAG -> X.

use crate::error::MongolConvertError;
use crate::unicode::mgl;
use crate::word::char_type::CharType;
use crate::word::nature::Nature;

pub struct LetterWordFragment {
    head: Option<CharType>,
    tail: Option<CharType>,
    content: Vec<char>,
    nature: Nature,
}

impl LetterWordFragment {
    pub fn new() -> Self {
        Self { head: None, tail: None, content: Vec::new(), nature: Nature::Saarmag }
    }

    /// Space-padded by head/tail context. Java `getKey` (throws on null head/tail — unreachable here).
    pub fn get_key(&self) -> String {
        if self.content.is_empty() {
            return String::new();
        }
        debug_assert!(
            self.head.is_some() && self.tail.is_some(),
            "head/tail must be set before get_key"
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

    /// Java `setNature`: one-way latch SAARMAG -> non-SAARMAG.
    pub fn set_nature(&mut self, nature: Nature) {
        if self.nature == Nature::Saarmag && nature != Nature::Saarmag {
            self.nature = nature;
        }
    }

    pub fn nature(&self) -> Nature {
        self.nature
    }

    pub fn push(&mut self, c: char) {
        self.content.push(c);
    }

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

    pub fn size(&self) -> usize {
        self.content.len()
    }

    pub fn content(&self) -> &[char] {
        &self.content
    }

    pub fn get_last_character(&self) -> Option<char> {
        self.content.last().copied()
    }

    pub fn set_head(&mut self, h: Option<CharType>) {
        self.head = h;
    }

    pub fn set_tail(&mut self, t: Option<CharType>) {
        self.tail = t;
    }
}

pub struct LetterWord {
    pub nature: Nature,
    pub fragments: Vec<LetterWordFragment>,
}

impl LetterWord {
    pub fn new() -> Self {
        Self { nature: Nature::Saarmag, fragments: Vec::new() }
    }

    /// Java `add`: latch the word nature from the first non-SAARMAG fragment, then append.
    pub fn add(&mut self, fragment: LetterWordFragment) {
        if self.nature == Nature::Saarmag && fragment.nature != Nature::Saarmag {
            self.nature = fragment.nature;
        }
        self.fragments.push(fragment);
    }

    pub fn is_blank(&self) -> bool {
        self.fragments.is_empty()
    }

    pub fn is_not_blank(&self) -> bool {
        !self.is_blank()
    }

    /// Java `removeInvalidCodePointFromEnd`: trim trailing single-char VS/FVS fragments that
    /// duplicate the preceding fragment's last char (and mark that fragment's tail OTHER).
    pub fn remove_invalid_code_point_from_end(&mut self) {
        if self.fragments.is_empty() {
            return;
        }
        let mut i = self.fragments.len() - 1;
        loop {
            if i == 0 {
                return;
            }
            let frag = &self.fragments[i];
            if frag.size() != 1 {
                return;
            }
            let ch = frag.content()[0];
            if !mgl::is_vowel_separator(ch) && !mgl::is_free_variation_selector(ch) {
                return;
            }
            let pre_last = self.fragments[i - 1].get_last_character();
            if pre_last == Some(ch) {
                self.fragments.remove(i);
                self.fragments[i - 1].set_tail(Some(CharType::Other));
            }
            i -= 1;
        }
    }
}
