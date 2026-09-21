//! String helpers. Port of `common/Strings.java`.
//!
//! **Parity warning (Java vs Rust):** Java `isBlank` is `s == null || s.trim().isEmpty()`, and
//! Java `String.trim()` strips only characters `<= U+0020`. Rust's `str::trim()` strips the full
//! Unicode `White_Space` set (e.g. NBSP U+00A0, U+2000–U+200A), so it is **not** equivalent.
//! We therefore reimplement the Java semantics directly: blank iff every char is `<= U+0020`.
//! This is load-bearing for the top-level `translate` short-circuit.

/// Java `Strings.isBlank`: empty, or every character is `<= U+0020`. (NBSP U+00A0 is **not** blank.)
pub fn is_blank(s: &str) -> bool {
    s.chars().all(|c| (c as u32) <= 0x20)
}

/// Java `Strings.isEmpty(CharSequence)`: length zero.
pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

/// Java `Strings.endOf(String, char)`: last char equals `c`; false if blank.
pub fn end_of_char(s: &str, c: char) -> bool {
    if is_blank(s) {
        return false;
    }
    s.chars().next_back() == Some(c)
}

/// Java `Strings.endOf(String, String)`: `s0` ends with `s1`; false if either is blank.
pub fn end_of_str(s0: &str, s1: &str) -> bool {
    if is_blank(s0) || is_blank(s1) {
        return false;
    }
    s0.ends_with(s1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_matches_java_trim_semantics() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(is_blank("\t\n\r "));
        assert!(!is_blank(" a "));
        assert!(!is_blank("x"));
        // The key divergence from Rust's str::trim(): NBSP (U+00A0 > U+0020) is NOT blank in Java.
        assert!(!is_blank("\u{00A0}"));
        assert!(!is_blank("\u{2003}")); // EM SPACE, also > U+0020
    }

    #[test]
    fn end_of_helpers() {
        assert!(end_of_char("abc", 'c'));
        assert!(!end_of_char("abc", 'b'));
        assert!(!end_of_char("   ", ' '));
        assert!(end_of_str("hello", "lo"));
        assert!(!end_of_str("hello", "he"));
        assert!(!end_of_str("hi", "")); // blank s1 -> false (Java parity)
    }
}
