//! Zvvnmod (intermediate PUA) Unicode block predicates. Port of `word/ZvvnModUnicodeBlock.java`,
//! backed by the generated membership sets (`tables::zvvnmod_block`).

use crate::tables::zvvnmod_birga::{MENK_SHAPE_BIRGAS, ZVVNMOD_BIRGAS};
use crate::tables::zvvnmod_block::{
    TO_Z52_PUNCTUATIONS, ZVVNMOD_CODES, ZVVNMOD_PUNCTUATIONS, ZVVNMOD_TAIL_CODES,
};
use crate::unicode::sorted_contains;

/// Java `ZvvnModUnicodeBlock.zvvnModCodes.contains`.
#[inline]
pub fn is_zvvnmod_code(c: char) -> bool {
    sorted_contains(ZVVNMOD_CODES, c)
}

/// Java `ZvvnModUnicodeBlock.zvvnModTailCodes.contains` — load-bearing for `resloveTsatslaga`.
#[inline]
pub fn is_zvvnmod_tail_code(c: char) -> bool {
    sorted_contains(ZVVNMOD_TAIL_CODES, c)
}

/// Java `ZvvnModUnicodeBlock.zvvnModPunctuations.contains`.
#[inline]
pub fn is_zvvnmod_punctuation(c: char) -> bool {
    if sorted_contains(&ZVVNMOD_BIRGAS, c) {
        return true;
    }
    if sorted_contains(&MENK_SHAPE_BIRGAS, c) {
        return false;
    }
    sorted_contains(ZVVNMOD_PUNCTUATIONS, c)
}

/// Java `ZvvnModUnicodeBlock.toZ52Punctuations.contains` — only 4 active entries.
#[inline]
pub fn is_to_z52_punctuation(c: char) -> bool {
    sorted_contains(TO_Z52_PUNCTUATIONS, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn membership() {
        assert!(is_zvvnmod_code('\u{e000}'));
        assert!(is_zvvnmod_code('\u{e144}'));
        assert!(is_zvvnmod_tail_code('\u{e00c}'));
        assert!(is_zvvnmod_tail_code('\u{e0d0}'));
        assert!(!is_zvvnmod_tail_code('\u{e000}')); // a code, but not a tail code
        assert!(is_zvvnmod_punctuation('\u{1800}'));
        // toZ52Punctuations: exactly {2048, 2049, 0021, 003f}
        assert!(is_to_z52_punctuation('\u{2048}'));
        assert!(is_to_z52_punctuation('!'));
        assert!(is_to_z52_punctuation('?'));
        assert!(!is_to_z52_punctuation('\u{00b7}'));
    }

    #[test]
    fn zvvnmod_birgas_replace_menk_shape_pua() {
        for c in '\u{11660}'..='\u{11663}' {
            assert!(is_zvvnmod_punctuation(c));
        }
        for c in '\u{e23f}'..='\u{e242}' {
            assert!(!is_zvvnmod_punctuation(c));
        }
    }
}
