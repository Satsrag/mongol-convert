//! Mongolian Unicode block predicates. Port of `word/MglUnicodeBlock.java`.
//!
//! All ranges/sets are transcribed verbatim from the Java source (the authoritative oracle).
//! Note the Java/PHP divergence captured by tests: `is_giiguulegch` upper bound is **U+1842**
//! in Java (PHP stops at U+183F); Java wins.

use crate::word::nature::Nature;

pub const MONGOLIAN_A: char = '\u{1820}';
pub const MONGOLIAN_ZRA: char = '\u{183F}';
pub const FREE_VARIATION_SELECTOR_ONE: char = '\u{180B}';
pub const FREE_VARIATION_SELECTOR_THREE: char = '\u{180D}';
pub const VOWEL_SEPARATOR: char = '\u{180E}';
pub const NIRUGU: char = '\u{180A}';

// chagh = {1820, 1823, 1824}
#[inline]
pub fn is_chagh(ch: char) -> bool {
    matches!(ch, '\u{1820}' | '\u{1823}' | '\u{1824}')
}

// hundii = {1821, 1825, 1826}
#[inline]
fn is_hundii(ch: char) -> bool {
    matches!(ch, '\u{1821}' | '\u{1825}' | '\u{1826}')
}

// ehshig = {1820, 1821, 1822, 1823, 1824, 1825, 1826}
#[inline]
fn in_ehshig_set(ch: char) -> bool {
    matches!(
        ch,
        '\u{1820}' | '\u{1821}' | '\u{1822}' | '\u{1823}' | '\u{1824}' | '\u{1825}' | '\u{1826}'
    )
}

#[inline]
pub fn is_normal_letter(ch: char) -> bool {
    MONGOLIAN_A <= ch && ch <= MONGOLIAN_ZRA
}

#[inline]
pub fn is_free_variation_selector(ch: char) -> bool {
    FREE_VARIATION_SELECTOR_ONE <= ch && ch <= FREE_VARIATION_SELECTOR_THREE
}

#[inline]
pub fn is_vowel_separator(ch: char) -> bool {
    ch == VOWEL_SEPARATOR
}

pub fn get_code_nature(ch: char) -> Nature {
    if is_chagh(ch) {
        Nature::Chagh
    } else if is_hundii(ch) {
        Nature::Hundii
    } else {
        Nature::Saarmag
    }
}

/// `isEhshig`: the ehshig set, plus U+1827. (Java takes a nullable `Character`; callers using
/// `Option<char>` should treat `None` as false, matching the Java null guard.)
#[inline]
pub fn is_ehshig(ch: char) -> bool {
    in_ehshig_set(ch) || ch == '\u{1827}'
}

/// `isTraditionalEhshig`: the ehshig set only (excludes U+1827).
#[inline]
pub fn is_traditional_ehshig(ch: char) -> bool {
    in_ehshig_set(ch)
}

/// `isGiiguulegch`: U+1828..=U+1842. (Java upper bound; PHP's U+183F is the wrong one.)
#[inline]
pub fn is_giiguulegch(ch: char) -> bool {
    ch >= '\u{1828}' && ch <= '\u{1842}'
}

#[inline]
pub fn other_mongolian_code(ch: char) -> bool {
    ch == NIRUGU
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_letter_range() {
        assert!(is_normal_letter('\u{1820}'));
        assert!(is_normal_letter('\u{183F}'));
        assert!(!is_normal_letter('\u{181F}'));
        assert!(!is_normal_letter('\u{1840}'));
    }

    #[test]
    fn fvs_and_vs() {
        assert!(is_free_variation_selector('\u{180B}'));
        assert!(is_free_variation_selector('\u{180D}'));
        assert!(!is_free_variation_selector('\u{180E}'));
        assert!(is_vowel_separator('\u{180E}'));
        assert!(other_mongolian_code('\u{180A}'));
    }

    #[test]
    fn nature_classification() {
        assert_eq!(get_code_nature('\u{1820}'), Nature::Chagh);
        assert_eq!(get_code_nature('\u{1823}'), Nature::Chagh);
        assert_eq!(get_code_nature('\u{1821}'), Nature::Hundii);
        assert_eq!(get_code_nature('\u{1826}'), Nature::Hundii);
        assert_eq!(get_code_nature('\u{1822}'), Nature::Saarmag);
    }

    #[test]
    fn ehshig_includes_1827_traditional_does_not() {
        assert!(is_ehshig('\u{1820}'));
        assert!(is_ehshig('\u{1827}'));
        assert!(is_traditional_ehshig('\u{1820}'));
        assert!(!is_traditional_ehshig('\u{1827}'));
    }

    #[test]
    fn giiguulegch_upper_bound_is_1842_java_not_183f() {
        assert!(is_giiguulegch('\u{1828}'));
        assert!(is_giiguulegch('\u{1842}')); // Java includes this; PHP wrongly stops at 183F
        assert!(!is_giiguulegch('\u{1827}'));
        assert!(!is_giiguulegch('\u{1843}'));
    }
}
