//! Z52 Unicode block predicates. Port of `word/Z52UnicodeBlock.java`, backed by the generated
//! membership sets (`tables::z52_block`).

use crate::tables::z52_block::{Z52_CODE_PUNCTUATIONS, Z52_CODES};
use crate::unicode::sorted_contains;

/// Java `Z52UnicodeBlock.z52Codes.contains` — the Z52 glyph code points (sparse).
#[inline]
pub fn is_z52_code(c: char) -> bool {
    sorted_contains(Z52_CODES, c)
}

/// Java `Z52UnicodeBlock.z52CodePunctuations.contains` — the Z52 punctuation code points.
#[inline]
pub fn is_z52_code_punctuation(c: char) -> bool {
    sorted_contains(Z52_CODE_PUNCTUATIONS, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn membership() {
        assert!(is_z52_code('\u{1865}'));
        assert!(is_z52_code('\u{18aa}'));
        assert!(!is_z52_code('\u{1869}')); // a real gap in the Z52 set
        assert!(!is_z52_code('\u{184f}')); // that's a punctuation, not a code
        assert!(is_z52_code_punctuation('\u{184f}'));
        assert!(is_z52_code_punctuation('\u{1863}'));
        assert!(!is_z52_code_punctuation('\u{1864}'));
    }
}
